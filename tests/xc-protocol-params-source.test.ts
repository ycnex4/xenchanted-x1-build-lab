import { describe, expect, it } from "vitest";
import {
  createXcProtocolParamsSourceFromEthereumReadProvider,
  normalizeXcProtocolParams
} from "../src/index.js";
import type { XcProtocolParamsReadProvider } from "../src/index.js";

const LENS_ADDRESS = "0xd4B90d7392c1565D558c80122DEE76b5b3bB6C23";

const OBJECT_RESULT = {
  genesisTs: 1780166915n,
  halvingInterval: 15552000n,
  xenBurnHalvingInterval: 31104000n,
  currentEpoch: 0n,
  nextHalvingTs: 1795718915n,
  initialNominal: 100000000000000000000n,
  currentBaseNominal: 100000000000000000000n,
  initialXenBurn: 100000000000000000000000000n,
  currentXenBurnAmount: 100000000000000000000000000n,
  enchantMultiplier: 3n,
  maxLevel: 22,
  baseAprBpsNow: 1000,
  bpsDenom: 10000n,
  earlyPenaltyBps: 100n,
  maxWalletNfts: 60n
};

const ARRAY_RESULT = [
  1780166915n,
  15552000n,
  31104000n,
  0n,
  1795718915n,
  100000000000000000000n,
  100000000000000000000n,
  100000000000000000000000000n,
  100000000000000000000000000n,
  3n,
  22n,
  1000n,
  10000n,
  100n,
  60n
];

function createProvider(result: unknown): {
  readonly provider: XcProtocolParamsReadProvider;
  readonly calls: unknown[];
} {
  const calls: unknown[] = [];

  return {
    calls,
    provider: {
      async readContract(input) {
        calls.push(input);
        return result;
      }
    }
  };
}

describe("XC protocol params source", () => {
  it("reads getProtocolParams through readContract", async () => {
    const { provider, calls } = createProvider(OBJECT_RESULT);
    const source = createXcProtocolParamsSourceFromEthereumReadProvider({
      provider,
      lensAddress: LENS_ADDRESS
    });

    const params = await source.readProtocolParams();

    expect(params.currentBaseNominal).toBe(100000000000000000000n);
    expect(params.currentXenBurnAmount).toBe(100000000000000000000000000n);
    expect(params.maxLevel).toBe(22);
    expect(params.baseAprBpsNow).toBe(1000);
    expect(calls).toHaveLength(1);
    expect(calls[0]).toMatchObject({
      address: LENS_ADDRESS,
      functionName: "getProtocolParams",
      args: []
    });
  });

  it("uses a minimal getProtocolParams ABI fragment", async () => {
    const { provider, calls } = createProvider(OBJECT_RESULT);
    const source = createXcProtocolParamsSourceFromEthereumReadProvider({
      provider,
      lensAddress: LENS_ADDRESS
    });

    await source.readProtocolParams();

    const call = calls[0] as { readonly abi: readonly unknown[] };
    expect(call.abi).toHaveLength(1);
    expect(call.abi[0]).toMatchObject({
      name: "getProtocolParams",
      type: "function",
      stateMutability: "view"
    });
  });

  it("normalizes object-like tuple returns", () => {
    expect(normalizeXcProtocolParams(OBJECT_RESULT)).toEqual(OBJECT_RESULT);
  });

  it("normalizes array-like tuple returns", () => {
    expect(normalizeXcProtocolParams(ARRAY_RESULT)).toEqual(OBJECT_RESULT);
  });

  it("normalizes numeric strings and small numbers", () => {
    const result = {
      ...OBJECT_RESULT,
      currentEpoch: "0",
      maxLevel: "22",
      baseAprBpsNow: 1000
    };

    expect(normalizeXcProtocolParams(result)).toMatchObject({
      currentEpoch: 0n,
      maxLevel: 22,
      baseAprBpsNow: 1000
    });
  });

  it("rejects invalid lens address", () => {
    expect(() =>
      createXcProtocolParamsSourceFromEthereumReadProvider({
        provider: createProvider(OBJECT_RESULT).provider,
        lensAddress: "not-an-address"
      })
    ).toThrow("Invalid XC protocol params source config: lensAddress");
  });

  it("rejects missing tuple fields with sanitized error", () => {
    const { currentBaseNominal: _removed, ...missing } = OBJECT_RESULT;

    expect(() => normalizeXcProtocolParams(missing)).toThrow(
      "Invalid XC protocol params result: missing currentBaseNominal"
    );
  });

  it("rejects malformed tuple shape with sanitized error", () => {
    expect(() => normalizeXcProtocolParams(null)).toThrow(
      "Invalid XC protocol params result: malformed tuple"
    );
  });

  it("rejects invalid number fields with sanitized error", () => {
    expect(() =>
      normalizeXcProtocolParams({
        ...OBJECT_RESULT,
        maxLevel: Number.MAX_SAFE_INTEGER + 1
      })
    ).toThrow("Invalid XC protocol params result: invalid maxLevel");
  });

  it("wraps provider read errors with sanitized error", async () => {
    const provider: XcProtocolParamsReadProvider = {
      async readContract() {
        throw new Error("provider error with hidden internals");
      }
    };

    const source = createXcProtocolParamsSourceFromEthereumReadProvider({
      provider,
      lensAddress: LENS_ADDRESS
    });

    await expect(source.readProtocolParams()).rejects.toThrow(
      "Failed to read XC protocol params"
    );
  });
});

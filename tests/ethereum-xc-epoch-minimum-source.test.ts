import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  type EthereumXcLensEpochMinimumSnapshot,
  createXcEpochMinimumSourceFromEthereumLensSnapshot
} from "../src/index.js";

const VALID_BLOCK_HASH =
  "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

function validSnapshot(
  overrides: Partial<EthereumXcLensEpochMinimumSnapshot> = {}
): EthereumXcLensEpochMinimumSnapshot {
  return {
    sourceChainId: "eip155-1",
    sourceBlockNumber: 100n,
    sourceBlockHash: VALID_BLOCK_HASH,
    observedAt: 1000n,
    finalizedPolicy: { kind: "finalized" },
    epochMinimums: [
      {
        lockEpoch: 0,
        minimumXntd: 100n
      },
      {
        lockEpoch: 1,
        minimumXntd: 50n
      }
    ],
    ...overrides
  };
}

function expectInvalidSnapshot(
  snapshot: EthereumXcLensEpochMinimumSnapshot
): void {
  expect(() =>
    createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot)
  ).toThrow(BuildError);

  try {
    createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot);
  } catch (error) {
    expect(error).toBeInstanceOf(BuildError);
    expect((error as BuildError).code).toBe(
      BuildErrorCode.InvalidXcEpochMinimumRecord
    );
  }
}

describe("Ethereum XC epoch minimum source", () => {
  it("builds source from valid mocked Ethereum Lens snapshot", () => {
    const source = createXcEpochMinimumSourceFromEthereumLensSnapshot(
      validSnapshot()
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(source.authoritativeEpochMinimum(1)).toBe(50n);
  });

  it("normalizes mixed-case Ethereum source block hash", () => {
    const source = createXcEpochMinimumSourceFromEthereumLensSnapshot(
      validSnapshot({
        sourceBlockHash:
          "0xAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAa"
      })
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
  });

  it("rejects missing or empty Ethereum source chain ID", () => {
    expectInvalidSnapshot(validSnapshot({ sourceChainId: "" }));
  });

  it("rejects non-EIP-155 Ethereum source chain ID", () => {
    expectInvalidSnapshot(validSnapshot({ sourceChainId: "1" }));
    expectInvalidSnapshot(validSnapshot({ sourceChainId: "ethereum-mainnet" }));
  });

  it("rejects non-positive Ethereum source block number", () => {
    expectInvalidSnapshot(validSnapshot({ sourceBlockNumber: 0n }));
  });

  it("rejects missing or invalid Ethereum source block hash", () => {
    expectInvalidSnapshot(validSnapshot({ sourceBlockHash: "" }));
    expectInvalidSnapshot(validSnapshot({ sourceBlockHash: "0xblock" }));
    expectInvalidSnapshot(
      validSnapshot({
        sourceBlockHash:
          "0xgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"
      })
    );
  });

  it("rejects non-positive Ethereum snapshot observedAt", () => {
    expectInvalidSnapshot(validSnapshot({ observedAt: 0n }));
  });

  it("accepts safe Ethereum finality policy", () => {
    const source = createXcEpochMinimumSourceFromEthereumLensSnapshot(
      validSnapshot({ finalizedPolicy: { kind: "safe" } })
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
  });

  it("accepts confirmed Ethereum finality policy with positive confirmations", () => {
    const source = createXcEpochMinimumSourceFromEthereumLensSnapshot(
      validSnapshot({ finalizedPolicy: { kind: "confirmed", confirmations: 12 } })
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
  });

  it("rejects invalid Ethereum finality policy", () => {
    expectInvalidSnapshot(
      validSnapshot({
        finalizedPolicy: { kind: "latest" } as unknown as EthereumXcLensEpochMinimumSnapshot[
          "finalizedPolicy"
        ]
      })
    );
  });

  it("rejects confirmed Ethereum finality policy without positive confirmations", () => {
    expectInvalidSnapshot(
      validSnapshot({ finalizedPolicy: { kind: "confirmed", confirmations: 0 } })
    );
  });

  it("rejects empty Ethereum epoch minimum entries", () => {
    expectInvalidSnapshot(validSnapshot({ epochMinimums: [] }));
  });

  it("rejects conflicting duplicate Ethereum epoch minimum entries", () => {
    expectInvalidSnapshot(
      validSnapshot({
        epochMinimums: [
          {
            lockEpoch: 1,
            minimumXntd: 50n
          },
          {
            lockEpoch: 1,
            minimumXntd: 40n
          }
        ]
      })
    );
  });

  it("returns null for missing epoch", () => {
    const source = createXcEpochMinimumSourceFromEthereumLensSnapshot(
      validSnapshot()
    );

    expect(source.authoritativeEpochMinimum(2)).toBeNull();
  });
});

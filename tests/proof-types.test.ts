import { describe, expect, it } from "vitest";
import {
  type BuildProof,
  type CoreRedeemProof,
  assertValidatedProof,
  createCanonicalEventKey,
  createProofSourceMetadata,
  isValidatedProof
} from "../src/index.js";

describe("proof object types", () => {
  it("creates deterministic canonical event keys", () => {
    const key = createCanonicalEventKey({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx",
      eventIndex: 0
    });

    expect(key).toBe("eip155-1:0xcore:CORE_REDEEM:0xtx:0");
  });

  it("rejects invalid canonical event key inputs", () => {
    expect(() =>
      createCanonicalEventKey({
        sourceChainId: "",
        sourceAddress: "0xcore",
        eventKind: "CORE_REDEEM",
        transactionHash: "0xtx",
        eventIndex: 0
      })
    ).toThrow("sourceChainId must not be empty");

    expect(() =>
      createCanonicalEventKey({
        sourceChainId: "eip155-1",
        sourceAddress: "0xcore",
        eventKind: "CORE:REDEEM",
        transactionHash: "0xtx",
        eventIndex: 0
      })
    ).toThrow("eventKind must not contain ':'");

    expect(() =>
      createCanonicalEventKey({
        sourceChainId: "eip155-1",
        sourceAddress: "0xcore",
        eventKind: "CORE_REDEEM",
        transactionHash: "0xtx",
        eventIndex: -1
      })
    ).toThrow("eventIndex must be a non-negative integer");
  });

  it("creates proof source metadata defaults", () => {
    const source = createProofSourceMetadata({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx",
      eventIndex: 0
    });

    expect(source).toEqual({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      transactionHash: "0xtx",
      eventIndex: 0,
      blockNumber: null,
      slot: null,
      observedAt: null,
      finalized: false
    });
  });

  it("models a validated Core redeem proof", () => {
    const canonicalEventKey = createCanonicalEventKey({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx",
      eventIndex: 0
    });

    const proof: CoreRedeemProof = {
      kind: "CORE_REDEEM_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata({
        sourceChainId: "eip155-1",
        sourceAddress: "0xcore",
        eventKind: "CORE_REDEEM",
        transactionHash: "0xtx",
        eventIndex: 0,
        blockNumber: 123n,
        observedAt: 1000n,
        finalized: true
      }),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        redeemKey: canonicalEventKey,
        amountBld: 121n,
        redeemedAt: 1000n,
        coreTokenId: "1"
      }
    };

    expect(isValidatedProof(proof)).toBe(true);
    expect(() => assertValidatedProof(proof)).not.toThrow();
    expect(proof.payload.amountBld).toBe(121n);
  });

  it("rejects non-validated proofs through assertion helper", () => {
    const proof: BuildProof = {
      kind: "XEN_BURN_PROOF",
      status: "CANDIDATE",
      source: createProofSourceMetadata({
        sourceChainId: "eip155-1",
        sourceAddress: "0xxen",
        eventKind: "XEN_BURN",
        transactionHash: "0xtx",
        eventIndex: 0
      }),
      canonicalEventKey: "eip155-1:0xxen:XEN_BURN:0xtx:0",
      validatedAt: null,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        xenBurnKey: "eip155-1:0xxen:XEN_BURN:0xtx:0",
        amountXbp: 1000n,
        burnedAt: 1000n,
        xenAmountBurned: 100000000n
      }
    };

    expect(isValidatedProof(proof)).toBe(false);
    expect(() => assertValidatedProof(proof)).toThrow(
      "Proof is not validated: XEN_BURN_PROOF"
    );
  });
});

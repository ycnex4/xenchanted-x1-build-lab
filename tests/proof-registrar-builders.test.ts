import { describe, expect, it } from "vitest";
import {
  type BuildProof,
  type CoreRedeemProof,
  type GenesisOriginEligibilityProof,
  buildCoreRedeemRegistrarPayload,
  buildRegistrarPayloadFromProof,
  buildX1FeeCheckpointRegistrarPayload,
  buildXenBurnRegistrarPayload,
  buildXntdLockRegistrarPayload,
  buildXntdRelockRegistrarPayload,
  createCanonicalEventKey,
  createProofSourceMetadata
} from "../src/index.js";

function sourceInput(eventKind: string) {
  return {
    sourceChainId: "eip155-1",
    sourceAddress: "0xsource",
    eventKind,
    transactionHash: "0xtx",
    eventIndex: 0,
    blockNumber: 123n,
    observedAt: 1000n,
    finalized: true
  };
}

describe("proof to registrar payload builders", () => {
  it("builds Core redeem registrar payload from validated proof", () => {
    const canonicalEventKey = createCanonicalEventKey(sourceInput("CORE_REDEEM"));

    const proof: CoreRedeemProof = {
      kind: "CORE_REDEEM_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("CORE_REDEEM")),
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

    const payload = buildCoreRedeemRegistrarPayload(proof, {
      submittedBy: "registrar-1",
      createdAt: 1200n
    });

    expect(payload).toEqual({
      message: {
        messageId: `proof:CORE_REDEEM_PROOF:${canonicalEventKey}`,
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1200n
      },
      buildId: "build-1",
      redeemKey: canonicalEventKey,
      amountBld: 121n,
      redeemedAt: 1000n
    });
  });

  it("builds XEN burn registrar payload from validated proof", () => {
    const canonicalEventKey = createCanonicalEventKey(sourceInput("XEN_BURN"));

    const proof: BuildProof = {
      kind: "XEN_BURN_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("XEN_BURN")),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        xenBurnKey: canonicalEventKey,
        amountXbp: 1000n,
        burnedAt: 1000n,
        xenAmountBurned: 100000000n
      }
    };

    const payload = buildXenBurnRegistrarPayload(proof, {
      submittedBy: "registrar-1",
      createdAt: 1200n,
      messageId: "custom-message-id"
    });

    expect(payload.message).toEqual({
      messageId: "custom-message-id",
      kind: "XEN_BURN",
      submittedBy: "registrar-1",
      createdAt: 1200n
    });
    expect(payload.buildId).toBe("build-1");
    expect(payload.xenBurnKey).toBe(canonicalEventKey);
    expect(payload.amountXbp).toBe(1000n);
    expect(payload.burnedAt).toBe(1000n);
  });

  it("builds XNTD lock and relock registrar payloads", () => {
    const lockKey = createCanonicalEventKey(sourceInput("XNTD_LOCK"));
    const relockKey = createCanonicalEventKey({
      ...sourceInput("XNTD_RELOCK"),
      transactionHash: "0xtx2"
    });

    const lockProof: BuildProof = {
      kind: "XNTD_LOCK_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("XNTD_LOCK")),
      canonicalEventKey: lockKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        amountXntd: 750n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 1,
        lockedAt: 1000n
      }
    };

    const relockProof: BuildProof = {
      kind: "XNTD_RELOCK_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata({
        ...sourceInput("XNTD_RELOCK"),
        transactionHash: "0xtx2"
      }),
      canonicalEventKey: relockKey,
      validatedAt: 1200n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        amountXntd: 400n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      }
    };

    const lockPayload = buildXntdLockRegistrarPayload(lockProof, {
      submittedBy: "registrar-1",
      createdAt: 1300n
    });

    const relockPayload = buildXntdRelockRegistrarPayload(relockProof, {
      submittedBy: "registrar-1",
      createdAt: 1400n
    });

    expect(lockPayload.message.kind).toBe("LOCK_XNTD");
    expect(lockPayload.amountXntd).toBe(750n);
    expect(lockPayload.observedRequiredXntdLock).toBe(500n);
    expect(lockPayload.lockEpoch).toBe(1);

    expect(relockPayload.message.kind).toBe("RELOCK_XNTD");
    expect(relockPayload.amountXntd).toBe(400n);
    expect(relockPayload.observedRequiredXntdLock).toBe(250n);
    expect(relockPayload.lockEpoch).toBe(2);
  });

  it("builds X1 fee checkpoint registrar payload from validated proof", () => {
    const canonicalEventKey = createCanonicalEventKey(
      sourceInput("X1_FEE_CHECKPOINT")
    );

    const proof: BuildProof = {
      kind: "X1_FEE_CHECKPOINT_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("X1_FEE_CHECKPOINT")),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        feeAmount: 777n,
        txCount: 11n,
        countedUntilSlot: 9000n,
        updatedAt: 1000n
      }
    };

    const payload = buildX1FeeCheckpointRegistrarPayload(proof, {
      submittedBy: "registrar-1",
      createdAt: 1200n
    });

    expect(payload.message.kind).toBe("X1_FEE_CHECKPOINT");
    expect(payload.feeAmount).toBe(777n);
    expect(payload.txCount).toBe(11n);
    expect(payload.countedUntilSlot).toBe(9000n);
  });

  it("rejects non-validated proof before building registrar payload", () => {
    const canonicalEventKey = createCanonicalEventKey(sourceInput("CORE_REDEEM"));

    const proof: CoreRedeemProof = {
      kind: "CORE_REDEEM_PROOF",
      status: "CANDIDATE",
      source: createProofSourceMetadata(sourceInput("CORE_REDEEM")),
      canonicalEventKey,
      validatedAt: null,
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

    expect(() =>
      buildCoreRedeemRegistrarPayload(proof, {
        submittedBy: "registrar-1",
        createdAt: 1200n
      })
    ).toThrow("Proof is not validated: CORE_REDEEM_PROOF");
  });

  it("rejects Genesis Origin proof through generic registrar payload builder", () => {
    const canonicalEventKey = createCanonicalEventKey(
      sourceInput("GENESIS_ORIGIN")
    );

    const proof: GenesisOriginEligibilityProof = {
      kind: "GENESIS_ORIGIN_ELIGIBILITY_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("GENESIS_ORIGIN")),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        historyBld: 121n,
        eligibleOriginBld: 55n,
        snapshotId: "snapshot-1",
        claimedAt: 1000n
      }
    };

    expect(() =>
      buildRegistrarPayloadFromProof(proof, {
        submittedBy: "registrar-1",
        createdAt: 1200n
      })
    ).toThrow("Genesis Origin proof does not map to a registrar payload");
  });

  it("builds payload through generic registrar payload builder", () => {
    const canonicalEventKey = createCanonicalEventKey(sourceInput("XEN_BURN"));

    const proof: BuildProof = {
      kind: "XEN_BURN_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata(sourceInput("XEN_BURN")),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: "build-1",
        owner: "x1-owner",
        xenBurnKey: canonicalEventKey,
        amountXbp: 1000n,
        burnedAt: 1000n,
        xenAmountBurned: 100000000n
      }
    };

    const payload = buildRegistrarPayloadFromProof(proof, {
      submittedBy: "registrar-1",
      createdAt: 1200n
    });

    expect(payload.message.kind).toBe("XEN_BURN");
    expect(payload.buildId).toBe("build-1");
  });
});

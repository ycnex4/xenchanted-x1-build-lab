import { describe, expect, it } from "vitest";
import {
  buildRegistrarPayloadFromProof,
  convertCoreRedeemCandidateToProof,
  convertWatcherCandidateToProof,
  convertX1FeeCheckpointCandidateToProof,
  convertXenBurnCandidateToProof,
  convertXntdLockCandidateToProof,
  convertXntdRelockCandidateToProof,
  createCoreRedeemCandidate,
  createX1FeeCheckpointCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
  createXntdRelockCandidate
} from "../src/index.js";

describe("watcher candidate to proof conversion", () => {
  it("converts finalized Core redeem candidate into validated proof", () => {
    const candidate = createCoreRedeemCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx",
      eventIndex: 0,
      blockNumber: 123n,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountBld: 121n,
      redeemedAt: 1000n,
      coreTokenId: "1"
    });

    const proof = convertCoreRedeemCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    expect(proof.kind).toBe("CORE_REDEEM_PROOF");
    expect(proof.status).toBe("VALIDATED");
    expect(proof.canonicalEventKey).toBe(candidate.canonicalEventKey);
    expect(proof.validatedAt).toBe(1100n);
    expect(proof.rejectionReason).toBe(null);
    expect(proof.payload.redeemKey).toBe(candidate.canonicalEventKey);
    expect(proof.payload.amountBld).toBe(121n);
  });

  it("converts finalized XEN burn candidate into validated proof", () => {
    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx",
      eventIndex: 1,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    const proof = convertXenBurnCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    expect(proof.kind).toBe("XEN_BURN_PROOF");
    expect(proof.status).toBe("VALIDATED");
    expect(proof.payload.xenBurnKey).toBe(candidate.canonicalEventKey);
    expect(proof.payload.amountXbp).toBe(1000n);
    expect(proof.payload.xenAmountBurned).toBe(100000000n);
  });

  it("converts finalized XNTD lock and relock candidates into validated proofs", () => {
    const lockCandidate = createXntdLockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_LOCK",
      transactionHash: "tx-lock",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1000n
    });

    const relockCandidate = createXntdRelockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_RELOCK",
      transactionHash: "tx-relock",
      eventIndex: 0,
      observedAt: 1100n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountXntd: 250n,
      lockEpoch: 2,
      relockedAt: 1100n
    });

    const lockProof = convertXntdLockCandidateToProof(lockCandidate, {
      validatedAt: 1200n
    });

    const relockProof = convertXntdRelockCandidateToProof(relockCandidate, {
      validatedAt: 1300n
    });

    expect(lockProof.kind).toBe("XNTD_LOCK_PROOF");
    expect(lockProof.payload.amountXntd).toBe(500n);
    expect(lockProof.payload.lockEpoch).toBe(1);

    expect(relockProof.kind).toBe("XNTD_RELOCK_PROOF");
    expect(relockProof.payload.amountXntd).toBe(250n);
    expect(relockProof.payload.lockEpoch).toBe(2);
  });

  it("converts finalized X1 fee checkpoint candidate into validated proof", () => {
    const candidate = createX1FeeCheckpointCandidate({
      sourceChainId: "x1",
      sourceAddress: "fee-indexer",
      eventKind: "X1_FEE_CHECKPOINT",
      transactionHash: "checkpoint-1",
      eventIndex: 0,
      slot: 9000n,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 1000n
    });

    const proof = convertX1FeeCheckpointCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    expect(proof.kind).toBe("X1_FEE_CHECKPOINT_PROOF");
    expect(proof.payload.feeAmount).toBe(777n);
    expect(proof.payload.txCount).toBe(11n);
    expect(proof.payload.countedUntilSlot).toBe(9000n);
  });

  it("routes generic watcher candidate conversion", () => {
    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    expect(proof.kind).toBe("XEN_BURN_PROOF");
    expect(proof.canonicalEventKey).toBe(candidate.canonicalEventKey);
  });

  it("rejects non-finalized candidates before proof conversion", () => {
    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: false,
      buildId: "build-1",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    expect(() =>
      convertWatcherCandidateToProof(candidate, {
        validatedAt: 1100n
      })
    ).toThrow("Watcher candidate is not finalized: XEN_BURN_CANDIDATE");
  });

  it("supports watcher candidate to proof to registrar payload pipeline", () => {
    const candidate = createCoreRedeemCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: "build-1",
      owner: "x1-owner",
      amountBld: 121n,
      redeemedAt: 1000n,
      coreTokenId: "1"
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const registrarPayload = buildRegistrarPayloadFromProof(proof, {
      submittedBy: "registrar-1",
      createdAt: 1200n
    });

    expect(registrarPayload.message.kind).toBe("CORE_REDEEM");
    expect(registrarPayload.message.messageId).toBe(
      `proof:CORE_REDEEM_PROOF:${candidate.canonicalEventKey}`
    );
    expect(registrarPayload.buildId).toBe("build-1");
  });
});

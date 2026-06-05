import { describe, expect, it } from "vitest";
import {
  appCreateBuild,
  appSubmitProof,
  createBuildApplicationState,
  createCoreRedeemCandidate,
  createX1FeeCheckpointCandidate,
  createXenBurnCandidate,
  createStaticXcEpochMinimumSource,
  createXntdLockCandidate,
  createXntdRelockCandidate,
  convertWatcherCandidateToProof
} from "../src/index.js";

function submitProofFromWatcherCandidate(
  app: ReturnType<typeof createBuildApplicationState>,
  candidate: Parameters<typeof convertWatcherCandidateToProof>[0],
  createdAt: bigint,
  extraInput: Partial<Parameters<typeof appSubmitProof>[2]> = {}
) {
  const proof = convertWatcherCandidateToProof(candidate, {
    validatedAt: createdAt - 1n
  });

  return appSubmitProof(app, proof, {
    submittedBy: "registrar-1",
    createdAt,
    ...extraInput
  });
}

describe("end-to-end watcher proof registrar scenario", () => {
  it("runs watcher candidates through proof conversion and registrar application submission", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-user-1",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 1000n
    });

    expect(created.ok).toBe(true);

    if (!created.ok) {
      throw new Error("Build creation failed");
    }

    const build = created.value;
    const xcEpochMinimumSource = createStaticXcEpochMinimumSource(
      new Map<number, bigint>([
        [1, 500n],
        [2, 250n]
      ])
    );

    const coreRedeemCandidate = createCoreRedeemCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx-core-redeem",
      eventIndex: 0,
      blockNumber: 101n,
      observedAt: 1100n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountBld: 121n,
      redeemedAt: 1100n,
      coreTokenId: "1"
    });

    const coreRedeem = submitProofFromWatcherCandidate(
      app,
      coreRedeemCandidate,
      1110n
    );

    expect(coreRedeem.ok).toBe(true);

    const xenBurnCandidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx-xen-burn",
      eventIndex: 0,
      blockNumber: 102n,
      observedAt: 1200n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXbp: 1000n,
      burnedAt: 1200n,
      xenAmountBurned: 100000000n
    });

    const xenBurn = submitProofFromWatcherCandidate(
      app,
      xenBurnCandidate,
      1210n
    );

    expect(xenBurn.ok).toBe(true);

    const lockCandidate = createXntdLockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_LOCK",
      transactionHash: "tx-lock",
      eventIndex: 0,
      slot: 2000n,
      observedAt: 1300n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1300n
    });

    const lock = submitProofFromWatcherCandidate(app, lockCandidate, 1310n, {
      xcEpochMinimumSource
    });

    expect(lock.ok).toBe(true);

    const relockCandidate = createXntdRelockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_RELOCK",
      transactionHash: "tx-relock",
      eventIndex: 0,
      slot: 3000n,
      observedAt: 1400n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1400n
    });

    const relock = submitProofFromWatcherCandidate(
      app,
      relockCandidate,
      1410n,
      { xcEpochMinimumSource }
    );

    expect(relock.ok).toBe(true);

    const feeCandidate = createX1FeeCheckpointCandidate({
      sourceChainId: "x1",
      sourceAddress: "fee-indexer",
      eventKind: "X1_FEE_CHECKPOINT",
      transactionHash: "checkpoint-1",
      eventIndex: 0,
      slot: 9000n,
      observedAt: 1500n,
      finalized: true,
      buildId: build.buildId,
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 1500n
    });

    const fee = submitProofFromWatcherCandidate(app, feeCandidate, 1510n);

    expect(fee.ok).toBe(true);

    expect(build.historyBld).toBe(121n);
    expect(build.availableBld).toBe(121n);

    expect(build.earnedXbp).toBe(1000n);
    expect(build.availableXbp).toBe(1000n);

    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xcCommitmentActive).toBe(true);

    expect(build.x1FeeContribution).toBe(777n);
    expect(build.x1TxCount).toBe(11n);
    expect(build.x1FeeCountedUntilSlot).toBe(9000n);
    expect(build.lastFeeUpdateAt).toBe(1500n);

    expect(app.registrar.processedMessages.size).toBe(5);
    expect(app.redeemEvents.usedRedeemEvents.has(
      coreRedeemCandidate.canonicalEventKey
    )).toBe(true);
    expect(app.xenBurnEvents.usedXenBurnEvents.has(
      xenBurnCandidate.canonicalEventKey
    )).toBe(true);

    const duplicateCoreRedeem = submitProofFromWatcherCandidate(
      app,
      coreRedeemCandidate,
      1600n
    );

    expect(duplicateCoreRedeem.ok).toBe(false);

    if (!duplicateCoreRedeem.ok) {
      expect(duplicateCoreRedeem.error.code).toBe(
        "DUPLICATE_REGISTRAR_MESSAGE"
      );
    }

    expect(build.historyBld).toBe(121n);
    expect(app.registrar.processedMessages.size).toBe(5);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(1);
  });
});

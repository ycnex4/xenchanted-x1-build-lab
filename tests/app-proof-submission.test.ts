import { describe, expect, it } from "vitest";
import {
  type CoreRedeemProof,
  type GenesisOriginEligibilityProof,
  appCreateBuild,
  appSubmitProof,
  createBuildApplicationState,
  createCanonicalEventKey,
  createCoreRedeemCandidate,
  createProofSourceMetadata,
  createX1FeeCheckpointCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
  createXntdRelockCandidate,
  convertWatcherCandidateToProof
} from "../src/index.js";

function createRegisteredApp() {
  const app = createBuildApplicationState("registrar-1");

  const created = appCreateBuild(app, {
    owner: "x1-owner",
    buildId: "build-1",
    ethereumIdentity: "0x0000000000000000000000000000000000000001",
    createdAt: 100n
  });

  expect(created.ok).toBe(true);

  if (!created.ok) {
    throw new Error("Build creation failed");
  }

  return {
    app,
    build: created.value
  };
}

function submitInput(messageId?: string) {
  return {
    submittedBy: "registrar-1",
    createdAt: 1200n,
    ...(messageId === undefined ? {} : { messageId })
  };
}

describe("application proof submission", () => {
  it("submits Core redeem proof through registrar application service", () => {
    const { app, build } = createRegisteredApp();

    const candidate = createCoreRedeemCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx-core",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountBld: 121n,
      redeemedAt: 1000n,
      coreTokenId: "1"
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(true);
    expect(build.historyBld).toBe(121n);
    expect(build.availableBld).toBe(121n);
    expect(app.registrar.processedMessages.size).toBe(1);
    expect(app.redeemEvents.usedRedeemEvents.has(candidate.canonicalEventKey)).toBe(
      true
    );
  });

  it("submits XEN burn proof through registrar application service", () => {
    const { app, build } = createRegisteredApp();

    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx-xen",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(true);
    expect(build.earnedXbp).toBe(1000n);
    expect(build.availableXbp).toBe(1000n);
    expect(app.registrar.processedMessages.size).toBe(1);
    expect(app.xenBurnEvents.usedXenBurnEvents.has(candidate.canonicalEventKey)).toBe(
      true
    );
  });

  it("submits XNTD lock and relock proofs through registrar application service", () => {
    const { app, build } = createRegisteredApp();

    const lockCandidate = createXntdLockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_LOCK",
      transactionHash: "tx-lock",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1000n
    });

    const lockProof = convertWatcherCandidateToProof(lockCandidate, {
      validatedAt: 1100n
    });

    const lock = appSubmitProof(app, lockProof, submitInput());

    expect(lock.ok).toBe(true);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);

    const relockCandidate = createXntdRelockCandidate({
      sourceChainId: "x1",
      sourceAddress: "lock-program",
      eventKind: "XNTD_RELOCK",
      transactionHash: "tx-relock",
      eventIndex: 0,
      observedAt: 1200n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountXntd: 250n,
      lockEpoch: 2,
      relockedAt: 1200n
    });

    const relockProof = convertWatcherCandidateToProof(relockCandidate, {
      validatedAt: 1300n
    });

    const relock = appSubmitProof(app, relockProof, submitInput());

    expect(relock.ok).toBe(true);
    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(app.registrar.processedMessages.size).toBe(2);
  });

  it("submits X1 fee checkpoint proof through registrar application service", () => {
    const { app, build } = createRegisteredApp();

    const candidate = createX1FeeCheckpointCandidate({
      sourceChainId: "x1",
      sourceAddress: "fee-indexer",
      eventKind: "X1_FEE_CHECKPOINT",
      transactionHash: "checkpoint-1",
      eventIndex: 0,
      slot: 9000n,
      observedAt: 1000n,
      finalized: true,
      buildId: build.buildId,
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 1000n
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(true);
    expect(build.x1FeeContribution).toBe(777n);
    expect(build.x1TxCount).toBe(11n);
    expect(build.x1FeeCountedUntilSlot).toBe(9000n);
    expect(app.registrar.processedMessages.size).toBe(1);
  });

  it("rejects non-validated proof without mutating state", () => {
    const { app, build } = createRegisteredApp();

    const canonicalEventKey = createCanonicalEventKey({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx-candidate",
      eventIndex: 0
    });

    const proof: CoreRedeemProof = {
      kind: "CORE_REDEEM_PROOF",
      status: "CANDIDATE",
      source: createProofSourceMetadata({
        sourceChainId: "eip155-1",
        sourceAddress: "0xcore",
        eventKind: "CORE_REDEEM",
        transactionHash: "0xtx-candidate",
        eventIndex: 0,
        observedAt: 1000n,
        finalized: true
      }),
      canonicalEventKey,
      validatedAt: null,
      rejectionReason: null,
      payload: {
        buildId: build.buildId,
        owner: build.owner,
        redeemKey: canonicalEventKey,
        amountBld: 121n,
        redeemedAt: 1000n,
        coreTokenId: "1"
      }
    };

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(false);

    if (!result.ok) {
      expect(result.error.code).toBe("PROOF_SUBMISSION_ERROR");
      expect(result.error.message).toBe("Proof is not validated: CORE_REDEEM_PROOF");
    }

    expect(build.historyBld).toBe(0n);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
  });

  it("rejects missing Build without writing registrar replay state", () => {
    const app = createBuildApplicationState("registrar-1");

    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx-missing-build",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: "missing-build",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(false);

    if (!result.ok) {
      expect(result.error.code).toBe("BUILD_NOT_FOUND");
      expect(result.error.message).toBe("Build not found: missing-build");
    }

    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
  });

  it("rejects Genesis Origin proof because it does not map to registrar payload", () => {
    const { app, build } = createRegisteredApp();

    const canonicalEventKey = createCanonicalEventKey({
      sourceChainId: "snapshot",
      sourceAddress: "genesis-origin",
      eventKind: "GENESIS_ORIGIN",
      transactionHash: "snapshot-1",
      eventIndex: 0
    });

    const proof: GenesisOriginEligibilityProof = {
      kind: "GENESIS_ORIGIN_ELIGIBILITY_PROOF",
      status: "VALIDATED",
      source: createProofSourceMetadata({
        sourceChainId: "snapshot",
        sourceAddress: "genesis-origin",
        eventKind: "GENESIS_ORIGIN",
        transactionHash: "snapshot-1",
        eventIndex: 0,
        observedAt: 1000n,
        finalized: true
      }),
      canonicalEventKey,
      validatedAt: 1100n,
      rejectionReason: null,
      payload: {
        buildId: build.buildId,
        owner: build.owner,
        historyBld: 121n,
        eligibleOriginBld: 55n,
        snapshotId: "snapshot-1",
        claimedAt: 1000n
      }
    };

    const result = appSubmitProof(app, proof, submitInput());

    expect(result.ok).toBe(false);

    if (!result.ok) {
      expect(result.error.code).toBe("PROOF_SUBMISSION_ERROR");
      expect(result.error.message).toBe(
        "Genesis Origin proof does not map to a registrar payload"
      );
    }

    expect(build.originBld).toBe(0n);
    expect(app.registrar.processedMessages.size).toBe(0);
  });

  it("rejects duplicate proof submission through existing replay protection", () => {
    const { app, build } = createRegisteredApp();

    const candidate = createCoreRedeemCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xcore",
      eventKind: "CORE_REDEEM",
      transactionHash: "0xtx-duplicate",
      eventIndex: 0,
      observedAt: 1000n,
      finalized: true,
      buildId: build.buildId,
      owner: build.owner,
      amountBld: 121n,
      redeemedAt: 1000n,
      coreTokenId: "1"
    });

    const proof = convertWatcherCandidateToProof(candidate, {
      validatedAt: 1100n
    });

    const first = appSubmitProof(app, proof, submitInput());
    const duplicate = appSubmitProof(app, proof, submitInput());

    expect(first.ok).toBe(true);
    expect(duplicate.ok).toBe(false);

    if (!duplicate.ok) {
      expect(duplicate.error.code).toBe("DUPLICATE_REGISTRAR_MESSAGE");
    }

    expect(build.historyBld).toBe(121n);
    expect(app.registrar.processedMessages.size).toBe(1);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(1);
  });
});

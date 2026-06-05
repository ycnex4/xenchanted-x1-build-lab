import { describe, expect, it } from "vitest";
import {
  assertFinalizedWatcherCandidate,
  createCoreRedeemCandidate,
  createX1FeeCheckpointCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
  createXntdRelockCandidate,
  isFinalizedWatcherCandidate
} from "../src/index.js";

describe("watcher candidates", () => {
  it("creates Core redeem candidate with canonical event key", () => {
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

    expect(candidate.kind).toBe("CORE_REDEEM_CANDIDATE");
    expect(candidate.canonicalEventKey).toBe(
      "eip155-1:0xcore:CORE_REDEEM:0xtx:0"
    );
    expect(candidate.source.finalized).toBe(true);
    expect(candidate.payload.amountBld).toBe(121n);
    expect(isFinalizedWatcherCandidate(candidate)).toBe(true);
    expect(() => assertFinalizedWatcherCandidate(candidate)).not.toThrow();
  });

  it("creates XEN burn candidate", () => {
    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx",
      eventIndex: 1,
      observedAt: 1000n,
      buildId: "build-1",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    expect(candidate.kind).toBe("XEN_BURN_CANDIDATE");
    expect(candidate.canonicalEventKey).toBe(
      "eip155-1:0xxen:XEN_BURN:0xtx:1"
    );
    expect(candidate.source.finalized).toBe(false);
    expect(candidate.payload.xenAmountBurned).toBe(100000000n);
  });

  it("creates XNTD lock and relock candidates", () => {
    const lock = createXntdLockCandidate({
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
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1000n
    });

    const relock = createXntdRelockCandidate({
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
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1100n
    });

    expect(lock.kind).toBe("XNTD_LOCK_CANDIDATE");
    expect(lock.payload.amountXntd).toBe(500n);
    expect(lock.payload.lockEpoch).toBe(1);

    expect(relock.kind).toBe("XNTD_RELOCK_CANDIDATE");
    expect(relock.payload.amountXntd).toBe(250n);
    expect(relock.payload.lockEpoch).toBe(2);
  });

  it("creates X1 fee checkpoint candidate", () => {
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

    expect(candidate.kind).toBe("X1_FEE_CHECKPOINT_CANDIDATE");
    expect(candidate.source.slot).toBe(9000n);
    expect(candidate.payload.feeAmount).toBe(777n);
    expect(candidate.payload.txCount).toBe(11n);
  });

  it("rejects non-finalized candidate through assertion helper", () => {
    const candidate = createXenBurnCandidate({
      sourceChainId: "eip155-1",
      sourceAddress: "0xxen",
      eventKind: "XEN_BURN",
      transactionHash: "0xtx",
      eventIndex: 0,
      observedAt: 1000n,
      buildId: "build-1",
      owner: "x1-owner",
      amountXbp: 1000n,
      burnedAt: 1000n,
      xenAmountBurned: 100000000n
    });

    expect(isFinalizedWatcherCandidate(candidate)).toBe(false);
    expect(() => assertFinalizedWatcherCandidate(candidate)).toThrow(
      "Watcher candidate is not finalized: XEN_BURN_CANDIDATE"
    );
  });
});

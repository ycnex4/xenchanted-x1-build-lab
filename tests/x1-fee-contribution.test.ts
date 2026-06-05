import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  applyX1FeeContributionCheckpoint,
  applyXenBurnPower,
  createBuild,
  lockXntd
} from "../src/index.js";

describe("X1 Fee Contribution checkpoint", () => {
  it("applies first fee contribution checkpoint", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.lastFeeUpdateAt).toBe(1100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accumulates fee amount and tx count with increasing slot", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 2500n,
      txCount: 7n,
      countedUntilSlot: 6000n,
      updatedAt: 1200n
    });

    expect(build.x1FeeContribution).toBe(3500n);
    expect(build.x1TxCount).toBe(10n);
    expect(build.x1FeeCountedUntilSlot).toBe(6000n);
    expect(build.lastFeeUpdateAt).toBe(1200n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("rejects zero fee amount without mutating state", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 0n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 0n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidFeeContributionAmount
      );
    }

    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects zero tx count without mutating state", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 1000n,
        txCount: 0n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 1000n,
        txCount: 0n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidFeeContributionTxCount
      );
    }

    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects non-increasing checkpoint slot without mutating state", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(() =>
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 2500n,
        txCount: 7n,
        countedUntilSlot: 5000n,
        updatedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 2500n,
        txCount: 7n,
        countedUntilSlot: 5000n,
        updatedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.NonIncreasingFeeCheckpointSlot
      );
    }

    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.lastFeeUpdateAt).toBe(1100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects lower checkpoint slot without mutating state", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(() =>
      applyX1FeeContributionCheckpoint({
        build,
        feeAmount: 2500n,
        txCount: 7n,
        countedUntilSlot: 4999n,
        updatedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("does not create or change BLD, XBP, or XNTD commitment", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1050n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1060n
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1070n
    });

    applyX1FeeContributionCheckpoint({
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.originBld).toBe(0n);
    expect(build.earnedXbp).toBe(100n);
    expect(build.availableXbp).toBe(100n);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);
  });
});

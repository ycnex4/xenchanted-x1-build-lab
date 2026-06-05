import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  claimGenesisOriginBld,
  createBuild,
  lockXntd,
  relockXntd
} from "../src/index.js";

describe("XNTD lock / relock", () => {
  it("locks XNTD and activates XC commitment", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1100n);
  });

  it("locks with amount above observed required lock", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    lockXntd({
      build,
      amountXntd: 750n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(build.lockedXntd).toBe(750n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects zero XNTD lock amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 0n,
        observedRequiredXntdLock: 0n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      lockXntd({
        build,
        amountXntd: 0n,
        observedRequiredXntdLock: 0n,
        lockEpoch: 1,
        lockedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidXntdLockAmount
      );
    }

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("relocks active commitment when availableBld covers historyBld", () => {
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

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    relockXntd({
      build,
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1200n
    });

    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1200n);
  });

  it("relocks with amount above observed required lock", () => {
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

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    relockXntd({
      build,
      amountXntd: 400n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1200n
    });

    expect(build.lockedXntd).toBe(400n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1200n);
  });

  it("allows relock when Genesis Origin makes availableBld greater than historyBld", () => {
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

    claimGenesisOriginBld({
      build,
      claimedAt: 1075n
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    relockXntd({
      build,
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1200n
    });

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect(build.availableBld).toBe(33n);
    expect(build.lockedXntd).toBe(250n);
    expect(build.lockEpoch).toBe(2);
  });

  it("rejects relock when commitment is not active", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.XntdCommitmentNotActive
      );
    }

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects relock when availableBld is below historyBld", () => {
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

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    build.availableBld = 10n;

    expect(() =>
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InsufficientAvailableBldForRelock
      );
    }

    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects zero observed required XNTD lock amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 500n,
        observedRequiredXntdLock: 0n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      lockXntd({
        build,
        amountXntd: 500n,
        observedRequiredXntdLock: 0n,
        lockEpoch: 1,
        lockedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidXntdLockAmount
      );
    }

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects lock amount below observed required lock", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      lockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 1,
        lockedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidXntdLockAmount
      );
    }

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects relock amount below observed required lock", () => {
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

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(() =>
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 2,
        relockedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 2,
        relockedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidXntdLockAmount
      );
    }

    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.updatedAt).toBe(1100n);
  });

  it("does not create BLD, XBP, or X1 fee contribution", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.originBld).toBe(0n);
    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  claimGenesisOriginBld,
  createBuild,
  lockXntd,
  relockXntd,
} from "../src/index.js";

describe("XNTD lock / relock", () => {
  it("locks XNTD and activates XC commitment", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n,
    });

    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xntdCommitmentAccepted).toBe(true);
    expect(build.updatedAt).toBe(1100n);
  });

  it("locks with amount above observed required lock", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    lockXntd({
      build,
      amountXntd: 750n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n,
    });

    expect(build.lockedXntd).toBe(750n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xntdCommitmentAccepted).toBe(true);
  });

  it("relocks active commitment without reading a public Build spendable balance", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1050n,
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n,
    });

    relockXntd({
      build,
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1200n,
    });

    expect(build.historyBld).toBe(11n);
    expect("availableBld" in build).toBe(false);
    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xntdCommitmentAccepted).toBe(true);
    expect(build.updatedAt).toBe(1200n);
  });

  it("allows relock after Genesis Origin without treating origin as a spendable Build balance", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1050n,
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1075n,
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n,
    });

    relockXntd({
      build,
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1200n,
    });

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect("availableBld" in build).toBe(false);
    expect(build.lockedXntd).toBe(250n);
    expect(build.lockEpoch).toBe(2);
  });

  it("rejects relock when commitment is not accepted", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n,
      }),
    ).toThrow(BuildError);

    try {
      relockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.XntdCommitmentNotAccepted,
      );
    }

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects zero XNTD lock amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 0n,
        observedRequiredXntdLock: 1n,
        lockEpoch: 1,
        lockedAt: 1100n,
      }),
    ).toThrow(BuildError);
  });

  it("rejects zero observed required XNTD lock amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 500n,
        observedRequiredXntdLock: 0n,
        lockEpoch: 1,
        lockedAt: 1100n,
      }),
    ).toThrow(BuildError);
  });

  it("rejects lock amount below observed required lock", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      lockXntd({
        build,
        amountXntd: 250n,
        observedRequiredXntdLock: 500n,
        lockEpoch: 1,
        lockedAt: 1100n,
      }),
    ).toThrow(BuildError);
  });

  it("rejects relock amount below observed required lock", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1050n,
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1100n,
    });

    expect(() =>
      relockXntd({
        build,
        amountXntd: 100n,
        observedRequiredXntdLock: 250n,
        lockEpoch: 2,
        relockedAt: 1200n,
      }),
    ).toThrow(BuildError);

    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
  });
});

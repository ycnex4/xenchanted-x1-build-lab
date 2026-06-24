import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  calculateGenesisOriginBld,
  calculateGenesisOriginBldDelta,
  claimGenesisOriginBld,
  createBuild,
  upgradeGenesisOriginBld,
} from "../src/index.js";

describe("Genesis Origin BLD", () => {
  it("calculates tiered Genesis Origin BLD", () => {
    expect(calculateGenesisOriginBld(0n)).toBe(0n);
    expect(calculateGenesisOriginBld(1n)).toBe(11n);
    expect(calculateGenesisOriginBld(10n)).toBe(11n);
    expect(calculateGenesisOriginBld(11n)).toBe(22n);
    expect(calculateGenesisOriginBld(120n)).toBe(22n);
    expect(calculateGenesisOriginBld(121n)).toBe(55n);
    expect(calculateGenesisOriginBld(1110n)).toBe(55n);
    expect(calculateGenesisOriginBld(1111n)).toBe(121n);
  });

  it("calculates only the upgrade delta from the current origin tier", () => {
    expect(calculateGenesisOriginBldDelta(0n, 1n)).toBe(11n);
    expect(calculateGenesisOriginBldDelta(11n, 11n)).toBe(11n);
    expect(calculateGenesisOriginBldDelta(22n, 121n)).toBe(33n);
    expect(calculateGenesisOriginBldDelta(55n, 1111n)).toBe(66n);
    expect(calculateGenesisOriginBldDelta(121n, 1111n)).toBe(0n);
  });

  it("upgrades to the eligible tier without minting a public spendable balance", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1100n,
    });

    upgradeGenesisOriginBld({
      build,
      upgradedAt: 1200n,
    });

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect("availableBld" in build).toBe(false);
    expect(build.updatedAt).toBe(1200n);
  });

  it("allows later tier upgrades by delta instead of one-time static claim", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({ build, amountBld: 1n, redeemedAt: 1100n });
    upgradeGenesisOriginBld({ build, upgradedAt: 1200n });
    expect(build.originBld).toBe(11n);

    applyCoreRedeemBld({ build, amountBld: 10n, redeemedAt: 1300n });
    upgradeGenesisOriginBld({ build, upgradedAt: 1400n });
    expect(build.originBld).toBe(22n);

    applyCoreRedeemBld({ build, amountBld: 110n, redeemedAt: 1500n });
    upgradeGenesisOriginBld({ build, upgradedAt: 1600n });
    expect(build.originBld).toBe(55n);

    applyCoreRedeemBld({ build, amountBld: 990n, redeemedAt: 1700n });
    upgradeGenesisOriginBld({ build, upgradedAt: 1800n });

    expect(build.historyBld).toBe(1111n);
    expect(build.originBld).toBe(121n);
    expect(build.updatedAt).toBe(1800n);
  });

  it("keeps claimGenesisOriginBld as a compatibility alias for upgrade", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({ build, amountBld: 121n, redeemedAt: 1100n });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n,
    });

    expect(build.originBld).toBe(55n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("rejects upgrade when historyBld is zero", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      upgradeGenesisOriginBld({
        build,
        upgradedAt: 1200n,
      }),
    ).toThrow(BuildError);

    try {
      upgradeGenesisOriginBld({
        build,
        upgradedAt: 1200n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.GenesisOriginNotEligible,
      );
    }

    expect(build.originBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects upgrade when the Build is already at the eligible tier", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({ build, amountBld: 11n, redeemedAt: 1100n });
    upgradeGenesisOriginBld({ build, upgradedAt: 1200n });

    expect(() =>
      upgradeGenesisOriginBld({
        build,
        upgradedAt: 1300n,
      }),
    ).toThrow(BuildError);

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect(build.updatedAt).toBe(1200n);
  });
});

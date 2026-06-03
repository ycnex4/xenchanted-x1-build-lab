import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  calculateGenesisOriginBld,
  claimGenesisOriginBld,
  createBuild
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

  it("claims 11 originBld for historyBld >= 1", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 1n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(build.historyBld).toBe(1n);
    expect(build.originBld).toBe(11n);
    expect(build.availableBld).toBe(12n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("claims 22 originBld for historyBld >= 11", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect(build.availableBld).toBe(33n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("claims 55 originBld for historyBld >= 121", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 121n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(build.historyBld).toBe(121n);
    expect(build.originBld).toBe(55n);
    expect(build.availableBld).toBe(176n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("claims 121 originBld for historyBld >= 1111", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 1111n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(build.historyBld).toBe(1111n);
    expect(build.originBld).toBe(121n);
    expect(build.availableBld).toBe(1232n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("rejects claim when historyBld is zero", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      claimGenesisOriginBld({
        build,
        claimedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      claimGenesisOriginBld({
        build,
        claimedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.GenesisOriginNotEligible
      );
    }

    expect(build.originBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects duplicate Genesis Origin claim", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(() =>
      claimGenesisOriginBld({
        build,
        claimedAt: 1300n
      })
    ).toThrow(BuildError);

    try {
      claimGenesisOriginBld({
        build,
        claimedAt: 1300n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.GenesisOriginAlreadyClaimed
      );
    }

    expect(build.historyBld).toBe(11n);
    expect(build.originBld).toBe(22n);
    expect(build.availableBld).toBe(33n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not create XBP or unrelated accounting values", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1200n
    });

    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

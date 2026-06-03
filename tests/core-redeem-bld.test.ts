import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  createBuild
} from "../src/index.js";

describe("applyCoreRedeemBld", () => {
  it("adds redeemed Core history to historyBld and availableBld", () => {
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

    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accumulates multiple accepted Core redeem amounts", () => {
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

    applyCoreRedeemBld({
      build,
      amountBld: 22n,
      redeemedAt: 1200n
    });

    expect(build.historyBld).toBe(33n);
    expect(build.availableBld).toBe(33n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not change originBld", () => {
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

    expect(build.originBld).toBe(0n);
  });

  it("does not create XBP", () => {
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

    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
  });

  it("does not change XNTD commitment fields", () => {
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

    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
  });

  it("does not create X1 fee contribution", () => {
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

    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
  });

  it("rejects zero BLD amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyCoreRedeemBld({
        build,
        amountBld: 0n,
        redeemedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyCoreRedeemBld({
        build,
        amountBld: 0n,
        redeemedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(BuildErrorCode.InvalidBldAmount);
    }

    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects negative BLD amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyCoreRedeemBld({
        build,
        amountBld: -1n,
        redeemedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });
});

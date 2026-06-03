import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  acceptCoreRedeemEvent,
  createBuild,
  createRedeemEventState
} from "../src/index.js";

describe("redeem event replay protection", () => {
  it("accepts a new Core redeem event and records redeemKey", () => {
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    acceptCoreRedeemEvent(redeemEvents, {
      redeemKey: "redeem-1",
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(redeemEvents.usedRedeemEvents.has("redeem-1")).toBe(true);
    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects duplicate redeemKey before applying BLD twice", () => {
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    acceptCoreRedeemEvent(redeemEvents, {
      redeemKey: "redeem-1",
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(() =>
      acceptCoreRedeemEvent(redeemEvents, {
        redeemKey: "redeem-1",
        build,
        amountBld: 22n,
        redeemedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      acceptCoreRedeemEvent(redeemEvents, {
        redeemKey: "redeem-1",
        build,
        amountBld: 22n,
        redeemedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateRedeemEvent
      );
    }

    expect(redeemEvents.usedRedeemEvents.size).toBe(1);
    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accepts different redeemKeys and accumulates BLD", () => {
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    acceptCoreRedeemEvent(redeemEvents, {
      redeemKey: "redeem-1",
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    acceptCoreRedeemEvent(redeemEvents, {
      redeemKey: "redeem-2",
      build,
      amountBld: 22n,
      redeemedAt: 1200n
    });

    expect(redeemEvents.usedRedeemEvents.size).toBe(2);
    expect(build.historyBld).toBe(33n);
    expect(build.availableBld).toBe(33n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not mark redeemKey when BLD amount is invalid", () => {
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      acceptCoreRedeemEvent(redeemEvents, {
        redeemKey: "redeem-1",
        build,
        amountBld: 0n,
        redeemedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(redeemEvents.usedRedeemEvents.has("redeem-1")).toBe(false);
    expect(redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("does not create unrelated accounting values", () => {
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    acceptCoreRedeemEvent(redeemEvents, {
      redeemKey: "redeem-1",
      build,
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(build.originBld).toBe(0n);
    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

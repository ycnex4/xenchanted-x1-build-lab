import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyXenBurnPower,
  createBuild
} from "../src/index.js";

describe("applyXenBurnPower", () => {
  it("adds XEN Burn Power to earnedXbp and availableXbp", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n
    });

    expect(build.earnedXbp).toBe(100n);
    expect(build.availableXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accumulates multiple accepted XBP amounts", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n
    });

    applyXenBurnPower({
      build,
      amountXbp: 250n,
      burnedAt: 1200n
    });

    expect(build.earnedXbp).toBe(350n);
    expect(build.availableXbp).toBe(350n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not create BLD", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n
    });

    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.originBld).toBe(0n);
  });

  it("does not change XNTD commitment fields", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n
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

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n
    });

    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
  });

  it("rejects zero XBP amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyXenBurnPower({
        build,
        amountXbp: 0n,
        burnedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyXenBurnPower({
        build,
        amountXbp: 0n,
        burnedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(BuildErrorCode.InvalidXbpAmount);
    }

    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects negative XBP amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyXenBurnPower({
        build,
        amountXbp: -1n,
        burnedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });
});

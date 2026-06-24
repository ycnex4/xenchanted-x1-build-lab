import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyXenBurnPower,
  createBuild,
} from "../src/index.js";

describe("applyXenBurnPower", () => {
  it("adds XEN Burn Power to historical historyXbp", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accumulates multiple accepted XBP amounts", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    applyXenBurnPower({
      build,
      amountXbp: 250n,
      burnedAt: 1200n,
    });

    expect(build.historyXbp).toBe(350n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not create BLD or other accounting values", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(build.historyBld).toBe(0n);
    expect(build.originBld).toBe(0n);
    expect(build.lockedXntd).toBe(0n);
    expect(build.x1FeeContribution).toBe(0n);
  });

  it("rejects zero XBP amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      applyXenBurnPower({
        build,
        amountXbp: 0n,
        burnedAt: 1100n,
      }),
    ).toThrow(BuildError);

    try {
      applyXenBurnPower({
        build,
        amountXbp: 0n,
        burnedAt: 1100n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(BuildErrorCode.InvalidXbpAmount);
    }

    expect(build.historyXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects negative XBP amount", () => {
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      applyXenBurnPower({
        build,
        amountXbp: -1n,
        burnedAt: 1100n,
      }),
    ).toThrow(BuildError);

    expect(build.historyXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });
});

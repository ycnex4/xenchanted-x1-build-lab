import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  acceptXenBurnEvent,
  createBuild,
  createXenBurnEventState,
} from "../src/index.js";

describe("XEN burn event replay protection", () => {
  it("accepts a new XEN burn event and records xenBurnKey", () => {
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    acceptXenBurnEvent(xenBurnEvents, {
      xenBurnKey: "xen-burn-1",
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(xenBurnEvents.usedXenBurnEvents.has("xen-burn-1")).toBe(true);
    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects duplicate xenBurnKey before applying XBP twice", () => {
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    acceptXenBurnEvent(xenBurnEvents, {
      xenBurnKey: "xen-burn-1",
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(() =>
      acceptXenBurnEvent(xenBurnEvents, {
        xenBurnKey: "xen-burn-1",
        build,
        amountXbp: 250n,
        burnedAt: 1200n,
      }),
    ).toThrow(BuildError);

    try {
      acceptXenBurnEvent(xenBurnEvents, {
        xenBurnKey: "xen-burn-1",
        build,
        amountXbp: 250n,
        burnedAt: 1200n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateXenBurnEvent,
      );
    }

    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(1);
    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accepts different xenBurnKeys and accumulates XBP", () => {
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    acceptXenBurnEvent(xenBurnEvents, {
      xenBurnKey: "xen-burn-1",
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    acceptXenBurnEvent(xenBurnEvents, {
      xenBurnKey: "xen-burn-2",
      build,
      amountXbp: 250n,
      burnedAt: 1200n,
    });

    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(2);
    expect(build.historyXbp).toBe(350n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("does not mark xenBurnKey when XBP amount is invalid", () => {
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      acceptXenBurnEvent(xenBurnEvents, {
        xenBurnKey: "xen-burn-1",
        build,
        amountXbp: 0n,
        burnedAt: 1100n,
      }),
    ).toThrow(BuildError);

    expect(xenBurnEvents.usedXenBurnEvents.has("xen-burn-1")).toBe(false);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(build.historyXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("does not create unrelated accounting values", () => {
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    acceptXenBurnEvent(xenBurnEvents, {
      xenBurnKey: "xen-burn-1",
      build,
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(build.historyBld).toBe(0n);
    expect(build.originBld).toBe(0n);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.xntdCommitmentAccepted).toBe(false);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

import { describe, expect, it } from "vitest";
import { BUILD_STATE_VERSION, createEmptyBuildState } from "../src/index.js";

describe("BuildState", () => {
  it("creates an empty BuildState with identity fields", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001"
    });

    expect(state.owner).toBe("x1-user-1");
    expect(state.buildId).toBe("build-1");
    expect(state.version).toBe(BUILD_STATE_VERSION);
    expect(state.createdAt).toBe(1000n);
    expect(state.updatedAt).toBe(1000n);
    expect(state.ethereumIdentity).toBe("0x0000000000000000000000000000000000000001");
  });

  it("initializes BLD fields to zero", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.historyBld).toBe(0n);
    expect(state.availableBld).toBe(0n);
    expect(state.originBld).toBe(0n);
  });

  it("initializes XBP fields to zero", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.earnedXbp).toBe(0n);
    expect(state.availableXbp).toBe(0n);
  });

  it("initializes XNTD commitment fields as inactive", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.lockedXntd).toBe(0n);
    expect(state.requiredXntdLock).toBe(0n);
    expect(state.lockEpoch).toBeNull();
    expect(state.xcCommitmentActive).toBe(false);
  });

  it("initializes X1 fee fields to zero or null", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.x1FeeContribution).toBe(0n);
    expect(state.x1TxCount).toBe(0n);
    expect(state.x1FeeCountedUntilSlot).toBeNull();
    expect(state.lastFeeUpdateAt).toBeNull();
  });
});

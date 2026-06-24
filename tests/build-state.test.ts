import { describe, expect, it } from "vitest";
import { BUILD_STATE_VERSION, createEmptyBuildState } from "../src/index.js";

describe("BuildState", () => {
  it("creates an empty BuildState", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(state.owner).toBe("x1-user-1");
    expect(state.buildId).toBe("build-1");
    expect(state.version).toBe(BUILD_STATE_VERSION);
    expect(state.createdAt).toBe(1000n);
    expect(state.updatedAt).toBe(1000n);
    expect(state.ethereumIdentity).toBe(
      "0x0000000000000000000000000000000000000001",
    );
  });

  it("initializes optional Build Identity fields as empty", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.buildName).toBeNull();
    expect(state.logoUri).toBeNull();
    expect(state.metadataUpdatedAt).toBeNull();
  });

  it("allows optional initial Build Identity metadata", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      buildName: "Sergey Build",
      logoUri: "ipfs://build-logo",
    });

    expect(state.buildName).toBe("Sergey Build");
    expect(state.logoUri).toBe("ipfs://build-logo");
    expect(state.metadataUpdatedAt).toBe(1000n);
  });

  it("initializes historical contribution fields to zero", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.historyBld).toBe(0n);
    expect(state.originBld).toBe(0n);
    expect(state.historyXbp).toBe(0n);
  });

  it("does not expose public spendable BLD or XBP balances", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect("availableBld" in state).toBe(false);
    expect("availableXbp" in state).toBe(false);
    expect("earnedXbp" in state).toBe(false);
  });

  it("initializes XNTD commitment fields as inactive", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.lockedXntd).toBe(0n);
    expect(state.requiredXntdLock).toBe(0n);
    expect(state.lockEpoch).toBeNull();
    expect(state.xntdCommitmentAccepted).toBe(false);
  });

  it("initializes X1 fee fields to zero or null", () => {
    const state = createEmptyBuildState({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.x1FeeContribution).toBe(0n);
    expect(state.x1TxCount).toBe(0n);
    expect(state.x1FeeCountedUntilSlot).toBeNull();
    expect(state.lastFeeUpdateAt).toBeNull();
  });
});

import { describe, expect, it } from "vitest";
import { BUILD_STATE_VERSION, createBuild } from "../src/index.js";

describe("createBuild", () => {
  it("creates a canonical empty BuildState from input identity", () => {
    const state = createBuild({
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

  it("allows creating a BuildState without Ethereum identity", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.ethereumIdentity).toBeNull();
  });

  it("does not create BLD balances", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.historyBld).toBe(0n);
    expect(state.availableBld).toBe(0n);
    expect(state.originBld).toBe(0n);
  });

  it("does not create XBP balances", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.earnedXbp).toBe(0n);
    expect(state.availableXbp).toBe(0n);
  });

  it("does not activate XNTD commitment", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(state.lockedXntd).toBe(0n);
    expect(state.requiredXntdLock).toBe(0n);
    expect(state.lockEpoch).toBeNull();
    expect(state.xcCommitmentActive).toBe(false);
  });

  it("does not create X1 fee contribution", () => {
    const state = createBuild({
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

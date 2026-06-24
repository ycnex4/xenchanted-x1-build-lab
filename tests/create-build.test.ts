import { describe, expect, it } from "vitest";
import { BUILD_STATE_VERSION, createBuild } from "../src/index.js";

describe("createBuild", () => {
  it("creates a Build with protocol state defaults", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.owner).toBe("x1-user-1");
    expect(state.buildId).toBe("build-1");
    expect(state.version).toBe(BUILD_STATE_VERSION);
    expect(state.createdAt).toBe(1000n);
    expect(state.updatedAt).toBe(1000n);
    expect(state.ethereumIdentity).toBeNull();

    expect(state.historyBld).toBe(0n);
    expect(state.originBld).toBe(0n);
    expect(state.historyXbp).toBe(0n);

    expect(state.lockedXntd).toBe(0n);
    expect(state.requiredXntdLock).toBe(0n);
    expect(state.lockEpoch).toBeNull();
    expect(state.xcCommitmentActive).toBe(false);

    expect(state.x1FeeContribution).toBe(0n);
    expect(state.x1TxCount).toBe(0n);
    expect(state.x1FeeCountedUntilSlot).toBeNull();
    expect(state.lastFeeUpdateAt).toBeNull();
  });

  it("can attach an Ethereum identity at creation", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(state.ethereumIdentity).toBe(
      "0x0000000000000000000000000000000000000001",
    );
  });

  it("can create a Build without name and logo", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.buildName).toBeNull();
    expect(state.logoUri).toBeNull();
    expect(state.metadataUpdatedAt).toBeNull();
  });

  it("can create a Build with optional name and logo URI", () => {
    const state = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      buildName: "Sergey Build",
      logoUri: "ipfs://logo",
    });

    expect(state.buildName).toBe("Sergey Build");
    expect(state.logoUri).toBe("ipfs://logo");
    expect(state.metadataUpdatedAt).toBe(1000n);
  });
});

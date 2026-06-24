import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  createEmptyBuildRegistry,
  createRegisteredBuild,
} from "../src/index.js";

describe("BuildRegistry", () => {
  it("registers the first canonical Build for an owner", () => {
    const registry = createEmptyBuildRegistry();

    const state = createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(registry.buildsById.get("build-1")).toBe(state);
    expect(registry.canonicalBuildByOwner.get("x1-user-1")).toBe("build-1");
    expect(
      registry.canonicalBuildByEthereumIdentity.get(
        "0x0000000000000000000000000000000000000001",
      ),
    ).toBe("build-1");
  });

  it("rejects duplicate buildId", () => {
    const registry = createEmptyBuildRegistry();

    createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      createRegisteredBuild(registry, {
        owner: "x1-user-2",
        buildId: "build-1",
        createdAt: 1001n,
      }),
    ).toThrow(BuildError);

    try {
      createRegisteredBuild(registry, {
        owner: "x1-user-2",
        buildId: "build-1",
        createdAt: 1001n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(BuildErrorCode.DuplicateBuildId);
    }
  });

  it("rejects duplicate owner", () => {
    const registry = createEmptyBuildRegistry();

    createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      createRegisteredBuild(registry, {
        owner: "x1-user-1",
        buildId: "build-2",
        createdAt: 1001n,
      }),
    ).toThrow(BuildError);

    try {
      createRegisteredBuild(registry, {
        owner: "x1-user-1",
        buildId: "build-2",
        createdAt: 1001n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateBuildOwner,
      );
    }
  });

  it("rejects duplicate Ethereum identity", () => {
    const registry = createEmptyBuildRegistry();

    createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(() =>
      createRegisteredBuild(registry, {
        owner: "x1-user-2",
        buildId: "build-2",
        createdAt: 1001n,
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
      }),
    ).toThrow(BuildError);

    try {
      createRegisteredBuild(registry, {
        owner: "x1-user-2",
        buildId: "build-2",
        createdAt: 1001n,
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateEthereumIdentity,
      );
    }
  });

  it("allows different owners without Ethereum identity", () => {
    const registry = createEmptyBuildRegistry();

    const first = createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    const second = createRegisteredBuild(registry, {
      owner: "x1-user-2",
      buildId: "build-2",
      createdAt: 1001n,
    });

    expect(first.buildId).toBe("build-1");
    expect(second.buildId).toBe("build-2");
    expect(registry.buildsById.size).toBe(2);
  });

  it("does not create accounting value when registering a Build", () => {
    const registry = createEmptyBuildRegistry();

    const state = createRegisteredBuild(registry, {
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(state.historyBld).toBe(0n);
    expect(state.originBld).toBe(0n);
    expect(state.historyXbp).toBe(0n);
    expect(state.lockedXntd).toBe(0n);
    expect(state.requiredXntdLock).toBe(0n);
    expect(state.xcCommitmentActive).toBe(false);
    expect(state.x1FeeContribution).toBe(0n);
    expect(state.x1TxCount).toBe(0n);
  });
});

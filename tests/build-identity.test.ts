import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  createBuild,
  lockXntd,
  updateBuildIdentity,
} from "../src/index.js";

describe("Build Identity", () => {
  it("updates optional owner-controlled Build Identity metadata", () => {
    const build = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    updateBuildIdentity({
      build,
      owner: "x1-owner-1",
      buildName: "Sergey Build",
      logoUri: "ipfs://build-logo",
      updatedAt: 1200n,
    });

    expect(build.buildName).toBe("Sergey Build");
    expect(build.logoUri).toBe("ipfs://build-logo");
    expect(build.metadataUpdatedAt).toBe(1200n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("allows clearing Build Identity metadata", () => {
    const build = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
      buildName: "Sergey Build",
      logoUri: "ipfs://build-logo",
    });

    updateBuildIdentity({
      build,
      owner: "x1-owner-1",
      buildName: null,
      logoUri: null,
      updatedAt: 1300n,
    });

    expect(build.buildName).toBeNull();
    expect(build.logoUri).toBeNull();
    expect(build.metadataUpdatedAt).toBe(1300n);
    expect(build.updatedAt).toBe(1300n);
  });

  it("does not require globally unique Build names", () => {
    const first = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    const second = createBuild({
      owner: "x1-owner-2",
      buildId: "build-2",
      createdAt: 1000n,
    });

    updateBuildIdentity({
      build: first,
      owner: "x1-owner-1",
      buildName: "Same Name",
      updatedAt: 1100n,
    });

    updateBuildIdentity({
      build: second,
      owner: "x1-owner-2",
      buildName: "Same Name",
      updatedAt: 1100n,
    });

    expect(first.buildName).toBe("Same Name");
    expect(second.buildName).toBe("Same Name");
    expect(first.buildId).not.toBe(second.buildId);
  });

  it("does not affect protocol accounting", () => {
    const build = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyCoreRedeemBld({
      build,
      amountBld: 121n,
      redeemedAt: 1100n,
    });

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 1150n,
    });

    updateBuildIdentity({
      build,
      owner: "x1-owner-1",
      buildName: "Readable Build",
      logoUri: "ipfs://readable-build-logo",
      updatedAt: 1200n,
    });

    expect(build.historyBld).toBe(121n);
    expect(build.originBld).toBe(0n);
    expect(build.historyXbp).toBe(0n);
    expect(build.lockedXntd).toBe(100n);
    expect(build.requiredXntdLock).toBe(100n);
    expect(build.lockEpoch).toBe(0);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });

  it("rejects non-owner identity updates", () => {
    const build = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      updateBuildIdentity({
        build,
        owner: "x1-owner-2",
        buildName: "Wrong Owner Update",
        updatedAt: 1200n,
      }),
    ).toThrow(BuildError);

    try {
      updateBuildIdentity({
        build,
        owner: "x1-owner-2",
        buildName: "Wrong Owner Update",
        updatedAt: 1200n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.UnauthorizedBuildIdentityUpdate,
      );
    }

    expect(build.buildName).toBeNull();
    expect(build.logoUri).toBeNull();
    expect(build.metadataUpdatedAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("does not change state when no identity fields are supplied", () => {
    const build = createBuild({
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    const returned = updateBuildIdentity({
      build,
      owner: "x1-owner-1",
      updatedAt: 1200n,
    });

    expect(returned).toBe(build);
    expect(build.buildName).toBeNull();
    expect(build.logoUri).toBeNull();
    expect(build.metadataUpdatedAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });
});

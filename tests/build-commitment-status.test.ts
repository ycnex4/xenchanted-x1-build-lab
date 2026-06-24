import { describe, expect, it } from "vitest";
import {
  createEmptyBuildState,
  getBuildCommitmentStatus,
  lockXntd,
} from "../src/index.js";

describe("Build commitment status", () => {
  it("returns NO_HISTORY for a Build without historical contribution", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    const status = getBuildCommitmentStatus({ build });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("UNCOMMITTED");
    expect(status.reason).toBe("NO_HISTORY");
    expect(status.historyBld).toBe(0n);
    expect(status.needsRelock).toBe(false);
  });

  it("returns NO_COMMITMENT when history exists but no XNTD is locked", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;

    const status = getBuildCommitmentStatus({ build });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("UNCOMMITTED");
    expect(status.reason).toBe("NO_COMMITMENT");
    expect(status.historyBld).toBe(121n);
    expect(status.lockedXntd).toBe(0n);
    expect(status.requiredXntdLock).toBe(0n);
    expect(status.lockEpoch).toBe(null);
  });

  it("returns COMMITTED when history and sufficient stored lock facts exist", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n,
    });

    const status = getBuildCommitmentStatus({ build });

    expect(status.isActive).toBe(true);
    expect(status.status).toBe("COMMITTED");
    expect(status.reason).toBe("COMMITMENT_CURRENT");
    expect(status.lockedXntd).toBe(100n);
    expect(status.requiredXntdLock).toBe(100n);
    expect(status.lockEpoch).toBe(0);
    expect(status.needsRelock).toBe(false);
  });

  it("returns COMMITMENT_BELOW_REQUIRED when stored lock facts are insufficient", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;
    build.lockedXntd = 100n;
    build.requiredXntdLock = 200n;
    build.lockEpoch = 0;
    build.xcCommitmentActive = true;

    const status = getBuildCommitmentStatus({ build });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("UNCOMMITTED");
    expect(status.reason).toBe("COMMITMENT_BELOW_REQUIRED");
    expect(status.needsRelock).toBe(true);
  });

  it("does not expose UNKNOWN as a public Build status", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n,
    });

    const status = getBuildCommitmentStatus({ build });

    expect(status.status).not.toBe("UNKNOWN");
    expect(status.reason).toBe("COMMITMENT_CURRENT");
    expect("currentEpoch" in status).toBe(false);
  });

  it("does not mutate Build state", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n,
    });

    const before = {
      historyBld: build.historyBld,
      originBld: build.originBld,
      lockedXntd: build.lockedXntd,
      requiredXntdLock: build.requiredXntdLock,
      lockEpoch: build.lockEpoch,
    };

    getBuildCommitmentStatus({ build });

    expect({
      historyBld: build.historyBld,
      originBld: build.originBld,
      lockedXntd: build.lockedXntd,
      requiredXntdLock: build.requiredXntdLock,
      lockEpoch: build.lockEpoch,
    }).toEqual(before);
  });
});

import { describe, expect, it } from "vitest";
import {
  createEmptyBuildState,
  getBuildActiveStatus,
  lockXntd
} from "../src/index.js";

describe("Build active status", () => {
  it("returns INACTIVE_NO_HISTORY for a Build without historical contribution", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    const status = getBuildActiveStatus({ build });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("INACTIVE");
    expect(status.reason).toBe("INACTIVE_NO_HISTORY");
    expect(status.historyBld).toBe(0n);
    expect(status.availableBld).toBe(0n);
    expect(status.needsRelock).toBe(false);
  });

  it("returns INACTIVE_NO_LOCK when history exists but no XNTD is locked", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    const status = getBuildActiveStatus({ build });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("INACTIVE");
    expect(status.reason).toBe("INACTIVE_NO_LOCK");
    expect(status.historyBld).toBe(121n);
    expect(status.availableBld).toBe(121n);
    expect(status.lockedXntd).toBe(0n);
    expect(status.requiredXntdLock).toBe(0n);
    expect(status.lockEpoch).toBe(null);
  });

  it("returns ACTIVE_LOCK_CURRENT when history and sufficient lock exist", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n
    });

    const status = getBuildActiveStatus({ build });

    expect(status.isActive).toBe(true);
    expect(status.status).toBe("ACTIVE");
    expect(status.reason).toBe("ACTIVE_LOCK_CURRENT");
    expect(status.historyBld).toBe(121n);
    expect(status.availableBld).toBe(121n);
    expect(status.lockedXntd).toBe(100n);
    expect(status.requiredXntdLock).toBe(100n);
    expect(status.lockEpoch).toBe(0);
    expect(status.needsRelock).toBe(false);
  });

  it("returns INACTIVE_LOCK_BELOW_REQUIRED when current requirement exceeds locked XNTD", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n
    });

    const status = getBuildActiveStatus({
      build,
      currentEpoch: 1n,
      currentRequiredXntdLock: 200n
    });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("INACTIVE");
    expect(status.reason).toBe("INACTIVE_LOCK_BELOW_REQUIRED");
    expect(status.lockedXntd).toBe(100n);
    expect(status.requiredXntdLock).toBe(200n);
    expect(status.currentEpoch).toBe(1n);
    expect(status.needsRelock).toBe(true);
  });

  it("returns UNKNOWN_NO_CURRENT_CONTEXT when strict current context is required but missing", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n
    });

    const status = getBuildActiveStatus({
      build,
      requireCurrentEpoch: true
    });

    expect(status.isActive).toBe(false);
    expect(status.status).toBe("UNKNOWN");
    expect(status.reason).toBe("UNKNOWN_NO_CURRENT_CONTEXT");
    expect(status.lockedXntd).toBe(100n);
    expect(status.requiredXntdLock).toBe(100n);
    expect(status.currentEpoch).toBe(null);
    expect(status.needsRelock).toBe(false);
  });

  it("does not mutate Build state", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    lockXntd({
      build,
      amountXntd: 100n,
      observedRequiredXntdLock: 100n,
      lockEpoch: 0,
      lockedAt: 10n
    });

    const before = {
      historyBld: build.historyBld,
      availableBld: build.availableBld,
      originBld: build.originBld,
      lockedXntd: build.lockedXntd,
      requiredXntdLock: build.requiredXntdLock,
      lockEpoch: build.lockEpoch
    };

    getBuildActiveStatus({
      build,
      currentEpoch: 1n,
      currentRequiredXntdLock: 200n
    });

    expect({
      historyBld: build.historyBld,
      availableBld: build.availableBld,
      originBld: build.originBld,
      lockedXntd: build.lockedXntd,
      requiredXntdLock: build.requiredXntdLock,
      lockEpoch: build.lockEpoch
    }).toEqual(before);
  });
});

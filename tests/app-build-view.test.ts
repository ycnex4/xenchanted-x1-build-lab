import { describe, expect, it } from "vitest";
import {
  appGetBuildView,
  createEmptyBuildState,
  lockXntd
} from "../src/index.js";

describe("app Build view", () => {
  it("returns Build state with COMMITTED commitment status", () => {
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

    const view = appGetBuildView({ build });

    expect(view.build).toBe(build);
    expect(view.commitmentStatus.status).toBe("COMMITTED");
    expect(view.commitmentStatus.reason).toBe("COMMITMENT_CURRENT");
  });

  it("returns Build state with UNCOMMITTED commitment status", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n
    });

    build.historyBld = 121n;
    build.availableBld = 121n;

    const view = appGetBuildView({ build });

    expect(view.build).toBe(build);
    expect(view.commitmentStatus.status).toBe("UNCOMMITTED");
    expect(view.commitmentStatus.reason).toBe("NO_COMMITMENT");
  });

  it("returns UNKNOWN when strict current context is required but missing", () => {
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

    const view = appGetBuildView({
      build,
      requireCurrentEpoch: true
    });

    expect(view.build).toBe(build);
    expect(view.commitmentStatus.status).toBe("UNKNOWN");
    expect(view.commitmentStatus.reason).toBe("UNKNOWN_NO_CURRENT_CONTEXT");
  });

  it("uses provided current requirement for commitment status", () => {
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

    const view = appGetBuildView({
      build,
      currentEpoch: 1n,
      currentRequiredXntdLock: 200n
    });

    expect(view.commitmentStatus.status).toBe("UNCOMMITTED");
    expect(view.commitmentStatus.reason).toBe("COMMITMENT_BELOW_REQUIRED");
    expect(view.commitmentStatus.requiredXntdLock).toBe(200n);
    expect(view.commitmentStatus.needsRelock).toBe(true);
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

    appGetBuildView({
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

import { describe, expect, it } from "vitest";
import {
  appGetBuildView,
  createEmptyBuildState,
  lockXntd,
} from "../src/index.js";

describe("app Build view", () => {
  it("returns Build state with COMMITTED commitment status", () => {
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

    const view = appGetBuildView({ build });

    expect(view.build).toBe(build);
    expect(view.commitmentStatus.status).toBe("COMMITTED");
    expect(view.commitmentStatus.reason).toBe("COMMITMENT_ACCEPTED");
  });

  it("returns Build state with UNCOMMITTED commitment status", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;

    const view = appGetBuildView({ build });

    expect(view.build).toBe(build);
    expect(view.commitmentStatus.status).toBe("UNCOMMITTED");
    expect(view.commitmentStatus.reason).toBe("NO_COMMITMENT");
  });

  it("does not expose UNKNOWN when current external context is unavailable", () => {
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

    const view = appGetBuildView({ build });

    expect(view.commitmentStatus.status).not.toBe("UNKNOWN");
    expect(view.commitmentStatus.reason).toBe("COMMITMENT_ACCEPTED");
    expect("currentEpoch" in view.commitmentStatus).toBe(false);
  });

  it("uses stable accepted XNTD commitment facts for commitment status", () => {
    const build = createEmptyBuildState({
      buildId: "build-1",
      owner: "owner-1",
      createdAt: 1n,
    });

    build.historyBld = 121n;
    build.lockedXntd = 100n;
    build.requiredXntdLock = 200n;
    build.lockEpoch = 0;
    build.xntdCommitmentAccepted = true;

    const view = appGetBuildView({ build });

    expect(view.commitmentStatus.status).toBe("UNCOMMITTED");
    expect(view.commitmentStatus.reason).toBe("COMMITMENT_INSUFFICIENT");
    expect(view.commitmentStatus.requiredXntdLock).toBe(200n);
  });
});

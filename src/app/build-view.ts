import type { BuildState } from "../model/build-state.js";
import {
  getBuildCommitmentStatus,
  type BuildCommitmentStatus
} from "../model/build-commitment-status.js";

export interface AppBuildView {
  readonly build: BuildState;
  readonly commitmentStatus: BuildCommitmentStatus;
}

export interface AppGetBuildViewInput {
  readonly build: BuildState;
  readonly currentEpoch?: bigint;
  readonly currentRequiredXntdLock?: bigint;
  readonly requireCurrentEpoch?: boolean;
}

export function appGetBuildView(input: AppGetBuildViewInput): AppBuildView {
  const commitmentStatus = getBuildCommitmentStatus({
    build: input.build,
    ...(input.currentEpoch !== undefined
      ? { currentEpoch: input.currentEpoch }
      : {}),
    ...(input.currentRequiredXntdLock !== undefined
      ? { currentRequiredXntdLock: input.currentRequiredXntdLock }
      : {}),
    ...(input.requireCurrentEpoch !== undefined
      ? { requireCurrentEpoch: input.requireCurrentEpoch }
      : {})
  });

  return {
    build: input.build,
    commitmentStatus
  };
}

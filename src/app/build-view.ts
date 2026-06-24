import type { BuildState } from "../model/build-state.js";
import {
  getBuildCommitmentStatus,
  type BuildCommitmentStatus,
} from "../model/build-commitment-status.js";

export interface AppBuildView {
  readonly build: BuildState;
  readonly commitmentStatus: BuildCommitmentStatus;
}

export interface AppGetBuildViewInput {
  readonly build: BuildState;
}

export function appGetBuildView(input: AppGetBuildViewInput): AppBuildView {
  const commitmentStatus = getBuildCommitmentStatus({
    build: input.build,
  });

  return {
    build: input.build,
    commitmentStatus,
  };
}

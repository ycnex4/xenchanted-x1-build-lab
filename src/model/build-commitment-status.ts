import type { BuildState } from "./build-state.js";

export type BuildCommitmentStatusValue = "COMMITTED" | "UNCOMMITTED";

export type BuildCommitmentStatusReason =
  | "COMMITMENT_CURRENT"
  | "NO_HISTORY"
  | "NO_COMMITMENT"
  | "COMMITMENT_BELOW_REQUIRED";

export interface BuildCommitmentStatus {
  readonly isActive: boolean;
  readonly status: BuildCommitmentStatusValue;
  readonly reason: BuildCommitmentStatusReason;
  readonly historyBld: bigint;
  readonly lockedXntd: bigint;
  readonly requiredXntdLock: bigint;
  readonly lockEpoch: number | null;
  readonly needsRelock: boolean;
}

export interface GetBuildCommitmentStatusInput {
  readonly build: BuildState;
}

export function getBuildCommitmentStatus(
  input: GetBuildCommitmentStatusInput,
): BuildCommitmentStatus {
  const { build } = input;

  const historyBld = build.historyBld;
  const lockedXntd = build.lockedXntd;
  const requiredXntdLock = build.requiredXntdLock;
  const lockEpoch = build.lockEpoch;

  if (historyBld === 0n) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "NO_HISTORY",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      needsRelock: false,
    };
  }

  if (!build.xcCommitmentActive || lockedXntd === 0n || lockEpoch === null) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "NO_COMMITMENT",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      needsRelock: false,
    };
  }

  if (requiredXntdLock > 0n && lockedXntd < requiredXntdLock) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "COMMITMENT_BELOW_REQUIRED",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      needsRelock: true,
    };
  }

  return {
    isActive: true,
    status: "COMMITTED",
    reason: "COMMITMENT_CURRENT",
    historyBld,
    lockedXntd,
    requiredXntdLock,
    lockEpoch,
    needsRelock: false,
  };
}

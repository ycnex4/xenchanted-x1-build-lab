import type { BuildState } from "./build-state.js";

export type BuildCommitmentStatusValue = "COMMITTED" | "UNCOMMITTED" | "UNKNOWN";

export type BuildCommitmentStatusReason =
  | "COMMITMENT_CURRENT"
  | "NO_HISTORY"
  | "NO_COMMITMENT"
  | "COMMITMENT_BELOW_REQUIRED"
  | "RECOMMITMENT_REQUIRED"
  | "UNKNOWN_NO_CURRENT_CONTEXT";

export interface BuildCommitmentStatus {
  readonly isActive: boolean;
  readonly status: BuildCommitmentStatusValue;
  readonly reason: BuildCommitmentStatusReason;
  readonly historyBld: bigint;
  readonly availableBld: bigint;
  readonly lockedXntd: bigint;
  readonly requiredXntdLock: bigint;
  readonly lockEpoch: number | null;
  readonly currentEpoch: bigint | null;
  readonly needsRelock: boolean;
}

export interface GetBuildCommitmentStatusInput {
  readonly build: BuildState;
  readonly currentEpoch?: bigint;
  readonly currentRequiredXntdLock?: bigint;
  readonly requireCurrentEpoch?: boolean;
}

export function getBuildCommitmentStatus(
  input: GetBuildCommitmentStatusInput
): BuildCommitmentStatus {
  const { build } = input;

  const historyBld = build.historyBld;
  const availableBld = build.availableBld;
  const lockedXntd = build.lockedXntd;
  const storedRequiredXntdLock = build.requiredXntdLock;
  const lockEpoch = build.lockEpoch;
  const currentEpoch = input.currentEpoch ?? null;

  const requiredXntdLock =
    input.currentRequiredXntdLock ?? storedRequiredXntdLock;

  if (input.requireCurrentEpoch === true && input.currentEpoch === undefined) {
    return {
      isActive: false,
      status: "UNKNOWN",
      reason: "UNKNOWN_NO_CURRENT_CONTEXT",
      historyBld,
      availableBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      currentEpoch,
      needsRelock: false
    };
  }

  if (historyBld === 0n) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "NO_HISTORY",
      historyBld,
      availableBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      currentEpoch,
      needsRelock: false
    };
  }

  if (lockedXntd === 0n || lockEpoch === null) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "NO_COMMITMENT",
      historyBld,
      availableBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      currentEpoch,
      needsRelock: false
    };
  }

  if (requiredXntdLock > 0n && lockedXntd < requiredXntdLock) {
    return {
      isActive: false,
      status: "UNCOMMITTED",
      reason: "COMMITMENT_BELOW_REQUIRED",
      historyBld,
      availableBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
      currentEpoch,
      needsRelock: true
    };
  }

  return {
    isActive: true,
    status: "COMMITTED",
    reason: "COMMITMENT_CURRENT",
    historyBld,
    availableBld,
    lockedXntd,
    requiredXntdLock,
    lockEpoch,
    currentEpoch,
    needsRelock: false
  };
}

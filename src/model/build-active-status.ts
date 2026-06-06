import type { BuildState } from "./build-state.js";

export type BuildActiveStatusValue = "ACTIVE" | "INACTIVE" | "UNKNOWN";

export type BuildActiveStatusReason =
  | "ACTIVE_LOCK_CURRENT"
  | "INACTIVE_NO_HISTORY"
  | "INACTIVE_NO_LOCK"
  | "INACTIVE_LOCK_BELOW_REQUIRED"
  | "INACTIVE_RELOCK_REQUIRED"
  | "UNKNOWN_NO_CURRENT_CONTEXT";

export interface BuildActiveStatus {
  readonly isActive: boolean;
  readonly status: BuildActiveStatusValue;
  readonly reason: BuildActiveStatusReason;
  readonly historyBld: bigint;
  readonly availableBld: bigint;
  readonly lockedXntd: bigint;
  readonly requiredXntdLock: bigint;
  readonly lockEpoch: number | null;
  readonly currentEpoch: bigint | null;
  readonly needsRelock: boolean;
}

export interface GetBuildActiveStatusInput {
  readonly build: BuildState;
  readonly currentEpoch?: bigint;
  readonly currentRequiredXntdLock?: bigint;
  readonly requireCurrentEpoch?: boolean;
}

export function getBuildActiveStatus(
  input: GetBuildActiveStatusInput
): BuildActiveStatus {
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
      status: "INACTIVE",
      reason: "INACTIVE_NO_HISTORY",
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
      status: "INACTIVE",
      reason: "INACTIVE_NO_LOCK",
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
      status: "INACTIVE",
      reason: "INACTIVE_LOCK_BELOW_REQUIRED",
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
    status: "ACTIVE",
    reason: "ACTIVE_LOCK_CURRENT",
    historyBld,
    availableBld,
    lockedXntd,
    requiredXntdLock,
    lockEpoch,
    currentEpoch,
    needsRelock: false
  };
}

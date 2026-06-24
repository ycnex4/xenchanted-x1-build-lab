import type { BuildState } from "./build-state.js";

export type BuildCommitmentStatusValue = "COMMITTED" | "UNCOMMITTED";

export type BuildCommitmentStatusReason =
  | "COMMITMENT_ACCEPTED"
  | "NO_HISTORY"
  | "NO_COMMITMENT"
  | "COMMITMENT_INSUFFICIENT";

export interface BuildCommitmentStatus {
  readonly isCommitted: boolean;
  readonly status: BuildCommitmentStatusValue;
  readonly reason: BuildCommitmentStatusReason;
  readonly historyBld: bigint;
  readonly lockedXntd: bigint;
  readonly requiredXntdLock: bigint;
  readonly lockEpoch: number | null;
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
      isCommitted: false,
      status: "UNCOMMITTED",
      reason: "NO_HISTORY",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
    };
  }

  if (
    !build.xntdCommitmentAccepted ||
    lockedXntd === 0n ||
    lockEpoch === null
  ) {
    return {
      isCommitted: false,
      status: "UNCOMMITTED",
      reason: "NO_COMMITMENT",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
    };
  }

  if (requiredXntdLock > 0n && lockedXntd < requiredXntdLock) {
    return {
      isCommitted: false,
      status: "UNCOMMITTED",
      reason: "COMMITMENT_INSUFFICIENT",
      historyBld,
      lockedXntd,
      requiredXntdLock,
      lockEpoch,
    };
  }

  return {
    isCommitted: true,
    status: "COMMITTED",
    reason: "COMMITMENT_ACCEPTED",
    historyBld,
    lockedXntd,
    requiredXntdLock,
    lockEpoch,
  };
}

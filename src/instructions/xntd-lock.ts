import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface LockXntdInput {
  build: BuildState;
  amountXntd: bigint;
  lockEpoch: number;
  lockedAt: bigint;
}

export interface RelockXntdInput {
  build: BuildState;
  amountXntd: bigint;
  lockEpoch: number;
  relockedAt: bigint;
}

function assertPositiveXntdLockAmount(amountXntd: bigint): void {
  if (amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${amountXntd.toString()}`
    );
  }
}

function assertRelockBldIntegrity(build: BuildState): void {
  if (build.availableBld < build.historyBld) {
    throw new BuildError(
      BuildErrorCode.InsufficientAvailableBldForRelock,
      `Relock requires availableBld >= historyBld: available=${build.availableBld.toString()}, history=${build.historyBld.toString()}`
    );
  }
}

export function lockXntd(input: LockXntdInput): BuildState {
  assertPositiveXntdLockAmount(input.amountXntd);

  input.build.lockedXntd = input.amountXntd;
  input.build.requiredXntdLock = input.amountXntd;
  input.build.lockEpoch = input.lockEpoch;
  input.build.xcCommitmentActive = true;
  input.build.updatedAt = input.lockedAt;

  return input.build;
}

export function relockXntd(input: RelockXntdInput): BuildState {
  assertPositiveXntdLockAmount(input.amountXntd);

  if (!input.build.xcCommitmentActive) {
    throw new BuildError(
      BuildErrorCode.XntdCommitmentNotActive,
      "Cannot relock XNTD when XC commitment is not active"
    );
  }

  assertRelockBldIntegrity(input.build);

  input.build.lockedXntd = input.amountXntd;
  input.build.requiredXntdLock = input.amountXntd;
  input.build.lockEpoch = input.lockEpoch;
  input.build.xcCommitmentActive = true;
  input.build.updatedAt = input.relockedAt;

  return input.build;
}

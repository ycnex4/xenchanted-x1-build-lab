import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface LockXntdInput {
  build: BuildState;
  amountXntd: bigint;
  observedRequiredXntdLock: bigint;
  lockEpoch: number;
  lockedAt: bigint;
}

export interface RelockXntdInput {
  build: BuildState;
  amountXntd: bigint;
  observedRequiredXntdLock: bigint;
  lockEpoch: number;
  relockedAt: bigint;
}

function assertPositiveXntdLockAmount(amountXntd: bigint): void {
  if (amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${amountXntd.toString()}`,
    );
  }
}

function assertPositiveObservedRequiredXntdLock(
  observedRequiredXntdLock: bigint,
): void {
  if (observedRequiredXntdLock <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `Observed required XNTD lock amount must be positive: ${observedRequiredXntdLock.toString()}`,
    );
  }
}

function assertSufficientXntdLockAmount(
  amountXntd: bigint,
  observedRequiredXntdLock: bigint,
): void {
  if (amountXntd < observedRequiredXntdLock) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must cover observed required lock: amount=${amountXntd.toString()}, required=${observedRequiredXntdLock.toString()}`,
    );
  }
}

export function lockXntd(input: LockXntdInput): BuildState {
  assertPositiveXntdLockAmount(input.amountXntd);
  assertPositiveObservedRequiredXntdLock(input.observedRequiredXntdLock);
  assertSufficientXntdLockAmount(
    input.amountXntd,
    input.observedRequiredXntdLock,
  );

  input.build.lockedXntd = input.amountXntd;
  input.build.requiredXntdLock = input.observedRequiredXntdLock;
  input.build.lockEpoch = input.lockEpoch;
  input.build.xntdCommitmentAccepted = true;
  input.build.updatedAt = input.lockedAt;

  return input.build;
}

export function relockXntd(input: RelockXntdInput): BuildState {
  assertPositiveXntdLockAmount(input.amountXntd);
  assertPositiveObservedRequiredXntdLock(input.observedRequiredXntdLock);
  assertSufficientXntdLockAmount(
    input.amountXntd,
    input.observedRequiredXntdLock,
  );

  if (!input.build.xntdCommitmentAccepted) {
    throw new BuildError(
      BuildErrorCode.XntdCommitmentNotAccepted,
      "Cannot relock XNTD when XNTD commitment is not accepted",
    );
  }

  input.build.lockedXntd = input.amountXntd;
  input.build.requiredXntdLock = input.observedRequiredXntdLock;
  input.build.lockEpoch = input.lockEpoch;
  input.build.xntdCommitmentAccepted = true;
  input.build.updatedAt = input.relockedAt;

  return input.build;
}

import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface ApplyX1FeeContributionCheckpointInput {
  build: BuildState;
  feeAmount: bigint;
  txCount: bigint;
  countedUntilSlot: bigint;
  updatedAt: bigint;
}

export function applyX1FeeContributionCheckpoint(
  input: ApplyX1FeeContributionCheckpointInput
): BuildState {
  if (input.feeAmount <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidFeeContributionAmount,
      `X1 fee contribution amount must be positive: ${input.feeAmount.toString()}`
    );
  }

  if (input.txCount <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidFeeContributionTxCount,
      `X1 fee contribution txCount must be positive: ${input.txCount.toString()}`
    );
  }

  if (
    input.build.x1FeeCountedUntilSlot !== null &&
    input.countedUntilSlot <= input.build.x1FeeCountedUntilSlot
  ) {
    throw new BuildError(
      BuildErrorCode.NonIncreasingFeeCheckpointSlot,
      `X1 fee checkpoint slot must increase: current=${input.build.x1FeeCountedUntilSlot.toString()}, next=${input.countedUntilSlot.toString()}`
    );
  }

  input.build.x1FeeContribution += input.feeAmount;
  input.build.x1TxCount += input.txCount;
  input.build.x1FeeCountedUntilSlot = input.countedUntilSlot;
  input.build.lastFeeUpdateAt = input.updatedAt;
  input.build.updatedAt = input.updatedAt;

  return input.build;
}

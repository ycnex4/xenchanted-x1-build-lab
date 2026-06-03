import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface ApplyXenBurnPowerInput {
  build: BuildState;
  amountXbp: bigint;
  burnedAt: bigint;
}

export function applyXenBurnPower(input: ApplyXenBurnPowerInput): BuildState {
  if (input.amountXbp <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXbpAmount,
      `XEN Burn Power amount must be positive: ${input.amountXbp.toString()}`
    );
  }

  input.build.earnedXbp += input.amountXbp;
  input.build.availableXbp += input.amountXbp;
  input.build.updatedAt = input.burnedAt;

  return input.build;
}

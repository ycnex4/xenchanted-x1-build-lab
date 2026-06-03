import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface ApplyCoreRedeemBldInput {
  build: BuildState;
  amountBld: bigint;
  redeemedAt: bigint;
}

export function applyCoreRedeemBld(input: ApplyCoreRedeemBldInput): BuildState {
  if (input.amountBld <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidBldAmount,
      `Core redeem BLD amount must be positive: ${input.amountBld.toString()}`
    );
  }

  input.build.historyBld += input.amountBld;
  input.build.availableBld += input.amountBld;
  input.build.updatedAt = input.redeemedAt;

  return input.build;
}

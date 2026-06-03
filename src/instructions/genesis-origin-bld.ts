import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface ClaimGenesisOriginBldInput {
  build: BuildState;
  claimedAt: bigint;
}

export function calculateGenesisOriginBld(historyBld: bigint): bigint {
  if (historyBld >= 1111n) {
    return 121n;
  }

  if (historyBld >= 121n) {
    return 55n;
  }

  if (historyBld >= 11n) {
    return 22n;
  }

  if (historyBld >= 1n) {
    return 11n;
  }

  return 0n;
}

export function claimGenesisOriginBld(
  input: ClaimGenesisOriginBldInput
): BuildState {
  if (input.build.originBld > 0n) {
    throw new BuildError(
      BuildErrorCode.GenesisOriginAlreadyClaimed,
      "Genesis Origin BLD has already been claimed for this Build"
    );
  }

  const originBld = calculateGenesisOriginBld(input.build.historyBld);

  if (originBld <= 0n) {
    throw new BuildError(
      BuildErrorCode.GenesisOriginNotEligible,
      `Build is not eligible for Genesis Origin BLD with historyBld: ${input.build.historyBld.toString()}`
    );
  }

  input.build.originBld = originBld;
  input.build.availableBld += originBld;
  input.build.updatedAt = input.claimedAt;

  return input.build;
}

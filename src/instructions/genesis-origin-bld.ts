import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";

export interface UpgradeGenesisOriginBldInput {
  build: BuildState;
  upgradedAt: bigint;
}

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

export function calculateGenesisOriginBldDelta(
  currentOriginBld: bigint,
  historyBld: bigint,
): bigint {
  const eligibleOriginBld = calculateGenesisOriginBld(historyBld);

  if (eligibleOriginBld <= currentOriginBld) {
    return 0n;
  }

  return eligibleOriginBld - currentOriginBld;
}

export function upgradeGenesisOriginBld(
  input: UpgradeGenesisOriginBldInput,
): BuildState {
  const eligibleOriginBld = calculateGenesisOriginBld(input.build.historyBld);
  const deltaOriginBld = calculateGenesisOriginBldDelta(
    input.build.originBld,
    input.build.historyBld,
  );

  if (eligibleOriginBld <= 0n || deltaOriginBld <= 0n) {
    throw new BuildError(
      BuildErrorCode.GenesisOriginNotEligible,
      `Build is not eligible for a Genesis Origin BLD upgrade: historyBld=${input.build.historyBld.toString()}, currentOriginBld=${input.build.originBld.toString()}`,
    );
  }

  input.build.originBld = eligibleOriginBld;
  input.build.updatedAt = input.upgradedAt;

  return input.build;
}

export function claimGenesisOriginBld(
  input: ClaimGenesisOriginBldInput,
): BuildState {
  return upgradeGenesisOriginBld({
    build: input.build,
    upgradedAt: input.claimedAt,
  });
}

import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState, type X1Address } from "../model/build-state.js";

export interface UpdateBuildIdentityInput {
  build: BuildState;
  owner: X1Address;
  buildName?: string | null;
  logoUri?: string | null;
  updatedAt: bigint;
}

export function updateBuildIdentity(
  input: UpdateBuildIdentityInput,
): BuildState {
  if (input.owner !== input.build.owner) {
    throw new BuildError(
      BuildErrorCode.UnauthorizedBuildIdentityUpdate,
      `Only the Build owner can update Build Identity: owner=${input.owner}, buildOwner=${input.build.owner}`,
    );
  }

  const hasBuildNameUpdate = input.buildName !== undefined;
  const hasLogoUriUpdate = input.logoUri !== undefined;

  if (!hasBuildNameUpdate && !hasLogoUriUpdate) {
    return input.build;
  }

  if (hasBuildNameUpdate) {
    input.build.buildName = input.buildName ?? null;
  }

  if (hasLogoUriUpdate) {
    input.build.logoUri = input.logoUri ?? null;
  }

  input.build.metadataUpdatedAt = input.updatedAt;
  input.build.updatedAt = input.updatedAt;

  return input.build;
}

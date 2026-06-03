import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import {
  type BuildId,
  type BuildState,
  type EthereumAddress,
  type X1Address
} from "./build-state.js";
import { type CreateBuildInput, createBuild } from "../instructions/create-build.js";

export interface BuildRegistry {
  buildsById: Map<BuildId, BuildState>;
  canonicalBuildByOwner: Map<X1Address, BuildId>;
  canonicalBuildByEthereumIdentity: Map<EthereumAddress, BuildId>;
}

export function createEmptyBuildRegistry(): BuildRegistry {
  return {
    buildsById: new Map<BuildId, BuildState>(),
    canonicalBuildByOwner: new Map<X1Address, BuildId>(),
    canonicalBuildByEthereumIdentity: new Map<EthereumAddress, BuildId>()
  };
}

export function createRegisteredBuild(
  registry: BuildRegistry,
  input: CreateBuildInput
): BuildState {
  if (registry.buildsById.has(input.buildId)) {
    throw new BuildError(
      BuildErrorCode.DuplicateBuildId,
      `Build already exists for buildId: ${input.buildId}`
    );
  }

  if (registry.canonicalBuildByOwner.has(input.owner)) {
    throw new BuildError(
      BuildErrorCode.DuplicateBuildOwner,
      `Canonical Build already exists for owner: ${input.owner}`
    );
  }

  const ethereumIdentity = input.ethereumIdentity ?? null;

  if (
    ethereumIdentity !== null &&
    registry.canonicalBuildByEthereumIdentity.has(ethereumIdentity)
  ) {
    throw new BuildError(
      BuildErrorCode.DuplicateEthereumIdentity,
      `Canonical Build already exists for Ethereum identity: ${ethereumIdentity}`
    );
  }

  const state = createBuild(input);

  registry.buildsById.set(state.buildId, state);
  registry.canonicalBuildByOwner.set(state.owner, state.buildId);

  if (state.ethereumIdentity !== null) {
    registry.canonicalBuildByEthereumIdentity.set(
      state.ethereumIdentity,
      state.buildId
    );
  }

  return state;
}

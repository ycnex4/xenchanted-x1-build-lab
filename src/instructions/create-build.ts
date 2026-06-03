import {
  type BuildId,
  type BuildState,
  type EthereumAddress,
  type X1Address,
  createEmptyBuildState
} from "../model/build-state.js";

export interface CreateBuildInput {
  owner: X1Address;
  buildId: BuildId;
  createdAt: bigint;
  ethereumIdentity?: EthereumAddress | null;
}

export function createBuild(input: CreateBuildInput): BuildState {
  return createEmptyBuildState({
    owner: input.owner,
    buildId: input.buildId,
    createdAt: input.createdAt,
    ethereumIdentity: input.ethereumIdentity ?? null
  });
}

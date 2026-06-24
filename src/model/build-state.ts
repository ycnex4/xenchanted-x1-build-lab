export type BuildId = string;
export type X1Address = string;
export type EthereumAddress = string;

export const BUILD_STATE_VERSION = 1;

export interface BuildState {
  owner: X1Address;
  buildId: BuildId;
  version: number;
  createdAt: bigint;
  updatedAt: bigint;

  ethereumIdentity: EthereumAddress | null;

  buildName: string | null;
  logoUri: string | null;
  metadataUpdatedAt: bigint | null;

  historyBld: bigint;
  originBld: bigint;

  historyXbp: bigint;

  lockedXntd: bigint;
  requiredXntdLock: bigint;
  lockEpoch: number | null;
  xcCommitmentActive: boolean;

  x1FeeContribution: bigint;
  x1TxCount: bigint;
  x1FeeCountedUntilSlot: bigint | null;
  lastFeeUpdateAt: bigint | null;
}

export interface CreateEmptyBuildStateInput {
  owner: X1Address;
  buildId: BuildId;
  createdAt: bigint;
  ethereumIdentity?: EthereumAddress | null;
  buildName?: string | null;
  logoUri?: string | null;
}

export function createEmptyBuildState(
  input: CreateEmptyBuildStateInput,
): BuildState {
  const hasInitialIdentity = input.buildName != null || input.logoUri != null;

  return {
    owner: input.owner,
    buildId: input.buildId,
    version: BUILD_STATE_VERSION,
    createdAt: input.createdAt,
    updatedAt: input.createdAt,

    ethereumIdentity: input.ethereumIdentity ?? null,

    buildName: input.buildName ?? null,
    logoUri: input.logoUri ?? null,
    metadataUpdatedAt: hasInitialIdentity ? input.createdAt : null,

    historyBld: 0n,
    originBld: 0n,

    historyXbp: 0n,

    lockedXntd: 0n,
    requiredXntdLock: 0n,
    lockEpoch: null,
    xcCommitmentActive: false,

    x1FeeContribution: 0n,
    x1TxCount: 0n,
    x1FeeCountedUntilSlot: null,
    lastFeeUpdateAt: null,
  };
}

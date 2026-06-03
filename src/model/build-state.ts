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

  historyBld: bigint;
  availableBld: bigint;
  originBld: bigint;

  earnedXbp: bigint;
  availableXbp: bigint;

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
}

export function createEmptyBuildState(input: CreateEmptyBuildStateInput): BuildState {
  return {
    owner: input.owner,
    buildId: input.buildId,
    version: BUILD_STATE_VERSION,
    createdAt: input.createdAt,
    updatedAt: input.createdAt,

    ethereumIdentity: input.ethereumIdentity ?? null,

    historyBld: 0n,
    availableBld: 0n,
    originBld: 0n,

    earnedXbp: 0n,
    availableXbp: 0n,

    lockedXntd: 0n,
    requiredXntdLock: 0n,
    lockEpoch: null,
    xcCommitmentActive: false,

    x1FeeContribution: 0n,
    x1TxCount: 0n,
    x1FeeCountedUntilSlot: null,
    lastFeeUpdateAt: null
  };
}

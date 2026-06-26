export const XXXL_PROGRAM_VERSION = 1 as const;

export const ETHEREUM_MAINNET_CHAIN_ID = 1 as const;

export const XXXL_GATEWAY_ROUTE_ID = "ETHEREUM_XNTD_TO_X1_XXXL" as const;

export const XXXL_MINT_TOKEN = "XXXL" as const;

export const XXXL_GENESIS_PHASE = "GATEWAY_ONLY" as const;

export const XXXL_TEMPORARY_UPGRADE_AUTHORITY_STATUS =
  "TEMPORARY_PROTOCOL_FINALIZATION" as const;

export const XXXL_FINAL_UPGRADE_AUTHORITY_STATUS = "REMOVED_OR_FROZEN" as const;

export type XXXLGenesisPhase = typeof XXXL_GENESIS_PHASE;

export type XXXLUpgradeAuthorityStatus =
  | typeof XXXL_TEMPORARY_UPGRADE_AUTHORITY_STATUS
  | typeof XXXL_FINAL_UPGRADE_AUTHORITY_STATUS;

export enum XXXLProgramErrorCode {
  INVALID_STATE_VERSION = "INVALID_STATE_VERSION",
  INVALID_ROUTE_ID = "INVALID_ROUTE_ID",
  INVALID_SOURCE_CHAIN = "INVALID_SOURCE_CHAIN",
  INVALID_SOURCE_TOKEN = "INVALID_SOURCE_TOKEN",
  INVALID_SOURCE_SENDER = "INVALID_SOURCE_SENDER",
  INVALID_SOURCE_BURN_TX_HASH = "INVALID_SOURCE_BURN_TX_HASH",
  INVALID_SOURCE_BURN_EVENT_INDEX = "INVALID_SOURCE_BURN_EVENT_INDEX",
  INVALID_SOURCE_BLOCK_NUMBER = "INVALID_SOURCE_BLOCK_NUMBER",
  INVALID_SOURCE_BLOCK_HASH = "INVALID_SOURCE_BLOCK_HASH",
  INVALID_CANONICAL_EVENT_KEY = "INVALID_CANONICAL_EVENT_KEY",
  INVALID_X1_RECIPIENT = "INVALID_X1_RECIPIENT",
  INVALID_MINT_TOKEN = "INVALID_MINT_TOKEN",
  INVALID_MINT_AMOUNT = "INVALID_MINT_AMOUNT",
  REPLAYED_GATEWAY_EVENT = "REPLAYED_GATEWAY_EVENT",
  MANUAL_MINT_FORBIDDEN = "MANUAL_MINT_FORBIDDEN",
}

export class XXXLProgramError extends Error {
  public readonly code: XXXLProgramErrorCode;

  public constructor(code: XXXLProgramErrorCode) {
    super(code);
    this.name = "XXXLProgramError";
    this.code = code;
  }
}

export type XXXLProgramState = {
  readonly programVersion: typeof XXXL_PROGRAM_VERSION;
  readonly genesisPhase: XXXLGenesisPhase;
  readonly upgradeAuthorityStatus: XXXLUpgradeAuthorityStatus;
  readonly totalSupply: bigint;
  readonly processedGatewayEvents: ReadonlySet<string>;
};

export type XXXLGatewayMintAuthorization = {
  readonly routeId: string;
  readonly sourceChainId: number;
  readonly sourceToken: string;
  readonly sourceSender: string;
  readonly sourceBurnTxHash: string;
  readonly sourceBurnEventIndex: number;
  readonly sourceBlockNumber: bigint;
  readonly sourceBlockHash: string;
  readonly canonicalEventKey: string;
  readonly x1Recipient: string;
  readonly xxxlMintAmount: bigint;
  readonly mintToken: string;
};

export type XXXLManualMintAttempt = {
  readonly recipient: string;
  readonly amount: bigint;
};

export function createEmptyXXXLProgramState(
  options: {
    readonly upgradeAuthorityStatus?: XXXLUpgradeAuthorityStatus;
  } = {},
): XXXLProgramState {
  return {
    programVersion: XXXL_PROGRAM_VERSION,
    genesisPhase: XXXL_GENESIS_PHASE,
    upgradeAuthorityStatus:
      options.upgradeAuthorityStatus ?? XXXL_TEMPORARY_UPGRADE_AUTHORITY_STATUS,
    totalSupply: 0n,
    processedGatewayEvents: new Set<string>(),
  };
}

export function processXXXLGatewayMintAuthorization(
  state: XXXLProgramState,
  authorization: XXXLGatewayMintAuthorization,
): XXXLProgramState {
  assertValidState(state);
  assertValidGatewayMintAuthorization(authorization);

  if (state.processedGatewayEvents.has(authorization.canonicalEventKey)) {
    throw new XXXLProgramError(XXXLProgramErrorCode.REPLAYED_GATEWAY_EVENT);
  }

  const processedGatewayEvents = new Set(state.processedGatewayEvents);
  processedGatewayEvents.add(authorization.canonicalEventKey);

  return {
    ...state,
    totalSupply: state.totalSupply + authorization.xxxlMintAmount,
    processedGatewayEvents,
  };
}

export function rejectManualXXXLMint(
  _attempt: XXXLManualMintAttempt,
): never {
  throw new XXXLProgramError(XXXLProgramErrorCode.MANUAL_MINT_FORBIDDEN);
}

export function assertGatewaySupplyInvariant(
  before: XXXLProgramState,
  after: XXXLProgramState,
  authorization: XXXLGatewayMintAuthorization,
): void {
  const expectedSupply = before.totalSupply + authorization.xxxlMintAmount;

  if (after.totalSupply !== expectedSupply) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_MINT_AMOUNT);
  }

  if (!after.processedGatewayEvents.has(authorization.canonicalEventKey)) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_CANONICAL_EVENT_KEY);
  }
}

function assertValidState(state: XXXLProgramState): void {
  if (state.programVersion !== XXXL_PROGRAM_VERSION) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_STATE_VERSION);
  }
}

function assertValidGatewayMintAuthorization(
  authorization: XXXLGatewayMintAuthorization,
): void {
  if (authorization.routeId !== XXXL_GATEWAY_ROUTE_ID) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_ROUTE_ID);
  }

  if (authorization.sourceChainId !== ETHEREUM_MAINNET_CHAIN_ID) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_SOURCE_CHAIN);
  }

  assertNonEmptyString(
    authorization.sourceToken,
    XXXLProgramErrorCode.INVALID_SOURCE_TOKEN,
  );

  assertNonEmptyString(
    authorization.sourceSender,
    XXXLProgramErrorCode.INVALID_SOURCE_SENDER,
  );

  assertNonEmptyString(
    authorization.sourceBurnTxHash,
    XXXLProgramErrorCode.INVALID_SOURCE_BURN_TX_HASH,
  );

  if (
    !Number.isInteger(authorization.sourceBurnEventIndex) ||
    authorization.sourceBurnEventIndex < 0
  ) {
    throw new XXXLProgramError(
      XXXLProgramErrorCode.INVALID_SOURCE_BURN_EVENT_INDEX,
    );
  }

  if (authorization.sourceBlockNumber <= 0n) {
    throw new XXXLProgramError(
      XXXLProgramErrorCode.INVALID_SOURCE_BLOCK_NUMBER,
    );
  }

  assertNonEmptyString(
    authorization.sourceBlockHash,
    XXXLProgramErrorCode.INVALID_SOURCE_BLOCK_HASH,
  );

  assertNonEmptyString(
    authorization.canonicalEventKey,
    XXXLProgramErrorCode.INVALID_CANONICAL_EVENT_KEY,
  );

  assertNonEmptyString(
    authorization.x1Recipient,
    XXXLProgramErrorCode.INVALID_X1_RECIPIENT,
  );

  if (authorization.mintToken !== XXXL_MINT_TOKEN) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_MINT_TOKEN);
  }

  if (authorization.xxxlMintAmount <= 0n) {
    throw new XXXLProgramError(XXXLProgramErrorCode.INVALID_MINT_AMOUNT);
  }
}

function assertNonEmptyString(
  value: string,
  errorCode: XXXLProgramErrorCode,
): void {
  if (value.trim().length === 0) {
    throw new XXXLProgramError(errorCode);
  }
}

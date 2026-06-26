import { type XXXLProgramState } from "./program-v1.js";

export const XXXL_GENESIS_SUPPLY_INVARIANT_ERROR = {
  InvalidGatewayMintAmount: "INVALID_GATEWAY_MINT_AMOUNT",
  ReplayedGatewayEvent: "REPLAYED_GATEWAY_EVENT",
  MissingProcessedGatewayEvent: "MISSING_PROCESSED_GATEWAY_EVENT",
  ProcessedGatewayEventDeltaMismatch: "PROCESSED_GATEWAY_EVENT_DELTA_MISMATCH",
  GatewaySupplyDeltaMismatch: "GATEWAY_SUPPLY_DELTA_MISMATCH",
  UnauthorizedSupplyIncrease: "UNAUTHORIZED_SUPPLY_INCREASE",
  RejectedTransitionMutatedSupply: "REJECTED_TRANSITION_MUTATED_SUPPLY",
  RejectedTransitionMutatedReplayState: "REJECTED_TRANSITION_MUTATED_REPLAY_STATE",
  AcceptedGatewayMintSumMismatch: "ACCEPTED_GATEWAY_MINT_SUM_MISMATCH",
} as const;

export type XXXLGenesisSupplyInvariantErrorCode =
  (typeof XXXL_GENESIS_SUPPLY_INVARIANT_ERROR)[keyof typeof XXXL_GENESIS_SUPPLY_INVARIANT_ERROR];

export class XXXLGenesisSupplyInvariantError extends Error {
  public readonly code: XXXLGenesisSupplyInvariantErrorCode;

  public constructor(code: XXXLGenesisSupplyInvariantErrorCode) {
    super(code);
    this.name = "XXXLGenesisSupplyInvariantError";
    this.code = code;
  }
}

export type XXXLGenesisGatewayMintDelta = {
  readonly canonicalEventKeyHex: string;
  readonly amount: bigint;
};

export function assertGenesisGatewayMintSupplyDelta(
  before: XXXLProgramState,
  after: XXXLProgramState,
  delta: XXXLGenesisGatewayMintDelta,
): void {
  if (delta.amount <= 0n) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.InvalidGatewayMintAmount,
    );
  }

  const canonicalEventKeyHex = delta.canonicalEventKeyHex.toLowerCase();

  if (before.processedGatewayEvents.has(canonicalEventKeyHex)) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.ReplayedGatewayEvent,
    );
  }

  if (!after.processedGatewayEvents.has(canonicalEventKeyHex)) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.MissingProcessedGatewayEvent,
    );
  }

  if (!isStrictSingleGatewayEventDelta(before, after, canonicalEventKeyHex)) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.ProcessedGatewayEventDeltaMismatch,
    );
  }

  if (after.totalSupply !== before.totalSupply + delta.amount) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.GatewaySupplyDeltaMismatch,
    );
  }
}

export function assertNoUnauthorizedGenesisSupplyIncrease(
  before: XXXLProgramState,
  after: XXXLProgramState,
): void {
  if (after.totalSupply > before.totalSupply) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.UnauthorizedSupplyIncrease,
    );
  }
}

export function assertRejectedGenesisTransitionPreservesState(
  before: XXXLProgramState,
  after: XXXLProgramState,
): void {
  if (after.totalSupply !== before.totalSupply) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.RejectedTransitionMutatedSupply,
    );
  }

  if (!setsEqual(before.processedGatewayEvents, after.processedGatewayEvents)) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.RejectedTransitionMutatedReplayState,
    );
  }
}

export function acceptedGatewayMintAmountSum(amounts: readonly bigint[]): bigint {
  return amounts.reduce((sum, amount) => {
    if (amount <= 0n) {
      throw new XXXLGenesisSupplyInvariantError(
        XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.InvalidGatewayMintAmount,
      );
    }

    return sum + amount;
  }, 0n);
}

export function assertGenesisSupplyEqualsAcceptedGatewayMintSum(
  state: XXXLProgramState,
  acceptedGatewayMintAmounts: readonly bigint[],
): void {
  const expectedSupply = acceptedGatewayMintAmountSum(acceptedGatewayMintAmounts);

  if (state.totalSupply !== expectedSupply) {
    throw new XXXLGenesisSupplyInvariantError(
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.AcceptedGatewayMintSumMismatch,
    );
  }
}

function isStrictSingleGatewayEventDelta(
  before: XXXLProgramState,
  after: XXXLProgramState,
  addedCanonicalEventKeyHex: string,
): boolean {
  if (after.processedGatewayEvents.size !== before.processedGatewayEvents.size + 1) {
    return false;
  }

  for (const key of before.processedGatewayEvents) {
    if (!after.processedGatewayEvents.has(key)) {
      return false;
    }
  }

  return after.processedGatewayEvents.has(addedCanonicalEventKeyHex);
}

function setsEqual(left: ReadonlySet<string>, right: ReadonlySet<string>): boolean {
  if (left.size !== right.size) {
    return false;
  }

  for (const value of left) {
    if (!right.has(value)) {
      return false;
    }
  }

  return true;
}

import { bytesToHex, type Stage1GatewayMintMessageFields } from "../gateway/stage-1-encoding.js";
import { type Stage1MintAuthorizationResult } from "../gateway/stage-1-mint-authorization.js";
import { stage1MintAmountFromFields } from "../gateway/stage-1-mint-core.js";
import { stage1CanonicalEventKeyHex } from "../gateway/stage-1-processed-burn-registry.js";
import { type XXXLProgramState } from "./program-v1.js";

export const XXXL_STAGE1_GATEWAY_CONSUMER_ERROR = {
  Stage1MintNotAuthorized: "STAGE1_MINT_NOT_AUTHORIZED",
  Stage1MintNotMarkedProcessed: "STAGE1_MINT_NOT_MARKED_PROCESSED",
  ReplayedGatewayEvent: "REPLAYED_GATEWAY_EVENT",
  InvalidMintAmount: "INVALID_MINT_AMOUNT",
} as const;

export type XXXLStage1GatewayConsumerErrorCode =
  (typeof XXXL_STAGE1_GATEWAY_CONSUMER_ERROR)[keyof typeof XXXL_STAGE1_GATEWAY_CONSUMER_ERROR];

export type XXXLStage1GatewayAuthorizationContract = {
  readonly authorizationOk: boolean;
  readonly authorized: boolean;
  readonly markedProcessed: boolean;
  readonly canonicalEventKey: Uint8Array;
  readonly amount: bigint;
};

export type XXXLStage1GatewayMintConsumerInput = {
  readonly state: XXXLProgramState;
  readonly fields: Pick<
    Stage1GatewayMintMessageFields,
    "canonicalEventKey" | "xxxlMintAmount"
  >;
  readonly x1RecipientBytes: Uint8Array;
  readonly authorization: Stage1MintAuthorizationResult;
};

export type XXXLStage1GatewayMintConsumerResult = {
  readonly ok: boolean;
  readonly minted: boolean;
  readonly state: XXXLProgramState;
  readonly canonicalEventKeyHex: string;
  readonly recipientHex: string;
  readonly amount: bigint;
  readonly totalSupplyAfter: bigint;
  readonly errors: XXXLStage1GatewayConsumerErrorCode[];
  readonly authorization: Stage1MintAuthorizationResult;
};

export function toXXXLStage1GatewayAuthorizationContract(
  input: Pick<XXXLStage1GatewayMintConsumerInput, "fields" | "authorization">,
): XXXLStage1GatewayAuthorizationContract {
  return {
    authorizationOk: input.authorization.ok,
    authorized: input.authorization.authorized,
    markedProcessed: input.authorization.markedProcessed,
    canonicalEventKey: input.fields.canonicalEventKey,
    amount: stage1MintAmountFromFields(input.fields),
  };
}

export function processXXXLStage1GatewayMintAuthorization(
  input: XXXLStage1GatewayMintConsumerInput,
): XXXLStage1GatewayMintConsumerResult {
  const contract = toXXXLStage1GatewayAuthorizationContract(input);
  const canonicalEventKeyHex = stage1CanonicalEventKeyHex(
    contract.canonicalEventKey,
  );
  const recipientHex = bytesToHex(input.x1RecipientBytes).toLowerCase();
  const amount = contract.amount;
  const errors: XXXLStage1GatewayConsumerErrorCode[] = [];

  if (!contract.authorizationOk || !contract.authorized) {
    errors.push(XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotAuthorized);
  }

  if (!contract.markedProcessed) {
    errors.push(
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotMarkedProcessed,
    );
  }

  if (input.state.processedGatewayEvents.has(canonicalEventKeyHex)) {
    errors.push(XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.ReplayedGatewayEvent);
  }

  if (amount <= 0n) {
    errors.push(XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.InvalidMintAmount);
  }

  if (errors.length !== 0) {
    return {
      ok: false,
      minted: false,
      state: input.state,
      canonicalEventKeyHex,
      recipientHex,
      amount,
      totalSupplyAfter: input.state.totalSupply,
      errors,
      authorization: input.authorization,
    };
  }

  const processedGatewayEvents = new Set(input.state.processedGatewayEvents);
  processedGatewayEvents.add(canonicalEventKeyHex);

  const nextState: XXXLProgramState = {
    ...input.state,
    totalSupply: input.state.totalSupply + amount,
    processedGatewayEvents,
  };

  return {
    ok: true,
    minted: true,
    state: nextState,
    canonicalEventKeyHex,
    recipientHex,
    amount,
    totalSupplyAfter: nextState.totalSupply,
    errors: [],
    authorization: input.authorization,
  };
}

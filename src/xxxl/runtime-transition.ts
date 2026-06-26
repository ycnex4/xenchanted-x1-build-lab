import { stage1CanonicalEventKeyHex } from "../gateway/stage-1-processed-burn-registry.js";
import {
  validateXXXLConsumeGatewayMintInstructionSchema,
  type XXXLConsumeGatewayMintInstructionAccounts,
  type XXXLConsumeGatewayMintInstructionSchema,
  type XXXLRuntimeValidationResult,
} from "./runtime-candidate.js";
import { type XXXLStage1GatewayAuthorizationContract } from "./stage-1-gateway-consumer.js";

export const XXXL_RUNTIME_TRANSITION_ERROR = {
  InvalidInstructionSchema: "INVALID_INSTRUCTION_SCHEMA",
  Stage1MintNotAuthorized: "STAGE1_MINT_NOT_AUTHORIZED",
  Stage1MintNotMarkedProcessed: "STAGE1_MINT_NOT_MARKED_PROCESSED",
  InvalidAmount: "INVALID_AMOUNT",
  AuthorizationEventKeyMismatch: "AUTHORIZATION_EVENT_KEY_MISMATCH",
  AuthorizationAmountMismatch: "AUTHORIZATION_AMOUNT_MISMATCH",
  EventAlreadyConsumed: "EVENT_ALREADY_CONSUMED",
} as const;

export type XXXLRuntimeTransitionErrorCode =
  (typeof XXXL_RUNTIME_TRANSITION_ERROR)[keyof typeof XXXL_RUNTIME_TRANSITION_ERROR];

export type XXXLRuntimeConsumeGatewayMintTransitionInput = {
  readonly schema: XXXLConsumeGatewayMintInstructionSchema;
  readonly authorization: XXXLStage1GatewayAuthorizationContract;
};

export type XXXLRuntimeConsumeGatewayMintTransitionResult = {
  readonly ok: boolean;
  readonly executed: boolean;
  readonly accounts: XXXLConsumeGatewayMintInstructionAccounts;
  readonly errors: XXXLRuntimeTransitionErrorCode[];
  readonly schemaValidation: XXXLRuntimeValidationResult;
};

export function executeXXXLRuntimeConsumeGatewayMintCandidate(
  input: XXXLRuntimeConsumeGatewayMintTransitionInput,
): XXXLRuntimeConsumeGatewayMintTransitionResult {
  const schemaValidation = validateXXXLConsumeGatewayMintInstructionSchema(
    input.schema,
  );
  const errors: XXXLRuntimeTransitionErrorCode[] = [];

  if (!schemaValidation.ok) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.InvalidInstructionSchema);
  }

  if (!input.authorization.authorizationOk || !input.authorization.authorized) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.Stage1MintNotAuthorized);
  }

  if (!input.authorization.markedProcessed) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.Stage1MintNotMarkedProcessed);
  }

  if (input.authorization.amount <= 0n) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.InvalidAmount);
  }

  const authorizationEventKeyHex = stage1CanonicalEventKeyHex(
    input.authorization.canonicalEventKey,
  );
  const instructionEventKey = input.schema.data.canonicalEventKey.toLowerCase();

  if (authorizationEventKeyHex !== instructionEventKey) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.AuthorizationEventKeyMismatch);
  }

  if (input.authorization.amount !== input.schema.data.amount) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.AuthorizationAmountMismatch);
  }

  if (input.schema.accounts.processedEvent.consumed) {
    errors.push(XXXL_RUNTIME_TRANSITION_ERROR.EventAlreadyConsumed);
  }

  if (errors.length !== 0) {
    return {
      ok: false,
      executed: false,
      accounts: input.schema.accounts,
      errors,
      schemaValidation,
    };
  }

  const accounts = cloneXXXLRuntimeAccounts(input.schema.accounts);

  const nextAccounts: XXXLConsumeGatewayMintInstructionAccounts = {
    ...accounts,
    mintState: {
      ...accounts.mintState,
      totalSupply: accounts.mintState.totalSupply + input.authorization.amount,
    },
    processedEvent: {
      ...accounts.processedEvent,
      consumed: true,
      consumedAmount: input.authorization.amount,
      recipient: input.schema.data.recipient,
    },
    recipientBalance: {
      ...accounts.recipientBalance,
      balance: accounts.recipientBalance.balance + input.authorization.amount,
    },
  };

  return {
    ok: true,
    executed: true,
    accounts: nextAccounts,
    errors: [],
    schemaValidation,
  };
}

function cloneXXXLRuntimeAccounts(
  accounts: XXXLConsumeGatewayMintInstructionAccounts,
): XXXLConsumeGatewayMintInstructionAccounts {
  return {
    mintState: { ...accounts.mintState },
    gatewayConfig: { ...accounts.gatewayConfig },
    guardianSet: {
      ...accounts.guardianSet,
      guardianPublicKeys: [...accounts.guardianSet.guardianPublicKeys],
    },
    processedEvent: { ...accounts.processedEvent },
    recipientBalance: { ...accounts.recipientBalance },
  };
}

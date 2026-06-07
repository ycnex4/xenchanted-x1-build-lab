import {
  checkStage1BurnNotProcessed,
  markStage1BurnProcessed,
  type Stage1ProcessedBurnRegistry,
  type Stage1ProcessedBurnRegistryCheckResult,
  type Stage1ProcessedBurnRegistryMarkResult,
} from "./stage-1-processed-burn-registry.js";
import {
  verifyStage1GuardianQuorum,
  type Stage1GuardianQuorumVerificationInput,
  type Stage1GuardianQuorumVerificationResult,
} from "./stage-1-guardian-quorum.js";

export const STAGE1_MINT_AUTHORIZATION_ERROR = {
  InvalidQuorum: "INVALID_QUORUM",
  BurnAlreadyProcessed: "BURN_ALREADY_PROCESSED",
} as const;

export type Stage1MintAuthorizationErrorCode =
  (typeof STAGE1_MINT_AUTHORIZATION_ERROR)[keyof typeof STAGE1_MINT_AUTHORIZATION_ERROR];

export type Stage1MintAuthorizationInput = Stage1GuardianQuorumVerificationInput & {
  processedBurnRegistry: Stage1ProcessedBurnRegistry;
};

export type Stage1MintAuthorizationResult = {
  ok: boolean;
  authorized: boolean;
  markedProcessed: boolean;
  errors: Stage1MintAuthorizationErrorCode[];
  quorum: Stage1GuardianQuorumVerificationResult;
  processedBurn: Stage1ProcessedBurnRegistryCheckResult | Stage1ProcessedBurnRegistryMarkResult;
};

export async function authorizeStage1Mint(
  input: Stage1MintAuthorizationInput,
): Promise<Stage1MintAuthorizationResult> {
  const quorum = await verifyStage1GuardianQuorum(input);
  const processedBurn = checkStage1BurnNotProcessed(
    input.processedBurnRegistry,
    input.fields.canonicalEventKey,
  );
  const errors: Stage1MintAuthorizationErrorCode[] = [];

  if (!quorum.ok) {
    errors.push(STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum);
  }

  if (!processedBurn.ok) {
    errors.push(STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed);
  }

  if (errors.length !== 0) {
    return {
      ok: false,
      authorized: false,
      markedProcessed: false,
      errors,
      quorum,
      processedBurn,
    };
  }

  const markedProcessed = markStage1BurnProcessed(
    input.processedBurnRegistry,
    input.fields.canonicalEventKey,
  );

  return {
    ok: true,
    authorized: true,
    markedProcessed: markedProcessed.marked,
    errors: [],
    quorum,
    processedBurn: markedProcessed,
  };
}

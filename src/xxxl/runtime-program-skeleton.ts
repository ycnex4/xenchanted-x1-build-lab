import {
  executeXXXLRuntimeConsumeGatewayMintCandidate,
  type XXXLRuntimeConsumeGatewayMintTransitionResult,
} from "./runtime-transition.js";
import {
  XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE,
  validateXXXLRuntimeInstructionSerializationVectors,
  xxxlRuntimeInstructionSerializationLayouts,
  xxxlRuntimeInstructionSerializationVectors,
  type XXXLRuntimeInstructionSerializationLayout,
  type XXXLRuntimeInstructionSerializationValidationResult,
  type XXXLRuntimeInstructionSerializationVector,
} from "./runtime-instruction-serialization-vectors.js";
import {
  validateXXXLMultichainLowWeightRoutePolicy,
  type XXXLMultichainLowWeightRouteCandidate,
  type XXXLMultichainLowWeightRouteValidationResult,
} from "./multichain-low-weight-route-policy.js";
import {
  type XXXLConsumeGatewayMintInstructionAccounts,
  type XXXLConsumeGatewayMintInstructionSchema,
} from "./runtime-candidate.js";
import { type XXXLStage1GatewayAuthorizationContract } from "./stage-1-gateway-consumer.js";

export const XXXL_RUNTIME_PROGRAM_SKELETON_VERSION = 1;

export const XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE =
  "STAGE_1_AUTHORIZATION_RESULT_ONLY" as const;

export const XXXL_RUNTIME_PROGRAM_SKELETON_STEP = {
  LoadAccounts: "LOAD_ACCOUNTS",
  ValidateInstructionSerializationBoundary:
    "VALIDATE_INSTRUCTION_SERIALIZATION_BOUNDARY",
  ValidateRoutePolicy: "VALIDATE_ROUTE_POLICY",
  ConsumeStage1AuthorizationResult: "CONSUME_STAGE_1_AUTHORIZATION_RESULT",
  SplTokenMintToCpi: "SPL_TOKEN_MINT_TO_CPI",
  MarkProcessedEvent: "MARK_PROCESSED_EVENT",
  UpdateMintStateMirror: "UPDATE_MINT_STATE_MIRROR",
  UpdateRecipientBalanceMirror: "UPDATE_RECIPIENT_BALANCE_MIRROR",
  AuditGenesisSupplyInvariant: "AUDIT_GENESIS_SUPPLY_INVARIANT",
} as const;

export type XXXLRuntimeProgramSkeletonStep =
  (typeof XXXL_RUNTIME_PROGRAM_SKELETON_STEP)[keyof typeof XXXL_RUNTIME_PROGRAM_SKELETON_STEP];

export const XXXL_RUNTIME_PROGRAM_SKELETON_ERROR = {
  InstructionSerializationInvalid: "INSTRUCTION_SERIALIZATION_INVALID",
  RoutePolicyInvalid: "ROUTE_POLICY_INVALID",
  RoutePolicyMissingRoute: "ROUTE_POLICY_MISSING_ROUTE",
  TransitionRejected: "TRANSITION_REJECTED",
  SupplyAuditFailed: "SUPPLY_AUDIT_FAILED",
} as const;

export type XXXLRuntimeProgramSkeletonErrorCode =
  (typeof XXXL_RUNTIME_PROGRAM_SKELETON_ERROR)[keyof typeof XXXL_RUNTIME_PROGRAM_SKELETON_ERROR];

export type XXXLRuntimeProgramSkeletonCpiStep = {
  readonly step: typeof XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi;
  readonly skipped: boolean;
  readonly atomicWithParentTransaction: boolean;
  readonly signerRole: typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda;
  readonly mintRole: typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint;
  readonly recipientTokenAccountRole: typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount;
  readonly tokenProgramRole: typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram;
  readonly amount: bigint;
  readonly recipient: string;
};

export type XXXLRuntimeProgramSkeletonSupplyAudit = {
  readonly ok: boolean;
  readonly totalSupplyBefore: bigint;
  readonly totalSupplyAfter: bigint;
  readonly expectedTotalSupplyAfter: bigint;
  readonly recipientBalanceBefore: bigint;
  readonly recipientBalanceAfter: bigint;
  readonly expectedRecipientBalanceAfter: bigint;
  readonly processedEventConsumed: boolean;
  readonly processedEventAmount: bigint;
  readonly acceptedMintAmount: bigint;
};

export type XXXLRuntimeProgramSkeletonInput = {
  readonly schema: XXXLConsumeGatewayMintInstructionSchema;
  readonly authorization: XXXLStage1GatewayAuthorizationContract;
  readonly routePolicy?: readonly XXXLMultichainLowWeightRouteCandidate[];
  readonly instructionSerializationLayouts?: readonly XXXLRuntimeInstructionSerializationLayout[];
  readonly instructionSerializationVectors?: readonly XXXLRuntimeInstructionSerializationVector[];
};

export type XXXLRuntimeProgramSkeletonResult = {
  readonly version: typeof XXXL_RUNTIME_PROGRAM_SKELETON_VERSION;
  readonly ok: boolean;
  readonly executed: boolean;
  readonly steps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly errors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly accountsBefore: XXXLConsumeGatewayMintInstructionAccounts;
  readonly accountsAfter: XXXLConsumeGatewayMintInstructionAccounts;
  readonly instructionSerializationValidation: XXXLRuntimeInstructionSerializationValidationResult;
  readonly routePolicyValidation: XXXLMultichainLowWeightRouteValidationResult | null;
  readonly transition: XXXLRuntimeConsumeGatewayMintTransitionResult | null;
  readonly cpiStep: XXXLRuntimeProgramSkeletonCpiStep;
  readonly supplyAudit: XXXLRuntimeProgramSkeletonSupplyAudit;
  readonly guardianSignatureVerificationScope: typeof XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE;
};

export function executeXXXLRuntimeProgramSkeleton(
  input: XXXLRuntimeProgramSkeletonInput,
): XXXLRuntimeProgramSkeletonResult {
  const steps: XXXLRuntimeProgramSkeletonStep[] = [
    XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
    XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
  ];

  const layouts =
    input.instructionSerializationLayouts ??
    xxxlRuntimeInstructionSerializationLayouts();
  const vectors =
    input.instructionSerializationVectors ??
    xxxlRuntimeInstructionSerializationVectors();

  const instructionSerializationValidation =
    validateXXXLRuntimeInstructionSerializationVectors(layouts, vectors);

  const errors: XXXLRuntimeProgramSkeletonErrorCode[] = [];

  if (!instructionSerializationValidation.ok) {
    errors.push(
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.InstructionSerializationInvalid,
    );
  }

  let routePolicyValidation: XXXLMultichainLowWeightRouteValidationResult | null =
    null;

  if (input.routePolicy) {
    steps.push(XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy);
    routePolicyValidation = validateXXXLMultichainLowWeightRoutePolicy(
      input.routePolicy,
    );

    if (!routePolicyValidation.ok) {
      errors.push(XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyInvalid);
    }

    if (!routePolicyContainsRoute(input.routePolicy, input.schema.data.routeId)) {
      errors.push(XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyMissingRoute);
    }
  }

  if (errors.length !== 0) {
    const supplyAudit = auditXXXLRuntimeProgramSkeletonSupply(
      input.schema.accounts,
      input.schema.accounts,
      input.authorization.amount,
    );

    return {
      version: XXXL_RUNTIME_PROGRAM_SKELETON_VERSION,
      ok: false,
      executed: false,
      steps,
      errors,
      accountsBefore: input.schema.accounts,
      accountsAfter: input.schema.accounts,
      instructionSerializationValidation,
      routePolicyValidation,
      transition: null,
      cpiStep: skippedXXXLRuntimeSplTokenMintToCpi(input),
      supplyAudit,
      guardianSignatureVerificationScope:
        XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE,
    };
  }

  steps.push(
    XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ConsumeStage1AuthorizationResult,
  );

  const transition = executeXXXLRuntimeConsumeGatewayMintCandidate({
    schema: input.schema,
    authorization: input.authorization,
  });

  if (!transition.ok) {
    errors.push(XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected);
  }

  const supplyAudit = auditXXXLRuntimeProgramSkeletonSupply(
    input.schema.accounts,
    transition.accounts,
    input.authorization.amount,
  );

  const cpiStep = transition.ok
    ? simulateXXXLRuntimeSplTokenMintToCpi(input)
    : skippedXXXLRuntimeSplTokenMintToCpi(input);

  if (transition.ok) {
    steps.push(
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.MarkProcessedEvent,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.UpdateMintStateMirror,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.UpdateRecipientBalanceMirror,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.AuditGenesisSupplyInvariant,
    );
  }

  if (transition.ok && !supplyAudit.ok) {
    errors.push(XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.SupplyAuditFailed);
  }

  const ok = errors.length === 0;
  const accountsAfter = ok ? transition.accounts : input.schema.accounts;

  return {
    version: XXXL_RUNTIME_PROGRAM_SKELETON_VERSION,
    ok,
    executed: ok,
    steps,
    errors,
    accountsBefore: input.schema.accounts,
    accountsAfter,
    instructionSerializationValidation,
    routePolicyValidation,
    transition,
    cpiStep,
    supplyAudit,
    guardianSignatureVerificationScope:
      XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE,
  };
}

export function auditXXXLRuntimeProgramSkeletonSupply(
  before: XXXLConsumeGatewayMintInstructionAccounts,
  after: XXXLConsumeGatewayMintInstructionAccounts,
  acceptedMintAmount: bigint,
): XXXLRuntimeProgramSkeletonSupplyAudit {
  const expectedTotalSupplyAfter =
    before.mintState.totalSupply + acceptedMintAmount;
  const expectedRecipientBalanceAfter =
    before.recipientBalance.balance + acceptedMintAmount;

  const ok =
    acceptedMintAmount > 0n &&
    after.mintState.totalSupply === expectedTotalSupplyAfter &&
    after.recipientBalance.balance === expectedRecipientBalanceAfter &&
    after.processedEvent.consumed &&
    after.processedEvent.consumedAmount === acceptedMintAmount;

  return {
    ok,
    totalSupplyBefore: before.mintState.totalSupply,
    totalSupplyAfter: after.mintState.totalSupply,
    expectedTotalSupplyAfter,
    recipientBalanceBefore: before.recipientBalance.balance,
    recipientBalanceAfter: after.recipientBalance.balance,
    expectedRecipientBalanceAfter,
    processedEventConsumed: after.processedEvent.consumed,
    processedEventAmount: after.processedEvent.consumedAmount,
    acceptedMintAmount,
  };
}

function routePolicyContainsRoute(
  routePolicy: readonly XXXLMultichainLowWeightRouteCandidate[],
  routeId: string,
): boolean {
  return routePolicy.some((route) => route.routeId === routeId);
}

function simulateXXXLRuntimeSplTokenMintToCpi(
  input: Pick<XXXLRuntimeProgramSkeletonInput, "schema" | "authorization">,
): XXXLRuntimeProgramSkeletonCpiStep {
  return {
    step: XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi,
    skipped: false,
    atomicWithParentTransaction: true,
    signerRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
    mintRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint,
    recipientTokenAccountRole:
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount,
    tokenProgramRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
    amount: input.authorization.amount,
    recipient: input.schema.data.recipient,
  };
}

function skippedXXXLRuntimeSplTokenMintToCpi(
  input: Pick<XXXLRuntimeProgramSkeletonInput, "schema">,
): XXXLRuntimeProgramSkeletonCpiStep {
  return {
    step: XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi,
    skipped: true,
    atomicWithParentTransaction: true,
    signerRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
    mintRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint,
    recipientTokenAccountRole:
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount,
    tokenProgramRole: XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
    amount: 0n,
    recipient: input.schema.data.recipient,
  };
}

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
} from "./program-v1.js";
import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_ROUTE_STATUS,
  type XXXLConsumeGatewayMintInstructionSchema,
} from "./runtime-candidate.js";
import {
  XXXL_RUNTIME_PROGRAM_SKELETON_ERROR,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP,
  executeXXXLRuntimeProgramSkeleton,
  type XXXLRuntimeProgramSkeletonErrorCode,
  type XXXLRuntimeProgramSkeletonInput,
  type XXXLRuntimeProgramSkeletonStep,
} from "./runtime-program-skeleton.js";
import {
  AVALANCHE_MAINNET_CHAIN_ID,
  XXXL_MULTICHAIN_ROUTE_ID,
  XXXL_MULTICHAIN_ROUTE_STATUS,
  xxxlAvalancheLowWeightRouteCandidate,
  xxxlEthereumPrimaryRouteCandidate,
} from "./multichain-low-weight-route-policy.js";
import {
  type XXXLRuntimeInstructionSerializationLayout,
  type XXXLRuntimeInstructionSerializationVector,
} from "./runtime-instruction-serialization-vectors.js";
import { type XXXLStage1GatewayAuthorizationContract } from "./stage-1-gateway-consumer.js";
import { stage1CanonicalEventKeyHex } from "../gateway/stage-1-processed-burn-registry.js";

export const XXXL_RUNTIME_EXECUTION_VECTOR_SET_VERSION = 1;

export const XXXL_RUNTIME_EXECUTION_VECTOR_ID = {
  ValidEthereumGatewayMint: "XXXL_RUNTIME_EXECUTION_VALID_ETHEREUM_GATEWAY_MINT",
  ValidAvalancheLowWeightRoute:
    "XXXL_RUNTIME_EXECUTION_VALID_AVALANCHE_LOW_WEIGHT_ROUTE",
  InvalidRoutePolicyRejected:
    "XXXL_RUNTIME_EXECUTION_INVALID_ROUTE_POLICY_REJECTED",
  MissingRouteRejected: "XXXL_RUNTIME_EXECUTION_MISSING_ROUTE_REJECTED",
  Stage1AuthorizationRejected:
    "XXXL_RUNTIME_EXECUTION_STAGE1_AUTHORIZATION_REJECTED",
  ReplayRejected: "XXXL_RUNTIME_EXECUTION_REPLAY_REJECTED",
  EventKeyMismatchRejected:
    "XXXL_RUNTIME_EXECUTION_EVENT_KEY_MISMATCH_REJECTED",
  InstructionSerializationRejected:
    "XXXL_RUNTIME_EXECUTION_INSTRUCTION_SERIALIZATION_REJECTED",
} as const;

export type XXXLRuntimeExecutionVectorId =
  (typeof XXXL_RUNTIME_EXECUTION_VECTOR_ID)[keyof typeof XXXL_RUNTIME_EXECUTION_VECTOR_ID];

export const XXXL_RUNTIME_EXECUTION_VECTOR_ERROR = {
  MissingVector: "MISSING_VECTOR",
  DuplicateVector: "DUPLICATE_VECTOR",
  ExecutionMismatch: "EXECUTION_MISMATCH",
  WrongCanonicalJson: "WRONG_CANONICAL_JSON",
} as const;

export type XXXLRuntimeExecutionVectorErrorCode =
  (typeof XXXL_RUNTIME_EXECUTION_VECTOR_ERROR)[keyof typeof XXXL_RUNTIME_EXECUTION_VECTOR_ERROR];

export type XXXLRuntimeExecutionVectorScenario = {
  readonly vectorId: XXXLRuntimeExecutionVectorId;
  readonly description: string;
  readonly input: XXXLRuntimeProgramSkeletonInput;
  readonly expectedOk: boolean;
  readonly expectedExecuted: boolean;
  readonly expectedErrors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly expectedSteps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly expectedCpiSkipped: boolean;
  readonly expectedSupplyAuditOk: boolean;
};

export type XXXLRuntimeExecutionVectorCore = {
  readonly version: typeof XXXL_RUNTIME_EXECUTION_VECTOR_SET_VERSION;
  readonly vectorId: XXXLRuntimeExecutionVectorId;
  readonly description: string;
  readonly routeId: string;
  readonly expectedOk: boolean;
  readonly actualOk: boolean;
  readonly expectedExecuted: boolean;
  readonly actualExecuted: boolean;
  readonly expectedErrors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly actualErrors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly expectedSteps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly actualSteps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly expectedCpiSkipped: boolean;
  readonly actualCpiSkipped: boolean;
  readonly expectedSupplyAuditOk: boolean;
  readonly actualSupplyAuditOk: boolean;
  readonly totalSupplyBefore: bigint;
  readonly totalSupplyAfter: bigint;
  readonly recipientBalanceBefore: bigint;
  readonly recipientBalanceAfter: bigint;
  readonly acceptedMintAmount: bigint;
};

export type XXXLRuntimeExecutionVector = XXXLRuntimeExecutionVectorCore & {
  readonly canonicalJson: string;
};

export type XXXLRuntimeExecutionVectorValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeExecutionVectorErrorCode[];
};

const CANONICAL_EVENT_KEY_BYTES = new Uint8Array(32).fill(0x55);
const OTHER_CANONICAL_EVENT_KEY_BYTES = new Uint8Array(32).fill(0x66);

const SUCCESSFUL_STEPS: readonly XXXLRuntimeProgramSkeletonStep[] = [
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ConsumeStage1AuthorizationResult,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.MarkProcessedEvent,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.UpdateMintStateMirror,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.UpdateRecipientBalanceMirror,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.AuditGenesisSupplyInvariant,
];

const PREFLIGHT_REJECT_STEPS: readonly XXXLRuntimeProgramSkeletonStep[] = [
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy,
];

const TRANSITION_REJECT_STEPS: readonly XXXLRuntimeProgramSkeletonStep[] = [
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ConsumeStage1AuthorizationResult,
];

export function xxxlRuntimeExecutionVectorScenarios(): readonly XXXLRuntimeExecutionVectorScenario[] {
  const ethereumPolicy = [xxxlEthereumPrimaryRouteCandidate()];
  const avalanchePolicy = [
    xxxlEthereumPrimaryRouteCandidate(),
    {
      ...xxxlAvalancheLowWeightRouteCandidate(10),
      status: XXXL_MULTICHAIN_ROUTE_STATUS.Active,
    },
  ];

  return [
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      description: "Valid Ethereum primary full-weight gateway mint execution.",
      input: {
        schema: validExecutionSchema(),
        authorization: validExecutionAuthorization(),
        routePolicy: ethereumPolicy,
      },
      expectedOk: true,
      expectedExecuted: true,
      expectedErrors: [],
      expectedSteps: SUCCESSFUL_STEPS,
      expectedCpiSkipped: false,
      expectedSupplyAuditOk: true,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
      description:
        "Valid Avalanche low-weight route-aware execution using an already authorized XXXL mint amount.",
      input: {
        schema: validExecutionSchema({
          routeId: XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight,
          sourceChainId: BigInt(AVALANCHE_MAINNET_CHAIN_ID),
          sourceToken: "0xAVALANCHE_XNTD",
          guardianSetId: "guardian-set-avalanche-1",
          finalityRuleId: "avalanche-finalized",
        }),
        authorization: validExecutionAuthorization(),
        routePolicy: avalanchePolicy,
      },
      expectedOk: true,
      expectedExecuted: true,
      expectedErrors: [],
      expectedSteps: SUCCESSFUL_STEPS,
      expectedCpiSkipped: false,
      expectedSupplyAuditOk: true,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
      description: "Invalid route policy is rejected before transition execution.",
      input: {
        schema: validExecutionSchema(),
        authorization: validExecutionAuthorization(),
        routePolicy: [
          {
            ...xxxlEthereumPrimaryRouteCandidate(),
            sourceChainWeightBps: 9_999,
          },
        ],
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [
        XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyInvalid,
      ],
      expectedSteps: PREFLIGHT_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
      description: "Instruction route id missing from supplied route policy is rejected before transition execution.",
      input: {
        schema: validExecutionSchema({
          routeId: "UNKNOWN_XC_ROUTE",
        }),
        authorization: validExecutionAuthorization(),
        routePolicy: ethereumPolicy,
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [
        XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyMissingRoute,
      ],
      expectedSteps: PREFLIGHT_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
      description: "Stage 1 authorization rejection preserves accounts and skips CPI.",
      input: {
        schema: validExecutionSchema(),
        authorization: validExecutionAuthorization({
          authorizationOk: false,
          authorized: false,
        }),
        routePolicy: ethereumPolicy,
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected],
      expectedSteps: TRANSITION_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
      description: "Already consumed processed event is rejected and no CPI is committed.",
      input: {
        schema: validExecutionSchema({
          consumed: true,
        }),
        authorization: validExecutionAuthorization(),
        routePolicy: ethereumPolicy,
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected],
      expectedSteps: TRANSITION_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
    {
      vectorId: XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
      description: "Authorization canonical event key mismatch is rejected by transition layer.",
      input: {
        schema: validExecutionSchema(),
        authorization: validExecutionAuthorization({
          canonicalEventKey: OTHER_CANONICAL_EVENT_KEY_BYTES,
        }),
        routePolicy: ethereumPolicy,
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected],
      expectedSteps: TRANSITION_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
    {
      vectorId:
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
      description: "Invalid instruction serialization boundary is rejected before transition execution.",
      input: {
        schema: validExecutionSchema(),
        authorization: validExecutionAuthorization(),
        routePolicy: ethereumPolicy,
        instructionSerializationLayouts:
          [] as readonly XXXLRuntimeInstructionSerializationLayout[],
        instructionSerializationVectors:
          [] as readonly XXXLRuntimeInstructionSerializationVector[],
      },
      expectedOk: false,
      expectedExecuted: false,
      expectedErrors: [
        XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.InstructionSerializationInvalid,
      ],
      expectedSteps: PREFLIGHT_REJECT_STEPS,
      expectedCpiSkipped: true,
      expectedSupplyAuditOk: false,
    },
  ];
}

export function xxxlRuntimeExecutionVectors(): readonly XXXLRuntimeExecutionVector[] {
  return xxxlRuntimeExecutionVectorScenarios().map((scenario) => {
    const result = executeXXXLRuntimeProgramSkeleton(scenario.input);

    const core: XXXLRuntimeExecutionVectorCore = {
      version: XXXL_RUNTIME_EXECUTION_VECTOR_SET_VERSION,
      vectorId: scenario.vectorId,
      description: scenario.description,
      routeId: scenario.input.schema.data.routeId,
      expectedOk: scenario.expectedOk,
      actualOk: result.ok,
      expectedExecuted: scenario.expectedExecuted,
      actualExecuted: result.executed,
      expectedErrors: scenario.expectedErrors,
      actualErrors: result.errors,
      expectedSteps: scenario.expectedSteps,
      actualSteps: result.steps,
      expectedCpiSkipped: scenario.expectedCpiSkipped,
      actualCpiSkipped: result.cpiStep.skipped,
      expectedSupplyAuditOk: scenario.expectedSupplyAuditOk,
      actualSupplyAuditOk: result.supplyAudit.ok,
      totalSupplyBefore: result.supplyAudit.totalSupplyBefore,
      totalSupplyAfter: result.supplyAudit.totalSupplyAfter,
      recipientBalanceBefore: result.supplyAudit.recipientBalanceBefore,
      recipientBalanceAfter: result.supplyAudit.recipientBalanceAfter,
      acceptedMintAmount: result.supplyAudit.acceptedMintAmount,
    };

    return {
      ...core,
      canonicalJson: xxxlCanonicalRuntimeExecutionVectorJson(core),
    };
  });
}

export function validateXXXLRuntimeExecutionVectors(
  vectors: readonly XXXLRuntimeExecutionVector[],
): XXXLRuntimeExecutionVectorValidationResult {
  const errors: XXXLRuntimeExecutionVectorErrorCode[] = [];
  const ids = new Set<XXXLRuntimeExecutionVectorId>();

  for (const vector of vectors) {
    if (ids.has(vector.vectorId)) {
      errors.push(XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.DuplicateVector);
    }
    ids.add(vector.vectorId);
  }

  for (const vectorId of mandatoryExecutionVectorIds()) {
    if (!ids.has(vectorId)) {
      errors.push(XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.MissingVector);
    }
  }

  for (const vector of vectors) {
    if (
      vector.expectedOk !== vector.actualOk ||
      vector.expectedExecuted !== vector.actualExecuted ||
      vector.expectedCpiSkipped !== vector.actualCpiSkipped ||
      vector.expectedSupplyAuditOk !== vector.actualSupplyAuditOk ||
      !sameStrings(vector.expectedErrors, vector.actualErrors) ||
      !sameStrings(vector.expectedSteps, vector.actualSteps)
    ) {
      errors.push(XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.ExecutionMismatch);
    }

    const { canonicalJson: _canonicalJson, ...core } = vector;

    if (xxxlCanonicalRuntimeExecutionVectorJson(core) !== vector.canonicalJson) {
      errors.push(XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.WrongCanonicalJson);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimeExecutionVectorJson(
  vector: XXXLRuntimeExecutionVectorCore,
): string {
  return JSON.stringify([
    ["version", vector.version],
    ["vectorId", vector.vectorId],
    ["description", vector.description],
    ["routeId", vector.routeId],
    ["expectedOk", vector.expectedOk],
    ["actualOk", vector.actualOk],
    ["expectedExecuted", vector.expectedExecuted],
    ["actualExecuted", vector.actualExecuted],
    ["expectedErrors", vector.expectedErrors],
    ["actualErrors", vector.actualErrors],
    ["expectedSteps", vector.expectedSteps],
    ["actualSteps", vector.actualSteps],
    ["expectedCpiSkipped", vector.expectedCpiSkipped],
    ["actualCpiSkipped", vector.actualCpiSkipped],
    ["expectedSupplyAuditOk", vector.expectedSupplyAuditOk],
    ["actualSupplyAuditOk", vector.actualSupplyAuditOk],
    ["totalSupplyBefore", vector.totalSupplyBefore.toString()],
    ["totalSupplyAfter", vector.totalSupplyAfter.toString()],
    ["recipientBalanceBefore", vector.recipientBalanceBefore.toString()],
    ["recipientBalanceAfter", vector.recipientBalanceAfter.toString()],
    ["acceptedMintAmount", vector.acceptedMintAmount.toString()],
  ]);
}

function mandatoryExecutionVectorIds(): readonly XXXLRuntimeExecutionVectorId[] {
  return Object.values(XXXL_RUNTIME_EXECUTION_VECTOR_ID);
}

function validExecutionAuthorization(
  overrides: Partial<XXXLStage1GatewayAuthorizationContract> = {},
): XXXLStage1GatewayAuthorizationContract {
  return {
    authorizationOk: true,
    authorized: true,
    markedProcessed: true,
    canonicalEventKey: CANONICAL_EVENT_KEY_BYTES,
    amount: 1_000n,
    ...overrides,
  };
}

function validExecutionSchema(
  options: {
    readonly routeId?: string;
    readonly sourceChainId?: bigint;
    readonly sourceToken?: string;
    readonly guardianSetId?: string;
    readonly finalityRuleId?: string;
    readonly amount?: bigint;
    readonly recipient?: string;
    readonly consumed?: boolean;
  } = {},
): XXXLConsumeGatewayMintInstructionSchema {
  const routeId = options.routeId ?? XXXL_GATEWAY_ROUTE_ID;
  const guardianSetId = options.guardianSetId ?? "guardian-set-1";
  const amount = options.amount ?? 1_000n;
  const recipient = options.recipient ?? "x1-recipient";
  const canonicalEventKey = stage1CanonicalEventKeyHex(
    CANONICAL_EVENT_KEY_BYTES,
  );

  const data = {
    instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    routeId,
    guardianSetId,
    mintId: "xxxl-mint",
    canonicalEventKey,
    recipient,
    amount,
  };

  return {
    accounts: {
      mintState: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
        version: 1,
        mintId: "xxxl-mint",
        decimals: 18,
        totalSupply: 500n,
        authorityMode: XXXL_RUNTIME_AUTHORITY_MODE.GatewayOnly,
        upgradeAuthorityStatus: "TEMPORARY_STAGED_FINALIZATION",
      },
      gatewayConfig: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
        version: 1,
        routeId,
        sourceChainId: options.sourceChainId ?? BigInt(ETHEREUM_MAINNET_CHAIN_ID),
        sourceToken: options.sourceToken ?? "0xETHEREUM_XNTD",
        targetMintToken: XXXL_MINT_TOKEN,
        targetX1NetworkId: "x1-mainnet",
        targetMintCoreId: "xxxl-mint-core",
        guardianSetId,
        quorumThreshold: 2,
        finalityRuleId: options.finalityRuleId ?? "ethereum-finalized",
        status: XXXL_RUNTIME_ROUTE_STATUS.Active,
      },
      guardianSet: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
        version: 1,
        guardianSetId,
        guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
        quorumThreshold: 2,
        status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
      },
      processedEvent: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
        version: 1,
        canonicalEventKey,
        routeId,
        consumed: options.consumed ?? false,
        consumedAmount: amount,
        recipient,
      },
      recipientBalance: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
        version: 1,
        mintId: "xxxl-mint",
        owner: recipient,
        balance: 200n,
      },
    },
    data,
  };
}

function sameStrings(
  left: readonly string[],
  right: readonly string[],
): boolean {
  return left.length === right.length && left.every((item, index) => item === right[index]);
}

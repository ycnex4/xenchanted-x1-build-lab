import {
  evaluateXXXLRuntimeDeploymentDryRunReport,
  type XXXLRuntimeDeploymentDryRunPolicyCandidate,
  type XXXLRuntimeDeploymentDryRunReport,
} from "./runtime-deployment-dry-run.js";
import { type XXXLRuntimeIncidentResponsePolicyCandidate } from "./runtime-incident-policy.js";
import { type XXXLRuntimeRouteGuardianFinalityPolicyCandidate } from "./runtime-route-policy.js";

export const XXXL_RUNTIME_AUTHORITY_FREEZE_VERSION = 1;

export const XXXL_RUNTIME_AUTHORITY_STATE = {
  StagedFinalization: "STAGED_FINALIZATION",
  FreezeProposed: "FREEZE_PROPOSED",
  Frozen: "FROZEN",
  FreezeCancelled: "FREEZE_CANCELLED",
} as const;

export type XXXLRuntimeAuthorityState =
  (typeof XXXL_RUNTIME_AUTHORITY_STATE)[keyof typeof XXXL_RUNTIME_AUTHORITY_STATE];

export const XXXL_RUNTIME_FREEZE_PREREQUISITE = {
  RuntimeSchemaComplete: "RUNTIME_SCHEMA_COMPLETE",
  TransitionSemanticsComplete: "TRANSITION_SEMANTICS_COMPLETE",
  RoutePolicyComplete: "ROUTE_POLICY_COMPLETE",
  IncidentPolicyComplete: "INCIDENT_POLICY_COMPLETE",
  DeploymentDryRunAccepted: "DEPLOYMENT_DRY_RUN_ACCEPTED",
  PublicDisclosureReady: "PUBLIC_DISCLOSURE_READY",
  FreezePlanReady: "FREEZE_PLAN_READY",
  X1NativeMechanicsComplete: "X1_NATIVE_MECHANICS_COMPLETE",
  ReviewCompleted: "REVIEW_COMPLETED",
} as const;

export type XXXLRuntimeFreezePrerequisite =
  (typeof XXXL_RUNTIME_FREEZE_PREREQUISITE)[keyof typeof XXXL_RUNTIME_FREEZE_PREREQUISITE];

export const XXXL_RUNTIME_MANDATORY_FREEZE_PREREQUISITES = [
  XXXL_RUNTIME_FREEZE_PREREQUISITE.RuntimeSchemaComplete,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.TransitionSemanticsComplete,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.RoutePolicyComplete,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.IncidentPolicyComplete,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.DeploymentDryRunAccepted,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.PublicDisclosureReady,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.FreezePlanReady,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.X1NativeMechanicsComplete,
  XXXL_RUNTIME_FREEZE_PREREQUISITE.ReviewCompleted,
] as const;

export const XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY = {
  ProgramUpgrade: "PROGRAM_UPGRADE",
  ManualMint: "MANUAL_MINT",
  Premine: "PREMINE",
  FounderAllocation: "FOUNDER_ALLOCATION",
  HiddenEmission: "HIDDEN_EMISSION",
  BalanceRewrite: "BALANCE_REWRITE",
  GatewayBypass: "GATEWAY_BYPASS",
  ArbitraryMintPath: "ARBITRARY_MINT_PATH",
  DiscretionarySupplyControl: "DISCRETIONARY_SUPPLY_CONTROL",
} as const;

export type XXXLRuntimeForbiddenPostFreezeCapability =
  (typeof XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY)[keyof typeof XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY];

export const XXXL_RUNTIME_MANDATORY_FORBIDDEN_POST_FREEZE_CAPABILITIES = [
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.ProgramUpgrade,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.ManualMint,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.Premine,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.FounderAllocation,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.HiddenEmission,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.BalanceRewrite,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.GatewayBypass,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.ArbitraryMintPath,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.DiscretionarySupplyControl,
] as const;

export const XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION = {
  ConsumeGatewayMint: "CONSUME_GATEWAY_MINT",
  RoutePause: "ROUTE_PAUSE",
  EmergencyFreeze: "EMERGENCY_FREEZE",
  GuardianRotation: "GUARDIAN_ROTATION",
  PublicNotice: "PUBLIC_NOTICE",
  PostMortem: "POST_MORTEM",
  RouteRetirement: "ROUTE_RETIREMENT",
} as const;

export type XXXLRuntimePostFreezeAllowedAction =
  (typeof XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION)[keyof typeof XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION];

export const XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS =
  7 * 24 * 60 * 60;

export const XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR = {
  InvalidDryRun: "INVALID_DRY_RUN",
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  InvalidGuardianCount: "INVALID_GUARDIAN_COUNT",
  InvalidGuardianQuorumThreshold: "INVALID_GUARDIAN_QUORUM_THRESHOLD",
  InvalidEmergencyFreezeThreshold: "INVALID_EMERGENCY_FREEZE_THRESHOLD",
  InvalidAuthorityFreezeThreshold: "INVALID_AUTHORITY_FREEZE_THRESHOLD",
  AuthorityFreezeWeakerThanEmergencyFreeze:
    "AUTHORITY_FREEZE_WEAKER_THAN_EMERGENCY_FREEZE",
  TimelockTooShort: "TIMELOCK_TOO_SHORT",
  MissingProposalId: "MISSING_PROPOSAL_ID",
  MissingEvidence: "MISSING_EVIDENCE",
  WrongAuthorityState: "WRONG_AUTHORITY_STATE",
  MissingMandatoryPrerequisite: "MISSING_MANDATORY_PREREQUISITE",
  DuplicatePrerequisite: "DUPLICATE_PREREQUISITE",
  TimelockNotExpired: "TIMELOCK_NOT_EXPIRED",
  InsufficientGuardianApprovals: "INSUFFICIENT_GUARDIAN_APPROVALS",
  MissingForbiddenCapabilityRemoval: "MISSING_FORBIDDEN_CAPABILITY_REMOVAL",
  DuplicateForbiddenCapabilityRemoval: "DUPLICATE_FORBIDDEN_CAPABILITY_REMOVAL",
  EmptyPostFreezeAllowedActions: "EMPTY_POST_FREEZE_ALLOWED_ACTIONS",
  NonDeterministicPostFreezeAction: "NON_DETERMINISTIC_POST_FREEZE_ACTION",
} as const;

export type XXXLRuntimeAuthorityFreezeErrorCode =
  (typeof XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR)[keyof typeof XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR];

export type XXXLRuntimeAuthorityFreezePolicyCandidate = {
  readonly version: number;
  readonly guardianCount: number;
  readonly guardianQuorumThreshold: number;
  readonly emergencyFreezeThreshold: number;
  readonly authorityFreezeThreshold: number;
  readonly minTimelockSeconds: number;
};

export type XXXLRuntimeAuthorityFreezeProposal = {
  readonly proposalId: string;
  readonly programVersion: number;
  readonly proposedBy: string;
  readonly currentAuthorityState: XXXLRuntimeAuthorityState;
  readonly timelockStartTs: number;
  readonly timelockEndTs: number;
  readonly executionTs: number;
  readonly guardianApprovals: number;
  readonly prerequisites: readonly XXXLRuntimeFreezePrerequisite[];
  readonly forbiddenCapabilitiesToRemove: readonly XXXLRuntimeForbiddenPostFreezeCapability[];
  readonly postFreezeAllowedActions: readonly XXXLRuntimePostFreezeAllowedAction[];
  readonly publicDisclosureId: string;
  readonly freezePlanId: string;
  readonly dryRunEvidenceId: string;
  readonly reviewEvidenceId: string;
};

export type XXXLRuntimeAuthorityFreezeEvaluationInput = {
  readonly routePolicy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate;
  readonly incidentPolicy: XXXLRuntimeIncidentResponsePolicyCandidate;
  readonly dryRunPolicy: XXXLRuntimeDeploymentDryRunPolicyCandidate;
  readonly dryRunReport: XXXLRuntimeDeploymentDryRunReport;
  readonly freezePolicy: XXXLRuntimeAuthorityFreezePolicyCandidate;
  readonly proposal: XXXLRuntimeAuthorityFreezeProposal;
};

export type XXXLRuntimeAuthorityFreezePolicyValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeAuthorityFreezeErrorCode[];
};

export type XXXLRuntimeAuthorityFreezeEvaluationResult = {
  readonly canExecute: boolean;
  readonly errors: XXXLRuntimeAuthorityFreezeErrorCode[];
  readonly nextAuthorityState: XXXLRuntimeAuthorityState;
};

export function validateXXXLRuntimeAuthorityFreezePolicy(
  policy: XXXLRuntimeAuthorityFreezePolicyCandidate,
): XXXLRuntimeAuthorityFreezePolicyValidationResult {
  const errors: XXXLRuntimeAuthorityFreezeErrorCode[] = [];

  if (policy.version !== XXXL_RUNTIME_AUTHORITY_FREEZE_VERSION) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.UnsupportedVersion);
  }

  if (policy.guardianCount <= 0) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidGuardianCount);
  }

  if (
    policy.guardianQuorumThreshold <= 0 ||
    policy.guardianQuorumThreshold > policy.guardianCount
  ) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidGuardianQuorumThreshold,
    );
  }

  if (
    policy.emergencyFreezeThreshold < policy.guardianQuorumThreshold ||
    policy.emergencyFreezeThreshold > policy.guardianCount
  ) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidEmergencyFreezeThreshold,
    );
  }

  if (
    policy.authorityFreezeThreshold < policy.guardianQuorumThreshold ||
    policy.authorityFreezeThreshold > policy.guardianCount
  ) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidAuthorityFreezeThreshold,
    );
  }

  if (policy.authorityFreezeThreshold < policy.emergencyFreezeThreshold) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR
        .AuthorityFreezeWeakerThanEmergencyFreeze,
    );
  }

  if (
    policy.minTimelockSeconds <
    XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS
  ) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.TimelockTooShort);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function evaluateXXXLRuntimeAuthorityFreezeProposal(
  input: XXXLRuntimeAuthorityFreezeEvaluationInput,
): XXXLRuntimeAuthorityFreezeEvaluationResult {
  const errors: XXXLRuntimeAuthorityFreezeErrorCode[] = [];
  const policyValidation = validateXXXLRuntimeAuthorityFreezePolicy(
    input.freezePolicy,
  );
  errors.push(...policyValidation.errors);

  const dryRun = evaluateXXXLRuntimeDeploymentDryRunReport({
    routePolicy: input.routePolicy,
    incidentPolicy: input.incidentPolicy,
    dryRunPolicy: input.dryRunPolicy,
    report: input.dryRunReport,
  });

  if (!dryRun.accepted) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidDryRun);
  }

  const { proposal, freezePolicy } = input;

  if (proposal.proposalId.trim() === "") {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.MissingProposalId);
  }

  if (
    proposal.proposedBy.trim() === "" ||
    proposal.publicDisclosureId.trim() === "" ||
    proposal.freezePlanId.trim() === "" ||
    proposal.dryRunEvidenceId.trim() === "" ||
    proposal.reviewEvidenceId.trim() === ""
  ) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.MissingEvidence);
  }

  if (
    proposal.currentAuthorityState !==
    XXXL_RUNTIME_AUTHORITY_STATE.StagedFinalization
  ) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.WrongAuthorityState);
  }

  const prerequisites = new Set(proposal.prerequisites);
  for (const prerequisite of XXXL_RUNTIME_MANDATORY_FREEZE_PREREQUISITES) {
    if (!prerequisites.has(prerequisite)) {
      errors.push(
        XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.MissingMandatoryPrerequisite,
      );
    }
  }

  if (prerequisites.size !== proposal.prerequisites.length) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.DuplicatePrerequisite);
  }

  if (
    proposal.timelockEndTs - proposal.timelockStartTs <
    freezePolicy.minTimelockSeconds
  ) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.TimelockTooShort);
  }

  if (proposal.executionTs < proposal.timelockEndTs) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.TimelockNotExpired);
  }

  if (proposal.guardianApprovals < freezePolicy.authorityFreezeThreshold) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InsufficientGuardianApprovals,
    );
  }

  const removedCapabilities = new Set(proposal.forbiddenCapabilitiesToRemove);
  for (const capability of
    XXXL_RUNTIME_MANDATORY_FORBIDDEN_POST_FREEZE_CAPABILITIES) {
    if (!removedCapabilities.has(capability)) {
      errors.push(
        XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR
          .MissingForbiddenCapabilityRemoval,
      );
    }
  }

  if (
    removedCapabilities.size !== proposal.forbiddenCapabilitiesToRemove.length
  ) {
    errors.push(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR
        .DuplicateForbiddenCapabilityRemoval,
    );
  }

  if (proposal.postFreezeAllowedActions.length === 0) {
    errors.push(XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.EmptyPostFreezeAllowedActions);
  }

  const deterministicAllowedActions = new Set<string>(
    Object.values(XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION),
  );

  for (const action of proposal.postFreezeAllowedActions as readonly string[]) {
    if (!deterministicAllowedActions.has(action)) {
      errors.push(
        XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.NonDeterministicPostFreezeAction,
      );
    }
  }

  return {
    canExecute: errors.length === 0,
    errors,
    nextAuthorityState:
      errors.length === 0
        ? XXXL_RUNTIME_AUTHORITY_STATE.Frozen
        : proposal.currentAuthorityState,
  };
}

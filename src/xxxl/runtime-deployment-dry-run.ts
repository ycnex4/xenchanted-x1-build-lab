import {
  validateXXXLRuntimeIncidentResponsePolicy,
  type XXXLRuntimeIncidentResponsePolicyCandidate,
} from "./runtime-incident-policy.js";
import {
  validateXXXLRuntimeRouteGuardianFinalityPolicy,
  type XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
} from "./runtime-route-policy.js";

export const XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_VERSION = 1;

export const XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE = {
  OfflineOnly: "OFFLINE_ONLY",
} as const;

export type XXXLRuntimeDeploymentDryRunMode =
  (typeof XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE)[keyof typeof XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE];

export const XXXL_RUNTIME_DRY_RUN_CHECK = {
  RoutePolicyValidation: "ROUTE_POLICY_VALIDATION",
  IncidentPolicyValidation: "INCIDENT_POLICY_VALIDATION",
  AccountSchemaValidation: "ACCOUNT_SCHEMA_VALIDATION",
  TransitionSimulation: "TRANSITION_SIMULATION",
  GenesisSupplyInvariant: "GENESIS_SUPPLY_INVARIANT",
  NoManualMintPath: "NO_MANUAL_MINT_PATH",
  NoPremine: "NO_PREMINE",
  NoFounderAllocation: "NO_FOUNDER_ALLOCATION",
  NoRpcUsage: "NO_RPC_USAGE",
  NoSecrets: "NO_SECRETS",
  AuthorityFreezePlan: "AUTHORITY_FREEZE_PLAN",
  PublicDisclosureReady: "PUBLIC_DISCLOSURE_READY",
} as const;

export type XXXLRuntimeDryRunCheck =
  (typeof XXXL_RUNTIME_DRY_RUN_CHECK)[keyof typeof XXXL_RUNTIME_DRY_RUN_CHECK];

export const XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS = [
  XXXL_RUNTIME_DRY_RUN_CHECK.RoutePolicyValidation,
  XXXL_RUNTIME_DRY_RUN_CHECK.IncidentPolicyValidation,
  XXXL_RUNTIME_DRY_RUN_CHECK.AccountSchemaValidation,
  XXXL_RUNTIME_DRY_RUN_CHECK.TransitionSimulation,
  XXXL_RUNTIME_DRY_RUN_CHECK.GenesisSupplyInvariant,
  XXXL_RUNTIME_DRY_RUN_CHECK.NoManualMintPath,
  XXXL_RUNTIME_DRY_RUN_CHECK.NoPremine,
  XXXL_RUNTIME_DRY_RUN_CHECK.NoFounderAllocation,
  XXXL_RUNTIME_DRY_RUN_CHECK.NoRpcUsage,
  XXXL_RUNTIME_DRY_RUN_CHECK.NoSecrets,
  XXXL_RUNTIME_DRY_RUN_CHECK.AuthorityFreezePlan,
  XXXL_RUNTIME_DRY_RUN_CHECK.PublicDisclosureReady,
] as const;

export const XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY = {
  RpcUsage: "RPC_USAGE",
  LiveDeployment: "LIVE_DEPLOYMENT",
  SecretMaterial: "SECRET_MATERIAL",
  ManualMint: "MANUAL_MINT",
  BalanceRewrite: "BALANCE_REWRITE",
  FounderAllocation: "FOUNDER_ALLOCATION",
  Premine: "PREMINE",
  UpgradeBypass: "UPGRADE_BYPASS",
} as const;

export type XXXLRuntimeForbiddenDryRunCapability =
  (typeof XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY)[keyof typeof XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY];

export const XXXL_RUNTIME_DRY_RUN_ARTIFACT = {
  ParameterManifest: "PARAMETER_MANIFEST",
  TestReport: "TEST_REPORT",
  SupplyInvariantReport: "SUPPLY_INVARIANT_REPORT",
  IncidentRunbook: "INCIDENT_RUNBOOK",
  FreezePlan: "FREEZE_PLAN",
  PublicDisclosureDraft: "PUBLIC_DISCLOSURE_DRAFT",
} as const;

export type XXXLRuntimeDryRunArtifact =
  (typeof XXXL_RUNTIME_DRY_RUN_ARTIFACT)[keyof typeof XXXL_RUNTIME_DRY_RUN_ARTIFACT];

export const XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS = [
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.ParameterManifest,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.TestReport,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.SupplyInvariantReport,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.IncidentRunbook,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.FreezePlan,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT.PublicDisclosureDraft,
] as const;

export const XXXL_RUNTIME_DRY_RUN_CHECK_STATUS = {
  Passed: "PASSED",
  Failed: "FAILED",
} as const;

export type XXXLRuntimeDryRunCheckStatus =
  (typeof XXXL_RUNTIME_DRY_RUN_CHECK_STATUS)[keyof typeof XXXL_RUNTIME_DRY_RUN_CHECK_STATUS];

export const XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR = {
  InvalidRoutePolicy: "INVALID_ROUTE_POLICY",
  InvalidIncidentPolicy: "INVALID_INCIDENT_POLICY",
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  WrongDryRunMode: "WRONG_DRY_RUN_MODE",
  MissingMandatoryCheck: "MISSING_MANDATORY_CHECK",
  DuplicateDryRunCheck: "DUPLICATE_DRY_RUN_CHECK",
  MissingForbiddenCapability: "MISSING_FORBIDDEN_CAPABILITY",
  MissingMandatoryArtifact: "MISSING_MANDATORY_ARTIFACT",
  DuplicateArtifact: "DUPLICATE_ARTIFACT",
  FailedCheck: "FAILED_CHECK",
  MissingCheckResult: "MISSING_CHECK_RESULT",
  ForbiddenCapabilityDetected: "FORBIDDEN_CAPABILITY_DETECTED",
  MissingArtifact: "MISSING_ARTIFACT",
  RpcWasUsed: "RPC_WAS_USED",
  DeploymentWasAttempted: "DEPLOYMENT_WAS_ATTEMPTED",
  SecretsWereDetected: "SECRETS_WERE_DETECTED",
} as const;

export type XXXLRuntimeDeploymentDryRunErrorCode =
  (typeof XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR)[keyof typeof XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR];

export type XXXLRuntimeDeploymentDryRunPolicyCandidate = {
  readonly version: number;
  readonly mode: XXXLRuntimeDeploymentDryRunMode;
  readonly requiredChecks: readonly XXXLRuntimeDryRunCheck[];
  readonly forbiddenCapabilities: readonly XXXLRuntimeForbiddenDryRunCapability[];
  readonly requiredArtifacts: readonly XXXLRuntimeDryRunArtifact[];
};

export type XXXLRuntimeDeploymentDryRunValidationInput = {
  readonly routePolicy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate;
  readonly incidentPolicy: XXXLRuntimeIncidentResponsePolicyCandidate;
  readonly dryRunPolicy: XXXLRuntimeDeploymentDryRunPolicyCandidate;
};

export type XXXLRuntimeDeploymentDryRunCheckResult = {
  readonly check: XXXLRuntimeDryRunCheck;
  readonly status: XXXLRuntimeDryRunCheckStatus;
  readonly evidenceId: string;
};

export type XXXLRuntimeDeploymentDryRunReport = {
  readonly checkResults: readonly XXXLRuntimeDeploymentDryRunCheckResult[];
  readonly detectedForbiddenCapabilities: readonly XXXLRuntimeForbiddenDryRunCapability[];
  readonly producedArtifacts: readonly XXXLRuntimeDryRunArtifact[];
  readonly rpcUsed: boolean;
  readonly deploymentAttempted: boolean;
  readonly secretsDetected: boolean;
};

export type XXXLRuntimeDeploymentDryRunValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeDeploymentDryRunErrorCode[];
};

export type XXXLRuntimeDeploymentDryRunEvaluationResult = {
  readonly accepted: boolean;
  readonly errors: XXXLRuntimeDeploymentDryRunErrorCode[];
  readonly passedChecks: readonly XXXLRuntimeDryRunCheck[];
  readonly failedChecks: readonly XXXLRuntimeDryRunCheck[];
};

export function validateXXXLRuntimeDeploymentDryRunPolicy(
  input: XXXLRuntimeDeploymentDryRunValidationInput,
): XXXLRuntimeDeploymentDryRunValidationResult {
  const errors: XXXLRuntimeDeploymentDryRunErrorCode[] = [];

  const routeValidation = validateXXXLRuntimeRouteGuardianFinalityPolicy(
    input.routePolicy,
  );
  if (!routeValidation.ok) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.InvalidRoutePolicy);
  }

  const incidentValidation = validateXXXLRuntimeIncidentResponsePolicy({
    routePolicy: input.routePolicy,
    incidentPolicy: input.incidentPolicy,
  });
  if (!incidentValidation.ok) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.InvalidIncidentPolicy);
  }

  const { dryRunPolicy } = input;

  if (dryRunPolicy.version !== XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_VERSION) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.UnsupportedVersion);
  }

  if (dryRunPolicy.mode !== XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE.OfflineOnly) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.WrongDryRunMode);
  }

  const requiredChecks = new Set(dryRunPolicy.requiredChecks);
  for (const mandatoryCheck of XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS) {
    if (!requiredChecks.has(mandatoryCheck)) {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingMandatoryCheck);
    }
  }

  if (requiredChecks.size !== dryRunPolicy.requiredChecks.length) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DuplicateDryRunCheck);
  }

  const forbiddenCapabilities = new Set(dryRunPolicy.forbiddenCapabilities);
  for (const capability of Object.values(
    XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY,
  )) {
    if (!forbiddenCapabilities.has(capability)) {
      errors.push(
        XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingForbiddenCapability,
      );
    }
  }

  const requiredArtifacts = new Set(dryRunPolicy.requiredArtifacts);
  for (const artifact of XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS) {
    if (!requiredArtifacts.has(artifact)) {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingMandatoryArtifact);
    }
  }

  if (requiredArtifacts.size !== dryRunPolicy.requiredArtifacts.length) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DuplicateArtifact);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function evaluateXXXLRuntimeDeploymentDryRunReport(
  input: XXXLRuntimeDeploymentDryRunValidationInput & {
    readonly report: XXXLRuntimeDeploymentDryRunReport;
  },
): XXXLRuntimeDeploymentDryRunEvaluationResult {
  const validation = validateXXXLRuntimeDeploymentDryRunPolicy(input);
  const errors: XXXLRuntimeDeploymentDryRunErrorCode[] = [...validation.errors];
  const { dryRunPolicy, report } = input;

  const resultByCheck = new Map<XXXLRuntimeDryRunCheck, XXXLRuntimeDeploymentDryRunCheckResult>();
  for (const result of report.checkResults) {
    resultByCheck.set(result.check, result);
  }

  const passedChecks: XXXLRuntimeDryRunCheck[] = [];
  const failedChecks: XXXLRuntimeDryRunCheck[] = [];

  for (const requiredCheck of dryRunPolicy.requiredChecks) {
    const result = resultByCheck.get(requiredCheck);

    if (result === undefined) {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingCheckResult);
      failedChecks.push(requiredCheck);
      continue;
    }

    if (result.status !== XXXL_RUNTIME_DRY_RUN_CHECK_STATUS.Passed) {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.FailedCheck);
      failedChecks.push(requiredCheck);
      continue;
    }

    if (result.evidenceId.trim() === "") {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingCheckResult);
      failedChecks.push(requiredCheck);
      continue;
    }

    passedChecks.push(requiredCheck);
  }

  for (const detected of report.detectedForbiddenCapabilities) {
    if (dryRunPolicy.forbiddenCapabilities.includes(detected)) {
      errors.push(
        XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.ForbiddenCapabilityDetected,
      );
    }
  }

  for (const artifact of dryRunPolicy.requiredArtifacts) {
    if (!report.producedArtifacts.includes(artifact)) {
      errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingArtifact);
    }
  }

  if (report.rpcUsed) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.RpcWasUsed);
  }

  if (report.deploymentAttempted) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DeploymentWasAttempted);
  }

  if (report.secretsDetected) {
    errors.push(XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.SecretsWereDetected);
  }

  return {
    accepted: errors.length === 0,
    errors,
    passedChecks,
    failedChecks,
  };
}

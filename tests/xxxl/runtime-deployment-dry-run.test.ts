import { describe, expect, it } from "vitest";

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
  XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR,
  XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE,
  XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_VERSION,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT,
  XXXL_RUNTIME_DRY_RUN_CHECK,
  XXXL_RUNTIME_DRY_RUN_CHECK_STATUS,
  XXXL_RUNTIME_FINALITY_KIND,
  XXXL_RUNTIME_FINALITY_STATUS,
  XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY,
  XXXL_RUNTIME_GUARDIAN_ROTATION_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INCIDENT_ACTION,
  XXXL_RUNTIME_INCIDENT_KIND,
  XXXL_RUNTIME_INCIDENT_POLICY_VERSION,
  XXXL_RUNTIME_INCIDENT_SEVERITY,
  XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS,
  XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS,
  XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
  XXXL_RUNTIME_ROUTE_POLICY_VERSION,
  XXXL_RUNTIME_ROUTE_STATUS,
  evaluateXXXLRuntimeDeploymentDryRunReport,
  validateXXXLRuntimeDeploymentDryRunPolicy,
  type XXXLRuntimeDeploymentDryRunPolicyCandidate,
  type XXXLRuntimeDeploymentDryRunReport,
  type XXXLRuntimeIncidentResponsePolicyCandidate,
  type XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
} from "../../src/index.js";

function validRoutePolicy(): XXXLRuntimeRouteGuardianFinalityPolicyCandidate {
  return {
    route: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      routeId: XXXL_GATEWAY_ROUTE_ID,
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      sourceToken: "0x1111111111111111111111111111111111111111",
      targetMintToken: XXXL_MINT_TOKEN,
      targetX1NetworkId: "x1-mainnet",
      targetMintCoreId: "xxxl-mint-core",
      guardianSetId: "guardian-set-1",
      quorumThreshold: 2,
      finalityRuleId: "ethereum-finalized",
      status: XXXL_RUNTIME_ROUTE_STATUS.Active,
    },
    guardian: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      guardianSetId: "guardian-set-1",
      guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
      quorumThreshold: 2,
      rotationMode: XXXL_RUNTIME_GUARDIAN_ROTATION_MODE.TimelockedMultisig,
      rotationTimelockSeconds: XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
      emergencyFreezeThreshold: 2,
      status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
    },
    finality: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      finalityRuleId: "ethereum-finalized",
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      kind: XXXL_RUNTIME_FINALITY_KIND.EthereumFinalized,
      minConfirmations: 0,
      status: XXXL_RUNTIME_FINALITY_STATUS.Active,
    },
  };
}

function validIncidentPolicy(): XXXLRuntimeIncidentResponsePolicyCandidate {
  return {
    version: XXXL_RUNTIME_INCIDENT_POLICY_VERSION,
    coveredIncidentKinds: [
      XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
      XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
      XXXL_RUNTIME_INCIDENT_KIND.ReplayAnomaly,
      XXXL_RUNTIME_INCIDENT_KIND.FinalityIssue,
      XXXL_RUNTIME_INCIDENT_KIND.SupplyMismatch,
      XXXL_RUNTIME_INCIDENT_KIND.UnexpectedMint,
    ],
    emergencyFreezeThreshold: 2,
    routePauseThreshold: 2,
    publicNoticeDeadlineSeconds: 6 * 60 * 60,
    postMortemDeadlineSeconds: 3 * 24 * 60 * 60,
    actionRules: [
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
          XXXL_RUNTIME_INCIDENT_ACTION.GuardianRotation,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
          XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
        ],
      },
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.High,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.PauseRoute,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
        ],
      },
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.ReplayAnomaly,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
          XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
        ],
      },
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.FinalityIssue,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
          XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
        ],
      },
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.SupplyMismatch,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
          XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
        ],
      },
      {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.UnexpectedMint,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        requiredActions: [
          XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
          XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
          XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
        ],
      },
    ],
  };
}

function validDryRunPolicy(): XXXLRuntimeDeploymentDryRunPolicyCandidate {
  return {
    version: XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_VERSION,
    mode: XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE.OfflineOnly,
    requiredChecks: [...XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS],
    forbiddenCapabilities: [
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.RpcUsage,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.LiveDeployment,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.SecretMaterial,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.ManualMint,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.BalanceRewrite,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.FounderAllocation,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.Premine,
      XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.UpgradeBypass,
    ],
    requiredArtifacts: [...XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS],
  };
}

function validReport(): XXXLRuntimeDeploymentDryRunReport {
  return {
    checkResults: XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS.map((check) => ({
      check,
      status: XXXL_RUNTIME_DRY_RUN_CHECK_STATUS.Passed,
      evidenceId: `${check}-evidence`,
    })),
    detectedForbiddenCapabilities: [],
    producedArtifacts: [...XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS],
    rpcUsed: false,
    deploymentAttempted: false,
    secretsDetected: false,
  };
}

describe("XXXL deployment dry-run model", () => {
  it("accepts a valid dry-run policy", () => {
    const result = validateXXXLRuntimeDeploymentDryRunPolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
    });

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects invalid route and incident policies", () => {
    const routePolicy = validRoutePolicy();
    const incidentPolicy = validIncidentPolicy();

    const result = validateXXXLRuntimeDeploymentDryRunPolicy({
      routePolicy: {
        ...routePolicy,
        route: {
          ...routePolicy.route,
          sourceToken: "",
        },
      },
      incidentPolicy: {
        ...incidentPolicy,
        emergencyFreezeThreshold: 1,
      },
      dryRunPolicy: validDryRunPolicy(),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.InvalidRoutePolicy,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.InvalidIncidentPolicy,
    ]);
  });

  it("rejects missing mandatory dry-run check", () => {
    const dryRunPolicy = validDryRunPolicy();

    const result = validateXXXLRuntimeDeploymentDryRunPolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: {
        ...dryRunPolicy,
        requiredChecks: dryRunPolicy.requiredChecks.filter(
          (check) => check !== XXXL_RUNTIME_DRY_RUN_CHECK.NoSecrets,
        ),
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingMandatoryCheck,
    ]);
  });

  it("rejects duplicate dry-run check and duplicate artifact", () => {
    const dryRunPolicy = validDryRunPolicy();

    const result = validateXXXLRuntimeDeploymentDryRunPolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: {
        ...dryRunPolicy,
        requiredChecks: [
          ...dryRunPolicy.requiredChecks,
          XXXL_RUNTIME_DRY_RUN_CHECK.NoSecrets,
        ],
        requiredArtifacts: [
          ...dryRunPolicy.requiredArtifacts,
          XXXL_RUNTIME_DRY_RUN_ARTIFACT.TestReport,
        ],
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DuplicateDryRunCheck,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DuplicateArtifact,
    ]);
  });

  it("rejects policy missing a forbidden capability", () => {
    const dryRunPolicy = validDryRunPolicy();

    const result = validateXXXLRuntimeDeploymentDryRunPolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: {
        ...dryRunPolicy,
        forbiddenCapabilities: dryRunPolicy.forbiddenCapabilities.filter(
          (capability) =>
            capability !== XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.ManualMint,
        ),
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingForbiddenCapability,
    ]);
  });

  it("accepts a successful dry-run report", () => {
    const result = evaluateXXXLRuntimeDeploymentDryRunReport({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
      report: validReport(),
    });

    expect(result.accepted).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.failedChecks).toEqual([]);
    expect(result.passedChecks).toEqual([...XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS]);
  });

  it("rejects missing check result", () => {
    const report = validReport();

    const result = evaluateXXXLRuntimeDeploymentDryRunReport({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
      report: {
        ...report,
        checkResults: report.checkResults.filter(
          (check) => check.check !== XXXL_RUNTIME_DRY_RUN_CHECK.NoRpcUsage,
        ),
      },
    });

    expect(result.accepted).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingCheckResult,
    ]);
    expect(result.failedChecks).toEqual([XXXL_RUNTIME_DRY_RUN_CHECK.NoRpcUsage]);
  });

  it("rejects failed check and missing evidence", () => {
    const report = validReport();

    const result = evaluateXXXLRuntimeDeploymentDryRunReport({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
      report: {
        ...report,
        checkResults: report.checkResults.map((entry) => {
          if (entry.check === XXXL_RUNTIME_DRY_RUN_CHECK.NoSecrets) {
            return {
              ...entry,
              status: XXXL_RUNTIME_DRY_RUN_CHECK_STATUS.Failed,
            };
          }

          if (entry.check === XXXL_RUNTIME_DRY_RUN_CHECK.NoRpcUsage) {
            return {
              ...entry,
              evidenceId: "",
            };
          }

          return entry;
        }),
      },
    });

    expect(result.accepted).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingCheckResult,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.FailedCheck,
    ]);
    expect(result.failedChecks).toEqual([
      XXXL_RUNTIME_DRY_RUN_CHECK.NoRpcUsage,
      XXXL_RUNTIME_DRY_RUN_CHECK.NoSecrets,
    ]);
  });

  it("rejects detected forbidden capability and unsafe execution flags", () => {
    const report = validReport();

    const result = evaluateXXXLRuntimeDeploymentDryRunReport({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
      report: {
        ...report,
        detectedForbiddenCapabilities: [
          XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY.ManualMint,
        ],
        rpcUsed: true,
        deploymentAttempted: true,
        secretsDetected: true,
      },
    });

    expect(result.accepted).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.ForbiddenCapabilityDetected,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.RpcWasUsed,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.DeploymentWasAttempted,
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.SecretsWereDetected,
    ]);
  });

  it("rejects missing required artifact", () => {
    const report = validReport();

    const result = evaluateXXXLRuntimeDeploymentDryRunReport({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      dryRunPolicy: validDryRunPolicy(),
      report: {
        ...report,
        producedArtifacts: report.producedArtifacts.filter(
          (artifact) =>
            artifact !== XXXL_RUNTIME_DRY_RUN_ARTIFACT.PublicDisclosureDraft,
        ),
      },
    });

    expect(result.accepted).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_ERROR.MissingArtifact,
    ]);
  });
});

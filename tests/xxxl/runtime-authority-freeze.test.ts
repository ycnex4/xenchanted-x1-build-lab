import { describe, expect, it } from "vitest";

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
  XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR,
  XXXL_RUNTIME_AUTHORITY_FREEZE_VERSION,
  XXXL_RUNTIME_AUTHORITY_STATE,
  XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_MODE,
  XXXL_RUNTIME_DEPLOYMENT_DRY_RUN_VERSION,
  XXXL_RUNTIME_DRY_RUN_ARTIFACT,
  XXXL_RUNTIME_DRY_RUN_CHECK,
  XXXL_RUNTIME_DRY_RUN_CHECK_STATUS,
  XXXL_RUNTIME_FINALITY_KIND,
  XXXL_RUNTIME_FINALITY_STATUS,
  XXXL_RUNTIME_FORBIDDEN_DRY_RUN_CAPABILITY,
  XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY,
  XXXL_RUNTIME_FREEZE_PREREQUISITE,
  XXXL_RUNTIME_GUARDIAN_ROTATION_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INCIDENT_ACTION,
  XXXL_RUNTIME_INCIDENT_KIND,
  XXXL_RUNTIME_INCIDENT_POLICY_VERSION,
  XXXL_RUNTIME_INCIDENT_SEVERITY,
  XXXL_RUNTIME_MANDATORY_DRY_RUN_ARTIFACTS,
  XXXL_RUNTIME_MANDATORY_DRY_RUN_CHECKS,
  XXXL_RUNTIME_MANDATORY_FORBIDDEN_POST_FREEZE_CAPABILITIES,
  XXXL_RUNTIME_MANDATORY_FREEZE_PREREQUISITES,
  XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS,
  XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
  XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION,
  XXXL_RUNTIME_ROUTE_POLICY_VERSION,
  XXXL_RUNTIME_ROUTE_STATUS,
  evaluateXXXLRuntimeAuthorityFreezeProposal,
  validateXXXLRuntimeAuthorityFreezePolicy,
  type XXXLRuntimeAuthorityFreezePolicyCandidate,
  type XXXLRuntimeAuthorityFreezeProposal,
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

function validDryRunReport(): XXXLRuntimeDeploymentDryRunReport {
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

function validFreezePolicy(): XXXLRuntimeAuthorityFreezePolicyCandidate {
  return {
    version: XXXL_RUNTIME_AUTHORITY_FREEZE_VERSION,
    guardianCount: 3,
    guardianQuorumThreshold: 2,
    emergencyFreezeThreshold: 2,
    authorityFreezeThreshold: 3,
    minTimelockSeconds: XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS,
  };
}

function validProposal(): XXXLRuntimeAuthorityFreezeProposal {
  return {
    proposalId: "freeze-proposal-1",
    programVersion: 1,
    proposedBy: "public-proposer",
    currentAuthorityState: XXXL_RUNTIME_AUTHORITY_STATE.StagedFinalization,
    timelockStartTs: 1_000,
    timelockEndTs:
      1_000 + XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS,
    executionTs:
      1_000 + XXXL_RUNTIME_MIN_AUTHORITY_FREEZE_TIMELOCK_SECONDS + 1,
    guardianApprovals: 3,
    prerequisites: [...XXXL_RUNTIME_MANDATORY_FREEZE_PREREQUISITES],
    forbiddenCapabilitiesToRemove: [
      ...XXXL_RUNTIME_MANDATORY_FORBIDDEN_POST_FREEZE_CAPABILITIES,
    ],
    postFreezeAllowedActions: [
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.ConsumeGatewayMint,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.RoutePause,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.EmergencyFreeze,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.GuardianRotation,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.PublicNotice,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.PostMortem,
      XXXL_RUNTIME_POST_FREEZE_ALLOWED_ACTION.RouteRetirement,
    ],
    publicDisclosureId: "public-disclosure-1",
    freezePlanId: "freeze-plan-1",
    dryRunEvidenceId: "dry-run-evidence-1",
    reviewEvidenceId: "review-evidence-1",
  };
}

function validInput(
  proposal: XXXLRuntimeAuthorityFreezeProposal = validProposal(),
  freezePolicy: XXXLRuntimeAuthorityFreezePolicyCandidate = validFreezePolicy(),
) {
  return {
    routePolicy: validRoutePolicy(),
    incidentPolicy: validIncidentPolicy(),
    dryRunPolicy: validDryRunPolicy(),
    dryRunReport: validDryRunReport(),
    freezePolicy,
    proposal,
  };
}

describe("XXXL authority freeze procedure model", () => {
  it("accepts a valid authority freeze policy", () => {
    const result = validateXXXLRuntimeAuthorityFreezePolicy(validFreezePolicy());

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("executes a valid authority freeze proposal into frozen state", () => {
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(validInput());

    expect(result.canExecute).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.nextAuthorityState).toBe(XXXL_RUNTIME_AUTHORITY_STATE.Frozen);
  });

  it("rejects freeze when deployment dry run is invalid", () => {
    const input = validInput();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal({
      ...input,
      dryRunReport: {
        ...input.dryRunReport,
        rpcUsed: true,
      },
    });

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InvalidDryRun,
    );
    expect(result.nextAuthorityState).toBe(
      XXXL_RUNTIME_AUTHORITY_STATE.StagedFinalization,
    );
  });

  it("rejects missing freeze prerequisites", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        prerequisites: proposal.prerequisites.filter(
          (item) =>
            item !== XXXL_RUNTIME_FREEZE_PREREQUISITE.X1NativeMechanicsComplete,
        ),
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.MissingMandatoryPrerequisite,
    );
  });

  it("rejects early or too-short timelock", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        timelockEndTs: proposal.timelockStartTs + 1,
        executionTs: proposal.timelockStartTs + 1,
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.TimelockTooShort,
    );
  });

  it("rejects execution before timelock expiry", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        executionTs: proposal.timelockEndTs - 1,
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.TimelockNotExpired,
    );
  });

  it("rejects insufficient guardian approvals", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        guardianApprovals: 2,
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.InsufficientGuardianApprovals,
    );
  });

  it("rejects authority freeze threshold weaker than emergency freeze", () => {
    const result = validateXXXLRuntimeAuthorityFreezePolicy({
      ...validFreezePolicy(),
      emergencyFreezeThreshold: 3,
      authorityFreezeThreshold: 2,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR
        .AuthorityFreezeWeakerThanEmergencyFreeze,
    );
  });

  it("rejects proposal that does not remove hidden admin capabilities", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        forbiddenCapabilitiesToRemove:
          proposal.forbiddenCapabilitiesToRemove.filter(
            (capability) =>
              capability !==
              XXXL_RUNTIME_FORBIDDEN_POST_FREEZE_CAPABILITY.ProgramUpgrade,
          ),
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR
        .MissingForbiddenCapabilityRemoval,
    );
  });

  it("rejects non-deterministic post-freeze allowed actions", () => {
    const proposal = validProposal();
    const result = evaluateXXXLRuntimeAuthorityFreezeProposal(
      validInput({
        ...proposal,
        postFreezeAllowedActions: [
          ...proposal.postFreezeAllowedActions,
          "PROGRAM_UPGRADE" as never,
        ],
      }),
    );

    expect(result.canExecute).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_AUTHORITY_FREEZE_ERROR.NonDeterministicPostFreezeAction,
    );
  });
});

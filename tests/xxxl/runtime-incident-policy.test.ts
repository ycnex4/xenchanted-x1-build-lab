import { describe, expect, it } from "vitest";

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
  XXXL_RUNTIME_FINALITY_KIND,
  XXXL_RUNTIME_FINALITY_STATUS,
  XXXL_RUNTIME_GUARDIAN_ROTATION_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INCIDENT_ACTION,
  XXXL_RUNTIME_INCIDENT_KIND,
  XXXL_RUNTIME_INCIDENT_POLICY_ERROR,
  XXXL_RUNTIME_INCIDENT_POLICY_VERSION,
  XXXL_RUNTIME_INCIDENT_SEVERITY,
  XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
  XXXL_RUNTIME_ROUTE_POLICY_VERSION,
  XXXL_RUNTIME_ROUTE_STATUS,
  evaluateXXXLRuntimeIncidentResponse,
  validateXXXLRuntimeIncidentResponsePolicy,
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

describe("XXXL runtime incident response and emergency freeze policy", () => {
  it("accepts a valid incident response policy", () => {
    const result = validateXXXLRuntimeIncidentResponsePolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
    });

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects missing mandatory incident kind", () => {
    const incidentPolicy = validIncidentPolicy();
    const result = validateXXXLRuntimeIncidentResponsePolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: {
        ...incidentPolicy,
        coveredIncidentKinds: incidentPolicy.coveredIncidentKinds.filter(
          (kind) => kind !== XXXL_RUNTIME_INCIDENT_KIND.UnexpectedMint,
        ),
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingMandatoryIncidentKind,
    );
    expect(result.errors).toContain(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.RuleForUncoveredIncident,
    );
  });

  it("rejects invalid thresholds and deadlines", () => {
    const result = validateXXXLRuntimeIncidentResponsePolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: {
        ...validIncidentPolicy(),
        emergencyFreezeThreshold: 1,
        routePauseThreshold: 4,
        publicNoticeDeadlineSeconds: 0,
        postMortemDeadlineSeconds: 8 * 24 * 60 * 60,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidEmergencyFreezeThreshold,
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidRoutePauseThreshold,
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidPublicNoticeDeadline,
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidPostMortemDeadline,
    ]);
  });

  it("rejects duplicate covered incident kinds", () => {
    const incidentPolicy = validIncidentPolicy();
    const result = validateXXXLRuntimeIncidentResponsePolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: {
        ...incidentPolicy,
        coveredIncidentKinds: [
          ...incidentPolicy.coveredIncidentKinds,
          XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
        ],
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.DuplicateIncidentKind,
    ]);
  });

  it("rejects critical action rules without emergency freeze and public notice", () => {
    const incidentPolicy = validIncidentPolicy();
    const result = validateXXXLRuntimeIncidentResponsePolicy({
      routePolicy: validRoutePolicy(),
      incidentPolicy: {
        ...incidentPolicy,
        actionRules: [
          {
            incidentKind: XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
            severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
            requiredActions: [XXXL_RUNTIME_INCIDENT_ACTION.PostMortem],
          },
        ],
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingEmergencyFreezeAction,
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingPublicNoticeAction,
    ]);
  });

  it("accepts a critical guardian compromise response with enough approvals", () => {
    const decision = evaluateXXXLRuntimeIncidentResponse({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      report: {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        evidenceIds: ["evidence-1"],
        guardianApprovals: 2,
      },
    });

    expect(decision.accepted).toBe(true);
    expect(decision.errors).toEqual([]);
    expect(decision.mustEmergencyFreeze).toBe(true);
    expect(decision.mustPublishNotice).toBe(true);
    expect(decision.mustPreparePostMortem).toBe(true);
  });

  it("rejects critical response without enough emergency freeze approvals", () => {
    const decision = evaluateXXXLRuntimeIncidentResponse({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      report: {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.SupplyMismatch,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.Critical,
        evidenceIds: ["evidence-1"],
        guardianApprovals: 1,
      },
    });

    expect(decision.accepted).toBe(false);
    expect(decision.errors).toEqual([
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InsufficientEmergencyFreezeApprovals,
    ]);
  });

  it("accepts high route anomaly response with enough pause approvals", () => {
    const decision = evaluateXXXLRuntimeIncidentResponse({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      report: {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.High,
        evidenceIds: ["evidence-1"],
        guardianApprovals: 2,
      },
    });

    expect(decision.accepted).toBe(true);
    expect(decision.mustPauseRoute).toBe(true);
    expect(decision.mustPublishNotice).toBe(true);
    expect(decision.mustEmergencyFreeze).toBe(false);
  });

  it("rejects incident report without evidence", () => {
    const decision = evaluateXXXLRuntimeIncidentResponse({
      routePolicy: validRoutePolicy(),
      incidentPolicy: validIncidentPolicy(),
      report: {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.High,
        evidenceIds: [],
        guardianApprovals: 2,
      },
    });

    expect(decision.accepted).toBe(false);
    expect(decision.errors).toEqual([
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingEvidence,
    ]);
  });

  it("rejects uncovered or unrouted incident response", () => {
    const incidentPolicy = validIncidentPolicy();
    const decision = evaluateXXXLRuntimeIncidentResponse({
      routePolicy: validRoutePolicy(),
      incidentPolicy: {
        ...incidentPolicy,
        coveredIncidentKinds: incidentPolicy.coveredIncidentKinds.filter(
          (kind) => kind !== XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        ),
        actionRules: incidentPolicy.actionRules.filter(
          (rule) => rule.incidentKind !== XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        ),
      },
      report: {
        incidentKind: XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
        severity: XXXL_RUNTIME_INCIDENT_SEVERITY.High,
        evidenceIds: ["evidence-1"],
        guardianApprovals: 2,
      },
    });

    expect(decision.accepted).toBe(false);
    expect(decision.errors).toContain(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingMandatoryIncidentKind,
    );
    expect(decision.errors).toContain(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.IncidentKindNotCovered,
    );
    expect(decision.errors).toContain(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingActionRule,
    );
  });
});

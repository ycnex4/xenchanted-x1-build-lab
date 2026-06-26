import {
  validateXXXLRuntimeRouteGuardianFinalityPolicy,
  type XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
} from "./runtime-route-policy.js";

export const XXXL_RUNTIME_INCIDENT_POLICY_VERSION = 1;

export const XXXL_RUNTIME_INCIDENT_KIND = {
  GuardianCompromise: "GUARDIAN_COMPROMISE",
  RouteAnomaly: "ROUTE_ANOMALY",
  ReplayAnomaly: "REPLAY_ANOMALY",
  FinalityIssue: "FINALITY_ISSUE",
  SupplyMismatch: "SUPPLY_MISMATCH",
  UnexpectedMint: "UNEXPECTED_MINT",
} as const;

export type XXXLRuntimeIncidentKind =
  (typeof XXXL_RUNTIME_INCIDENT_KIND)[keyof typeof XXXL_RUNTIME_INCIDENT_KIND];

export const XXXL_RUNTIME_INCIDENT_SEVERITY = {
  Watch: "WATCH",
  High: "HIGH",
  Critical: "CRITICAL",
} as const;

export type XXXLRuntimeIncidentSeverity =
  (typeof XXXL_RUNTIME_INCIDENT_SEVERITY)[keyof typeof XXXL_RUNTIME_INCIDENT_SEVERITY];

export const XXXL_RUNTIME_INCIDENT_ACTION = {
  Observe: "OBSERVE",
  PauseRoute: "PAUSE_ROUTE",
  EmergencyFreeze: "EMERGENCY_FREEZE",
  GuardianRotation: "GUARDIAN_ROTATION",
  PublicNotice: "PUBLIC_NOTICE",
  PostMortem: "POST_MORTEM",
} as const;

export type XXXLRuntimeIncidentAction =
  (typeof XXXL_RUNTIME_INCIDENT_ACTION)[keyof typeof XXXL_RUNTIME_INCIDENT_ACTION];

export const XXXL_RUNTIME_MANDATORY_INCIDENT_KINDS = [
  XXXL_RUNTIME_INCIDENT_KIND.GuardianCompromise,
  XXXL_RUNTIME_INCIDENT_KIND.RouteAnomaly,
  XXXL_RUNTIME_INCIDENT_KIND.ReplayAnomaly,
  XXXL_RUNTIME_INCIDENT_KIND.FinalityIssue,
  XXXL_RUNTIME_INCIDENT_KIND.SupplyMismatch,
  XXXL_RUNTIME_INCIDENT_KIND.UnexpectedMint,
] as const;

export const XXXL_RUNTIME_INCIDENT_POLICY_ERROR = {
  InvalidRoutePolicy: "INVALID_ROUTE_POLICY",
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  MissingMandatoryIncidentKind: "MISSING_MANDATORY_INCIDENT_KIND",
  DuplicateIncidentKind: "DUPLICATE_INCIDENT_KIND",
  InvalidEmergencyFreezeThreshold: "INVALID_EMERGENCY_FREEZE_THRESHOLD",
  InvalidRoutePauseThreshold: "INVALID_ROUTE_PAUSE_THRESHOLD",
  InvalidPublicNoticeDeadline: "INVALID_PUBLIC_NOTICE_DEADLINE",
  InvalidPostMortemDeadline: "INVALID_POST_MORTEM_DEADLINE",
  RuleForUncoveredIncident: "RULE_FOR_UNCOVERED_INCIDENT",
  EmptyActionRule: "EMPTY_ACTION_RULE",
  MissingEmergencyFreezeAction: "MISSING_EMERGENCY_FREEZE_ACTION",
  MissingPublicNoticeAction: "MISSING_PUBLIC_NOTICE_ACTION",
  IncidentKindNotCovered: "INCIDENT_KIND_NOT_COVERED",
  MissingEvidence: "MISSING_EVIDENCE",
  MissingActionRule: "MISSING_ACTION_RULE",
  InsufficientEmergencyFreezeApprovals: "INSUFFICIENT_EMERGENCY_FREEZE_APPROVALS",
  InsufficientRoutePauseApprovals: "INSUFFICIENT_ROUTE_PAUSE_APPROVALS",
} as const;

export type XXXLRuntimeIncidentPolicyErrorCode =
  (typeof XXXL_RUNTIME_INCIDENT_POLICY_ERROR)[keyof typeof XXXL_RUNTIME_INCIDENT_POLICY_ERROR];

export type XXXLRuntimeIncidentActionRule = {
  readonly incidentKind: XXXLRuntimeIncidentKind;
  readonly severity: XXXLRuntimeIncidentSeverity;
  readonly requiredActions: readonly XXXLRuntimeIncidentAction[];
};

export type XXXLRuntimeIncidentResponsePolicyCandidate = {
  readonly version: number;
  readonly coveredIncidentKinds: readonly XXXLRuntimeIncidentKind[];
  readonly emergencyFreezeThreshold: number;
  readonly routePauseThreshold: number;
  readonly publicNoticeDeadlineSeconds: number;
  readonly postMortemDeadlineSeconds: number;
  readonly actionRules: readonly XXXLRuntimeIncidentActionRule[];
};

export type XXXLRuntimeIncidentResponsePolicyValidationInput = {
  readonly routePolicy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate;
  readonly incidentPolicy: XXXLRuntimeIncidentResponsePolicyCandidate;
};

export type XXXLRuntimeIncidentReport = {
  readonly incidentKind: XXXLRuntimeIncidentKind;
  readonly severity: XXXLRuntimeIncidentSeverity;
  readonly evidenceIds: readonly string[];
  readonly guardianApprovals: number;
};

export type XXXLRuntimeIncidentResponseDecision = {
  readonly accepted: boolean;
  readonly errors: XXXLRuntimeIncidentPolicyErrorCode[];
  readonly requiredActions: readonly XXXLRuntimeIncidentAction[];
  readonly mustPauseRoute: boolean;
  readonly mustEmergencyFreeze: boolean;
  readonly mustPublishNotice: boolean;
  readonly mustPreparePostMortem: boolean;
};

export type XXXLRuntimeIncidentPolicyValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeIncidentPolicyErrorCode[];
};

export function validateXXXLRuntimeIncidentResponsePolicy(
  input: XXXLRuntimeIncidentResponsePolicyValidationInput,
): XXXLRuntimeIncidentPolicyValidationResult {
  const errors: XXXLRuntimeIncidentPolicyErrorCode[] = [];
  const { routePolicy, incidentPolicy } = input;
  const routeValidation =
    validateXXXLRuntimeRouteGuardianFinalityPolicy(routePolicy);

  if (!routeValidation.ok) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidRoutePolicy);
  }

  if (incidentPolicy.version !== XXXL_RUNTIME_INCIDENT_POLICY_VERSION) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.UnsupportedVersion);
  }

  const covered = new Set(incidentPolicy.coveredIncidentKinds);

  for (const mandatoryKind of XXXL_RUNTIME_MANDATORY_INCIDENT_KINDS) {
    if (!covered.has(mandatoryKind)) {
      errors.push(
        XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingMandatoryIncidentKind,
      );
    }
  }

  if (covered.size !== incidentPolicy.coveredIncidentKinds.length) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.DuplicateIncidentKind);
  }

  const guardianCount = routePolicy.guardian.guardianPublicKeys.length;
  const quorumThreshold = routePolicy.guardian.quorumThreshold;

  if (
    incidentPolicy.emergencyFreezeThreshold < quorumThreshold ||
    incidentPolicy.emergencyFreezeThreshold > guardianCount
  ) {
    errors.push(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidEmergencyFreezeThreshold,
    );
  }

  if (
    incidentPolicy.routePauseThreshold < quorumThreshold ||
    incidentPolicy.routePauseThreshold > guardianCount
  ) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidRoutePauseThreshold);
  }

  if (
    incidentPolicy.publicNoticeDeadlineSeconds <= 0 ||
    incidentPolicy.publicNoticeDeadlineSeconds > 24 * 60 * 60
  ) {
    errors.push(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidPublicNoticeDeadline,
    );
  }

  if (
    incidentPolicy.postMortemDeadlineSeconds <= 0 ||
    incidentPolicy.postMortemDeadlineSeconds > 7 * 24 * 60 * 60
  ) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InvalidPostMortemDeadline);
  }

  for (const rule of incidentPolicy.actionRules) {
    if (!covered.has(rule.incidentKind)) {
      errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.RuleForUncoveredIncident);
    }

    if (rule.requiredActions.length === 0) {
      errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.EmptyActionRule);
    }

    if (
      rule.severity === XXXL_RUNTIME_INCIDENT_SEVERITY.Critical &&
      !rule.requiredActions.includes(
        XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
      )
    ) {
      errors.push(
        XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingEmergencyFreezeAction,
      );
    }

    if (
      rule.severity === XXXL_RUNTIME_INCIDENT_SEVERITY.Critical &&
      !rule.requiredActions.includes(XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice)
    ) {
      errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingPublicNoticeAction);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function evaluateXXXLRuntimeIncidentResponse(
  input: XXXLRuntimeIncidentResponsePolicyValidationInput & {
    readonly report: XXXLRuntimeIncidentReport;
  },
): XXXLRuntimeIncidentResponseDecision {
  const policyValidation = validateXXXLRuntimeIncidentResponsePolicy(input);
  const errors: XXXLRuntimeIncidentPolicyErrorCode[] = [
    ...policyValidation.errors,
  ];
  const { incidentPolicy, report } = input;

  if (!incidentPolicy.coveredIncidentKinds.includes(report.incidentKind)) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.IncidentKindNotCovered);
  }

  if (report.evidenceIds.length === 0) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingEvidence);
  }

  const rule = incidentPolicy.actionRules.find(
    (candidate) =>
      candidate.incidentKind === report.incidentKind &&
      candidate.severity === report.severity,
  );

  if (rule === undefined) {
    errors.push(XXXL_RUNTIME_INCIDENT_POLICY_ERROR.MissingActionRule);
  }

  const requiredActions = rule?.requiredActions ?? [];

  if (
    requiredActions.includes(XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze) &&
    report.guardianApprovals < incidentPolicy.emergencyFreezeThreshold
  ) {
    errors.push(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InsufficientEmergencyFreezeApprovals,
    );
  }

  if (
    requiredActions.includes(XXXL_RUNTIME_INCIDENT_ACTION.PauseRoute) &&
    report.guardianApprovals < incidentPolicy.routePauseThreshold
  ) {
    errors.push(
      XXXL_RUNTIME_INCIDENT_POLICY_ERROR.InsufficientRoutePauseApprovals,
    );
  }

  return {
    accepted: errors.length === 0,
    errors,
    requiredActions,
    mustPauseRoute: requiredActions.includes(
      XXXL_RUNTIME_INCIDENT_ACTION.PauseRoute,
    ),
    mustEmergencyFreeze: requiredActions.includes(
      XXXL_RUNTIME_INCIDENT_ACTION.EmergencyFreeze,
    ),
    mustPublishNotice: requiredActions.includes(
      XXXL_RUNTIME_INCIDENT_ACTION.PublicNotice,
    ),
    mustPreparePostMortem: requiredActions.includes(
      XXXL_RUNTIME_INCIDENT_ACTION.PostMortem,
    ),
  };
}

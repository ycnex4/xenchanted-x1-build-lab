import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
} from "./program-v1.js";
import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_ROUTE_STATUS,
  type XXXLGatewayConfigAccount,
  type XXXLGuardianSetAccount,
} from "./runtime-candidate.js";

export const XXXL_RUNTIME_ROUTE_POLICY_VERSION = 1;

export const XXXL_RUNTIME_FINALITY_KIND = {
  EthereumFinalized: "ETHEREUM_FINALIZED",
  EthereumSafeWithConfirmations: "ETHEREUM_SAFE_WITH_CONFIRMATIONS",
} as const;

export type XXXLRuntimeFinalityKind =
  (typeof XXXL_RUNTIME_FINALITY_KIND)[keyof typeof XXXL_RUNTIME_FINALITY_KIND];

export const XXXL_RUNTIME_FINALITY_STATUS = {
  Active: "ACTIVE",
  Retired: "RETIRED",
} as const;

export type XXXLRuntimeFinalityStatus =
  (typeof XXXL_RUNTIME_FINALITY_STATUS)[keyof typeof XXXL_RUNTIME_FINALITY_STATUS];

export const XXXL_RUNTIME_GUARDIAN_ROTATION_MODE = {
  TimelockedMultisig: "TIMELOCKED_MULTISIG",
} as const;

export type XXXLRuntimeGuardianRotationMode =
  (typeof XXXL_RUNTIME_GUARDIAN_ROTATION_MODE)[keyof typeof XXXL_RUNTIME_GUARDIAN_ROTATION_MODE];

export const XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS = 7 * 24 * 60 * 60;

export const XXXL_RUNTIME_POLICY_ERROR = {
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  WrongRouteId: "WRONG_ROUTE_ID",
  WrongSourceChainId: "WRONG_SOURCE_CHAIN_ID",
  MissingSourceToken: "MISSING_SOURCE_TOKEN",
  MissingTargetX1NetworkId: "MISSING_TARGET_X1_NETWORK_ID",
  MissingTargetMintCoreId: "MISSING_TARGET_MINT_CORE_ID",
  WrongTargetMintToken: "WRONG_TARGET_MINT_TOKEN",
  RouteNotActive: "ROUTE_NOT_ACTIVE",
  GuardianSetIdMismatch: "GUARDIAN_SET_ID_MISMATCH",
  FinalityRuleIdMismatch: "FINALITY_RULE_ID_MISMATCH",
  FinalitySourceChainMismatch: "FINALITY_SOURCE_CHAIN_MISMATCH",
  QuorumThresholdMismatch: "QUORUM_THRESHOLD_MISMATCH",
  EmptyGuardianSet: "EMPTY_GUARDIAN_SET",
  DuplicateGuardianPublicKey: "DUPLICATE_GUARDIAN_PUBLIC_KEY",
  InvalidQuorumThreshold: "INVALID_QUORUM_THRESHOLD",
  GuardianSetNotActive: "GUARDIAN_SET_NOT_ACTIVE",
  FinalityRuleNotActive: "FINALITY_RULE_NOT_ACTIVE",
  InvalidFinalityConfirmations: "INVALID_FINALITY_CONFIRMATIONS",
  RotationTimelockTooShort: "ROTATION_TIMELOCK_TOO_SHORT",
  InvalidEmergencyFreezeThreshold: "INVALID_EMERGENCY_FREEZE_THRESHOLD",
} as const;

export type XXXLRuntimePolicyErrorCode =
  (typeof XXXL_RUNTIME_POLICY_ERROR)[keyof typeof XXXL_RUNTIME_POLICY_ERROR];

export type XXXLRuntimePolicyValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimePolicyErrorCode[];
};

export type XXXLRuntimeRoutePolicyCandidate = {
  readonly version: number;
  readonly routeId: string;
  readonly sourceChainId: bigint;
  readonly sourceToken: string;
  readonly targetMintToken: string;
  readonly targetX1NetworkId: string;
  readonly targetMintCoreId: string;
  readonly guardianSetId: string;
  readonly quorumThreshold: number;
  readonly finalityRuleId: string;
  readonly status: typeof XXXL_RUNTIME_ROUTE_STATUS.Active | typeof XXXL_RUNTIME_ROUTE_STATUS.Frozen;
};

export type XXXLRuntimeGuardianPolicyCandidate = {
  readonly version: number;
  readonly guardianSetId: string;
  readonly guardianPublicKeys: readonly string[];
  readonly quorumThreshold: number;
  readonly rotationMode: XXXLRuntimeGuardianRotationMode;
  readonly rotationTimelockSeconds: number;
  readonly emergencyFreezeThreshold: number;
  readonly status:
    | typeof XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active
    | typeof XXXL_RUNTIME_GUARDIAN_SET_STATUS.Retired;
};

export type XXXLRuntimeFinalityPolicyCandidate = {
  readonly version: number;
  readonly finalityRuleId: string;
  readonly sourceChainId: bigint;
  readonly kind: XXXLRuntimeFinalityKind;
  readonly minConfirmations: number;
  readonly status: XXXLRuntimeFinalityStatus;
};

export type XXXLRuntimeRouteGuardianFinalityPolicyCandidate = {
  readonly route: XXXLRuntimeRoutePolicyCandidate;
  readonly guardian: XXXLRuntimeGuardianPolicyCandidate;
  readonly finality: XXXLRuntimeFinalityPolicyCandidate;
};

export function validateXXXLRuntimeRouteGuardianFinalityPolicy(
  policy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
): XXXLRuntimePolicyValidationResult {
  const errors: XXXLRuntimePolicyErrorCode[] = [];
  const { route, guardian, finality } = policy;

  if (
    route.version !== XXXL_RUNTIME_ROUTE_POLICY_VERSION ||
    guardian.version !== XXXL_RUNTIME_ROUTE_POLICY_VERSION ||
    finality.version !== XXXL_RUNTIME_ROUTE_POLICY_VERSION
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.UnsupportedVersion);
  }

  if (route.routeId !== XXXL_GATEWAY_ROUTE_ID) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.WrongRouteId);
  }

  if (route.sourceChainId !== BigInt(ETHEREUM_MAINNET_CHAIN_ID)) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.WrongSourceChainId);
  }

  if (route.sourceToken.trim() === "") {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.MissingSourceToken);
  }

  if (route.targetX1NetworkId.trim() === "") {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.MissingTargetX1NetworkId);
  }

  if (route.targetMintCoreId.trim() === "") {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.MissingTargetMintCoreId);
  }

  if (route.targetMintToken !== XXXL_MINT_TOKEN) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.WrongTargetMintToken);
  }

  if (route.status !== XXXL_RUNTIME_ROUTE_STATUS.Active) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.RouteNotActive);
  }

  if (route.guardianSetId !== guardian.guardianSetId) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.GuardianSetIdMismatch);
  }

  if (route.finalityRuleId !== finality.finalityRuleId) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.FinalityRuleIdMismatch);
  }

  if (route.sourceChainId !== finality.sourceChainId) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.FinalitySourceChainMismatch);
  }

  if (route.quorumThreshold !== guardian.quorumThreshold) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.QuorumThresholdMismatch);
  }

  if (guardian.guardianPublicKeys.length === 0) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.EmptyGuardianSet);
  }

  const normalizedGuardianKeys = guardian.guardianPublicKeys.map((key) =>
    key.toLowerCase(),
  );
  if (new Set(normalizedGuardianKeys).size !== normalizedGuardianKeys.length) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.DuplicateGuardianPublicKey);
  }

  if (
    guardian.quorumThreshold <= 0 ||
    guardian.quorumThreshold > guardian.guardianPublicKeys.length
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.InvalidQuorumThreshold);
  }

  if (guardian.status !== XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.GuardianSetNotActive);
  }

  if (finality.status !== XXXL_RUNTIME_FINALITY_STATUS.Active) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.FinalityRuleNotActive);
  }

  if (
    finality.kind === XXXL_RUNTIME_FINALITY_KIND.EthereumFinalized &&
    finality.minConfirmations !== 0
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.InvalidFinalityConfirmations);
  }

  if (
    finality.kind === XXXL_RUNTIME_FINALITY_KIND.EthereumSafeWithConfirmations &&
    finality.minConfirmations < 64
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.InvalidFinalityConfirmations);
  }

  if (
    guardian.rotationTimelockSeconds <
    XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.RotationTimelockTooShort);
  }

  if (
    guardian.emergencyFreezeThreshold < guardian.quorumThreshold ||
    guardian.emergencyFreezeThreshold > guardian.guardianPublicKeys.length
  ) {
    errors.push(XXXL_RUNTIME_POLICY_ERROR.InvalidEmergencyFreezeThreshold);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function buildXXXLRuntimeGatewayConfigAccount(
  policy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
): XXXLGatewayConfigAccount {
  return {
    kind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
    version: policy.route.version,
    routeId: policy.route.routeId,
    sourceChainId: policy.route.sourceChainId,
    sourceToken: policy.route.sourceToken,
    targetMintToken: policy.route.targetMintToken,
    targetX1NetworkId: policy.route.targetX1NetworkId,
    targetMintCoreId: policy.route.targetMintCoreId,
    guardianSetId: policy.route.guardianSetId,
    quorumThreshold: policy.route.quorumThreshold,
    finalityRuleId: policy.route.finalityRuleId,
    status: policy.route.status,
  };
}

export function buildXXXLRuntimeGuardianSetAccount(
  policy: XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
): XXXLGuardianSetAccount {
  return {
    kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
    version: policy.guardian.version,
    guardianSetId: policy.guardian.guardianSetId,
    guardianPublicKeys: [...policy.guardian.guardianPublicKeys],
    quorumThreshold: policy.guardian.quorumThreshold,
    status: policy.guardian.status,
  };
}

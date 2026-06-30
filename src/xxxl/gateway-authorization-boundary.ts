import { bytesToHex } from "../gateway/stage-1-encoding.js";
import {
  XXXL_GUARDIAN_APPROVAL_ERROR,
  verifyXxxlGuardianPayloadQuorum,
  type XXXLGuardianApproval,
  type XXXLGuardianQuorumConfig,
  type XXXLGuardianQuorumVerificationResult,
} from "./guardian-approval-verifier.js";
import {
  type XXXLGuardianPayloadFields,
  type XXXLGuardianPayloadInteger,
} from "./guardian-payload-encoding.js";

export const XXXL_GATEWAY_AUTHORIZATION_ERROR = {
  InvalidGuardianQuorum: "INVALID_GUARDIAN_QUORUM",
  CanonicalEventAlreadyProcessed: "CANONICAL_EVENT_ALREADY_PROCESSED",
  PayloadExpired: "PAYLOAD_EXPIRED",
  RouteIdMismatch: "ROUTE_ID_MISMATCH",
  SourceChainIdMismatch: "SOURCE_CHAIN_ID_MISMATCH",
  TargetMintMismatch: "TARGET_MINT_MISMATCH",
  GuardianSetIdMismatch: "GUARDIAN_SET_ID_MISMATCH",
  SourceChainWeightMismatch: "SOURCE_CHAIN_WEIGHT_MISMATCH",
  InvalidPayload: "INVALID_PAYLOAD",
} as const;

export type XXXLGatewayAuthorizationErrorCode =
  (typeof XXXL_GATEWAY_AUTHORIZATION_ERROR)[keyof typeof XXXL_GATEWAY_AUTHORIZATION_ERROR];

export const XXXL_GATEWAY_AUTHORIZATION_STATUS = {
  Authorized: "AUTHORIZED",
  Rejected: "REJECTED",
} as const;

export type XXXLGatewayAuthorizationStatus =
  (typeof XXXL_GATEWAY_AUTHORIZATION_STATUS)[keyof typeof XXXL_GATEWAY_AUTHORIZATION_STATUS];

export type XXXLGatewayProcessedRegistrySnapshot = {
  readonly processedCanonicalEventKeyHexes: readonly string[];
};

export type XXXLGatewayRuntimeBindingExpectations = {
  readonly route_id: Uint8Array;
  readonly source_chain_id: XXXLGuardianPayloadInteger;
  readonly target_mint: Uint8Array;
  readonly guardian_set_id: Uint8Array;
  readonly source_chain_weight_bps?: number;
};

export type XXXLGatewayAuthorizationInput = {
  readonly fields: XXXLGuardianPayloadFields;
  readonly quorum: XXXLGuardianQuorumConfig;
  readonly approvals: readonly XXXLGuardianApproval[];
  readonly currentTimeOrSlot: XXXLGuardianPayloadInteger;
  readonly processedRegistry: XXXLGatewayProcessedRegistrySnapshot;
  readonly expectedBindings: XXXLGatewayRuntimeBindingExpectations;
};

export type XXXLGatewayReplayCheckResult = {
  readonly ok: boolean;
  readonly canonicalEventKeyHex: string;
  readonly alreadyProcessed: boolean;
  readonly errors: readonly XXXLGatewayAuthorizationErrorCode[];
};

export type XXXLGatewayExpirationCheckResult = {
  readonly ok: boolean;
  readonly currentTimeOrSlot: string;
  readonly expirationSlotOrUnixTs: string;
  readonly expired: boolean;
  readonly errors: readonly XXXLGatewayAuthorizationErrorCode[];
};

export type XXXLGatewayRouteBindingCheckResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLGatewayAuthorizationErrorCode[];
  readonly expected: {
    readonly routeIdHex: string;
    readonly sourceChainId: string;
    readonly targetMintHex: string;
    readonly guardianSetIdHex: string;
    readonly sourceChainWeightBps: number | null;
  };
  readonly actual: {
    readonly routeIdHex: string;
    readonly sourceChainId: string;
    readonly targetMintHex: string;
    readonly guardianSetIdHex: string;
    readonly sourceChainWeightBps: number;
  };
};

export type XXXLGatewayAuthorizationResult = {
  readonly ok: boolean;
  readonly payloadHash: string | null;
  readonly authorizationStatus: XXXLGatewayAuthorizationStatus;
  readonly errors: readonly XXXLGatewayAuthorizationErrorCode[];
  readonly quorum: XXXLGuardianQuorumVerificationResult;
  readonly replayCheck: XXXLGatewayReplayCheckResult;
  readonly expirationCheck: XXXLGatewayExpirationCheckResult;
  readonly routeBindingCheck: XXXLGatewayRouteBindingCheckResult;
};

function pushUnique<T>(items: T[], item: T): void {
  if (!items.includes(item)) {
    items.push(item);
  }
}

function bytesHex(bytes: Uint8Array): string {
  return bytesToHex(bytes).toLowerCase();
}

function integerAsBigInt(value: XXXLGuardianPayloadInteger): bigint {
  if (typeof value === "number") {
    if (!Number.isSafeInteger(value)) {
      throw new Error("integer value must be a safe integer when passed as number");
    }

    return BigInt(value);
  }

  return value;
}

function integerString(value: XXXLGuardianPayloadInteger): string {
  return integerAsBigInt(value).toString();
}

function checkReplay(
  fields: XXXLGuardianPayloadFields,
  processedRegistry: XXXLGatewayProcessedRegistrySnapshot,
): XXXLGatewayReplayCheckResult {
  const canonicalEventKeyHex = bytesHex(fields.canonical_event_key);
  const processedCanonicalEventKeys = new Set(
    processedRegistry.processedCanonicalEventKeyHexes.map((keyHex) =>
      keyHex.toLowerCase(),
    ),
  );
  const alreadyProcessed = processedCanonicalEventKeys.has(canonicalEventKeyHex);
  const errors = alreadyProcessed
    ? [XXXL_GATEWAY_AUTHORIZATION_ERROR.CanonicalEventAlreadyProcessed]
    : [];

  return {
    ok: errors.length === 0,
    canonicalEventKeyHex,
    alreadyProcessed,
    errors,
  };
}

function checkExpiration(input: {
  readonly fields: XXXLGuardianPayloadFields;
  readonly currentTimeOrSlot: XXXLGuardianPayloadInteger;
}): XXXLGatewayExpirationCheckResult {
  const currentTimeOrSlot = integerAsBigInt(input.currentTimeOrSlot);
  const expirationSlotOrUnixTs = integerAsBigInt(
    input.fields.expiration_slot_or_unix_ts,
  );
  const expired = currentTimeOrSlot > expirationSlotOrUnixTs;
  const errors = expired
    ? [XXXL_GATEWAY_AUTHORIZATION_ERROR.PayloadExpired]
    : [];

  return {
    ok: errors.length === 0,
    currentTimeOrSlot: currentTimeOrSlot.toString(),
    expirationSlotOrUnixTs: expirationSlotOrUnixTs.toString(),
    expired,
    errors,
  };
}

function checkRouteBindings(input: {
  readonly fields: XXXLGuardianPayloadFields;
  readonly expectedBindings: XXXLGatewayRuntimeBindingExpectations;
}): XXXLGatewayRouteBindingCheckResult {
  const errors: XXXLGatewayAuthorizationErrorCode[] = [];
  const actual = {
    routeIdHex: bytesHex(input.fields.route_id),
    sourceChainId: integerString(input.fields.source_chain_id),
    targetMintHex: bytesHex(input.fields.target_mint),
    guardianSetIdHex: bytesHex(input.fields.guardian_set_id),
    sourceChainWeightBps: input.fields.source_chain_weight_bps,
  };
  const expected = {
    routeIdHex: bytesHex(input.expectedBindings.route_id),
    sourceChainId: integerString(input.expectedBindings.source_chain_id),
    targetMintHex: bytesHex(input.expectedBindings.target_mint),
    guardianSetIdHex: bytesHex(input.expectedBindings.guardian_set_id),
    sourceChainWeightBps:
      input.expectedBindings.source_chain_weight_bps ?? null,
  };

  if (actual.routeIdHex !== expected.routeIdHex) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.RouteIdMismatch);
  }

  if (actual.sourceChainId !== expected.sourceChainId) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainIdMismatch);
  }

  if (actual.targetMintHex !== expected.targetMintHex) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.TargetMintMismatch);
  }

  if (actual.guardianSetIdHex !== expected.guardianSetIdHex) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.GuardianSetIdMismatch);
  }

  if (
    expected.sourceChainWeightBps !== null &&
    actual.sourceChainWeightBps !== expected.sourceChainWeightBps
  ) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainWeightMismatch);
  }

  return {
    ok: errors.length === 0,
    errors,
    expected,
    actual,
  };
}

export async function authorizeXxxlGatewayMintBoundary(
  input: XXXLGatewayAuthorizationInput,
): Promise<XXXLGatewayAuthorizationResult> {
  const errors: XXXLGatewayAuthorizationErrorCode[] = [];
  const quorum = await verifyXxxlGuardianPayloadQuorum({
    fields: input.fields,
    quorum: input.quorum,
    approvals: input.approvals,
  });
  const replayCheck = checkReplay(input.fields, input.processedRegistry);
  const expirationCheck = checkExpiration({
    fields: input.fields,
    currentTimeOrSlot: input.currentTimeOrSlot,
  });
  const routeBindingCheck = checkRouteBindings({
    fields: input.fields,
    expectedBindings: input.expectedBindings,
  });

  if (!quorum.ok) {
    errors.push(XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidGuardianQuorum);
  }

  if (quorum.errors.includes(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidPayload)) {
    pushUnique(errors, XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidPayload);
  }

  for (const error of replayCheck.errors) {
    pushUnique(errors, error);
  }

  for (const error of expirationCheck.errors) {
    pushUnique(errors, error);
  }

  for (const error of routeBindingCheck.errors) {
    pushUnique(errors, error);
  }

  return {
    ok: errors.length === 0,
    payloadHash: quorum.payloadHash,
    authorizationStatus:
      errors.length === 0
        ? XXXL_GATEWAY_AUTHORIZATION_STATUS.Authorized
        : XXXL_GATEWAY_AUTHORIZATION_STATUS.Rejected,
    errors,
    quorum,
    replayCheck,
    expirationCheck,
    routeBindingCheck,
  };
}

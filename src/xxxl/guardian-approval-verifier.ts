import * as ed25519 from "@noble/ed25519";

import { bytesToHex, hexToBytes } from "../gateway/stage-1-encoding.js";
import {
  hashXxxlGuardianPayload,
  type XXXLGuardianPayloadFields,
} from "./guardian-payload-encoding.js";

export const XXXL_GUARDIAN_APPROVAL_ERROR = {
  EmptyGuardianSet: "EMPTY_GUARDIAN_SET",
  InvalidThreshold: "INVALID_THRESHOLD",
  DuplicateGuardianPublicKey: "DUPLICATE_GUARDIAN_PUBLIC_KEY",
  InvalidGuardianPublicKeyLength: "INVALID_GUARDIAN_PUBLIC_KEY_LENGTH",
  InvalidGuardianSignatureLength: "INVALID_GUARDIAN_SIGNATURE_LENGTH",
  UnknownGuardian: "UNKNOWN_GUARDIAN",
  DuplicateGuardianApproval: "DUPLICATE_GUARDIAN_APPROVAL",
  InvalidPayload: "INVALID_PAYLOAD",
  InvalidSignature: "INVALID_SIGNATURE",
  QuorumNotReached: "QUORUM_NOT_REACHED",
} as const;

export type XXXLGuardianApprovalErrorCode =
  (typeof XXXL_GUARDIAN_APPROVAL_ERROR)[keyof typeof XXXL_GUARDIAN_APPROVAL_ERROR];

export type XXXLGuardianApproval = {
  readonly guardianPublicKey: Uint8Array;
  readonly guardianSignature: Uint8Array;
};

export type XXXLGuardianQuorumConfig = {
  readonly guardianPublicKeys: readonly Uint8Array[];
  readonly threshold: number;
};

export type XXXLGuardianPayloadApprovalVerificationInput =
  XXXLGuardianApproval & {
    readonly fields: XXXLGuardianPayloadFields;
  };

export type XXXLGuardianApprovalResult = {
  readonly guardianPublicKeyHex: string;
  readonly ok: boolean;
  readonly payloadHash: string | null;
  readonly errors: readonly XXXLGuardianApprovalErrorCode[];
};

export type XXXLGuardianQuorumVerificationInput = {
  readonly fields: XXXLGuardianPayloadFields;
  readonly quorum: XXXLGuardianQuorumConfig;
  readonly approvals: readonly XXXLGuardianApproval[];
};

export type XXXLGuardianQuorumVerificationResult = {
  readonly ok: boolean;
  readonly payloadHash: string | null;
  readonly threshold: number;
  readonly validApprovalCount: number;
  readonly acceptedGuardianPublicKeyHexes: readonly string[];
  readonly errors: readonly XXXLGuardianApprovalErrorCode[];
  readonly approvals: readonly XXXLGuardianApprovalResult[];
};

type PayloadHashResult = {
  readonly payloadHash: string | null;
  readonly payloadHashBytes: Uint8Array | null;
  readonly error: XXXLGuardianApprovalErrorCode | null;
};

function pushUnique<T>(items: T[], item: T): void {
  if (!items.includes(item)) {
    items.push(item);
  }
}

function guardianKeyHex(guardianPublicKey: Uint8Array): string {
  return bytesToHex(guardianPublicKey).toLowerCase();
}

function computePayloadHashBytes(
  fields: XXXLGuardianPayloadFields,
): PayloadHashResult {
  try {
    const payloadHash = hashXxxlGuardianPayload(fields);
    return {
      payloadHash,
      payloadHashBytes: hexToBytes(payloadHash, 32, "XXXL guardian payload hash"),
      error: null,
    };
  } catch {
    return {
      payloadHash: null,
      payloadHashBytes: null,
      error: XXXL_GUARDIAN_APPROVAL_ERROR.InvalidPayload,
    };
  }
}

function validateApprovalByteLengths(
  approval: XXXLGuardianApproval,
): XXXLGuardianApprovalErrorCode[] {
  const errors: XXXLGuardianApprovalErrorCode[] = [];

  if (approval.guardianPublicKey.length !== 32) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidGuardianPublicKeyLength);
  }

  if (approval.guardianSignature.length !== 64) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidGuardianSignatureLength);
  }

  return errors;
}

export function validateXxxlGuardianQuorumConfig(
  config: XXXLGuardianQuorumConfig,
): XXXLGuardianApprovalErrorCode[] {
  const errors: XXXLGuardianApprovalErrorCode[] = [];
  const guardianPublicKeyHexes = new Set<string>();
  let hasDuplicateGuardianPublicKey = false;

  if (config.guardianPublicKeys.length === 0) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.EmptyGuardianSet);
  }

  if (
    !Number.isInteger(config.threshold) ||
    config.threshold <= 0 ||
    config.threshold > config.guardianPublicKeys.length
  ) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidThreshold);
  }

  for (const guardianPublicKey of config.guardianPublicKeys) {
    if (guardianPublicKey.length !== 32) {
      pushUnique(
        errors,
        XXXL_GUARDIAN_APPROVAL_ERROR.InvalidGuardianPublicKeyLength,
      );
      continue;
    }

    const keyHex = guardianKeyHex(guardianPublicKey);

    if (guardianPublicKeyHexes.has(keyHex)) {
      hasDuplicateGuardianPublicKey = true;
    }

    guardianPublicKeyHexes.add(keyHex);
  }

  if (hasDuplicateGuardianPublicKey) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianPublicKey);
  }

  return errors;
}

async function verifyPayloadHashSignature(input: {
  readonly payloadHashBytes: Uint8Array;
  readonly guardianPublicKey: Uint8Array;
  readonly guardianSignature: Uint8Array;
}): Promise<boolean> {
  try {
    return await ed25519.verifyAsync(
      input.guardianSignature,
      input.payloadHashBytes,
      input.guardianPublicKey,
    );
  } catch {
    return false;
  }
}

export async function verifyXxxlGuardianPayloadApproval(
  input: XXXLGuardianPayloadApprovalVerificationInput,
): Promise<XXXLGuardianApprovalResult> {
  const payloadHash = computePayloadHashBytes(input.fields);
  const errors = validateApprovalByteLengths(input);

  if (payloadHash.error !== null) {
    errors.push(payloadHash.error);
  }

  if (errors.length === 0 && payloadHash.payloadHashBytes !== null) {
    const signatureVerified = await verifyPayloadHashSignature({
      payloadHashBytes: payloadHash.payloadHashBytes,
      guardianPublicKey: input.guardianPublicKey,
      guardianSignature: input.guardianSignature,
    });

    if (!signatureVerified) {
      errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature);
    }
  }

  return {
    guardianPublicKeyHex: guardianKeyHex(input.guardianPublicKey),
    ok: errors.length === 0,
    payloadHash: payloadHash.payloadHash,
    errors,
  };
}

export async function verifyXxxlGuardianPayloadQuorum(
  input: XXXLGuardianQuorumVerificationInput,
): Promise<XXXLGuardianQuorumVerificationResult> {
  const errors = validateXxxlGuardianQuorumConfig(input.quorum);
  const payloadHash = computePayloadHashBytes(input.fields);
  const allowedGuardianKeys = new Set(
    input.quorum.guardianPublicKeys
      .filter((guardianPublicKey) => guardianPublicKey.length === 32)
      .map((guardianPublicKey) => guardianKeyHex(guardianPublicKey)),
  );
  const seenKnownApprovalKeys = new Set<string>();
  const acceptedGuardianKeys = new Set<string>();
  const approvalResults: XXXLGuardianApprovalResult[] = [];

  if (payloadHash.error !== null) {
    errors.push(payloadHash.error);
  }

  for (const approval of input.approvals) {
    const guardianPublicKeyHex = guardianKeyHex(approval.guardianPublicKey);
    const approvalErrors = validateApprovalByteLengths(approval);
    const hasValidGuardianPublicKeyLength =
      approval.guardianPublicKey.length === 32;

    if (hasValidGuardianPublicKeyLength) {
      if (!allowedGuardianKeys.has(guardianPublicKeyHex)) {
        approvalErrors.push(XXXL_GUARDIAN_APPROVAL_ERROR.UnknownGuardian);
      } else if (seenKnownApprovalKeys.has(guardianPublicKeyHex)) {
        approvalErrors.push(
          XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval,
        );
      } else {
        seenKnownApprovalKeys.add(guardianPublicKeyHex);
      }
    }

    if (payloadHash.error !== null) {
      approvalErrors.push(payloadHash.error);
    }

    if (approvalErrors.length === 0 && payloadHash.payloadHashBytes !== null) {
      const signatureVerified = await verifyPayloadHashSignature({
        payloadHashBytes: payloadHash.payloadHashBytes,
        guardianPublicKey: approval.guardianPublicKey,
        guardianSignature: approval.guardianSignature,
      });

      if (!signatureVerified) {
        approvalErrors.push(XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature);
      }
    }

    const approvalOk = approvalErrors.length === 0;

    if (approvalOk) {
      acceptedGuardianKeys.add(guardianPublicKeyHex);
    }

    approvalResults.push({
      guardianPublicKeyHex,
      ok: approvalOk,
      payloadHash: payloadHash.payloadHash,
      errors: approvalErrors,
    });
  }

  if (acceptedGuardianKeys.size < input.quorum.threshold) {
    errors.push(XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached);
  }

  return {
    ok: errors.length === 0,
    payloadHash: payloadHash.payloadHash,
    threshold: input.quorum.threshold,
    validApprovalCount: acceptedGuardianKeys.size,
    acceptedGuardianPublicKeyHexes: [...acceptedGuardianKeys],
    errors,
    approvals: approvalResults,
  };
}

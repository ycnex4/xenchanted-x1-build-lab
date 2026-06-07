import { bytesToHex } from "./stage-1-encoding.js";
import {
  verifyStage1GatewayApproval,
  type Stage1GatewayApprovalVerificationInput,
  type Stage1GatewayApprovalVerificationResult,
} from "./stage-1-approval-verifier.js";

export const STAGE1_GUARDIAN_QUORUM_ERROR = {
  EmptyGuardianSet: "EMPTY_GUARDIAN_SET",
  InvalidThreshold: "INVALID_THRESHOLD",
  UnknownGuardian: "UNKNOWN_GUARDIAN",
  DuplicateGuardianApproval: "DUPLICATE_GUARDIAN_APPROVAL",
  InvalidApproval: "INVALID_APPROVAL",
  QuorumNotReached: "QUORUM_NOT_REACHED",
} as const;

export type Stage1GuardianQuorumErrorCode =
  (typeof STAGE1_GUARDIAN_QUORUM_ERROR)[keyof typeof STAGE1_GUARDIAN_QUORUM_ERROR];

export type Stage1GuardianQuorumConfig = {
  guardianPublicKeys: Uint8Array[];
  threshold: number;
};

export type Stage1GuardianApproval = {
  guardianPublicKey: Uint8Array;
  guardianSignature: Uint8Array;
};

export type Stage1GuardianApprovalResult = {
  guardianPublicKeyHex: string;
  ok: boolean;
  errors: Stage1GuardianQuorumErrorCode[];
  approval?: Stage1GatewayApprovalVerificationResult;
};

export type Stage1GuardianQuorumVerificationInput = Omit<
  Stage1GatewayApprovalVerificationInput,
  "guardianPublicKey" | "guardianSignature"
> & {
  quorum: Stage1GuardianQuorumConfig;
  approvals: Stage1GuardianApproval[];
};

export type Stage1GuardianQuorumVerificationResult = {
  ok: boolean;
  threshold: number;
  validApprovalCount: number;
  acceptedGuardianPublicKeyHexes: string[];
  errors: Stage1GuardianQuorumErrorCode[];
  approvals: Stage1GuardianApprovalResult[];
};

function guardianKeyHex(guardianPublicKey: Uint8Array): string {
  return bytesToHex(guardianPublicKey).toLowerCase();
}

export function validateStage1GuardianQuorumConfig(
  config: Stage1GuardianQuorumConfig,
): Stage1GuardianQuorumErrorCode[] {
  const errors: Stage1GuardianQuorumErrorCode[] = [];

  if (config.guardianPublicKeys.length === 0) {
    errors.push(STAGE1_GUARDIAN_QUORUM_ERROR.EmptyGuardianSet);
  }

  if (
    !Number.isInteger(config.threshold) ||
    config.threshold <= 0 ||
    config.threshold > config.guardianPublicKeys.length
  ) {
    errors.push(STAGE1_GUARDIAN_QUORUM_ERROR.InvalidThreshold);
  }

  return errors;
}

export async function verifyStage1GuardianQuorum(
  input: Stage1GuardianQuorumVerificationInput,
): Promise<Stage1GuardianQuorumVerificationResult> {
  const errors = validateStage1GuardianQuorumConfig(input.quorum);
  const allowedGuardianKeys = new Set(
    input.quorum.guardianPublicKeys.map((guardianPublicKey) =>
      guardianKeyHex(guardianPublicKey),
    ),
  );
  const acceptedGuardianKeys = new Set<string>();
  const approvalResults: Stage1GuardianApprovalResult[] = [];

  for (const approval of input.approvals) {
    const guardianPublicKeyHex = guardianKeyHex(approval.guardianPublicKey);
    const approvalErrors: Stage1GuardianQuorumErrorCode[] = [];

    if (!allowedGuardianKeys.has(guardianPublicKeyHex)) {
      approvalErrors.push(STAGE1_GUARDIAN_QUORUM_ERROR.UnknownGuardian);
    }

    if (acceptedGuardianKeys.has(guardianPublicKeyHex)) {
      approvalErrors.push(STAGE1_GUARDIAN_QUORUM_ERROR.DuplicateGuardianApproval);
    }

    const approvalVerification = await verifyStage1GatewayApproval({
      fields: input.fields,
      x1RecipientBytes: input.x1RecipientBytes,
      domainSeparator: input.domainSeparator,
      messageHash: input.messageHash,
      routeConfig: input.routeConfig,
      guardianPublicKey: approval.guardianPublicKey,
      guardianSignature: approval.guardianSignature,
    });

    if (!approvalVerification.ok) {
      approvalErrors.push(STAGE1_GUARDIAN_QUORUM_ERROR.InvalidApproval);
    }

    const approvalOk = approvalErrors.length === 0;

    if (approvalOk) {
      acceptedGuardianKeys.add(guardianPublicKeyHex);
    }

    approvalResults.push({
      guardianPublicKeyHex,
      ok: approvalOk,
      errors: approvalErrors,
      approval: approvalVerification,
    });
  }

  if (acceptedGuardianKeys.size < input.quorum.threshold) {
    errors.push(STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached);
  }

  return {
    ok: errors.length === 0,
    threshold: input.quorum.threshold,
    validApprovalCount: acceptedGuardianKeys.size,
    acceptedGuardianPublicKeyHexes: [...acceptedGuardianKeys],
    errors,
    approvals: approvalResults,
  };
}

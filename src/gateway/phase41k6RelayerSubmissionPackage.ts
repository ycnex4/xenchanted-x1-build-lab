import {
  buildPhase41K6GatewayMintPayloadV2,
  normalizeBytes32Hex,
  type Bytes32Hex,
  type Phase41K6GatewayMintCandidate,
  type Phase41K6PayloadV2BuildResult,
} from "./phase41k6PayloadV2.js";
import {
  buildPhase41K6QuorumPackage,
  type Phase41K6PriorEd25519EvidenceInput,
  type Phase41K6QuorumPackage,
} from "./phase41k6QuorumPackage.js";

export const PHASE_41K6_RELAYER_SUBMISSION_PACKAGE_KIND =
  "phase41k6_relayer_submission_package";

export interface Phase41K6RelayerQuorumPackageInput {
  guardianSetId: Bytes32Hex;
  threshold: number;
  guardianSetPublicKeys: Bytes32Hex[];
  evidenceFormat: "prior_ed25519_instruction";
  evidence: Phase41K6PriorEd25519EvidenceInput[];
}

export interface Phase41K6RelayerSubmissionPackageInput {
  eventId: string;
  journalId: string;
  candidate: Phase41K6GatewayMintCandidate;
  quorum: Phase41K6RelayerQuorumPackageInput;
}

export interface Phase41K6RelayerSubmissionPackage {
  schemaVersion: 1;
  kind: typeof PHASE_41K6_RELAYER_SUBMISSION_PACKAGE_KIND;
  eventId: string;
  journalId: string;
  payload: Phase41K6PayloadV2BuildResult;
  quorum: Phase41K6QuorumPackage;
  handlerInstructionBoundary: {
    processedEvent: Bytes32Hex;
    routeId: Bytes32Hex;
    mint: Bytes32Hex;
    recipientTokenAccount: Bytes32Hex;
    amount: bigint;
    guardianSetId: Bytes32Hex;
    priorEvidenceInstructionCount: number;
  };
  noSendBoundary: {
    noLiveRpc: true;
    noSigning: true;
    noSubmit: true;
    noSolSpend: true;
    noPrivateKeys: true;
  };
  relayerPackageReady: true;
}

export function buildPhase41K6RelayerSubmissionPackage(
  input: Phase41K6RelayerSubmissionPackageInput,
): Phase41K6RelayerSubmissionPackage {
  const eventId = normalizeNonEmptyString(input.eventId, "eventId");
  const journalId = normalizeNonEmptyString(input.journalId, "journalId");

  const payload = buildPhase41K6GatewayMintPayloadV2(input.candidate);

  const candidateGuardianSetId = normalizeBytes32Hex(
    input.candidate.handlerBinding.guardianSetId,
    "candidateGuardianSetId",
  );
  const quorumGuardianSetId = normalizeBytes32Hex(
    input.quorum.guardianSetId,
    "quorumGuardianSetId",
  );

  if (candidateGuardianSetId !== quorumGuardianSetId) {
    throw new Error("guardian_set_id_package_candidate_mismatch");
  }

  const quorum = buildPhase41K6QuorumPackage({
    payloadV2Hash: payload.payloadV2Hash,
    guardianSetId: quorumGuardianSetId,
    threshold: input.quorum.threshold,
    guardianSetPublicKeys: input.quorum.guardianSetPublicKeys,
    evidenceFormat: input.quorum.evidenceFormat,
    evidence: input.quorum.evidence,
  });

  return {
    schemaVersion: 1,
    kind: PHASE_41K6_RELAYER_SUBMISSION_PACKAGE_KIND,
    eventId,
    journalId,
    payload,
    quorum,
    handlerInstructionBoundary: {
      processedEvent: payload.processedEvent,
      routeId: payload.routeId,
      mint: payload.mint,
      recipientTokenAccount: payload.recipientTokenAccount,
      amount: payload.amount,
      guardianSetId: payload.guardianSetId,
      priorEvidenceInstructionCount: quorum.evidence.length,
    },
    noSendBoundary: {
      noLiveRpc: true,
      noSigning: true,
      noSubmit: true,
      noSolSpend: true,
      noPrivateKeys: true,
    },
    relayerPackageReady: true,
  };
}

function normalizeNonEmptyString(value: string, fieldName: string): string {
  const trimmed = value.trim();

  if (trimmed.length === 0) {
    throw new Error(`invalid_${fieldName}`);
  }

  return trimmed;
}

import {
  normalizeBytes32Hex,
  type Bytes32Hex,
} from "./phase41k6PayloadV2.js";

export const PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT =
  "prior_ed25519_instruction";

const SIGNATURE64_HEX_RE = /^0x[0-9a-fA-F]{128}$/;

export type Signature64Hex = `0x${string}`;

export interface Phase41K6PriorEd25519EvidenceInput {
  sourceInstructionIndex: number;
  guardianPublicKey: Bytes32Hex;
  signature: Signature64Hex;
  signedMessage: Bytes32Hex;
}

export interface Phase41K6QuorumPackageInput {
  payloadV2Hash: Bytes32Hex;
  guardianSetId: Bytes32Hex;
  threshold: number;
  guardianSetPublicKeys: Bytes32Hex[];
  evidenceFormat: typeof PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT;
  evidence: Phase41K6PriorEd25519EvidenceInput[];
}

export interface Phase41K6PriorEd25519EvidenceEntry {
  sourceInstructionIndex: number;
  guardianPublicKey: Bytes32Hex;
  signature: Signature64Hex;
  signedMessage: Bytes32Hex;
  bindsPayloadHash: true;
  knownGuardian: true;
  uniqueGuardian: true;
}

export interface Phase41K6QuorumPackage {
  payloadV2Hash: Bytes32Hex;
  guardianSetId: Bytes32Hex;
  threshold: number;
  guardianSetPublicKeys: Bytes32Hex[];
  evidenceFormat: typeof PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT;
  evidence: Phase41K6PriorEd25519EvidenceEntry[];
  uniqueGuardianCount: number;
  quorumMet: true;
  relayerMaySubmitToHandler: true;
}

export function buildPhase41K6QuorumPackage(
  input: Phase41K6QuorumPackageInput,
): Phase41K6QuorumPackage {
  if (input.evidenceFormat !== PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT) {
    throw new Error("invalid_evidence_format");
  }

  const payloadV2Hash = normalizeBytes32Hex(input.payloadV2Hash, "payloadV2Hash");
  const guardianSetId = normalizeBytes32Hex(input.guardianSetId, "guardianSetId");
  const guardianSetPublicKeys = normalizeGuardianSet(input.guardianSetPublicKeys);

  if (
    !Number.isInteger(input.threshold) ||
    input.threshold <= 0 ||
    input.threshold > guardianSetPublicKeys.length
  ) {
    throw new Error("invalid_threshold");
  }

  const knownGuardians = new Set<string>(guardianSetPublicKeys);
  const seenGuardians = new Set<string>();
  const evidence: Phase41K6PriorEd25519EvidenceEntry[] = [];

  for (const rawEntry of input.evidence) {
    const sourceInstructionIndex = normalizeSourceInstructionIndex(
      rawEntry.sourceInstructionIndex,
    );
    const guardianPublicKey = normalizeBytes32Hex(
      rawEntry.guardianPublicKey,
      "guardianPublicKey",
    );

    if (!knownGuardians.has(guardianPublicKey)) {
      throw new Error("unknown_guardian_public_key");
    }

    if (seenGuardians.has(guardianPublicKey)) {
      throw new Error("duplicate_guardian_evidence");
    }

    seenGuardians.add(guardianPublicKey);

    const signedMessage = normalizeBytes32Hex(rawEntry.signedMessage, "signedMessage");

    if (signedMessage !== payloadV2Hash) {
      throw new Error("signed_message_payload_hash_mismatch");
    }

    evidence.push({
      sourceInstructionIndex,
      guardianPublicKey,
      signature: normalizeSignature64Hex(rawEntry.signature, "signature"),
      signedMessage,
      bindsPayloadHash: true,
      knownGuardian: true,
      uniqueGuardian: true,
    });
  }

  if (evidence.length < input.threshold) {
    throw new Error("insufficient_quorum");
  }

  return {
    payloadV2Hash,
    guardianSetId,
    threshold: input.threshold,
    guardianSetPublicKeys,
    evidenceFormat: PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT,
    evidence,
    uniqueGuardianCount: evidence.length,
    quorumMet: true,
    relayerMaySubmitToHandler: true,
  };
}

export function normalizeSignature64Hex(
  value: Signature64Hex,
  fieldName: string,
): Signature64Hex {
  if (!SIGNATURE64_HEX_RE.test(value)) {
    throw new Error(`invalid_${fieldName}_signature64_hex`);
  }

  return `0x${value.slice(2).toLowerCase()}`;
}

function normalizeGuardianSet(values: Bytes32Hex[]): Bytes32Hex[] {
  if (values.length === 0 || values.length > 8) {
    throw new Error("invalid_guardian_set_size");
  }

  const normalized = values.map((value) =>
    normalizeBytes32Hex(value, "guardianSetPublicKey"),
  );

  if (new Set(normalized).size !== normalized.length) {
    throw new Error("duplicate_guardian_set_public_key");
  }

  return normalized;
}

function normalizeSourceInstructionIndex(value: number): number {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error("invalid_source_instruction_index");
  }

  return value;
}

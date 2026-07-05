import { describe, expect, it } from "vitest";

import {
  buildPhase41K6GatewayMintPayloadV2,
  repeatByte32Hex,
} from "../src/gateway/phase41k6PayloadV2.js";
import {
  PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT,
  buildPhase41K6QuorumPackage,
  type Phase41K6QuorumPackageInput,
  type Signature64Hex,
} from "../src/gateway/phase41k6QuorumPackage.js";

function signature64Hex(byte: number): Signature64Hex {
  return `0x${byte.toString(16).padStart(2, "0").repeat(64)}`;
}

function payloadHash() {
  return buildPhase41K6GatewayMintPayloadV2({
    sourceObservation: {
      sourceChainId: 1n,
      sourceToken: repeatByte32Hex(0x10),
      sourceSender: repeatByte32Hex(0x11),
      sourceBurnTxHash: repeatByte32Hex(0x12),
      sourceBurnEventIndex: 7n,
      sourceBlockNumber: 123456n,
      sourceBlockHash: repeatByte32Hex(0x13),
      sourceFinalityState: "finalized",
      burnedAmount: 1000n,
      canonicalEventKey: repeatByte32Hex(0x44),
    },
    handlerBinding: {
      processedEvent: repeatByte32Hex(0x01),
      routeId: repeatByte32Hex(0x04),
      mint: repeatByte32Hex(0x02),
      recipientTokenAccount: repeatByte32Hex(0x03),
      amount: 123n,
      guardianSetId: repeatByte32Hex(0x07),
    },
  }).payloadV2Hash;
}

function validPackageInput(): Phase41K6QuorumPackageInput {
  const hash = payloadHash();

  return {
    payloadV2Hash: hash,
    guardianSetId: repeatByte32Hex(0x07),
    threshold: 2,
    guardianSetPublicKeys: [
      repeatByte32Hex(0xa1),
      repeatByte32Hex(0xa2),
      repeatByte32Hex(0xa3),
    ],
    evidenceFormat: PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT,
    evidence: [
      {
        sourceInstructionIndex: 0,
        guardianPublicKey: repeatByte32Hex(0xa1),
        signature: signature64Hex(0x55),
        signedMessage: hash,
      },
      {
        sourceInstructionIndex: 1,
        guardianPublicKey: repeatByte32Hex(0xa2),
        signature: signature64Hex(0x56),
        signedMessage: hash,
      },
    ],
  };
}

describe("Phase 41K.6 B5 quorum package boundary", () => {
  it("accepts a valid unique guardian quorum bound to the payload v2 hash", () => {
    const result = buildPhase41K6QuorumPackage(validPackageInput());

    expect(result.payloadV2Hash).toBe(payloadHash());
    expect(result.guardianSetId).toBe(repeatByte32Hex(0x07));
    expect(result.threshold).toBe(2);
    expect(result.uniqueGuardianCount).toBe(2);
    expect(result.quorumMet).toBe(true);
    expect(result.relayerMaySubmitToHandler).toBe(true);
    expect(result.evidence.map((entry) => entry.guardianPublicKey)).toEqual([
      repeatByte32Hex(0xa1),
      repeatByte32Hex(0xa2),
    ]);
    expect(result.evidence.every((entry) => entry.bindsPayloadHash)).toBe(true);
  });

  it("rejects duplicate guardian evidence before relayer submission", () => {
    const input = validPackageInput();

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [
          input.evidence[0]!,
          {
            ...input.evidence[1]!,
            guardianPublicKey: input.evidence[0]!.guardianPublicKey,
          },
        ],
      }),
    ).toThrow("duplicate_guardian_evidence");
  });

  it("rejects unknown guardian evidence before relayer submission", () => {
    const input = validPackageInput();

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [
          input.evidence[0]!,
          {
            ...input.evidence[1]!,
            guardianPublicKey: repeatByte32Hex(0xee),
          },
        ],
      }),
    ).toThrow("unknown_guardian_public_key");
  });

  it("rejects insufficient quorum before relayer submission", () => {
    const input = validPackageInput();

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [input.evidence[0]!],
      }),
    ).toThrow("insufficient_quorum");
  });

  it("rejects signed message drift before relayer submission", () => {
    const input = validPackageInput();

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [
          input.evidence[0]!,
          {
            ...input.evidence[1]!,
            signedMessage: repeatByte32Hex(0x99),
          },
        ],
      }),
    ).toThrow("signed_message_payload_hash_mismatch");
  });

  it("rejects malformed package fields before relayer submission", () => {
    const input = validPackageInput();

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        threshold: 0,
      }),
    ).toThrow("invalid_threshold");

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [
          {
            ...input.evidence[0]!,
            sourceInstructionIndex: -1,
          },
          input.evidence[1]!,
        ],
      }),
    ).toThrow("invalid_source_instruction_index");

    expect(() =>
      buildPhase41K6QuorumPackage({
        ...input,
        evidence: [
          {
            ...input.evidence[0]!,
            signature: "0x1234",
          },
          input.evidence[1]!,
        ],
      }),
    ).toThrow("invalid_signature_signature64_hex");
  });
});

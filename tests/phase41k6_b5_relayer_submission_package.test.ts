import { describe, expect, it } from "vitest";

import {
  buildPhase41K6GatewayMintPayloadV2,
  repeatByte32Hex,
  type Phase41K6GatewayMintCandidate,
} from "../src/gateway/phase41k6PayloadV2.js";
import { PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT } from "../src/gateway/phase41k6QuorumPackage.js";
import {
  PHASE_41K6_RELAYER_SUBMISSION_PACKAGE_KIND,
  buildPhase41K6RelayerSubmissionPackage,
  type Phase41K6RelayerSubmissionPackageInput,
} from "../src/gateway/phase41k6RelayerSubmissionPackage.js";

function signature64Hex(byte: number): `0x${string}` {
  return `0x${byte.toString(16).padStart(2, "0").repeat(64)}`;
}

function candidate(): Phase41K6GatewayMintCandidate {
  return {
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
  };
}

function validInput(candidateOverride = candidate()): Phase41K6RelayerSubmissionPackageInput {
  const payloadHash = buildPhase41K6GatewayMintPayloadV2(candidateOverride).payloadV2Hash;

  return {
    eventId: "eth-mainnet:0xabc:7",
    journalId: "journal:eth-mainnet:0xabc:7",
    candidate: candidateOverride,
    quorum: {
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
          signedMessage: payloadHash,
        },
        {
          sourceInstructionIndex: 1,
          guardianPublicKey: repeatByte32Hex(0xa2),
          signature: signature64Hex(0x56),
          signedMessage: payloadHash,
        },
      ],
    },
  };
}

describe("Phase 41K.6 B5 relayer submission package boundary", () => {
  it("assembles a no-send relayer package from candidate, payload hash, and quorum package", () => {
    const result = buildPhase41K6RelayerSubmissionPackage(validInput());

    expect(result.schemaVersion).toBe(1);
    expect(result.kind).toBe(PHASE_41K6_RELAYER_SUBMISSION_PACKAGE_KIND);
    expect(result.eventId).toBe("eth-mainnet:0xabc:7");
    expect(result.journalId).toBe("journal:eth-mainnet:0xabc:7");
    expect(result.relayerPackageReady).toBe(true);

    expect(result.handlerInstructionBoundary).toEqual({
      processedEvent: repeatByte32Hex(0x01),
      routeId: repeatByte32Hex(0x04),
      mint: repeatByte32Hex(0x02),
      recipientTokenAccount: repeatByte32Hex(0x03),
      amount: 123n,
      guardianSetId: repeatByte32Hex(0x07),
      priorEvidenceInstructionCount: 2,
    });

    expect(result.quorum.quorumMet).toBe(true);
    expect(result.noSendBoundary).toEqual({
      noLiveRpc: true,
      noSigning: true,
      noSubmit: true,
      noSolSpend: true,
      noPrivateKeys: true,
    });
  });

  it("rejects empty operational identifiers before package creation", () => {
    expect(() =>
      buildPhase41K6RelayerSubmissionPackage({
        ...validInput(),
        eventId: " ",
      }),
    ).toThrow("invalid_eventId");

    expect(() =>
      buildPhase41K6RelayerSubmissionPackage({
        ...validInput(),
        journalId: "",
      }),
    ).toThrow("invalid_journalId");
  });

  it("rejects guardian_set_id drift between candidate and quorum package", () => {
    expect(() =>
      buildPhase41K6RelayerSubmissionPackage({
        ...validInput(),
        quorum: {
          ...validInput().quorum,
          guardianSetId: repeatByte32Hex(0x99),
        },
      }),
    ).toThrow("guardian_set_id_package_candidate_mismatch");
  });

  it("rejects stale signatures when a handler-bound candidate field changes after signing", () => {
    const originalInput = validInput();
    const changedCandidate: Phase41K6GatewayMintCandidate = {
      ...originalInput.candidate,
      handlerBinding: {
        ...originalInput.candidate.handlerBinding,
        amount: 124n,
      },
    };

    expect(() =>
      buildPhase41K6RelayerSubmissionPackage({
        ...originalInput,
        candidate: changedCandidate,
      }),
    ).toThrow("signed_message_payload_hash_mismatch");
  });

  it("keeps the payload hash stable when only relayer operational identifiers change", () => {
    const first = buildPhase41K6RelayerSubmissionPackage(validInput());
    const second = buildPhase41K6RelayerSubmissionPackage({
      ...validInput(),
      eventId: "different-event-id",
      journalId: "different-journal-id",
    });

    expect(second.payload.payloadV2Hash).toBe(first.payload.payloadV2Hash);
    expect(second.handlerInstructionBoundary).toEqual(first.handlerInstructionBoundary);
  });
});

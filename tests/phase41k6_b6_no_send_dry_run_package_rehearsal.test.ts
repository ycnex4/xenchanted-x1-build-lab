import { describe, expect, it } from "vitest";

import {
  buildPhase41K6GatewayMintPayloadV2,
  repeatByte32Hex,
  type Phase41K6GatewayMintCandidate,
} from "../src/gateway/phase41k6PayloadV2.js";
import { PHASE_41K6_PRIOR_ED25519_EVIDENCE_FORMAT } from "../src/gateway/phase41k6QuorumPackage.js";
import {
  buildPhase41K6RelayerSubmissionPackage,
  type Phase41K6RelayerSubmissionPackageInput,
} from "../src/gateway/phase41k6RelayerSubmissionPackage.js";

function signature64Hex(byte: number): `0x${string}` {
  return `0x${byte.toString(16).padStart(2, "0").repeat(64)}`;
}

function b6TestnetReadinessCandidate(): Phase41K6GatewayMintCandidate {
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
      burnedAmount: 1_234_567_890n,
      canonicalEventKey: repeatByte32Hex(0xb2),
    },
    handlerBinding: {
      processedEvent: repeatByte32Hex(0xb2),
      routeId: repeatByte32Hex(0x41),
      mint: repeatByte32Hex(0x51),
      recipientTokenAccount: repeatByte32Hex(0x61),
      amount: 1_234_567_890n,
      guardianSetId: repeatByte32Hex(0xc7),
    },
  };
}

function b6NoSendDryRunInput(): Phase41K6RelayerSubmissionPackageInput {
  const candidate = b6TestnetReadinessCandidate();
  const payloadV2Hash = buildPhase41K6GatewayMintPayloadV2(candidate).payloadV2Hash;

  return {
    eventId: "b6.3:x1-testnet:no-send:canonical-event-b2",
    journalId: "b6.3:dry-run-journal:canonical-event-b2",
    candidate,
    quorum: {
      guardianSetId: repeatByte32Hex(0xc7),
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
          signedMessage: payloadV2Hash,
        },
        {
          sourceInstructionIndex: 1,
          guardianPublicKey: repeatByte32Hex(0xa2),
          signature: signature64Hex(0x56),
          signedMessage: payloadV2Hash,
        },
      ],
    },
  };
}

describe("Phase 41K.6 B6.3 no-send dry-run package rehearsal", () => {
  it("assembles the B5 relayer package shape without RPC, signing, submit, SOL, or private keys", () => {
    const result = buildPhase41K6RelayerSubmissionPackage(b6NoSendDryRunInput());

    expect(result.schemaVersion).toBe(1);
    expect(result.eventId).toBe("b6.3:x1-testnet:no-send:canonical-event-b2");
    expect(result.journalId).toBe("b6.3:dry-run-journal:canonical-event-b2");

    expect(result.payload.payloadV2Hash).toBe(
      "0x56a318440e188d864052b8518f41deb7e4f998a975e3b6e19ca63815535ec77d",
    );

    expect(result.handlerInstructionBoundary).toEqual({
      processedEvent: repeatByte32Hex(0xb2),
      routeId: repeatByte32Hex(0x41),
      mint: repeatByte32Hex(0x51),
      recipientTokenAccount: repeatByte32Hex(0x61),
      amount: 1_234_567_890n,
      guardianSetId: repeatByte32Hex(0xc7),
      priorEvidenceInstructionCount: 2,
    });

    expect(result.quorum.quorumMet).toBe(true);
    expect(result.quorum.uniqueGuardianCount).toBe(2);
    expect(result.relayerPackageReady).toBe(true);

    expect(result.noSendBoundary).toEqual({
      noLiveRpc: true,
      noSigning: true,
      noSubmit: true,
      noSolSpend: true,
      noPrivateKeys: true,
    });
  });

  it("rejects dry-run package rehearsal if a handler-bound field drifts after evidence preparation", () => {
    const input = b6NoSendDryRunInput();

    expect(() =>
      buildPhase41K6RelayerSubmissionPackage({
        ...input,
        candidate: {
          ...input.candidate,
          handlerBinding: {
            ...input.candidate.handlerBinding,
            amount: 1_234_567_891n,
          },
        },
      }),
    ).toThrow("signed_message_payload_hash_mismatch");
  });

  it("keeps dry-run operational identifiers outside payload hash binding", () => {
    const first = buildPhase41K6RelayerSubmissionPackage(b6NoSendDryRunInput());
    const second = buildPhase41K6RelayerSubmissionPackage({
      ...b6NoSendDryRunInput(),
      eventId: "b6.3:x1-testnet:no-send:different-event-id",
      journalId: "b6.3:dry-run-journal:different-journal-id",
    });

    expect(second.payload.payloadV2Hash).toBe(first.payload.payloadV2Hash);
    expect(second.handlerInstructionBoundary).toEqual(first.handlerInstructionBoundary);
  });
});

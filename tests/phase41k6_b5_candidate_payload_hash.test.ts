import { describe, expect, it } from "vitest";

import {
  buildPhase41K6GatewayMintPayloadV2,
  repeatByte32Hex,
  type Phase41K6GatewayMintCandidate,
} from "../src/gateway/phase41k6PayloadV2.js";

const PHASE_41K6_B2_KNOWN_ANSWER_PAYLOAD_HASH =
  "0x56a318440e188d864052b8518f41deb7e4f998a975e3b6e19ca63815535ec77d";

const PHASE_41K6_B2_U64_MAX_KNOWN_ANSWER_PAYLOAD_HASH =
  "0xa6b9e3901a04a6da11d100912cb1f5ebf294464d5b11376f2b7eb71a0cb9f893";

function knownAnswerCandidate(amount = 1_234_567_890n): Phase41K6GatewayMintCandidate {
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
      burnedAmount: amount,
      canonicalEventKey: repeatByte32Hex(0xb2),
    },
    handlerBinding: {
      processedEvent: repeatByte32Hex(0xb2),
      routeId: repeatByte32Hex(0x41),
      mint: repeatByte32Hex(0x51),
      recipientTokenAccount: repeatByte32Hex(0x61),
      amount,
      guardianSetId: repeatByte32Hex(0xc7),
    },
  };
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

describe("Phase 41K.6 B5 candidate payload v2 hash builder", () => {
  it("matches the Rust B1C payload hash known-answer vector", () => {
    const result = buildPhase41K6GatewayMintPayloadV2(knownAnswerCandidate());

    expect(result.domain).toBe("consume_gateway_mint_authorization_v2");
    expect(result.processedEvent).toBe(repeatByte32Hex(0xb2));
    expect(result.routeId).toBe(repeatByte32Hex(0x41));
    expect(result.mint).toBe(repeatByte32Hex(0x51));
    expect(result.recipientTokenAccount).toBe(repeatByte32Hex(0x61));
    expect(result.amount).toBe(1_234_567_890n);
    expect(result.amountLeHex).toBe(
      "0xd202964900000000000000000000000000000000000000000000000000000000",
    );
    expect(result.guardianSetId).toBe(repeatByte32Hex(0xc7));
    expect(result.payloadV2Hash).toBe(PHASE_41K6_B2_KNOWN_ANSWER_PAYLOAD_HASH);
  });

  it("matches the Rust B1C u64 max known-answer vector", () => {
    const result = buildPhase41K6GatewayMintPayloadV2(
      knownAnswerCandidate(0xffff_ffff_ffff_ffffn),
    );

    expect(result.amount).toBe(0xffff_ffff_ffff_ffffn);
    expect(result.amountLeHex).toBe(
      "0xffffffffffffffff000000000000000000000000000000000000000000000000",
    );
    expect(result.payloadV2Hash).toBe(
      PHASE_41K6_B2_U64_MAX_KNOWN_ANSWER_PAYLOAD_HASH,
    );
  });

  it("builds a deterministic sha256 payload v2 hash from handler-bound fields", () => {
    const first = buildPhase41K6GatewayMintPayloadV2(candidate());
    const second = buildPhase41K6GatewayMintPayloadV2(candidate());

    expect(first.domain).toBe("consume_gateway_mint_authorization_v2");
    expect(first.hashAlgorithm).toBe("sha256");
    expect(first.amountLeHex).toBe(
      "0x7b00000000000000000000000000000000000000000000000000000000000000",
    );
    expect(first.payloadV2Hash).toMatch(/^0x[0-9a-f]{64}$/);
    expect(first).toEqual(second);
  });

  it("changes the payload hash when any handler-bound field changes", () => {
    const base = candidate();
    const baseHash = buildPhase41K6GatewayMintPayloadV2(base).payloadV2Hash;

    const mutations: Phase41K6GatewayMintCandidate[] = [
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          processedEvent: repeatByte32Hex(0x21),
        },
      },
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          routeId: repeatByte32Hex(0x22),
        },
      },
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          mint: repeatByte32Hex(0x23),
        },
      },
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          recipientTokenAccount: repeatByte32Hex(0x24),
        },
      },
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          amount: 124n,
        },
      },
      {
        ...base,
        handlerBinding: {
          ...base.handlerBinding,
          guardianSetId: repeatByte32Hex(0x25),
        },
      },
    ];

    for (const mutation of mutations) {
      expect(buildPhase41K6GatewayMintPayloadV2(mutation).payloadV2Hash).not.toBe(
        baseHash,
      );
    }
  });

  it("does not change the payload hash when only watcher operational metadata changes", () => {
    const base = candidate();
    const baseHash = buildPhase41K6GatewayMintPayloadV2(base).payloadV2Hash;

    const changedOperationalMetadata: Phase41K6GatewayMintCandidate = {
      ...base,
      sourceObservation: {
        ...base.sourceObservation,
        sourceBlockNumber: 999999n,
        sourceFinalityState: "safe",
        sourceBurnEventIndex: 99n,
      },
    };

    expect(buildPhase41K6GatewayMintPayloadV2(changedOperationalMetadata).payloadV2Hash).toBe(
      baseHash,
    );
  });

  it("rejects malformed bytes32 fields and invalid u64 amounts before package creation", () => {
    expect(() =>
      buildPhase41K6GatewayMintPayloadV2({
        ...candidate(),
        handlerBinding: {
          ...candidate().handlerBinding,
          guardianSetId: "0x1234",
        },
      }),
    ).toThrow("invalid_guardianSetId_bytes32_hex");

    expect(() =>
      buildPhase41K6GatewayMintPayloadV2({
        ...candidate(),
        handlerBinding: {
          ...candidate().handlerBinding,
          amount: 0x1_0000_0000_0000_0000n,
        },
      }),
    ).toThrow("invalid_amount_u64");
  });
});

import { createHash } from "node:crypto";

export const PHASE_41K6_PAYLOAD_V2_DOMAIN =
  "consume_gateway_mint_authorization_v2";

const BYTES32_HEX_RE = /^0x[0-9a-fA-F]{64}$/;

export type Bytes32Hex = `0x${string}`;

export interface Phase41K6HandlerBindingInput {
  processedEvent: Bytes32Hex;
  routeId: Bytes32Hex;
  mint: Bytes32Hex;
  recipientTokenAccount: Bytes32Hex;
  amount: bigint;
  guardianSetId: Bytes32Hex;
}

export interface Phase41K6WatcherSourceObservation {
  sourceChainId: bigint;
  sourceToken: Bytes32Hex;
  sourceSender: Bytes32Hex;
  sourceBurnTxHash: Bytes32Hex;
  sourceBurnEventIndex: bigint;
  sourceBlockNumber: bigint;
  sourceBlockHash: Bytes32Hex;
  sourceFinalityState: "finalized" | "safe" | "confirmed";
  burnedAmount: bigint;
  canonicalEventKey: Bytes32Hex;
}

export interface Phase41K6GatewayMintCandidate {
  sourceObservation: Phase41K6WatcherSourceObservation;
  handlerBinding: Phase41K6HandlerBindingInput;
}

export interface Phase41K6PayloadV2BuildResult {
  domain: typeof PHASE_41K6_PAYLOAD_V2_DOMAIN;
  hashAlgorithm: "sha256";
  processedEvent: Bytes32Hex;
  routeId: Bytes32Hex;
  mint: Bytes32Hex;
  recipientTokenAccount: Bytes32Hex;
  amount: bigint;
  amountLeHex: Bytes32Hex;
  guardianSetId: Bytes32Hex;
  payloadV2Hash: Bytes32Hex;
}

export function buildPhase41K6GatewayMintPayloadV2(
  candidate: Phase41K6GatewayMintCandidate,
): Phase41K6PayloadV2BuildResult {
  const binding = candidate.handlerBinding;
  const amountLe = encodeU64Le(binding.amount);

  const payloadHash = sha256Hashv([
    new TextEncoder().encode(PHASE_41K6_PAYLOAD_V2_DOMAIN),
    bytes32HexToBytes(binding.processedEvent, "processedEvent"),
    bytes32HexToBytes(binding.routeId, "routeId"),
    bytes32HexToBytes(binding.mint, "mint"),
    bytes32HexToBytes(binding.recipientTokenAccount, "recipientTokenAccount"),
    amountLe,
    bytes32HexToBytes(binding.guardianSetId, "guardianSetId"),
  ]);

  return {
    domain: PHASE_41K6_PAYLOAD_V2_DOMAIN,
    hashAlgorithm: "sha256",
    processedEvent: normalizeBytes32Hex(binding.processedEvent, "processedEvent"),
    routeId: normalizeBytes32Hex(binding.routeId, "routeId"),
    mint: normalizeBytes32Hex(binding.mint, "mint"),
    recipientTokenAccount: normalizeBytes32Hex(
      binding.recipientTokenAccount,
      "recipientTokenAccount",
    ),
    amount: binding.amount,
    amountLeHex: bytesToHex32(amountLe),
    guardianSetId: normalizeBytes32Hex(binding.guardianSetId, "guardianSetId"),
    payloadV2Hash: bytesToHex32(payloadHash),
  };
}

export function buildPhase41K6GatewayMintPayloadV2FromBinding(
  binding: Phase41K6HandlerBindingInput,
): Phase41K6PayloadV2BuildResult {
  return buildPhase41K6GatewayMintPayloadV2({
    sourceObservation: {
      sourceChainId: 1n,
      sourceToken: repeatByte32Hex(0x10),
      sourceSender: repeatByte32Hex(0x11),
      sourceBurnTxHash: repeatByte32Hex(0x12),
      sourceBurnEventIndex: 0n,
      sourceBlockNumber: 1n,
      sourceBlockHash: repeatByte32Hex(0x13),
      sourceFinalityState: "finalized",
      burnedAmount: binding.amount,
      canonicalEventKey: binding.processedEvent,
    },
    handlerBinding: binding,
  });
}

export function normalizeBytes32Hex(value: Bytes32Hex, fieldName: string): Bytes32Hex {
  if (!BYTES32_HEX_RE.test(value)) {
    throw new Error(`invalid_${fieldName}_bytes32_hex`);
  }

  return `0x${value.slice(2).toLowerCase()}`;
}

export function repeatByte32Hex(byte: number): Bytes32Hex {
  if (!Number.isInteger(byte) || byte < 0 || byte > 255) {
    throw new Error("invalid_repeat_byte");
  }

  return `0x${byte.toString(16).padStart(2, "0").repeat(32)}`;
}

function bytes32HexToBytes(value: Bytes32Hex, fieldName: string): Uint8Array {
  const normalized = normalizeBytes32Hex(value, fieldName);
  const out = new Uint8Array(32);

  for (let i = 0; i < 32; i += 1) {
    const start = 2 + i * 2;
    out[i] = Number.parseInt(normalized.slice(start, start + 2), 16);
  }

  return out;
}

function encodeU64Le(value: bigint): Uint8Array {
  if (value < 0n || value > 0xffff_ffff_ffff_ffffn) {
    throw new Error("invalid_amount_u64");
  }

  const out = new Uint8Array(8);
  let remaining = value;

  for (let i = 0; i < 8; i += 1) {
    out[i] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }

  return out;
}

function sha256Hashv(chunks: readonly Uint8Array[]): Uint8Array {
  const hash = createHash("sha256");

  for (const chunk of chunks) {
    hash.update(chunk);
  }

  return hash.digest();
}

function bytesToHex32(bytes: Uint8Array): Bytes32Hex {
  if (bytes.length > 32) {
    throw new Error("bytes_too_long_for_hex32");
  }

  const padded = new Uint8Array(32);
  padded.set(bytes, 0);

  return `0x${Array.from(padded)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("")}`;
}

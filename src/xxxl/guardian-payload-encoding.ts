import {
  bytesToHex,
  concatBytes,
  hexToBytes,
  keccakBytes,
  keccakUtf8Label,
  utf8Bytes,
} from "../gateway/stage-1-encoding.js";

export const XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER = [
  "message_type",
  "schema_version",
  "instruction_layout_version",
  "route_id",
  "source_chain_id",
  "source_token",
  "source_sender",
  "source_burn_tx_hash",
  "source_burn_event_index",
  "source_block_number",
  "source_block_hash",
  "source_finality_block",
  "canonical_event_key",
  "x1_recipient",
  "burned_amount",
  "source_chain_weight_bps",
  "xxxl_mint_amount",
  "target_mint",
  "guardian_set_id",
  "message_nonce",
  "expiration_slot_or_unix_ts",
] as const;

export const XXXL_GUARDIAN_PAYLOAD_ENCODING =
  "XXXL_GUARDIAN_PAYLOAD_CANONICAL_BINARY_V1";

export const XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE = "XXXL_GATEWAY_MINT";

export const XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION = 1;

export const XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION = 2;

export const XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL =
  "XXXL_GUARDIAN_PAYLOAD_HASH_V1";

export type XXXLGuardianPayloadFieldName =
  (typeof XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER)[number];

export type XXXLGuardianPayloadInteger = number | bigint;

export type XXXLGuardianPayloadFields = {
  readonly message_type: string;
  readonly schema_version: number;
  readonly instruction_layout_version: number;
  readonly route_id: Uint8Array;
  readonly source_chain_id: XXXLGuardianPayloadInteger;
  readonly source_token: Uint8Array;
  readonly source_sender: Uint8Array;
  readonly source_burn_tx_hash: Uint8Array;
  readonly source_burn_event_index: XXXLGuardianPayloadInteger;
  readonly source_block_number: XXXLGuardianPayloadInteger;
  readonly source_block_hash: Uint8Array;
  readonly source_finality_block: XXXLGuardianPayloadInteger;
  readonly canonical_event_key: Uint8Array;
  readonly x1_recipient: Uint8Array;
  readonly burned_amount: XXXLGuardianPayloadInteger;
  readonly source_chain_weight_bps: number;
  readonly xxxl_mint_amount: XXXLGuardianPayloadInteger;
  readonly target_mint: Uint8Array;
  readonly guardian_set_id: Uint8Array;
  readonly message_nonce: Uint8Array;
  readonly expiration_slot_or_unix_ts: XXXLGuardianPayloadInteger;
};

export type XXXLGuardianPayloadVector = {
  readonly vectorId: string;
  readonly encoding: typeof XXXL_GUARDIAN_PAYLOAD_ENCODING;
  readonly fieldOrder: readonly XXXLGuardianPayloadFieldName[];
  readonly fields: XXXLGuardianPayloadFields;
  readonly hashDomainLabel: typeof XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL;
  readonly hashDomainSeparatorHex: string;
  readonly encodedPayloadHex: string;
  readonly hashPreimageHex: string;
  readonly payloadHash: string;
};

export type XXXLGuardianPayloadVectorValidationResult = {
  readonly ok: boolean;
  readonly errors: string[];
};

export type XXXLGuardianPayloadInvalidVectorCase = {
  readonly caseId: string;
  readonly reason: string;
};

function requireBytes32(name: string, value: Uint8Array): void {
  if (value.length !== 32) {
    throw new Error(`${name} must be exactly 32 bytes, got ${value.length}`);
  }
}

function requireVarBytes(name: string, value: Uint8Array): void {
  if (value.length === 0) {
    throw new Error(`${name} must be non-empty`);
  }

  if (value.length > 65_535) {
    throw new Error(`${name} must be at most 65535 bytes`);
  }
}

function integerAsBigInt(
  value: XXXLGuardianPayloadInteger,
  name: string,
): bigint {
  if (typeof value === "number") {
    if (!Number.isSafeInteger(value)) {
      throw new Error(`${name} must be a safe integer when passed as number`);
    }

    return BigInt(value);
  }

  return value;
}

function requireUnsignedRange(
  value: XXXLGuardianPayloadInteger,
  bits: 64 | 128,
  name: string,
): bigint {
  const asBigInt = integerAsBigInt(value, name);
  const max = (1n << BigInt(bits)) - 1n;

  if (asBigInt < 0n || asBigInt > max) {
    throw new Error(`${name} must be within u${bits} range`);
  }

  return asBigInt;
}

function requirePositiveUnsignedRange(
  value: XXXLGuardianPayloadInteger,
  bits: 128,
  name: string,
): bigint {
  const asBigInt = requireUnsignedRange(value, bits, name);

  if (asBigInt === 0n) {
    throw new Error(`${name} must be greater than zero`);
  }

  return asBigInt;
}

function requireU16(value: number, name: string): number {
  if (!Number.isInteger(value) || value < 0 || value > 65_535) {
    throw new Error(`${name} must be within u16 range`);
  }

  return value;
}

function u16Le(value: number, name: string): Uint8Array {
  const out = new Uint8Array(2);
  new DataView(out.buffer).setUint16(0, requireU16(value, name), true);
  return out;
}

function u64Le(value: XXXLGuardianPayloadInteger, name: string): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(
    0,
    requireUnsignedRange(value, 64, name),
    true,
  );
  return out;
}

function u128Le(value: XXXLGuardianPayloadInteger, name: string): Uint8Array {
  const asBigInt = requireUnsignedRange(value, 128, name);
  const out = new Uint8Array(16);
  let remaining = asBigInt;

  for (let i = 0; i < out.length; i += 1) {
    out[i] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }

  return out;
}

function lengthPrefixedBytes(name: string, value: Uint8Array): Uint8Array {
  requireVarBytes(name, value);
  return concatBytes(u16Le(value.length, `${name} length`), value);
}

function validateXxxlGuardianPayloadFields(
  fields: XXXLGuardianPayloadFields,
): void {
  if (fields.message_type !== XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE) {
    throw new Error("message_type must equal XXXL guardian payload message type");
  }

  if (fields.schema_version !== XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION) {
    throw new Error("schema_version must equal XXXL guardian payload schema version");
  }

  if (
    fields.instruction_layout_version !==
    XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION
  ) {
    throw new Error("instruction_layout_version must equal current Phase 21 layout version");
  }

  requireBytes32("route_id", fields.route_id);
  requireUnsignedRange(fields.source_chain_id, 64, "source_chain_id");
  requireVarBytes("source_token", fields.source_token);
  requireVarBytes("source_sender", fields.source_sender);
  requireVarBytes("source_burn_tx_hash", fields.source_burn_tx_hash);
  requireUnsignedRange(
    fields.source_burn_event_index,
    64,
    "source_burn_event_index",
  );
  requireUnsignedRange(fields.source_block_number, 64, "source_block_number");
  requireVarBytes("source_block_hash", fields.source_block_hash);
  requireUnsignedRange(
    fields.source_finality_block,
    64,
    "source_finality_block",
  );
  requireBytes32("canonical_event_key", fields.canonical_event_key);
  requireBytes32("x1_recipient", fields.x1_recipient);
  requirePositiveUnsignedRange(fields.burned_amount, 128, "burned_amount");

  if (
    !Number.isInteger(fields.source_chain_weight_bps) ||
    fields.source_chain_weight_bps < 0 ||
    fields.source_chain_weight_bps > 10_000
  ) {
    throw new Error("source_chain_weight_bps must be within 0..10000");
  }

  requirePositiveUnsignedRange(fields.xxxl_mint_amount, 128, "xxxl_mint_amount");
  requireBytes32("target_mint", fields.target_mint);
  requireBytes32("guardian_set_id", fields.guardian_set_id);
  requireBytes32("message_nonce", fields.message_nonce);
  requireUnsignedRange(
    fields.expiration_slot_or_unix_ts,
    64,
    "expiration_slot_or_unix_ts",
  );
}

export function encodeXxxlGuardianPayload(
  fields: XXXLGuardianPayloadFields,
): Uint8Array {
  validateXxxlGuardianPayloadFields(fields);

  return concatBytes(
    lengthPrefixedBytes("message_type", utf8Bytes(fields.message_type)),
    u16Le(fields.schema_version, "schema_version"),
    u16Le(fields.instruction_layout_version, "instruction_layout_version"),
    fields.route_id,
    u64Le(fields.source_chain_id, "source_chain_id"),
    lengthPrefixedBytes("source_token", fields.source_token),
    lengthPrefixedBytes("source_sender", fields.source_sender),
    lengthPrefixedBytes("source_burn_tx_hash", fields.source_burn_tx_hash),
    u64Le(fields.source_burn_event_index, "source_burn_event_index"),
    u64Le(fields.source_block_number, "source_block_number"),
    lengthPrefixedBytes("source_block_hash", fields.source_block_hash),
    u64Le(fields.source_finality_block, "source_finality_block"),
    fields.canonical_event_key,
    fields.x1_recipient,
    u128Le(fields.burned_amount, "burned_amount"),
    u16Le(fields.source_chain_weight_bps, "source_chain_weight_bps"),
    u128Le(fields.xxxl_mint_amount, "xxxl_mint_amount"),
    fields.target_mint,
    fields.guardian_set_id,
    fields.message_nonce,
    u64Le(fields.expiration_slot_or_unix_ts, "expiration_slot_or_unix_ts"),
  );
}

function guardianPayloadDomainSeparatorBytes(): Uint8Array {
  return hexToBytes(
    keccakUtf8Label(XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL),
    32,
    "guardian payload hash domain separator",
  );
}

export function buildXxxlGuardianPayloadHashPreimage(
  fields: XXXLGuardianPayloadFields,
): Uint8Array {
  return concatBytes(
    guardianPayloadDomainSeparatorBytes(),
    encodeXxxlGuardianPayload(fields),
  );
}

export function hashXxxlGuardianPayload(
  fields: XXXLGuardianPayloadFields,
): string {
  return keccakBytes(buildXxxlGuardianPayloadHashPreimage(fields));
}

function repeatedByte(byte: number): Uint8Array {
  return new Uint8Array(32).fill(byte);
}

export const XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS: XXXLGuardianPayloadFields = {
  message_type: XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE,
  schema_version: XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION,
  instruction_layout_version: XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION,
  route_id: repeatedByte(0x11),
  source_chain_id: 1n,
  source_token: hexToBytes("0x010203040506", undefined, "source_token"),
  source_sender: hexToBytes(
    "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    undefined,
    "source_sender",
  ),
  source_burn_tx_hash: repeatedByte(0xaa),
  source_burn_event_index: 7n,
  source_block_number: 123_456_789n,
  source_block_hash: repeatedByte(0xbb),
  source_finality_block: 123_456_900n,
  canonical_event_key: repeatedByte(0x44),
  x1_recipient: repeatedByte(0x55),
  burned_amount: 1_000_000_000_000n,
  source_chain_weight_bps: 10_000,
  xxxl_mint_amount: 1_000_000_000_000n,
  target_mint: repeatedByte(0x33),
  guardian_set_id: repeatedByte(0x22),
  message_nonce: repeatedByte(0x66),
  expiration_slot_or_unix_ts: 987_654_321n,
};

function buildXxxlGuardianPayloadVector(
  fields: XXXLGuardianPayloadFields,
): XXXLGuardianPayloadVector {
  const encodedPayload = encodeXxxlGuardianPayload(fields);
  const hashPreimage = buildXxxlGuardianPayloadHashPreimage(fields);

  return {
    vectorId: "xxxl-guardian-payload-canonical-binary-v1-valid-001",
    encoding: XXXL_GUARDIAN_PAYLOAD_ENCODING,
    fieldOrder: XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER,
    fields,
    hashDomainLabel: XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL,
    hashDomainSeparatorHex: bytesToHex(guardianPayloadDomainSeparatorBytes()),
    encodedPayloadHex: bytesToHex(encodedPayload),
    hashPreimageHex: bytesToHex(hashPreimage),
    payloadHash: keccakBytes(hashPreimage),
  };
}

export const XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR =
  buildXxxlGuardianPayloadVector(XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS);

export const XXXL_GUARDIAN_PAYLOAD_INVALID_VECTOR_CASES: readonly XXXLGuardianPayloadInvalidVectorCase[] =
  [
    {
      caseId: "wrong-message-type",
      reason: "message_type must match the Phase 23 constant",
    },
    {
      caseId: "wrong-schema-version",
      reason: "schema_version must match the Phase 23 constant",
    },
    {
      caseId: "wrong-instruction-layout-version",
      reason: "instruction_layout_version must remain Phase 21 layout version 2",
    },
    {
      caseId: "non-bytes32-route-id",
      reason: "route_id must be exactly 32 bytes",
    },
    {
      caseId: "non-bytes32-canonical-event-key",
      reason: "canonical_event_key must be exactly 32 bytes",
    },
    {
      caseId: "non-bytes32-x1-recipient",
      reason: "x1_recipient must be exactly 32 bytes",
    },
    {
      caseId: "non-bytes32-target-mint",
      reason: "target_mint must be exactly 32 bytes",
    },
    {
      caseId: "non-bytes32-guardian-set-id",
      reason: "guardian_set_id must be exactly 32 bytes",
    },
    {
      caseId: "non-bytes32-message-nonce",
      reason: "message_nonce must be exactly 32 bytes",
    },
    {
      caseId: "empty-source-token",
      reason: "source_token must be non-empty length-prefixed bytes",
    },
    {
      caseId: "empty-source-sender",
      reason: "source_sender must be non-empty length-prefixed bytes",
    },
    {
      caseId: "empty-source-burn-tx-hash",
      reason: "source_burn_tx_hash must be non-empty length-prefixed bytes",
    },
    {
      caseId: "empty-source-block-hash",
      reason: "source_block_hash must be non-empty length-prefixed bytes",
    },
    {
      caseId: "source-chain-weight-above-10000",
      reason: "source_chain_weight_bps must remain within 0..10000",
    },
    {
      caseId: "zero-burned-amount",
      reason: "burned_amount must be greater than zero",
    },
    {
      caseId: "zero-xxxl-mint-amount",
      reason: "xxxl_mint_amount must be greater than zero",
    },
    {
      caseId: "tampered-encoded-payload",
      reason: "encodedPayloadHex must match canonical encoding",
    },
    {
      caseId: "tampered-payload-hash",
      reason: "payloadHash must match hash preimage",
    },
  ];

export function validateXxxlGuardianPayloadVector(
  vector: XXXLGuardianPayloadVector,
): XXXLGuardianPayloadVectorValidationResult {
  const errors: string[] = [];

  if (vector.encoding !== XXXL_GUARDIAN_PAYLOAD_ENCODING) {
    errors.push("wrong encoding");
  }

  if (
    JSON.stringify(vector.fieldOrder) !==
    JSON.stringify(XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER)
  ) {
    errors.push("wrong field order");
  }

  if (vector.hashDomainLabel !== XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL) {
    errors.push("wrong hash domain label");
  }

  try {
    const encodedPayload = encodeXxxlGuardianPayload(vector.fields);
    const hashPreimage = buildXxxlGuardianPayloadHashPreimage(vector.fields);
    const encodedPayloadHex = bytesToHex(encodedPayload);
    const hashPreimageHex = bytesToHex(hashPreimage);
    const payloadHash = keccakBytes(hashPreimage);
    const hashDomainSeparatorHex = bytesToHex(
      guardianPayloadDomainSeparatorBytes(),
    );

    if (vector.encodedPayloadHex !== encodedPayloadHex) {
      errors.push("encodedPayloadHex mismatch");
    }

    if (vector.hashPreimageHex !== hashPreimageHex) {
      errors.push("hashPreimageHex mismatch");
    }

    if (vector.payloadHash !== payloadHash) {
      errors.push("payloadHash mismatch");
    }

    if (vector.hashDomainSeparatorHex !== hashDomainSeparatorHex) {
      errors.push("hashDomainSeparatorHex mismatch");
    }
  } catch (error) {
    errors.push(error instanceof Error ? error.message : String(error));
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

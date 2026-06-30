import { describe, expect, it } from "vitest";

import * as guardianPayloadModule from "../../src/xxxl/guardian-payload-encoding.js";
import {
  XXXL_GUARDIAN_PAYLOAD_ENCODING,
  XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER,
  XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL,
  XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION,
  XXXL_GUARDIAN_PAYLOAD_INVALID_VECTOR_CASES,
  XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE,
  XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION,
  XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  buildXxxlGuardianPayloadHashPreimage,
  bytesToHex,
  concatBytes,
  encodeXxxlGuardianPayload,
  hashXxxlGuardianPayload,
  hexToBytes,
  keccakUtf8Label,
  validateXxxlGuardianPayloadVector,
  type XXXLGuardianPayloadFields,
} from "../../src/index.js";

const EXPECTED_VALID_VECTOR_HASH_DOMAIN_SEPARATOR_HEX =
  "0xf1958bbf04d45ddbc5a9f93f200f5005ee47b05cf61a90faf4d93cd6e3eccd66";

const EXPECTED_VALID_VECTOR_ENCODED_PAYLOAD_HEX =
  "0x11005858584c5f474154455741595f4d494e5401000200" +
  "1111111111111111111111111111111111111111111111111111111111111111" +
  "010000000000000006000102030405061400" +
  "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" +
  "2000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" +
  "070000000000000015cd5b07000000002000" +
  "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" +
  "84cd5b0700000000" +
  "4444444444444444444444444444444444444444444444444444444444444444" +
  "5555555555555555555555555555555555555555555555555555555555555555" +
  "0010a5d4e8000000000000000000000010270010a5d4e80000000000000000000000" +
  "3333333333333333333333333333333333333333333333333333333333333333" +
  "2222222222222222222222222222222222222222222222222222222222222222" +
  "6666666666666666666666666666666666666666666666666666666666666666" +
  "b168de3a00000000";

const EXPECTED_VALID_VECTOR_PAYLOAD_HASH =
  "0xab0ee59a1268f3eebf4a9d42725640ce68226e642a61dabd5f904e7680f08015";

function cloneFields(
  overrides: Partial<XXXLGuardianPayloadFields> = {},
): XXXLGuardianPayloadFields {
  return {
    ...XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
    route_id: new Uint8Array(XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.route_id),
    source_token: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_token,
    ),
    source_sender: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_sender,
    ),
    source_burn_tx_hash: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_burn_tx_hash,
    ),
    source_block_hash: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_block_hash,
    ),
    canonical_event_key: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.canonical_event_key,
    ),
    x1_recipient: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.x1_recipient,
    ),
    target_mint: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.target_mint,
    ),
    guardian_set_id: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.guardian_set_id,
    ),
    message_nonce: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.message_nonce,
    ),
    ...overrides,
  };
}

function expectEncodingAndHashChange(
  mutate: (fields: XXXLGuardianPayloadFields) => XXXLGuardianPayloadFields,
): void {
  const baselineFields = cloneFields();
  const mutatedFields = mutate(cloneFields());

  expect(bytesToHex(encodeXxxlGuardianPayload(mutatedFields))).not.toBe(
    bytesToHex(encodeXxxlGuardianPayload(baselineFields)),
  );
  expect(hashXxxlGuardianPayload(mutatedFields)).not.toBe(
    hashXxxlGuardianPayload(baselineFields),
  );
}

function tamperHex(hex: string): string {
  const replacement = hex[2] === "0" ? "1" : "0";
  return `${hex.slice(0, 2)}${replacement}${hex.slice(3)}`;
}

describe("XXXL guardian payload canonical byte encoding", () => {
  it("keeps field order exactly as Phase 22 froze it", () => {
    expect(XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER).toEqual([
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
    ]);
  });

  it("valid vector validates", () => {
    expect(
      validateXxxlGuardianPayloadVector(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR),
    ).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("encoded payload hex is deterministic", () => {
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encoding).toBe(
      XXXL_GUARDIAN_PAYLOAD_ENCODING,
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex).toBe(
      bytesToHex(encodeXxxlGuardianPayload(cloneFields())),
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex).toBe(
      EXPECTED_VALID_VECTOR_ENCODED_PAYLOAD_HEX,
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex).toMatch(
      /^0x[0-9a-f]+$/,
    );
  });

  it("hash preimage is domain separator plus encoded payload", () => {
    const domainSeparator = hexToBytes(
      keccakUtf8Label(XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL),
      32,
      "domain separator",
    );
    const encodedPayload = encodeXxxlGuardianPayload(cloneFields());

    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.hashDomainSeparatorHex).toBe(
      EXPECTED_VALID_VECTOR_HASH_DOMAIN_SEPARATOR_HEX,
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.hashPreimageHex).toBe(
      bytesToHex(concatBytes(domainSeparator, encodedPayload)),
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.hashPreimageHex).toBe(
      `${EXPECTED_VALID_VECTOR_HASH_DOMAIN_SEPARATOR_HEX}${EXPECTED_VALID_VECTOR_ENCODED_PAYLOAD_HEX.slice(
        2,
      )}`,
    );
  });

  it("payload hash is deterministic", () => {
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash).toBe(
      hashXxxlGuardianPayload(cloneFields()),
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash).toBe(
      EXPECTED_VALID_VECTOR_PAYLOAD_HASH,
    );
    expect(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash).toMatch(
      /^0x[0-9a-f]{64}$/,
    );
  });

  it("changing source_chain_id changes encoded bytes and payload hash", () => {
    expectEncodingAndHashChange((fields) => ({
      ...fields,
      source_chain_id: 2n,
    }));
  });

  it("changing canonical_event_key changes encoded bytes and payload hash", () => {
    expectEncodingAndHashChange((fields) => ({
      ...fields,
      canonical_event_key: new Uint8Array(32).fill(0x45),
    }));
  });

  it("changing source_chain_weight_bps changes encoded bytes and payload hash", () => {
    expectEncodingAndHashChange((fields) => ({
      ...fields,
      source_chain_weight_bps: 9_999,
    }));
  });

  it("changing message_nonce changes encoded bytes and payload hash", () => {
    expectEncodingAndHashChange((fields) => ({
      ...fields,
      message_nonce: new Uint8Array(32).fill(0x67),
    }));
  });

  it("rejects wrong message_type", () => {
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ message_type: "WRONG" })),
    ).toThrow(/message_type/);
  });

  it("rejects wrong schema_version", () => {
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ schema_version: 2 })),
    ).toThrow(/schema_version/);
  });

  it("rejects wrong instruction_layout_version if not 2", () => {
    expect(XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION).toBe(2);
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ instruction_layout_version: 1 })),
    ).toThrow(/instruction_layout_version/);
  });

  it("rejects non-bytes32 route_id", () => {
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ route_id: new Uint8Array(31) })),
    ).toThrow(/route_id/);
  });

  it("rejects non-bytes32 canonical_event_key", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ canonical_event_key: new Uint8Array(31) }),
      ),
    ).toThrow(/canonical_event_key/);
  });

  it("rejects non-bytes32 x1_recipient", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ x1_recipient: new Uint8Array(31) }),
      ),
    ).toThrow(/x1_recipient/);
  });

  it("rejects non-bytes32 target_mint", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ target_mint: new Uint8Array(31) }),
      ),
    ).toThrow(/target_mint/);
  });

  it("rejects non-bytes32 guardian_set_id", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ guardian_set_id: new Uint8Array(31) }),
      ),
    ).toThrow(/guardian_set_id/);
  });

  it("rejects non-bytes32 message_nonce", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ message_nonce: new Uint8Array(31) }),
      ),
    ).toThrow(/message_nonce/);
  });

  it("rejects empty source_token/source_sender/source_burn_tx_hash/source_block_hash", () => {
    for (const field of [
      "source_token",
      "source_sender",
      "source_burn_tx_hash",
      "source_block_hash",
    ] as const) {
      expect(() =>
        encodeXxxlGuardianPayload(cloneFields({ [field]: new Uint8Array() })),
      ).toThrow(new RegExp(field));
    }
  });

  it("rejects source_chain_weight_bps above 10000", () => {
    expect(() =>
      encodeXxxlGuardianPayload(
        cloneFields({ source_chain_weight_bps: 10_001 }),
      ),
    ).toThrow(/source_chain_weight_bps/);
  });

  it("rejects zero burned_amount", () => {
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ burned_amount: 0n })),
    ).toThrow(/burned_amount/);
  });

  it("rejects zero xxxl_mint_amount", () => {
    expect(() =>
      encodeXxxlGuardianPayload(cloneFields({ xxxl_mint_amount: 0n })),
    ).toThrow(/xxxl_mint_amount/);
  });

  it("exported vector validation catches tampered encodedPayloadHex", () => {
    const result = validateXxxlGuardianPayloadVector({
      ...XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
      encodedPayloadHex: tamperHex(
        XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex,
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain("encodedPayloadHex mismatch");
  });

  it("exported vector validation catches tampered payloadHash", () => {
    const result = validateXxxlGuardianPayloadVector({
      ...XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
      payloadHash: tamperHex(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain("payloadHash mismatch");
  });

  it("does not expose signature or quorum behavior in this module", () => {
    const exportedNames = Object.keys(guardianPayloadModule);

    expect(exportedNames).not.toContain("verifyGuardianSignature");
    expect(exportedNames).not.toContain("validateGuardianQuorum");
    expect(
      exportedNames.some((name) => /signature|quorum|ed25519/i.test(name)),
    ).toBe(false);
    expect(XXXL_GUARDIAN_PAYLOAD_INVALID_VECTOR_CASES).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ caseId: "tampered-encoded-payload" }),
        expect.objectContaining({ caseId: "tampered-payload-hash" }),
      ]),
    );
    expect(
      bytesToHex(buildXxxlGuardianPayloadHashPreimage(cloneFields())),
    ).toBe(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.hashPreimageHex);
    expect(XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE).toBe("XXXL_GATEWAY_MINT");
    expect(XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION).toBe(1);
  });
});

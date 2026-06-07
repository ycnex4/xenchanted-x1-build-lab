import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";
const EXPECTED_MESSAGE_HASH =
  "0xe0d6278f3ca300a33f07f5d799cfa1072807aa2287ccc6edada206d529c8dea6";

const EXPECTED_GUARDIAN_PUBLIC_KEY =
  "0x03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8";

const REQUIRED_INVALID_VECTOR_IDS = [
  "INVALID_WRONG_FIELD_ORDER_ROUTE_ID_SOURCE_CHAIN_ID_SWAPPED",
  "INVALID_DEADLINEORFINALITYBLOCK_OMITTED",
  "INVALID_MESSAGENONCE_OMITTED",
  "INVALID_BURNED_AMOUNT_DECIMAL_STRING_ENCODING",
  "INVALID_WRONG_SOURCE_CHAIN_ID",
  "INVALID_WRONG_SOURCE_TOKEN",
  "INVALID_WRONG_SOURCE_BURN_TX_HASH",
  "INVALID_WRONG_SOURCE_BURN_EVENT_INDEX",
  "INVALID_WRONG_SOURCE_BLOCK_NUMBER",
  "INVALID_WRONG_SOURCE_BLOCK_HASH",
  "INVALID_WRONG_CANONICAL_EVENT_KEY",
  "INVALID_WRONG_X1_RECIPIENT_HASH",
  "INVALID_EMPTY_X1_RECIPIENT_BYTES",
  "INVALID_NON_32_BYTE_X1_RECIPIENT_BYTES",
  "INVALID_ZERO_X1_RECIPIENT_BYTES",
  "INVALID_BURNED_AMOUNT_ZERO",
  "INVALID_XXXL_MINT_AMOUNT_DIFFERS_FROM_BURNED_AMOUNT",
  "INVALID_SOURCE_CHAIN_WEIGHT_BPS_NOT_10000",
  "INVALID_WRONG_MINT_TOKEN",
  "INVALID_WRONG_ROUTE_ID",
  "INVALID_WRONG_DOMAIN_SEPARATOR",
  "INVALID_WRONG_TARGET_X1_NETWORK_ID",
  "INVALID_WRONG_TARGET_MINT_CORE_ID",
  "INVALID_WRONG_MESSAGE_HASH",
  "INVALID_WRONG_ED25519_SIGNATURE",
  "INVALID_VALID_SIGNATURE_OVER_DIFFERENT_MESSAGE_HASH",
  "INVALID_DUPLICATE_CANONICAL_EVENT_KEY_ALREADY_PROCESSED",
] as const;

function readVectorFixture(): Record<string, unknown> {
  return JSON.parse(readFileSync(VECTOR_PATH, "utf8")) as Record<string, unknown>;
}

function record(value: unknown): Record<string, unknown> {
  expect(value).toBeTypeOf("object");
  expect(value).not.toBeNull();

  return value as Record<string, unknown>;
}

function stringValue(value: unknown): string {
  expect(value).toBeTypeOf("string");

  return value as string;
}

function numberValue(value: unknown): number {
  expect(value).toBeTypeOf("number");

  return value as number;
}

function booleanValue(value: unknown): boolean {
  expect(value).toBeTypeOf("boolean");

  return value as boolean;
}

function arrayValue(value: unknown): unknown[] {
  expect(Array.isArray(value)).toBe(true);

  return value as unknown[];
}

describe("Stage 1 gateway generated vectors", () => {
  it("locks the valid Stage 1 gateway vector profile", () => {
    const fixture = readVectorFixture();

    const metadata = record(fixture.metadata);
    const lengths = record(fixture.lengths);
    const validVector = record(fixture.validVector);
    const guardianSignature = record(validVector.guardianSignature);
    const signatureVerificationChecks = record(fixture.signatureVerificationChecks);
    const fieldOrder = arrayValue(fixture.fieldOrder);

    expect(stringValue(metadata.vectorProfile)).toBe(
      "Stage 1 XNTD-to-XXXL Gateway fixed-width custom big-endian signed payload encoding",
    );
    expect(stringValue(metadata.hashFunction)).toBe("keccak256");
    expect(stringValue(metadata.signatureStandard)).toBe("Ed25519");
    expect(stringValue(metadata.signaturePayload)).toBe("messageHash");
    expect(stringValue(metadata.x1RecipientType)).toBe(
      "32 raw bytes X1/SVM public key",
    );
    expect(stringValue(metadata.processedRegistryKey)).toBe("canonicalEventKey");

    expect(fieldOrder).toHaveLength(19);
    expect(numberValue(lengths.canonicalEventKeyPreimageBytes)).toBe(128);
    expect(numberValue(lengths.domainSeparatorPreimageBytes)).toBe(160);
    expect(numberValue(lengths.encodedGatewayMintMessageBytes)).toBe(608);
    expect(numberValue(lengths.messageHashPreimageBytes)).toBe(640);
    expect(numberValue(lengths.ed25519PrivateKeySeedBytes)).toBe(32);
    expect(numberValue(lengths.ed25519PublicKeyBytes)).toBe(32);
    expect(numberValue(lengths.ed25519SignatureBytes)).toBe(64);

    expect(stringValue(validVector.id)).toBe("STAGE1_GATEWAY_VALID_001");
    expect(stringValue(validVector.messageHash)).toBe(EXPECTED_MESSAGE_HASH);
    expect(stringValue(guardianSignature.messageHash)).toBe(EXPECTED_MESSAGE_HASH);
    expect(stringValue(guardianSignature.guardianPublicKey)).toBe(
      EXPECTED_GUARDIAN_PUBLIC_KEY,
    );
    expect(booleanValue(guardianSignature.signatureVerifies)).toBe(true);

    expect(booleanValue(signatureVerificationChecks.validSignatureOverMessageHash)).toBe(
      true,
    );
    expect(
      booleanValue(signatureVerificationChecks.validSignatureWrongMessageHashFails),
    ).toBe(true);
    expect(
      booleanValue(signatureVerificationChecks.validSignatureWrongPublicKeyFails),
    ).toBe(true);
    expect(booleanValue(signatureVerificationChecks.alteredSignatureFails)).toBe(true);
    expect(
      booleanValue(
        signatureVerificationChecks
          .validSignatureOverDifferentMessageHashFailsForOriginalMessageHash,
      ),
    ).toBe(true);
  });

  it("covers the required invalid Stage 1 gateway vector cases", () => {
    const fixture = readVectorFixture();
    const invalidVectors = arrayValue(fixture.invalidVectors);
    const invalidVectorIds = new Set(
      invalidVectors.map((vector) => stringValue(record(vector).id)),
    );

    expect(invalidVectors).toHaveLength(REQUIRED_INVALID_VECTOR_IDS.length);

    for (const requiredId of REQUIRED_INVALID_VECTOR_IDS) {
      expect(invalidVectorIds.has(requiredId), requiredId).toBe(true);
    }
  });
});

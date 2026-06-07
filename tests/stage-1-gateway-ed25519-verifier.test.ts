import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_ED25519_VERIFICATION_ERROR,
  hexToBytes,
  verifyStage1Ed25519GuardianSignature,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type GuardianSignatureFixture = {
  guardianPublicKey: string;
  messageHash: string;
  guardianSignature: string;
  signatureVerifies: boolean;
};

type SignatureVerificationChecksFixture = {
  validSignatureOverMessageHash: boolean;
  validSignatureWrongMessageHashFails: boolean;
  validSignatureWrongPublicKeyFails: boolean;
  alteredSignatureFails: boolean;
  validSignatureOverDifferentMessageHashFailsForOriginalMessageHash: boolean;
};

type InvalidVectorFixture = {
  id: string;
  messageHash?: string;
  guardianPublicKey?: string;
  guardianSignature?: string;
  signatureVerifies?: boolean;
};

type Stage1GeneratedFixture = {
  validVector: {
    guardianSignature: GuardianSignatureFixture;
  };
  signatureVerificationChecks: SignatureVerificationChecksFixture;
  invalidVectors: InvalidVectorFixture[];
};

function readFixture(): Stage1GeneratedFixture {
  return JSON.parse(
    readFileSync(VECTOR_PATH, "utf8"),
  ) as Stage1GeneratedFixture;
}

function validInputFromFixture() {
  const fixture = readFixture();
  const guardianSignature = fixture.validVector.guardianSignature;

  return {
    fixture,
    input: {
      messageHash: hexToBytes(guardianSignature.messageHash, 32, "messageHash"),
      guardianPublicKey: hexToBytes(
        guardianSignature.guardianPublicKey,
        32,
        "guardianPublicKey",
      ),
      guardianSignature: hexToBytes(
        guardianSignature.guardianSignature,
        64,
        "guardianSignature",
      ),
    },
  };
}

function invalidVectorById(
  fixture: Stage1GeneratedFixture,
  id: string,
): InvalidVectorFixture {
  const vector = fixture.invalidVectors.find((candidate) => candidate.id === id);

  if (vector === undefined) {
    throw new Error(`Missing invalid vector: ${id}`);
  }

  return vector;
}

describe("Stage 1 Ed25519 guardian signature verifier helpers", () => {
  it("accepts the valid deterministic test-only guardian signature", async () => {
    const { fixture, input } = validInputFromFixture();

    const result = await verifyStage1Ed25519GuardianSignature(input);

    expect(fixture.validVector.guardianSignature.signatureVerifies).toBe(true);
    expect(fixture.signatureVerificationChecks.validSignatureOverMessageHash).toBe(
      true,
    );
    expect(result).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("rejects the valid signature against a wrong messageHash", async () => {
    const { fixture, input } = validInputFromFixture();
    const wrongMessageHashVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_MESSAGE_HASH",
    );

    const result = await verifyStage1Ed25519GuardianSignature({
      ...input,
      messageHash: hexToBytes(
        wrongMessageHashVector.messageHash!,
        32,
        "wrongMessageHash",
      ),
    });

    expect(wrongMessageHashVector.signatureVerifies).toBe(false);
    expect(
      fixture.signatureVerificationChecks.validSignatureWrongMessageHashFails,
    ).toBe(true);
    expect(result).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects an altered signature", async () => {
    const { fixture, input } = validInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    const result = await verifyStage1Ed25519GuardianSignature({
      ...input,
      guardianSignature: hexToBytes(
        alteredSignatureVector.guardianSignature!,
        64,
        "alteredGuardianSignature",
      ),
    });

    expect(alteredSignatureVector.signatureVerifies).toBe(false);
    expect(fixture.signatureVerificationChecks.alteredSignatureFails).toBe(true);
    expect(result).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects a signature produced over a different messageHash", async () => {
    const { fixture, input } = validInputFromFixture();
    const differentMessageHashSignatureVector = invalidVectorById(
      fixture,
      "INVALID_VALID_SIGNATURE_OVER_DIFFERENT_MESSAGE_HASH",
    );

    const result = await verifyStage1Ed25519GuardianSignature({
      ...input,
      guardianSignature: hexToBytes(
        differentMessageHashSignatureVector.guardianSignature!,
        64,
        "differentMessageHashGuardianSignature",
      ),
    });

    expect(
      fixture.signatureVerificationChecks
        .validSignatureOverDifferentMessageHashFailsForOriginalMessageHash,
    ).toBe(true);
    expect(result).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects a valid signature with a wrong guardian public key", async () => {
    const { fixture, input } = validInputFromFixture();
    const wrongPublicKey = new Uint8Array(input.guardianPublicKey);
    wrongPublicKey[0]! ^= 1;

    const result = await verifyStage1Ed25519GuardianSignature({
      ...input,
      guardianPublicKey: wrongPublicKey,
    });

    expect(
      fixture.signatureVerificationChecks.validSignatureWrongPublicKeyFails,
    ).toBe(true);
    expect(result).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects malformed Ed25519 input lengths before verification", async () => {
    const { input } = validInputFromFixture();

    await expect(
      verifyStage1Ed25519GuardianSignature({
        ...input,
        messageHash: new Uint8Array(31),
        guardianPublicKey: new Uint8Array(31),
        guardianSignature: new Uint8Array(63),
      }),
    ).resolves.toEqual({
      ok: false,
      errors: [
        STAGE1_ED25519_VERIFICATION_ERROR.InvalidMessageHashLength,
        STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianPublicKeyLength,
        STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianSignatureLength,
      ],
    });
  });
});

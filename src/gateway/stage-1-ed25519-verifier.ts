import * as ed25519 from "@noble/ed25519";

export const STAGE1_ED25519_VERIFICATION_ERROR = {
  InvalidMessageHashLength: "INVALID_MESSAGE_HASH_LENGTH",
  InvalidGuardianPublicKeyLength: "INVALID_GUARDIAN_PUBLIC_KEY_LENGTH",
  InvalidGuardianSignatureLength: "INVALID_GUARDIAN_SIGNATURE_LENGTH",
  SignatureVerificationFailed: "SIGNATURE_VERIFICATION_FAILED",
} as const;

export type Stage1Ed25519VerificationErrorCode =
  (typeof STAGE1_ED25519_VERIFICATION_ERROR)[keyof typeof STAGE1_ED25519_VERIFICATION_ERROR];

export type Stage1Ed25519VerificationInput = {
  messageHash: Uint8Array;
  guardianPublicKey: Uint8Array;
  guardianSignature: Uint8Array;
};

export type Stage1Ed25519VerificationResult = {
  ok: boolean;
  errors: Stage1Ed25519VerificationErrorCode[];
};

export function validateStage1Ed25519InputLengths(
  input: Stage1Ed25519VerificationInput,
): Stage1Ed25519VerificationErrorCode[] {
  const errors: Stage1Ed25519VerificationErrorCode[] = [];

  if (input.messageHash.length !== 32) {
    errors.push(STAGE1_ED25519_VERIFICATION_ERROR.InvalidMessageHashLength);
  }

  if (input.guardianPublicKey.length !== 32) {
    errors.push(
      STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianPublicKeyLength,
    );
  }

  if (input.guardianSignature.length !== 64) {
    errors.push(
      STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianSignatureLength,
    );
  }

  return errors;
}

export async function verifyStage1Ed25519GuardianSignature(
  input: Stage1Ed25519VerificationInput,
): Promise<Stage1Ed25519VerificationResult> {
  const errors = validateStage1Ed25519InputLengths(input);

  if (errors.length !== 0) {
    return {
      ok: false,
      errors,
    };
  }

  const signatureVerifies = await ed25519.verifyAsync(
    input.guardianSignature,
    input.messageHash,
    input.guardianPublicKey,
  );

  if (!signatureVerifies) {
    errors.push(STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

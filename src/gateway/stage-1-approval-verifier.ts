import {
  verifyStage1Ed25519GuardianSignature,
  type Stage1Ed25519VerificationErrorCode,
} from "./stage-1-ed25519-verifier.js";
import {
  verifyStage1GatewayMintMessage,
  type Stage1GatewayVerificationInput,
  type Stage1VerificationErrorCode,
} from "./stage-1-verifier.js";

export type Stage1GatewayApprovalVerificationInput =
  Stage1GatewayVerificationInput & {
    guardianPublicKey: Uint8Array;
    guardianSignature: Uint8Array;
  };

export type Stage1GatewayApprovalVerificationResult = {
  ok: boolean;
  message: {
    ok: boolean;
    errors: Stage1VerificationErrorCode[];
  };
  signature: {
    ok: boolean;
    errors: Stage1Ed25519VerificationErrorCode[];
  };
};

export async function verifyStage1GatewayApproval(
  input: Stage1GatewayApprovalVerificationInput,
): Promise<Stage1GatewayApprovalVerificationResult> {
  const message = verifyStage1GatewayMintMessage(input);
  const signature = await verifyStage1Ed25519GuardianSignature({
    messageHash: input.messageHash,
    guardianPublicKey: input.guardianPublicKey,
    guardianSignature: input.guardianSignature,
  });

  return {
    ok: message.ok && signature.ok,
    message,
    signature,
  };
}

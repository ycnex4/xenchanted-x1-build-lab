import { BuildError, BuildErrorCode } from "../errors/build-error.js";

export type RegistrarMessageKind =
  | "CORE_REDEEM"
  | "XEN_BURN"
  | "LOCK_XNTD"
  | "RELOCK_XNTD";

export interface RegistrarMessage {
  messageId: string;
  kind: RegistrarMessageKind;
  submittedBy: string;
  createdAt: bigint;
}

export interface RegistrarState {
  registrarAuthority: string;
  processedMessages: Set<string>;
}

export function createRegistrarState(registrarAuthority: string): RegistrarState {
  return {
    registrarAuthority,
    processedMessages: new Set<string>()
  };
}

export function acceptRegistrarMessage(
  state: RegistrarState,
  message: RegistrarMessage
): RegistrarMessage {
  if (message.submittedBy !== state.registrarAuthority) {
    throw new BuildError(
      BuildErrorCode.UnauthorizedRegistrar,
      `Unauthorized registrar: ${message.submittedBy}`
    );
  }

  if (state.processedMessages.has(message.messageId)) {
    throw new BuildError(
      BuildErrorCode.DuplicateRegistrarMessage,
      `Registrar message already processed: ${message.messageId}`
    );
  }

  state.processedMessages.add(message.messageId);

  return message;
}

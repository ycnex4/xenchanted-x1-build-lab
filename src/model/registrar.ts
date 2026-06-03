import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type X1Address } from "./build-state.js";

export type RegistrarMessageId = string;
export type RegistrarMessageKind =
  | "CORE_REDEEM"
  | "XEN_BURN"
  | "GENESIS_ORIGIN"
  | "LOCK_XNTD"
  | "UNLOCK_XNTD"
  | "RELOCK_XNTD"
  | "FEE_CHECKPOINT";

export interface RegistrarMessage {
  messageId: RegistrarMessageId;
  kind: RegistrarMessageKind;
  submittedBy: X1Address;
  createdAt: bigint;
}

export interface RegistrarState {
  registrarAuthority: X1Address;
  processedMessages: Set<RegistrarMessageId>;
}

export function createRegistrarState(registrarAuthority: X1Address): RegistrarState {
  return {
    registrarAuthority,
    processedMessages: new Set<RegistrarMessageId>()
  };
}

export function acceptRegistrarMessage(
  state: RegistrarState,
  message: RegistrarMessage
): void {
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
}

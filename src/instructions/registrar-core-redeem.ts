import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import {
  type RegistrarMessage,
  type RegistrarState,
  acceptRegistrarMessage
} from "../model/registrar.js";
import {
  type RedeemEventKey,
  type RedeemEventState,
  acceptCoreRedeemEvent
} from "../model/redeem-events.js";
import { type BuildState } from "../model/build-state.js";

export interface ApplyRegistrarCoreRedeemInput {
  registrar: RegistrarState;
  redeemEvents: RedeemEventState;
  message: RegistrarMessage;
  build: BuildState;
  redeemKey: RedeemEventKey;
  amountBld: bigint;
  redeemedAt: bigint;
}

export function applyRegistrarCoreRedeem(
  input: ApplyRegistrarCoreRedeemInput
): BuildState {
  if (input.message.kind !== "CORE_REDEEM") {
    throw new BuildError(
      BuildErrorCode.InvalidRegistrarMessageKind,
      `Expected CORE_REDEEM message, got: ${input.message.kind}`
    );
  }

  if (input.message.submittedBy !== input.registrar.registrarAuthority) {
    throw new BuildError(
      BuildErrorCode.UnauthorizedRegistrar,
      `Unauthorized registrar: ${input.message.submittedBy}`
    );
  }

  if (input.registrar.processedMessages.has(input.message.messageId)) {
    throw new BuildError(
      BuildErrorCode.DuplicateRegistrarMessage,
      `Registrar message already processed: ${input.message.messageId}`
    );
  }

  if (input.redeemEvents.usedRedeemEvents.has(input.redeemKey)) {
    throw new BuildError(
      BuildErrorCode.DuplicateRedeemEvent,
      `Core redeem event already used: ${input.redeemKey}`
    );
  }

  const build = acceptCoreRedeemEvent(input.redeemEvents, {
    redeemKey: input.redeemKey,
    build: input.build,
    amountBld: input.amountBld,
    redeemedAt: input.redeemedAt
  });

  acceptRegistrarMessage(input.registrar, input.message);

  return build;
}

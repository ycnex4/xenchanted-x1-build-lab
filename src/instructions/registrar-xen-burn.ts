import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import {
  type RegistrarMessage,
  type RegistrarState,
  acceptRegistrarMessage
} from "../model/registrar.js";
import {
  type XenBurnEventKey,
  type XenBurnEventState,
  acceptXenBurnEvent
} from "../model/xen-burn-events.js";
import { type BuildState } from "../model/build-state.js";

export interface ApplyRegistrarXenBurnInput {
  registrar: RegistrarState;
  xenBurnEvents: XenBurnEventState;
  message: RegistrarMessage;
  build: BuildState;
  xenBurnKey: XenBurnEventKey;
  amountXbp: bigint;
  burnedAt: bigint;
}

export function applyRegistrarXenBurn(
  input: ApplyRegistrarXenBurnInput
): BuildState {
  if (input.message.kind !== "XEN_BURN") {
    throw new BuildError(
      BuildErrorCode.InvalidRegistrarMessageKind,
      `Expected XEN_BURN message, got: ${input.message.kind}`
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

  if (input.xenBurnEvents.usedXenBurnEvents.has(input.xenBurnKey)) {
    throw new BuildError(
      BuildErrorCode.DuplicateXenBurnEvent,
      `XEN burn event already used: ${input.xenBurnKey}`
    );
  }

  if (input.amountXbp <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXbpAmount,
      `XEN Burn Power amount must be positive: ${input.amountXbp.toString()}`
    );
  }

  acceptRegistrarMessage(input.registrar, input.message);

  return acceptXenBurnEvent(input.xenBurnEvents, {
    xenBurnKey: input.xenBurnKey,
    build: input.build,
    amountXbp: input.amountXbp,
    burnedAt: input.burnedAt
  });
}

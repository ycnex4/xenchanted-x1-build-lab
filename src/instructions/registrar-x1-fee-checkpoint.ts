import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";
import {
  type RegistrarMessage,
  type RegistrarState,
  acceptRegistrarMessage
} from "../model/registrar.js";
import { applyX1FeeContributionCheckpoint } from "./x1-fee-contribution.js";

export interface ApplyRegistrarX1FeeCheckpointInput {
  registrar: RegistrarState;
  message: RegistrarMessage;
  build: BuildState;
  feeAmount: bigint;
  txCount: bigint;
  countedUntilSlot: bigint;
  updatedAt: bigint;
}

export function applyRegistrarX1FeeCheckpoint(
  input: ApplyRegistrarX1FeeCheckpointInput
): BuildState {
  if (input.message.kind !== "X1_FEE_CHECKPOINT") {
    throw new BuildError(
      BuildErrorCode.InvalidRegistrarMessageKind,
      `Expected X1_FEE_CHECKPOINT message, got: ${input.message.kind}`
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

  const build = applyX1FeeContributionCheckpoint({
    build: input.build,
    feeAmount: input.feeAmount,
    txCount: input.txCount,
    countedUntilSlot: input.countedUntilSlot,
    updatedAt: input.updatedAt
  });

  acceptRegistrarMessage(input.registrar, input.message);

  return build;
}

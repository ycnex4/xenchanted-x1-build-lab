import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";
import {
  type RegistrarMessage,
  type RegistrarState,
  acceptRegistrarMessage
} from "../model/registrar.js";
import {
  type XntdCommitmentEventKey,
  type XntdCommitmentEventState,
  acceptXntdCommitmentEvent
} from "../model/xntd-commitment-events.js";
import { lockXntd, relockXntd } from "./xntd-lock.js";

export interface ApplyRegistrarXntdLockInput {
  registrar: RegistrarState;
  xntdCommitmentEvents: XntdCommitmentEventState;
  message: RegistrarMessage;
  build: BuildState;
  xntdCommitmentEventKey: XntdCommitmentEventKey;
  amountXntd: bigint;
  lockEpoch: number;
  lockedAt: bigint;
}

export interface ApplyRegistrarXntdRelockInput {
  registrar: RegistrarState;
  xntdCommitmentEvents: XntdCommitmentEventState;
  message: RegistrarMessage;
  build: BuildState;
  xntdCommitmentEventKey: XntdCommitmentEventKey;
  amountXntd: bigint;
  lockEpoch: number;
  relockedAt: bigint;
}

function assertRegistrarPrechecks(
  registrar: RegistrarState,
  xntdCommitmentEvents: XntdCommitmentEventState,
  message: RegistrarMessage,
  expectedKind: "LOCK_XNTD" | "RELOCK_XNTD",
  xntdCommitmentEventKey: XntdCommitmentEventKey
): void {
  if (message.kind !== expectedKind) {
    throw new BuildError(
      BuildErrorCode.InvalidRegistrarMessageKind,
      `Expected ${expectedKind} message, got: ${message.kind}`
    );
  }

  if (message.submittedBy !== registrar.registrarAuthority) {
    throw new BuildError(
      BuildErrorCode.UnauthorizedRegistrar,
      `Unauthorized registrar: ${message.submittedBy}`
    );
  }

  if (registrar.processedMessages.has(message.messageId)) {
    throw new BuildError(
      BuildErrorCode.DuplicateRegistrarMessage,
      `Registrar message already processed: ${message.messageId}`
    );
  }

  if (
    xntdCommitmentEvents.usedXntdCommitmentEvents.has(
      xntdCommitmentEventKey
    )
  ) {
    throw new BuildError(
      BuildErrorCode.DuplicateXntdCommitmentEvent,
      `XNTD commitment event already used: ${xntdCommitmentEventKey}`
    );
  }
}

export function applyRegistrarXntdLock(
  input: ApplyRegistrarXntdLockInput
): BuildState {
  assertRegistrarPrechecks(
    input.registrar,
    input.xntdCommitmentEvents,
    input.message,
    "LOCK_XNTD",
    input.xntdCommitmentEventKey
  );

  if (input.amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${input.amountXntd.toString()}`
    );
  }

  acceptRegistrarMessage(input.registrar, input.message);
  acceptXntdCommitmentEvent(
    input.xntdCommitmentEvents,
    input.xntdCommitmentEventKey
  );

  return lockXntd({
    build: input.build,
    amountXntd: input.amountXntd,
    lockEpoch: input.lockEpoch,
    lockedAt: input.lockedAt
  });
}

export function applyRegistrarXntdRelock(
  input: ApplyRegistrarXntdRelockInput
): BuildState {
  assertRegistrarPrechecks(
    input.registrar,
    input.xntdCommitmentEvents,
    input.message,
    "RELOCK_XNTD",
    input.xntdCommitmentEventKey
  );

  if (input.amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${input.amountXntd.toString()}`
    );
  }

  if (!input.build.xcCommitmentActive) {
    throw new BuildError(
      BuildErrorCode.XntdCommitmentNotActive,
      "Cannot relock XNTD when XC commitment is not active"
    );
  }

  if (input.build.availableBld < input.build.historyBld) {
    throw new BuildError(
      BuildErrorCode.InsufficientAvailableBldForRelock,
      `Relock requires availableBld >= historyBld: available=${input.build.availableBld.toString()}, history=${input.build.historyBld.toString()}`
    );
  }

  acceptRegistrarMessage(input.registrar, input.message);
  acceptXntdCommitmentEvent(
    input.xntdCommitmentEvents,
    input.xntdCommitmentEventKey
  );

  return relockXntd({
    build: input.build,
    amountXntd: input.amountXntd,
    lockEpoch: input.lockEpoch,
    relockedAt: input.relockedAt
  });
}

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
import {
  type XcEpochMinimumSource,
  assertAuthoritativeXcEpochMinimum
} from "../model/xc-epoch-minimum-source.js";
import { lockXntd, relockXntd } from "./xntd-lock.js";

export interface ApplyRegistrarXntdLockInput {
  registrar: RegistrarState;
  xntdCommitmentEvents: XntdCommitmentEventState;
  message: RegistrarMessage;
  build: BuildState;
  xntdCommitmentEventKey: XntdCommitmentEventKey;
  amountXntd: bigint;
  observedRequiredXntdLock: bigint;
  xcEpochMinimumSource?: XcEpochMinimumSource;
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
  observedRequiredXntdLock: bigint;
  xcEpochMinimumSource?: XcEpochMinimumSource;
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

function assertIncreasingLockEpoch(
  build: BuildState,
  incomingLockEpoch: number
): void {
  if (build.lockEpoch !== null && incomingLockEpoch <= build.lockEpoch) {
    throw new BuildError(
      BuildErrorCode.NonIncreasingXntdLockEpoch,
      `XNTD lock epoch must increase: current=${build.lockEpoch.toString()}, incoming=${incomingLockEpoch.toString()}`
    );
  }
}

function assertValidObservedRequiredXntdLock(
  amountXntd: bigint,
  observedRequiredXntdLock: bigint
): void {
  if (observedRequiredXntdLock <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `Observed required XNTD lock amount must be positive: ${observedRequiredXntdLock.toString()}`
    );
  }

  if (amountXntd < observedRequiredXntdLock) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must cover observed required lock: amount=${amountXntd.toString()}, required=${observedRequiredXntdLock.toString()}`
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

  assertIncreasingLockEpoch(input.build, input.lockEpoch);

  if (input.amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${input.amountXntd.toString()}`
    );
  }

  assertValidObservedRequiredXntdLock(
    input.amountXntd,
    input.observedRequiredXntdLock
  );

  if (input.xcEpochMinimumSource !== undefined) {
    assertAuthoritativeXcEpochMinimum(
      input.xcEpochMinimumSource,
      input.lockEpoch,
      input.observedRequiredXntdLock
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
    observedRequiredXntdLock: input.observedRequiredXntdLock,
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

  assertIncreasingLockEpoch(input.build, input.lockEpoch);

  if (input.amountXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXntdLockAmount,
      `XNTD lock amount must be positive: ${input.amountXntd.toString()}`
    );
  }

  assertValidObservedRequiredXntdLock(
    input.amountXntd,
    input.observedRequiredXntdLock
  );

  if (input.xcEpochMinimumSource !== undefined) {
    assertAuthoritativeXcEpochMinimum(
      input.xcEpochMinimumSource,
      input.lockEpoch,
      input.observedRequiredXntdLock
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
    observedRequiredXntdLock: input.observedRequiredXntdLock,
    lockEpoch: input.lockEpoch,
    relockedAt: input.relockedAt
  });
}

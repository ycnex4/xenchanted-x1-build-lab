import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";
import {
  type RegistrarMessage,
  type RegistrarState,
  acceptRegistrarMessage
} from "../model/registrar.js";
import { lockXntd, relockXntd } from "./xntd-lock.js";

export interface ApplyRegistrarXntdLockInput {
  registrar: RegistrarState;
  message: RegistrarMessage;
  build: BuildState;
  amountXntd: bigint;
  lockEpoch: number;
  lockedAt: bigint;
}

export interface ApplyRegistrarXntdRelockInput {
  registrar: RegistrarState;
  message: RegistrarMessage;
  build: BuildState;
  amountXntd: bigint;
  lockEpoch: number;
  relockedAt: bigint;
}

function assertRegistrarPrechecks(
  registrar: RegistrarState,
  message: RegistrarMessage,
  expectedKind: "LOCK_XNTD" | "RELOCK_XNTD"
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
}

export function applyRegistrarXntdLock(
  input: ApplyRegistrarXntdLockInput
): BuildState {
  assertRegistrarPrechecks(input.registrar, input.message, "LOCK_XNTD");

  const build = lockXntd({
    build: input.build,
    amountXntd: input.amountXntd,
    lockEpoch: input.lockEpoch,
    lockedAt: input.lockedAt
  });

  acceptRegistrarMessage(input.registrar, input.message);

  return build;
}

export function applyRegistrarXntdRelock(
  input: ApplyRegistrarXntdRelockInput
): BuildState {
  assertRegistrarPrechecks(input.registrar, input.message, "RELOCK_XNTD");

  const build = relockXntd({
    build: input.build,
    amountXntd: input.amountXntd,
    lockEpoch: input.lockEpoch,
    relockedAt: input.relockedAt
  });

  acceptRegistrarMessage(input.registrar, input.message);

  return build;
}

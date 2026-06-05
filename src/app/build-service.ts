import {
  type ApplyRegistrarCoreRedeemInput,
  applyRegistrarCoreRedeem
} from "../instructions/registrar-core-redeem.js";
import {
  type ApplyRegistrarX1FeeCheckpointInput,
  applyRegistrarX1FeeCheckpoint
} from "../instructions/registrar-x1-fee-checkpoint.js";
import {
  type ApplyRegistrarXenBurnInput,
  applyRegistrarXenBurn
} from "../instructions/registrar-xen-burn.js";
import {
  type ApplyRegistrarXntdLockInput,
  type ApplyRegistrarXntdRelockInput,
  applyRegistrarXntdLock,
  applyRegistrarXntdRelock
} from "../instructions/registrar-xntd-lock.js";
import {
  type ClaimGenesisOriginBldInput,
  claimGenesisOriginBld
} from "../instructions/genesis-origin-bld.js";
import {
  type CreateBuildInput
} from "../instructions/create-build.js";
import {
  type BuildRegistry,
  createEmptyBuildRegistry,
  createRegisteredBuild
} from "../model/build-registry.js";
import { type BuildId, type BuildState } from "../model/build-state.js";
import {
  type RedeemEventState,
  createRedeemEventState
} from "../model/redeem-events.js";
import {
  type RegistrarState,
  createRegistrarState
} from "../model/registrar.js";
import {
  type XenBurnEventState,
  createXenBurnEventState
} from "../model/xen-burn-events.js";
import {
  type XntdCommitmentEventState,
  createXntdCommitmentEventState
} from "../model/xntd-commitment-events.js";
import { BuildError } from "../errors/build-error.js";

export interface BuildApplicationState {
  registry: BuildRegistry;
  registrar: RegistrarState;
  redeemEvents: RedeemEventState;
  xenBurnEvents: XenBurnEventState;
  xntdCommitmentEvents: XntdCommitmentEventState;
}

export interface AppErrorResult {
  code: string;
  message: string;
}

export type AppResult<T> =
  | {
      ok: true;
      value: T;
    }
  | {
      ok: false;
      error: AppErrorResult;
    };

export function createBuildApplicationState(
  registrarAuthority: string
): BuildApplicationState {
  return {
    registry: createEmptyBuildRegistry(),
    registrar: createRegistrarState(registrarAuthority),
    redeemEvents: createRedeemEventState(),
    xenBurnEvents: createXenBurnEventState(),
    xntdCommitmentEvents: createXntdCommitmentEventState()
  };
}

function toAppError(error: unknown): AppErrorResult {
  if (error instanceof BuildError) {
    return {
      code: error.code,
      message: error.message
    };
  }

  if (error instanceof Error) {
    return {
      code: "UnknownError",
      message: error.message
    };
  }

  return {
    code: "UnknownError",
    message: "Unknown error"
  };
}

function runAppCommand<T>(fn: () => T): AppResult<T> {
  try {
    return {
      ok: true,
      value: fn()
    };
  } catch (error) {
    return {
      ok: false,
      error: toAppError(error)
    };
  }
}

export function appCreateBuild(
  app: BuildApplicationState,
  input: CreateBuildInput
): AppResult<BuildState> {
  return runAppCommand(() => createRegisteredBuild(app.registry, input));
}

export function appGetBuildById(
  app: BuildApplicationState,
  buildId: BuildId
): AppResult<BuildState> {
  return runAppCommand(() => {
    const build = app.registry.buildsById.get(buildId);

    if (build === undefined) {
      throw new Error(`Build not found: ${buildId}`);
    }

    return build;
  });
}

export function appClaimGenesisOriginBld(
  input: ClaimGenesisOriginBldInput
): AppResult<BuildState> {
  return runAppCommand(() => claimGenesisOriginBld(input));
}

export function appApplyRegistrarCoreRedeem(
  input: Omit<ApplyRegistrarCoreRedeemInput, "registrar" | "redeemEvents"> & {
    app: BuildApplicationState;
  }
): AppResult<BuildState> {
  return runAppCommand(() =>
    applyRegistrarCoreRedeem({
      registrar: input.app.registrar,
      redeemEvents: input.app.redeemEvents,
      message: input.message,
      build: input.build,
      redeemKey: input.redeemKey,
      amountBld: input.amountBld,
      redeemedAt: input.redeemedAt
    })
  );
}

export function appApplyRegistrarXenBurn(
  input: Omit<ApplyRegistrarXenBurnInput, "registrar" | "xenBurnEvents"> & {
    app: BuildApplicationState;
  }
): AppResult<BuildState> {
  return runAppCommand(() =>
    applyRegistrarXenBurn({
      registrar: input.app.registrar,
      xenBurnEvents: input.app.xenBurnEvents,
      message: input.message,
      build: input.build,
      xenBurnKey: input.xenBurnKey,
      amountXbp: input.amountXbp,
      burnedAt: input.burnedAt
    })
  );
}

export function appApplyRegistrarXntdLock(
  input: Omit<ApplyRegistrarXntdLockInput, "registrar" | "xntdCommitmentEvents"> & {
    app: BuildApplicationState;
  }
): AppResult<BuildState> {
  return runAppCommand(() =>
    applyRegistrarXntdLock({
      registrar: input.app.registrar,
      xntdCommitmentEvents: input.app.xntdCommitmentEvents,
      message: input.message,
      build: input.build,
      xntdCommitmentEventKey: input.xntdCommitmentEventKey,
      amountXntd: input.amountXntd,
      observedRequiredXntdLock: input.observedRequiredXntdLock,
      ...(input.xcEpochMinimumSource !== undefined
        ? { xcEpochMinimumSource: input.xcEpochMinimumSource }
        : {}),
      lockEpoch: input.lockEpoch,
      lockedAt: input.lockedAt
    })
  );
}

export function appApplyRegistrarXntdRelock(
  input: Omit<ApplyRegistrarXntdRelockInput, "registrar" | "xntdCommitmentEvents"> & {
    app: BuildApplicationState;
  }
): AppResult<BuildState> {
  return runAppCommand(() =>
    applyRegistrarXntdRelock({
      registrar: input.app.registrar,
      xntdCommitmentEvents: input.app.xntdCommitmentEvents,
      message: input.message,
      build: input.build,
      xntdCommitmentEventKey: input.xntdCommitmentEventKey,
      amountXntd: input.amountXntd,
      observedRequiredXntdLock: input.observedRequiredXntdLock,
      ...(input.xcEpochMinimumSource !== undefined
        ? { xcEpochMinimumSource: input.xcEpochMinimumSource }
        : {}),
      lockEpoch: input.lockEpoch,
      relockedAt: input.relockedAt
    })
  );
}

export function appApplyRegistrarX1FeeCheckpoint(
  input: Omit<ApplyRegistrarX1FeeCheckpointInput, "registrar"> & {
    app: BuildApplicationState;
  }
): AppResult<BuildState> {
  return runAppCommand(() =>
    applyRegistrarX1FeeCheckpoint({
      registrar: input.app.registrar,
      message: input.message,
      build: input.build,
      feeAmount: input.feeAmount,
      txCount: input.txCount,
      countedUntilSlot: input.countedUntilSlot,
      updatedAt: input.updatedAt
    })
  );
}

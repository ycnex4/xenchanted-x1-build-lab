import {
  type AppErrorResult,
  type AppResult,
  type BuildApplicationState,
  appApplyRegistrarCoreRedeem,
  appApplyRegistrarX1FeeCheckpoint,
  appApplyRegistrarXenBurn,
  appApplyRegistrarXntdLock,
  appApplyRegistrarXntdRelock
} from "./build-service.js";
import { BuildError } from "../errors/build-error.js";
import { type BuildState } from "../model/build-state.js";
import {
  type CoreRedeemRegistrarPayload,
  type CreateRegistrarPayloadInput,
  type X1FeeCheckpointRegistrarPayload,
  type XenBurnRegistrarPayload,
  type XntdLockRegistrarPayload,
  type XntdRelockRegistrarPayload,
  buildRegistrarPayloadFromProof
} from "../proofs/registrar-builders.js";
import { type BuildProof } from "../proofs/proof-types.js";

export interface AppSubmitProofInput extends CreateRegistrarPayloadInput {}

function toAppError(error: unknown): AppErrorResult {
  if (error instanceof BuildError) {
    return {
      code: error.code,
      message: error.message
    };
  }

  if (error instanceof Error) {
    return {
      code: "PROOF_SUBMISSION_ERROR",
      message: error.message
    };
  }

  return {
    code: "PROOF_SUBMISSION_ERROR",
    message: "Unknown proof submission error"
  };
}

function buildNotFound(buildId: string): AppResult<BuildState> {
  return {
    ok: false,
    error: {
      code: "BUILD_NOT_FOUND",
      message: `Build not found: ${buildId}`
    }
  };
}

export function appSubmitProof(
  app: BuildApplicationState,
  proof: BuildProof,
  input: AppSubmitProofInput
): AppResult<BuildState> {
  const payloadResult = (() => {
    try {
      return {
        ok: true as const,
        value: buildRegistrarPayloadFromProof(proof, input)
      };
    } catch (error) {
      return {
        ok: false as const,
        error: toAppError(error)
      };
    }
  })();

  if (!payloadResult.ok) {
    return payloadResult;
  }

  const payload = payloadResult.value;
  const build = app.registry.buildsById.get(payload.buildId);

  if (build === undefined) {
    return buildNotFound(payload.buildId);
  }

  switch (payload.message.kind) {
    case "CORE_REDEEM": {
      const coreRedeemPayload = payload as CoreRedeemRegistrarPayload;

      return appApplyRegistrarCoreRedeem({
        app,
        message: coreRedeemPayload.message,
        build,
        redeemKey: coreRedeemPayload.redeemKey,
        amountBld: coreRedeemPayload.amountBld,
        redeemedAt: coreRedeemPayload.redeemedAt
      });
    }

    case "XEN_BURN": {
      const xenBurnPayload = payload as XenBurnRegistrarPayload;

      return appApplyRegistrarXenBurn({
        app,
        message: xenBurnPayload.message,
        build,
        xenBurnKey: xenBurnPayload.xenBurnKey,
        amountXbp: xenBurnPayload.amountXbp,
        burnedAt: xenBurnPayload.burnedAt
      });
    }

    case "LOCK_XNTD": {
      const lockPayload = payload as XntdLockRegistrarPayload;

      return appApplyRegistrarXntdLock({
        app,
        message: lockPayload.message,
        build,
        xntdCommitmentEventKey: lockPayload.xntdCommitmentEventKey,
        amountXntd: lockPayload.amountXntd,
        observedRequiredXntdLock: lockPayload.amountXntd,
        lockEpoch: lockPayload.lockEpoch,
        lockedAt: lockPayload.lockedAt
      });
    }

    case "RELOCK_XNTD": {
      const relockPayload = payload as XntdRelockRegistrarPayload;

      return appApplyRegistrarXntdRelock({
        app,
        message: relockPayload.message,
        build,
        xntdCommitmentEventKey: relockPayload.xntdCommitmentEventKey,
        amountXntd: relockPayload.amountXntd,
        observedRequiredXntdLock: relockPayload.amountXntd,
        lockEpoch: relockPayload.lockEpoch,
        relockedAt: relockPayload.relockedAt
      });
    }

    case "X1_FEE_CHECKPOINT": {
      const feePayload = payload as X1FeeCheckpointRegistrarPayload;

      return appApplyRegistrarX1FeeCheckpoint({
        app,
        message: feePayload.message,
        build,
        feeAmount: feePayload.feeAmount,
        txCount: feePayload.txCount,
        countedUntilSlot: feePayload.countedUntilSlot,
        updatedAt: feePayload.updatedAt
      });
    }

    default:
      return {
        ok: false,
        error: {
          code: "UNSUPPORTED_REGISTRAR_MESSAGE_KIND",
          message: `Unsupported registrar message kind: ${payload.message.kind}`
        }
      };
  }
}

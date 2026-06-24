import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import type {
  BuildId,
  EthereumAddress,
  X1Address,
} from "../model/build-state.js";
import type {
  CoreRedeemProof,
  ProofValidationStatus,
  XenBurnProof,
  XntdLockProof,
} from "../proofs/proof-types.js";
import {
  type AppErrorResult,
  type AppResult,
  type BuildApplicationState,
  appCreateBuild,
} from "./build-service.js";
import {
  appSubmitProof,
  type AppSubmitProofInput,
} from "./proof-submission.js";
import {
  deserializeBuildRegistry,
  deserializeRedeemEventState,
  deserializeRegistrarState,
  deserializeXenBurnEventState,
  deserializeXntdCommitmentEventState,
  serializeBuildRegistry,
  serializeRedeemEventState,
  serializeRegistrarState,
  serializeXenBurnEventState,
  serializeXntdCommitmentEventState,
} from "../storage/serialization.js";

export interface GatewayFullProfileBuildActivationBundle {
  readonly buildId: BuildId;
  readonly owner: X1Address;
  readonly ethereumIdentity: EthereumAddress;

  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;

  readonly coreRedeemProofs: readonly CoreRedeemProof[];
  readonly xenBurnProofs: readonly XenBurnProof[];
  readonly xntdLockProof: XntdLockProof | null;
}

export interface GatewayFullProfileBuildActivationBoundary {
  readonly buildExists: boolean;
  readonly requiresAcceptedXntdLock: boolean;
  readonly coreRedeemProofCount: number;
  readonly xenBurnProofCount: number;
  readonly hasXntdLockProof: boolean;
  readonly hasMinimumCoreRedeemHistory: boolean;
}

export interface GatewayFullProfileBuildPreview {
  readonly buildExists: boolean;
  readonly owner: X1Address;
  readonly buildId: BuildId;
  readonly ethereumIdentity: EthereumAddress;

  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;

  readonly coreRedeemProofCount: number;
  readonly xenBurnProofCount: number;
  readonly hasXntdLockProof: boolean;

  readonly existingHistoryBld: bigint;
  readonly incomingHistoryBld: bigint;
  readonly totalPreviewHistoryBld: bigint;

  readonly existingHistoryXbp: bigint;
  readonly incomingHistoryXbp: bigint;
  readonly totalPreviewHistoryXbp: bigint;

  readonly previewLockedXntd: bigint;
  readonly previewRequiredXntdLock: bigint;
  readonly previewLockEpoch: number | null;

  readonly hasMinimumCoreRedeemHistory: boolean;
  readonly hasMinimumXntdLock: boolean;
  readonly eligible: boolean;
  readonly missingRequirements: readonly string[];
}

function reject(message: string): never {
  throw new BuildError(
    BuildErrorCode.InvalidGatewayFullProfileActivation,
    message,
  );
}

function assertScanCompleted(completed: boolean, name: string): void {
  if (!completed) {
    reject(
      `${name} scan must be completed for gateway full-profile activation`,
    );
  }
}

function assertValidatedStatus(
  status: ProofValidationStatus,
  proofName: string,
): void {
  if (status !== "VALIDATED") {
    reject(`${proofName} must be validated before gateway activation`);
  }
}

function assertSameBuildAndOwner(
  proofName: string,
  proofBuildId: string,
  proofOwner: string,
  expectedBuildId: BuildId,
  expectedOwner: X1Address,
): void {
  if (proofBuildId !== expectedBuildId) {
    reject(
      `${proofName} buildId mismatch: expected=${expectedBuildId}, actual=${proofBuildId}`,
    );
  }

  if (proofOwner !== expectedOwner) {
    reject(
      `${proofName} owner mismatch: expected=${expectedOwner}, actual=${proofOwner}`,
    );
  }
}

function assertNonEmpty(value: string, field: string): void {
  if (value.length === 0) {
    reject(`${field} must not be empty`);
  }
}

export function previewGatewayFullProfileBuildActivation(
  app: BuildApplicationState,
  bundle: GatewayFullProfileBuildActivationBundle,
): GatewayFullProfileBuildPreview {
  assertNonEmpty(bundle.buildId, "buildId");
  assertNonEmpty(bundle.owner, "owner");
  assertNonEmpty(bundle.ethereumIdentity, "ethereumIdentity");

  const existingBuild = app.registry.buildsById.get(bundle.buildId);
  const buildExists = existingBuild !== undefined;

  const existingHistoryBld = existingBuild?.historyBld ?? 0n;
  const existingHistoryXbp = existingBuild?.historyXbp ?? 0n;

  const incomingHistoryBld = bundle.coreRedeemProofs.reduce(
    (sum, proof) => sum + proof.payload.amountBld,
    0n,
  );

  const incomingHistoryXbp = bundle.xenBurnProofs.reduce(
    (sum, proof) => sum + proof.payload.amountXbp,
    0n,
  );

  const totalPreviewHistoryBld = existingHistoryBld + incomingHistoryBld;
  const totalPreviewHistoryXbp = existingHistoryXbp + incomingHistoryXbp;

  const previewLockedXntd =
    bundle.xntdLockProof?.payload.amountXntd ?? existingBuild?.lockedXntd ?? 0n;

  const previewRequiredXntdLock =
    bundle.xntdLockProof?.payload.observedRequiredXntdLock ??
    existingBuild?.requiredXntdLock ??
    0n;

  const previewLockEpoch =
    bundle.xntdLockProof?.payload.lockEpoch ?? existingBuild?.lockEpoch ?? null;

  const hasMinimumCoreRedeemHistory = totalPreviewHistoryBld > 0n;

  const hasMinimumXntdLock =
    previewLockedXntd > 0n &&
    previewLockEpoch !== null &&
    (previewRequiredXntdLock === 0n ||
      previewLockedXntd >= previewRequiredXntdLock);

  const missingRequirements: string[] = [];

  if (!bundle.coreRedeemScanCompleted) {
    missingRequirements.push("CORE_REDEEM_SCAN");
  }

  if (!bundle.xenBurnScanCompleted) {
    missingRequirements.push("XEN_BURN_SCAN");
  }

  if (!bundle.xntdLockScanCompleted) {
    missingRequirements.push("XNTD_LOCK_SCAN");
  }

  if (!hasMinimumCoreRedeemHistory) {
    missingRequirements.push("MINIMUM_CORE_REDEEM_HISTORY");
  }

  if (!hasMinimumXntdLock) {
    missingRequirements.push("MINIMUM_XNTD_LOCK");
  }

  return {
    buildExists,
    owner: bundle.owner,
    buildId: bundle.buildId,
    ethereumIdentity: bundle.ethereumIdentity,

    coreRedeemScanCompleted: bundle.coreRedeemScanCompleted,
    xenBurnScanCompleted: bundle.xenBurnScanCompleted,
    xntdLockScanCompleted: bundle.xntdLockScanCompleted,

    coreRedeemProofCount: bundle.coreRedeemProofs.length,
    xenBurnProofCount: bundle.xenBurnProofs.length,
    hasXntdLockProof: bundle.xntdLockProof !== null,

    existingHistoryBld,
    incomingHistoryBld,
    totalPreviewHistoryBld,

    existingHistoryXbp,
    incomingHistoryXbp,
    totalPreviewHistoryXbp,

    previewLockedXntd,
    previewRequiredXntdLock,
    previewLockEpoch,

    hasMinimumCoreRedeemHistory,
    hasMinimumXntdLock,
    eligible: missingRequirements.length === 0,
    missingRequirements,
  };
}

export function validateGatewayFullProfileBuildActivationBoundary(
  app: BuildApplicationState,
  bundle: GatewayFullProfileBuildActivationBundle,
): GatewayFullProfileBuildActivationBoundary {
  assertNonEmpty(bundle.buildId, "buildId");
  assertNonEmpty(bundle.owner, "owner");
  assertNonEmpty(bundle.ethereumIdentity, "ethereumIdentity");

  assertScanCompleted(bundle.coreRedeemScanCompleted, "Core redeem");
  assertScanCompleted(bundle.xenBurnScanCompleted, "XEN.burn");
  assertScanCompleted(bundle.xntdLockScanCompleted, "XNTD lock");

  const existingBuild = app.registry.buildsById.get(bundle.buildId);
  const buildExists = existingBuild !== undefined;

  if (existingBuild !== undefined) {
    if (existingBuild.owner !== bundle.owner) {
      reject(
        `Existing Build owner mismatch: expected=${existingBuild.owner}, actual=${bundle.owner}`,
      );
    }

    if (existingBuild.ethereumIdentity !== bundle.ethereumIdentity) {
      reject(
        `Existing Build Ethereum identity mismatch: expected=${String(
          existingBuild.ethereumIdentity,
        )}, actual=${bundle.ethereumIdentity}`,
      );
    }
  }

  for (const proof of bundle.coreRedeemProofs) {
    assertValidatedStatus(proof.status, "Core redeem proof");
    assertSameBuildAndOwner(
      "Core redeem proof",
      proof.payload.buildId,
      proof.payload.owner,
      bundle.buildId,
      bundle.owner,
    );
  }

  for (const proof of bundle.xenBurnProofs) {
    assertValidatedStatus(proof.status, "XEN.burn proof");
    assertSameBuildAndOwner(
      "XEN.burn proof",
      proof.payload.buildId,
      proof.payload.owner,
      bundle.buildId,
      bundle.owner,
    );
  }

  if (bundle.xntdLockProof !== null) {
    assertValidatedStatus(bundle.xntdLockProof.status, "XNTD lock proof");
    assertSameBuildAndOwner(
      "XNTD lock proof",
      bundle.xntdLockProof.payload.buildId,
      bundle.xntdLockProof.payload.owner,
      bundle.buildId,
      bundle.owner,
    );
  }

  const hasMinimumCoreRedeemHistory =
    (existingBuild?.historyBld ?? 0n) > 0n ||
    bundle.coreRedeemProofs.some((proof) => proof.payload.amountBld > 0n);

  if (!hasMinimumCoreRedeemHistory) {
    reject("Gateway Build activation requires minimum Core redeem history");
  }

  const requiresAcceptedXntdLock =
    existingBuild === undefined || !existingBuild.xntdCommitmentAccepted;

  if (requiresAcceptedXntdLock && bundle.xntdLockProof === null) {
    reject(
      "New or uncommitted gateway Build activation requires accepted XNTD lock proof",
    );
  }

  return {
    buildExists,
    requiresAcceptedXntdLock,
    coreRedeemProofCount: bundle.coreRedeemProofs.length,
    xenBurnProofCount: bundle.xenBurnProofs.length,
    hasXntdLockProof: bundle.xntdLockProof !== null,
    hasMinimumCoreRedeemHistory,
  };
}

export interface GatewayFullProfileBuildActivationInput extends AppSubmitProofInput {}

export interface GatewayFullProfileBuildActivationResult {
  readonly build: import("../model/build-state.js").BuildState;
  readonly boundary: GatewayFullProfileBuildActivationBoundary;
  readonly appliedCoreRedeemProofs: number;
  readonly appliedXenBurnProofs: number;
  readonly appliedXntdLockProof: boolean;
}

function toGatewayAppError(error: unknown): AppErrorResult {
  if (error instanceof BuildError) {
    return {
      code: error.code,
      message: error.message,
    };
  }

  if (error instanceof Error) {
    return {
      code: BuildErrorCode.InvalidGatewayFullProfileActivation,
      message: error.message,
    };
  }

  return {
    code: BuildErrorCode.InvalidGatewayFullProfileActivation,
    message: "Unknown gateway full-profile activation error",
  };
}

function cloneBuildApplicationState(
  app: BuildApplicationState,
): BuildApplicationState {
  return {
    registry: deserializeBuildRegistry(serializeBuildRegistry(app.registry)),
    registrar: deserializeRegistrarState(
      serializeRegistrarState(app.registrar),
    ),
    redeemEvents: deserializeRedeemEventState(
      serializeRedeemEventState(app.redeemEvents),
    ),
    xenBurnEvents: deserializeXenBurnEventState(
      serializeXenBurnEventState(app.xenBurnEvents),
    ),
    xntdCommitmentEvents: deserializeXntdCommitmentEventState(
      serializeXntdCommitmentEventState(app.xntdCommitmentEvents),
    ),
  };
}

function commitBuildApplicationState(
  target: BuildApplicationState,
  source: BuildApplicationState,
): void {
  target.registry = source.registry;
  target.registrar = source.registrar;
  target.redeemEvents = source.redeemEvents;
  target.xenBurnEvents = source.xenBurnEvents;
  target.xntdCommitmentEvents = source.xntdCommitmentEvents;
}

function rejectAppResult(error: AppErrorResult): never {
  throw new BuildError(
    BuildErrorCode.InvalidGatewayFullProfileActivation,
    error.message,
  );
}

export function appGatewayActivateBuild(
  app: BuildApplicationState,
  bundle: GatewayFullProfileBuildActivationBundle,
  input: GatewayFullProfileBuildActivationInput,
): AppResult<GatewayFullProfileBuildActivationResult> {
  try {
    const boundary = validateGatewayFullProfileBuildActivationBoundary(
      app,
      bundle,
    );

    const sandbox = cloneBuildApplicationState(app);

    if (!boundary.buildExists) {
      const created = appCreateBuild(sandbox, {
        owner: bundle.owner,
        buildId: bundle.buildId,
        ethereumIdentity: bundle.ethereumIdentity,
        createdAt: input.createdAt,
      });

      if (!created.ok) {
        rejectAppResult(created.error);
      }
    }

    if (bundle.xntdLockProof !== null) {
      const lockResult = appSubmitProof(sandbox, bundle.xntdLockProof, input);

      if (!lockResult.ok) {
        rejectAppResult(lockResult.error);
      }
    }

    for (const proof of bundle.coreRedeemProofs) {
      const result = appSubmitProof(sandbox, proof, input);

      if (!result.ok) {
        rejectAppResult(result.error);
      }
    }

    for (const proof of bundle.xenBurnProofs) {
      const result = appSubmitProof(sandbox, proof, input);

      if (!result.ok) {
        rejectAppResult(result.error);
      }
    }

    const build = sandbox.registry.buildsById.get(bundle.buildId);

    if (build === undefined) {
      reject(`Gateway activation did not produce Build: ${bundle.buildId}`);
    }

    commitBuildApplicationState(app, sandbox);

    return {
      ok: true,
      value: {
        build,
        boundary,
        appliedCoreRedeemProofs: bundle.coreRedeemProofs.length,
        appliedXenBurnProofs: bundle.xenBurnProofs.length,
        appliedXntdLockProof: bundle.xntdLockProof !== null,
      },
    };
  } catch (error) {
    return {
      ok: false,
      error: toGatewayAppError(error),
    };
  }
}

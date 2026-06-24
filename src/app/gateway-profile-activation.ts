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
import type { BuildApplicationState } from "./build-service.js";

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
  };
}

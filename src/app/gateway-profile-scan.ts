import type {
  BuildId,
  EthereumAddress,
  X1Address,
} from "../model/build-state.js";
import type {
  CoreRedeemCandidate,
  XenBurnCandidate,
  XntdLockCandidate,
} from "../watchers/watcher-candidates.js";
import {
  convertCoreRedeemCandidateToProof,
  convertXenBurnCandidateToProof,
  convertXntdLockCandidateToProof,
} from "../watchers/proof-conversion.js";
import type { GatewayFullProfileBuildActivationBundle } from "./gateway-profile-activation.js";

export interface GatewayProfileScanInput {
  readonly buildId: BuildId;
  readonly owner: X1Address;
  readonly ethereumIdentity: EthereumAddress;
}

export interface GatewayProfileScanResult extends GatewayProfileScanInput {
  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;
  readonly coreRedeemCandidates: readonly CoreRedeemCandidate[];
  readonly xenBurnCandidates: readonly XenBurnCandidate[];
  readonly xntdLockCandidate: XntdLockCandidate | null;
  readonly scannedAt: bigint | null;
}

export interface CreateGatewayProfileScanResultInput extends GatewayProfileScanInput {
  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;
  readonly coreRedeemCandidates?: readonly CoreRedeemCandidate[];
  readonly xenBurnCandidates?: readonly XenBurnCandidate[];
  readonly xntdLockCandidate?: XntdLockCandidate | null;
  readonly scannedAt?: bigint | null;
}

export interface GatewayProfileScanner {
  scan(input: GatewayProfileScanInput): GatewayProfileScanResult;
}

export interface CreateStaticGatewayProfileScannerInput {
  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;
  readonly coreRedeemCandidates?: readonly CoreRedeemCandidate[];
  readonly xenBurnCandidates?: readonly XenBurnCandidate[];
  readonly xntdLockCandidate?: XntdLockCandidate | null;
  readonly scannedAt?: bigint | null;
}

export interface BuildGatewayFullProfileActivationBundleFromScanInput {
  readonly scanResult: GatewayProfileScanResult;
  readonly validatedAt: bigint;
}

export function createGatewayProfileScanResult(
  input: CreateGatewayProfileScanResultInput,
): GatewayProfileScanResult {
  return {
    buildId: input.buildId,
    owner: input.owner,
    ethereumIdentity: input.ethereumIdentity,
    coreRedeemScanCompleted: input.coreRedeemScanCompleted,
    xenBurnScanCompleted: input.xenBurnScanCompleted,
    xntdLockScanCompleted: input.xntdLockScanCompleted,
    coreRedeemCandidates: input.coreRedeemCandidates ?? [],
    xenBurnCandidates: input.xenBurnCandidates ?? [],
    xntdLockCandidate: input.xntdLockCandidate ?? null,
    scannedAt: input.scannedAt ?? null,
  };
}

export function createStaticGatewayProfileScanner(
  input: CreateStaticGatewayProfileScannerInput,
): GatewayProfileScanner {
  return {
    scan(scanInput: GatewayProfileScanInput): GatewayProfileScanResult {
      return createGatewayProfileScanResult({
        ...scanInput,
        coreRedeemScanCompleted: input.coreRedeemScanCompleted,
        xenBurnScanCompleted: input.xenBurnScanCompleted,
        xntdLockScanCompleted: input.xntdLockScanCompleted,
        coreRedeemCandidates: input.coreRedeemCandidates ?? [],
        xenBurnCandidates: input.xenBurnCandidates ?? [],
        xntdLockCandidate: input.xntdLockCandidate ?? null,
        scannedAt: input.scannedAt ?? null,
      });
    },
  };
}

export function buildGatewayFullProfileActivationBundleFromScan(
  input: BuildGatewayFullProfileActivationBundleFromScanInput,
): GatewayFullProfileBuildActivationBundle {
  const scan = input.scanResult;

  return {
    buildId: scan.buildId,
    owner: scan.owner,
    ethereumIdentity: scan.ethereumIdentity,
    coreRedeemScanCompleted: scan.coreRedeemScanCompleted,
    xenBurnScanCompleted: scan.xenBurnScanCompleted,
    xntdLockScanCompleted: scan.xntdLockScanCompleted,
    coreRedeemProofs: scan.coreRedeemCandidates.map((candidate) =>
      convertCoreRedeemCandidateToProof(candidate, {
        validatedAt: input.validatedAt,
      }),
    ),
    xenBurnProofs: scan.xenBurnCandidates.map((candidate) =>
      convertXenBurnCandidateToProof(candidate, {
        validatedAt: input.validatedAt,
      }),
    ),
    xntdLockProof:
      scan.xntdLockCandidate === null
        ? null
        : convertXntdLockCandidateToProof(scan.xntdLockCandidate, {
            validatedAt: input.validatedAt,
          }),
  };
}

import { describe, expect, it } from "vitest";
import {
  appGetGatewayProfilePreviewDto,
  buildGatewayFullProfileActivationBundleFromScan,
  createBuildApplicationState,
  createCoreRedeemCandidate,
  createGatewayProfileScanResult,
  createStaticGatewayProfileScanner,
  createXenBurnCandidate,
  createXntdLockCandidate,
} from "../src/index.js";

function coreRedeemCandidate(finalized = true) {
  return createCoreRedeemCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "core",
    eventKind: "CORE_REDEEM",
    transactionHash: "tx-core",
    eventIndex: 0,
    observedAt: 1000n,
    finalized,
    buildId: "build-1",
    owner: "x1-owner",
    amountBld: 121n,
    redeemedAt: 1000n,
    coreTokenId: "1",
  });
}

function xenBurnCandidate(finalized = true) {
  return createXenBurnCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "xen",
    eventKind: "XEN_BURN",
    transactionHash: "tx-xen",
    eventIndex: 0,
    observedAt: 1000n,
    finalized,
    buildId: "build-1",
    owner: "x1-owner",
    amountXbp: 1000n,
    burnedAt: 1000n,
    xenAmountBurned: 100000000n,
  });
}

function xntdLockCandidate(finalized = true) {
  return createXntdLockCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "xntd-lock",
    eventKind: "XNTD_LOCK",
    transactionHash: "tx-lock",
    eventIndex: 0,
    observedAt: 1000n,
    finalized,
    buildId: "build-1",
    owner: "x1-owner",
    amountXntd: 100000000n,
    observedRequiredXntdLock: 100000000n,
    lockEpoch: 0,
    lockedAt: 1000n,
  });
}

describe("gateway profile scan boundary", () => {
  it("builds a full-profile activation bundle from a completed scan result", () => {
    const scanResult = createGatewayProfileScanResult({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemCandidates: [coreRedeemCandidate()],
      xenBurnCandidates: [xenBurnCandidate()],
      xntdLockCandidate: xntdLockCandidate(),
      scannedAt: 1200n,
    });

    const bundle = buildGatewayFullProfileActivationBundleFromScan({
      scanResult,
      validatedAt: 1300n,
    });

    expect(bundle.buildId).toBe("build-1");
    expect(bundle.owner).toBe("x1-owner");
    expect(bundle.ethereumIdentity).toBe(
      "0x0000000000000000000000000000000000000001",
    );
    expect(bundle.coreRedeemScanCompleted).toBe(true);
    expect(bundle.xenBurnScanCompleted).toBe(true);
    expect(bundle.xntdLockScanCompleted).toBe(true);
    expect(bundle.coreRedeemProofs).toHaveLength(1);
    expect(bundle.xenBurnProofs).toHaveLength(1);
    expect(bundle.xntdLockProof).not.toBeNull();
    expect(bundle.coreRedeemProofs[0]?.status).toBe("VALIDATED");
    expect(bundle.xenBurnProofs[0]?.status).toBe("VALIDATED");
    expect(bundle.xntdLockProof?.status).toBe("VALIDATED");

    const dto = appGetGatewayProfilePreviewDto({
      app: createBuildApplicationState("registrar-1"),
      bundle,
    });

    expect(dto.action).toBe("CREATE_BUILD");
    expect(dto.canCreateOrUpdateBuild).toBe(true);
    expect(dto.preview.totalPreviewHistoryBld).toBe("121");
    expect(dto.preview.totalPreviewHistoryXbp).toBe("1000");
    expect(dto.preview.previewLockedXntd).toBe("100000000");
  });

  it("preserves a verified-zero scan as completed but ineligible", () => {
    const scanResult = createGatewayProfileScanResult({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      scannedAt: 1200n,
    });

    const bundle = buildGatewayFullProfileActivationBundleFromScan({
      scanResult,
      validatedAt: 1300n,
    });

    expect(bundle.coreRedeemProofs).toHaveLength(0);
    expect(bundle.xenBurnProofs).toHaveLength(0);
    expect(bundle.xntdLockProof).toBeNull();

    const dto = appGetGatewayProfilePreviewDto({
      app: createBuildApplicationState("registrar-1"),
      bundle,
    });

    expect(dto.action).toBe("UNAVAILABLE");
    expect(dto.canCreateOrUpdateBuild).toBe(false);
    expect(dto.preview.missingRequirements).toEqual([
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);
  });

  it("creates a static scanner for deterministic Stage 6 boundary tests", () => {
    const scanner = createStaticGatewayProfileScanner({
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemCandidates: [coreRedeemCandidate()],
      xenBurnCandidates: [xenBurnCandidate()],
      xntdLockCandidate: xntdLockCandidate(),
      scannedAt: 1200n,
    });

    const scanResult = scanner.scan({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(scanResult.coreRedeemCandidates).toHaveLength(1);
    expect(scanResult.xenBurnCandidates).toHaveLength(1);
    expect(scanResult.xntdLockCandidate).not.toBeNull();
    expect(scanResult.scannedAt).toBe(1200n);
  });

  it("rejects non-finalized candidates when building the activation bundle", () => {
    const scanResult = createGatewayProfileScanResult({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemCandidates: [coreRedeemCandidate(false)],
      scannedAt: 1200n,
    });

    expect(() =>
      buildGatewayFullProfileActivationBundleFromScan({
        scanResult,
        validatedAt: 1300n,
      }),
    ).toThrow("Watcher candidate is not finalized");
  });
});

import { describe, expect, it } from "vitest";
import {
  appGetGatewayProfilePreviewDtoFromScan,
  createBuildApplicationState,
  createCoreRedeemCandidate,
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

describe("gateway profile scan-to-preview app flow", () => {
  it("scans a profile and returns a JSON-safe eligible preview DTO without mutating state", () => {
    const app = createBuildApplicationState("registrar-1");

    const scanner = createStaticGatewayProfileScanner({
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemCandidates: [coreRedeemCandidate()],
      xenBurnCandidates: [xenBurnCandidate()],
      xntdLockCandidate: xntdLockCandidate(),
      scannedAt: 1200n,
    });

    const result = appGetGatewayProfilePreviewDtoFromScan({
      app,
      scanner,
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      validatedAt: 1300n,
    });

    expect(result.scanResult.scannedAt).toBe(1200n);
    expect(result.scanResult.coreRedeemCandidates).toHaveLength(1);
    expect(result.scanResult.xenBurnCandidates).toHaveLength(1);
    expect(result.scanResult.xntdLockCandidate).not.toBeNull();

    expect(result.dto.action).toBe("CREATE_BUILD");
    expect(result.dto.canCreateOrUpdateBuild).toBe(true);
    expect(result.dto.preview.totalPreviewHistoryBld).toBe("121");
    expect(result.dto.preview.totalPreviewHistoryXbp).toBe("1000");
    expect(result.dto.preview.previewLockedXntd).toBe("100000000");

    expect(() => JSON.stringify(result.dto)).not.toThrow();

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });

  it("returns an unavailable preview DTO for a completed verified-zero scan", () => {
    const app = createBuildApplicationState("registrar-1");

    const scanner = createStaticGatewayProfileScanner({
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      scannedAt: 1200n,
    });

    const result = appGetGatewayProfilePreviewDtoFromScan({
      app,
      scanner,
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      validatedAt: 1300n,
    });

    expect(result.dto.action).toBe("UNAVAILABLE");
    expect(result.dto.canCreateOrUpdateBuild).toBe(false);
    expect(result.dto.preview.missingRequirements).toEqual([
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);
    expect(result.dto.preview.totalPreviewHistoryBld).toBe("0");
    expect(result.dto.preview.previewLockedXntd).toBe("0");

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
  });

  it("preserves incomplete scan requirements in the preview DTO", () => {
    const app = createBuildApplicationState("registrar-1");

    const scanner = createStaticGatewayProfileScanner({
      coreRedeemScanCompleted: false,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: false,
      scannedAt: 1200n,
    });

    const result = appGetGatewayProfilePreviewDtoFromScan({
      app,
      scanner,
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      validatedAt: 1300n,
    });

    expect(result.dto.action).toBe("UNAVAILABLE");
    expect(result.dto.preview.missingRequirements).toEqual([
      "CORE_REDEEM_SCAN",
      "XNTD_LOCK_SCAN",
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);
  });

  it("fails before preview DTO creation if scan contains a non-finalized candidate", () => {
    const app = createBuildApplicationState("registrar-1");

    const scanner = createStaticGatewayProfileScanner({
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemCandidates: [coreRedeemCandidate(false)],
      scannedAt: 1200n,
    });

    expect(() =>
      appGetGatewayProfilePreviewDtoFromScan({
        app,
        scanner,
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        validatedAt: 1300n,
      }),
    ).toThrow("Watcher candidate is not finalized");

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
  });
});

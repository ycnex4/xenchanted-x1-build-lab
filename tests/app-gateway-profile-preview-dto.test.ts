import { describe, expect, it } from "vitest";
import type {
  CoreRedeemProof,
  XenBurnProof,
  XntdLockProof,
} from "../src/proofs/proof-types.js";
import {
  appGetGatewayProfilePreviewDto,
  createBuildApplicationState,
  createCoreRedeemCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
  convertWatcherCandidateToProof,
} from "../src/index.js";

function lockProof(buildId = "build-1", owner = "x1-owner"): XntdLockProof {
  const candidate = createXntdLockCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "xntd-lock",
    eventKind: "XNTD_LOCK",
    transactionHash: "tx-lock",
    eventIndex: 0,
    observedAt: 1000n,
    finalized: true,
    buildId,
    owner,
    amountXntd: 100000000n,
    observedRequiredXntdLock: 100000000n,
    lockEpoch: 0,
    lockedAt: 1000n,
  });

  const proof = convertWatcherCandidateToProof(candidate, {
    validatedAt: 1100n,
  });

  if (proof.kind !== "XNTD_LOCK_PROOF") {
    throw new Error(`Expected XNTD_LOCK_PROOF, got ${proof.kind}`);
  }

  return proof;
}

function coreRedeemProof(
  buildId = "build-1",
  owner = "x1-owner",
): CoreRedeemProof {
  const candidate = createCoreRedeemCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "core",
    eventKind: "CORE_REDEEM",
    transactionHash: "tx-core",
    eventIndex: 0,
    observedAt: 1000n,
    finalized: true,
    buildId,
    owner,
    amountBld: 121n,
    redeemedAt: 1000n,
    coreTokenId: "1",
  });

  const proof = convertWatcherCandidateToProof(candidate, {
    validatedAt: 1100n,
  });

  if (proof.kind !== "CORE_REDEEM_PROOF") {
    throw new Error(`Expected CORE_REDEEM_PROOF, got ${proof.kind}`);
  }

  return proof;
}

function xenBurnProof(buildId = "build-1", owner = "x1-owner"): XenBurnProof {
  const candidate = createXenBurnCandidate({
    sourceChainId: "eip155-1",
    sourceAddress: "xen",
    eventKind: "XEN_BURN",
    transactionHash: "tx-xen",
    eventIndex: 0,
    observedAt: 1000n,
    finalized: true,
    buildId,
    owner,
    amountXbp: 1000n,
    burnedAt: 1000n,
    xenAmountBurned: 100000000n,
  });

  const proof = convertWatcherCandidateToProof(candidate, {
    validatedAt: 1100n,
  });

  if (proof.kind !== "XEN_BURN_PROOF") {
    throw new Error(`Expected XEN_BURN_PROOF, got ${proof.kind}`);
  }

  return proof;
}

function metricValue(
  dto: ReturnType<typeof appGetGatewayProfilePreviewDto>,
  key: string,
) {
  return dto.metrics.find((metric) => metric.key === key)?.value;
}

describe("gateway profile preview DTO", () => {
  it("returns JSON-safe decimal strings for bigint preview values", () => {
    const app = createBuildApplicationState("registrar-1");

    const dto = appGetGatewayProfilePreviewDto({
      app,
      bundle: {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [coreRedeemProof()],
        xenBurnProofs: [xenBurnProof()],
        xntdLockProof: lockProof(),
      },
    });

    expect(dto.action).toBe("CREATE_BUILD");
    expect(dto.canCreateOrUpdateBuild).toBe(true);
    expect(dto.preview.totalPreviewHistoryBld).toBe("121");
    expect(dto.preview.totalPreviewHistoryXbp).toBe("1000");
    expect(dto.preview.previewLockedXntd).toBe("100000000");
    expect(dto.preview.previewRequiredXntdLock).toBe("100000000");

    expect(metricValue(dto, "history_bld_total")).toBe("121");
    expect(metricValue(dto, "history_xbp_total")).toBe("1000");
    expect(metricValue(dto, "locked_xntd")).toBe("100000000");
    expect(metricValue(dto, "required_xntd_lock")).toBe("100000000");

    expect(() => JSON.stringify(dto)).not.toThrow();

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });

  it("returns JSON-safe missing requirements for an ineligible preview", () => {
    const app = createBuildApplicationState("registrar-1");

    const dto = appGetGatewayProfilePreviewDto({
      app,
      bundle: {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [],
        xenBurnProofs: [],
        xntdLockProof: null,
      },
    });

    expect(dto.action).toBe("UNAVAILABLE");
    expect(dto.canCreateOrUpdateBuild).toBe(false);
    expect(dto.preview.totalPreviewHistoryBld).toBe("0");
    expect(dto.preview.totalPreviewHistoryXbp).toBe("0");
    expect(dto.preview.previewLockedXntd).toBe("0");
    expect(dto.preview.previewRequiredXntdLock).toBe("0");
    expect(dto.preview.missingRequirements).toEqual([
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);

    expect(metricValue(dto, "history_bld_total")).toBe("0");
    expect(metricValue(dto, "locked_xntd")).toBe("0");

    expect(() => JSON.stringify(dto)).not.toThrow();

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });
});

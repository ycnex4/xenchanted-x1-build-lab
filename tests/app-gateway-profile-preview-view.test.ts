import { describe, expect, it } from "vitest";
import type {
  CoreRedeemProof,
  XenBurnProof,
  XntdLockProof,
} from "../src/proofs/proof-types.js";
import {
  appCreateBuild,
  appGetGatewayProfilePreviewView,
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
  view: ReturnType<typeof appGetGatewayProfilePreviewView>,
  key: string,
) {
  return view.metrics.find((metric) => metric.key === key)?.value;
}

function requirementSatisfied(
  view: ReturnType<typeof appGetGatewayProfilePreviewView>,
  code: string,
) {
  return view.requirements.find((requirement) => requirement.code === code)
    ?.satisfied;
}

describe("gateway profile preview view", () => {
  it("returns frontend-ready metrics for an eligible new Build", () => {
    const app = createBuildApplicationState("registrar-1");

    const view = appGetGatewayProfilePreviewView({
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

    expect(view.title).toBe("Build creation preview");
    expect(view.action).toBe("CREATE_BUILD");
    expect(view.canCreateOrUpdateBuild).toBe(true);
    expect(view.summary).toBe("Eligible to create Build.");
    expect(view.preview.eligible).toBe(true);

    expect(metricValue(view, "build_exists")).toBe(false);
    expect(metricValue(view, "core_redeem_proof_count")).toBe(1);
    expect(metricValue(view, "history_bld_total")).toBe(121n);
    expect(metricValue(view, "xen_burn_proof_count")).toBe(1);
    expect(metricValue(view, "history_xbp_total")).toBe(1000n);
    expect(metricValue(view, "locked_xntd")).toBe(100000000n);
    expect(metricValue(view, "required_xntd_lock")).toBe(100000000n);
    expect(metricValue(view, "lock_epoch")).toBe(0);

    expect(view.requirements.every((item) => item.satisfied)).toBe(true);

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });

  it("returns missing requirements for an ineligible preview", () => {
    const app = createBuildApplicationState("registrar-1");

    const view = appGetGatewayProfilePreviewView({
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

    expect(view.action).toBe("UNAVAILABLE");
    expect(view.canCreateOrUpdateBuild).toBe(false);
    expect(view.summary).toBe(
      "Missing requirements: MINIMUM_CORE_REDEEM_HISTORY, MINIMUM_XNTD_LOCK",
    );

    expect(requirementSatisfied(view, "CORE_REDEEM_SCAN")).toBe(true);
    expect(requirementSatisfied(view, "XEN_BURN_SCAN")).toBe(true);
    expect(requirementSatisfied(view, "XNTD_LOCK_SCAN")).toBe(true);
    expect(requirementSatisfied(view, "MINIMUM_CORE_REDEEM_HISTORY")).toBe(
      false,
    );
    expect(requirementSatisfied(view, "MINIMUM_XNTD_LOCK")).toBe(false);

    expect(app.registry.buildsById.size).toBe(0);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });

  it("returns activation action when Build already exists", () => {
    const app = createBuildApplicationState("registrar-1");

    const createResult = appCreateBuild(app, {
      buildId: "build-1",
      owner: "x1-owner",
      createdAt: 1n,
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
    });

    expect(createResult.ok).toBe(true);

    const view = appGetGatewayProfilePreviewView({
      app,
      bundle: {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [coreRedeemProof()],
        xenBurnProofs: [],
        xntdLockProof: lockProof(),
      },
    });

    expect(view.title).toBe("Build update preview");
    expect(view.action).toBe("UPDATE_BUILD");
    expect(view.canCreateOrUpdateBuild).toBe(true);
    expect(view.summary).toBe("Build can be updated with this profile.");
    expect(metricValue(view, "build_exists")).toBe(true);

    expect(app.registry.buildsById.size).toBe(1);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(app.xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(app.xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(0);
  });
});

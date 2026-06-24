import { describe, expect, it } from "vitest";
import type {
  CoreRedeemProof,
  XenBurnProof,
  XntdLockProof,
} from "../src/proofs/proof-types.js";
import {
  appCreateBuild,
  appSubmitProof,
  createBuildApplicationState,
  createCoreRedeemCandidate,
  createStaticXcEpochMinimumSource,
  createXenBurnCandidate,
  createXntdLockCandidate,
  convertWatcherCandidateToProof,
  validateGatewayFullProfileBuildActivationBoundary,
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

describe("gateway full-profile Build activation boundary", () => {
  it("allows new gateway Build activation only with completed scans and XNTD lock proof", () => {
    const app = createBuildApplicationState("registrar-1");

    const boundary = validateGatewayFullProfileBuildActivationBoundary(app, {
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemProofs: [coreRedeemProof()],
      xenBurnProofs: [xenBurnProof()],
      xntdLockProof: lockProof(),
    });

    expect(boundary.buildExists).toBe(false);
    expect(boundary.requiresAcceptedXntdLock).toBe(true);
    expect(boundary.coreRedeemProofCount).toBe(1);
    expect(boundary.xenBurnProofCount).toBe(1);
    expect(boundary.hasXntdLockProof).toBe(true);
  });

  it("rejects a new gateway Build without XNTD lock proof", () => {
    const app = createBuildApplicationState("registrar-1");

    expect(() =>
      validateGatewayFullProfileBuildActivationBoundary(app, {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [],
        xenBurnProofs: [],
        xntdLockProof: null,
      }),
    ).toThrow("requires accepted XNTD lock proof");
  });

  it("rejects unchecked Core redeem or XEN.burn scans", () => {
    const app = createBuildApplicationState("registrar-1");

    expect(() =>
      validateGatewayFullProfileBuildActivationBoundary(app, {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: false,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [],
        xenBurnProofs: [],
        xntdLockProof: lockProof(),
      }),
    ).toThrow("Core redeem scan must be completed");

    expect(() =>
      validateGatewayFullProfileBuildActivationBoundary(app, {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: false,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [],
        xenBurnProofs: [],
        xntdLockProof: lockProof(),
      }),
    ).toThrow("XEN.burn scan must be completed");
  });

  it("allows verified zero history when scans completed and XNTD lock proof exists", () => {
    const app = createBuildApplicationState("registrar-1");

    const boundary = validateGatewayFullProfileBuildActivationBoundary(app, {
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemProofs: [],
      xenBurnProofs: [],
      xntdLockProof: lockProof(),
    });

    expect(boundary.coreRedeemProofCount).toBe(0);
    expect(boundary.xenBurnProofCount).toBe(0);
    expect(boundary.hasXntdLockProof).toBe(true);
  });

  it("does not require new lock proof for an already committed existing Build", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n,
    });

    expect(created.ok).toBe(true);

    const lock = appSubmitProof(app, lockProof(), {
      submittedBy: "registrar-1",
      createdAt: 1200n,
      xcEpochMinimumSource: createStaticXcEpochMinimumSource(
        new Map<number, bigint>([[0, 100000000n]]),
      ),
    });

    expect(lock.ok).toBe(true);

    const boundary = validateGatewayFullProfileBuildActivationBoundary(app, {
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemProofs: [coreRedeemProof()],
      xenBurnProofs: [xenBurnProof()],
      xntdLockProof: null,
    });

    expect(boundary.buildExists).toBe(true);
    expect(boundary.requiresAcceptedXntdLock).toBe(false);
    expect(boundary.hasXntdLockProof).toBe(false);
  });

  it("rejects mismatched proof owner or buildId", () => {
    const app = createBuildApplicationState("registrar-1");

    expect(() =>
      validateGatewayFullProfileBuildActivationBoundary(app, {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [coreRedeemProof("other-build", "x1-owner")],
        xenBurnProofs: [],
        xntdLockProof: lockProof(),
      }),
    ).toThrow("Core redeem proof buildId mismatch");

    expect(() =>
      validateGatewayFullProfileBuildActivationBoundary(app, {
        buildId: "build-1",
        owner: "x1-owner",
        ethereumIdentity: "0x0000000000000000000000000000000000000001",
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemProofs: [],
        xenBurnProofs: [xenBurnProof("build-1", "other-owner")],
        xntdLockProof: lockProof(),
      }),
    ).toThrow("XEN.burn proof owner mismatch");
  });
});

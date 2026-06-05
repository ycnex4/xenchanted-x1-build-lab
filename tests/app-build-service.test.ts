import { describe, expect, it } from "vitest";
import {
  appApplyRegistrarCoreRedeem,
  appApplyRegistrarX1FeeCheckpoint,
  appApplyRegistrarXenBurn,
  appApplyRegistrarXntdLock,
  appApplyRegistrarXntdRelock,
  appClaimGenesisOriginBld,
  appCreateBuild,
  appGetBuildById,
  createBuildApplicationState
} from "../src/index.js";

describe("Build application service", () => {
  it("creates and queries a registered Build", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    const queried = appGetBuildById(app, "build-1");

    expect(queried.ok).toBe(true);

    if (queried.ok) {
      expect(queried.value.owner).toBe("x1-owner");
      expect(queried.value.buildId).toBe("build-1");
    }
  });

  it("returns structured errors for duplicate Build creation", () => {
    const app = createBuildApplicationState("registrar-1");

    appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n
    });

    const duplicate = appCreateBuild(app, {
      owner: "x1-owner-2",
      buildId: "build-1",
      createdAt: 101n
    });

    expect(duplicate.ok).toBe(false);

    if (!duplicate.ok) {
      expect(duplicate.error.code).toBe("DUPLICATE_BUILD_ID");
    }
  });

  it("runs a full lifecycle through the application service layer", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    if (!created.ok) {
      throw new Error("Build creation failed");
    }

    const build = created.value;

    const coreRedeem = appApplyRegistrarCoreRedeem({
      app,
      message: {
        messageId: "message-core-redeem-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 110n
      },
      build,
      redeemKey: "redeem-1",
      amountBld: 121n,
      redeemedAt: 110n
    });

    expect(coreRedeem.ok).toBe(true);

    const xenBurn = appApplyRegistrarXenBurn({
      app,
      message: {
        messageId: "message-xen-burn-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 120n
      },
      build,
      xenBurnKey: "xen-burn-1",
      amountXbp: 1000n,
      burnedAt: 120n
    });

    expect(xenBurn.ok).toBe(true);

    const origin = appClaimGenesisOriginBld({
      build,
      claimedAt: 130n
    });

    expect(origin.ok).toBe(true);

    const lock = appApplyRegistrarXntdLock({
      app,
      message: {
        messageId: "message-lock-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 140n
      },
      build,
      xntdCommitmentEventKey: "app-xntd-commitment-1",
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 140n
    });

    expect(lock.ok).toBe(true);

    const relock = appApplyRegistrarXntdRelock({
      app,
      message: {
        messageId: "message-relock-1",
        kind: "RELOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 150n
      },
      build,
      xntdCommitmentEventKey: "app-xntd-commitment-2",
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 150n
    });

    expect(relock.ok).toBe(true);

    const fee = appApplyRegistrarX1FeeCheckpoint({
      app,
      message: {
        messageId: "message-fee-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 160n
      },
      build,
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 160n
    });

    expect(fee.ok).toBe(true);

    expect(build.historyBld).toBe(121n);
    expect(build.availableBld).toBe(176n);
    expect(build.originBld).toBe(55n);
    expect(build.earnedXbp).toBe(1000n);
    expect(build.availableXbp).toBe(1000n);
    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.x1FeeContribution).toBe(777n);
    expect(build.x1TxCount).toBe(11n);
    expect(app.registrar.processedMessages.size).toBe(5);
  });

  it("returns structured errors for registrar rejection without throwing", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    if (!created.ok) {
      throw new Error("Build creation failed");
    }

    const result = appApplyRegistrarCoreRedeem({
      app,
      message: {
        messageId: "message-core-redeem-1",
        kind: "CORE_REDEEM",
        submittedBy: "wrong-registrar",
        createdAt: 110n
      },
      build: created.value,
      redeemKey: "redeem-1",
      amountBld: 121n,
      redeemedAt: 110n
    });

    expect(result.ok).toBe(false);

    if (!result.ok) {
      expect(result.error.code).toBe("UNAUTHORIZED_REGISTRAR");
    }

    expect(created.value.historyBld).toBe(0n);
    expect(app.registrar.processedMessages.size).toBe(0);
    expect(app.redeemEvents.usedRedeemEvents.size).toBe(0);
  });
});

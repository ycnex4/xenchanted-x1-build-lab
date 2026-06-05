import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyRegistrarCoreRedeem,
  applyRegistrarX1FeeCheckpoint,
  applyRegistrarXenBurn,
  applyRegistrarXntdLock,
  applyRegistrarXntdRelock,
  claimGenesisOriginBld,
  createBuild,
  createRedeemEventState,
  createRegistrarState,
  createXenBurnEventState,
  createXntdCommitmentEventState
} from "../src/index.js";

describe("End-to-end Build scenario", () => {
  it("runs a full MVP Build lifecycle through registrar flows", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const redeemEvents = createRedeemEventState();
    const xenBurnEvents = createXenBurnEventState();

    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 1000n
    });

    applyRegistrarCoreRedeem({
      registrar,
      redeemEvents,
      message: {
        messageId: "message-core-redeem-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      redeemKey: "eth:core:redeem:tx-1:0",
      amountBld: 121n,
      redeemedAt: 1100n
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-xen-burn-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1200n
      },
      build,
      xenBurnKey: "eth:xen:burn:tx-2:0",
      amountXbp: 1000n,
      burnedAt: 1200n
    });

    claimGenesisOriginBld({
      build,
      claimedAt: 1300n
    });

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-xntd-lock-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1400n
      },
      build,
      xntdCommitmentEventKey: "e2e-xntd-commitment-1",
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1400n
    });

    applyRegistrarXntdRelock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-xntd-relock-1",
        kind: "RELOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1500n
      },
      build,
      xntdCommitmentEventKey: "e2e-xntd-commitment-2",
      amountXntd: 250n,
      observedRequiredXntdLock: 250n,
      lockEpoch: 2,
      relockedAt: 1500n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-x1-fee-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1600n
      },
      build,
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 1600n
    });

    expect(build.buildId).toBe("build-1");
    expect(build.owner).toBe("x1-user-1");
    expect(build.ethereumIdentity).toBe(
      "0x0000000000000000000000000000000000000001"
    );

    expect(build.historyBld).toBe(121n);
    expect(build.availableBld).toBe(176n);
    expect(build.originBld).toBe(55n);

    expect(build.earnedXbp).toBe(1000n);
    expect(build.availableXbp).toBe(1000n);

    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xcCommitmentActive).toBe(true);

    expect(build.x1FeeContribution).toBe(777n);
    expect(build.x1TxCount).toBe(11n);
    expect(build.x1FeeCountedUntilSlot).toBe(9000n);
    expect(build.lastFeeUpdateAt).toBe(1600n);
    expect(build.updatedAt).toBe(1600n);

    expect(registrar.processedMessages.size).toBe(5);
    expect(redeemEvents.usedRedeemEvents.has("eth:core:redeem:tx-1:0")).toBe(
      true
    );
    expect(xenBurnEvents.usedXenBurnEvents.has("eth:xen:burn:tx-2:0")).toBe(
      true
    );
  });

  it("rejects duplicate event and message replay after lifecycle start", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const redeemEvents = createRedeemEventState();
    const xenBurnEvents = createXenBurnEventState();

    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarCoreRedeem({
      registrar,
      redeemEvents,
      message: {
        messageId: "message-core-redeem-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      redeemKey: "eth:core:redeem:tx-1:0",
      amountBld: 121n,
      redeemedAt: 1100n
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-xen-burn-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1200n
      },
      build,
      xenBurnKey: "eth:xen:burn:tx-2:0",
      amountXbp: 1000n,
      burnedAt: 1200n
    });

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-core-redeem-2",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1300n
        },
        build,
        redeemKey: "eth:core:redeem:tx-1:0",
        amountBld: 121n,
        redeemedAt: 1300n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-core-redeem-2",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1300n
        },
        build,
        redeemKey: "eth:core:redeem:tx-1:0",
        amountBld: 121n,
        redeemedAt: 1300n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateRedeemEvent
      );
    }

    expect(() =>
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-xen-burn-2",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1300n
        },
        build,
        xenBurnKey: "eth:xen:burn:tx-2:0",
        amountXbp: 1000n,
        burnedAt: 1300n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-xen-burn-2",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1300n
        },
        build,
        xenBurnKey: "eth:xen:burn:tx-2:0",
        amountXbp: 1000n,
        burnedAt: 1300n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateXenBurnEvent
      );
    }

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-core-redeem-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1400n
        },
        build,
        redeemKey: "eth:core:redeem:tx-3:0",
        amountBld: 11n,
        redeemedAt: 1400n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-core-redeem-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1400n
        },
        build,
        redeemKey: "eth:core:redeem:tx-3:0",
        amountBld: 11n,
        redeemedAt: 1400n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateRegistrarMessage
      );
    }

    expect(build.historyBld).toBe(121n);
    expect(build.availableBld).toBe(121n);
    expect(build.earnedXbp).toBe(1000n);
    expect(build.availableXbp).toBe(1000n);

    expect(registrar.processedMessages.size).toBe(2);
    expect(redeemEvents.usedRedeemEvents.size).toBe(1);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(1);
  });
});

import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  applyRegistrarX1FeeCheckpoint,
  applyXenBurnPower,
  createBuild,
  createRegistrarState,
  lockXntd
} from "../src/index.js";

describe("Registrar X1_FEE_CHECKPOINT integration", () => {
  it("accepts X1_FEE_CHECKPOINT registrar message and applies fee checkpoint", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.lastFeeUpdateAt).toBe(1100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accumulates multiple valid registrar fee checkpoints", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-2",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1200n
      },
      build,
      feeAmount: 2500n,
      txCount: 7n,
      countedUntilSlot: 6000n,
      updatedAt: 1200n
    });

    expect(registrar.processedMessages.size).toBe(2);
    expect(build.x1FeeContribution).toBe(3500n);
    expect(build.x1TxCount).toBe(10n);
    expect(build.x1FeeCountedUntilSlot).toBe(6000n);
    expect(build.lastFeeUpdateAt).toBe(1200n);
    expect(build.updatedAt).toBe(1200n);
  });

  it("rejects wrong message kind without mutating state", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        feeAmount: 1000n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        feeAmount: 1000n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidRegistrarMessageKind
      );
    }

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects unauthorized registrar without mutating state", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-1",
          kind: "X1_FEE_CHECKPOINT",
          submittedBy: "bad-registrar",
          createdAt: 1100n
        },
        build,
        feeAmount: 1000n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects duplicate registrar message without applying second checkpoint", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(() =>
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-1",
          kind: "X1_FEE_CHECKPOINT",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        feeAmount: 2500n,
        txCount: 7n,
        countedUntilSlot: 6000n,
        updatedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(1);
    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.lastFeeUpdateAt).toBe(1100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects invalid fee amount without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-1",
          kind: "X1_FEE_CHECKPOINT",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        feeAmount: 0n,
        txCount: 3n,
        countedUntilSlot: 5000n,
        updatedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
    expect(build.x1FeeCountedUntilSlot).toBeNull();
    expect(build.lastFeeUpdateAt).toBeNull();
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects non-increasing slot without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(() =>
      applyRegistrarX1FeeCheckpoint({
        registrar,
        message: {
          messageId: "message-2",
          kind: "X1_FEE_CHECKPOINT",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        feeAmount: 2500n,
        txCount: 7n,
        countedUntilSlot: 5000n,
        updatedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.has("message-2")).toBe(false);
    expect(registrar.processedMessages.size).toBe(1);
    expect(build.x1FeeContribution).toBe(1000n);
    expect(build.x1TxCount).toBe(3n);
    expect(build.x1FeeCountedUntilSlot).toBe(5000n);
    expect(build.lastFeeUpdateAt).toBe(1100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("does not create or change BLD, XBP, or XNTD commitment", () => {
    const registrar = createRegistrarState("registrar-1");
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyCoreRedeemBld({
      build,
      amountBld: 11n,
      redeemedAt: 1050n
    });

    applyXenBurnPower({
      build,
      amountXbp: 100n,
      burnedAt: 1060n
    });

    lockXntd({
      build,
      amountXntd: 500n,
      observedRequiredXntdLock: 500n,
      lockEpoch: 1,
      lockedAt: 1070n
    });

    applyRegistrarX1FeeCheckpoint({
      registrar,
      message: {
        messageId: "message-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      feeAmount: 1000n,
      txCount: 3n,
      countedUntilSlot: 5000n,
      updatedAt: 1100n
    });

    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.originBld).toBe(0n);
    expect(build.earnedXbp).toBe(100n);
    expect(build.availableXbp).toBe(100n);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);
  });
});

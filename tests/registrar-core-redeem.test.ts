import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyRegistrarCoreRedeem,
  createBuild,
  createRedeemEventState,
  createRegistrarState
} from "../src/index.js";

describe("applyRegistrarCoreRedeem", () => {
  it("accepts CORE_REDEEM registrar message and applies BLD once", () => {
    const registrar = createRegistrarState("registrar-1");
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarCoreRedeem({
      registrar,
      redeemEvents,
      message: {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      redeemKey: "redeem-1",
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(redeemEvents.usedRedeemEvents.has("redeem-1")).toBe(true);
    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects non-CORE_REDEEM message without mutating state", () => {
    const registrar = createRegistrarState("registrar-1");
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        redeemKey: "redeem-1",
        amountBld: 11n,
        redeemedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        redeemKey: "redeem-1",
        amountBld: 11n,
        redeemedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidRegistrarMessageKind
      );
    }

    expect(registrar.processedMessages.size).toBe(0);
    expect(redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects duplicate registrar message without applying second redeem", () => {
    const registrar = createRegistrarState("registrar-1");
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarCoreRedeem({
      registrar,
      redeemEvents,
      message: {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      redeemKey: "redeem-1",
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        redeemKey: "redeem-2",
        amountBld: 22n,
        redeemedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(1);
    expect(redeemEvents.usedRedeemEvents.size).toBe(1);
    expect(redeemEvents.usedRedeemEvents.has("redeem-2")).toBe(false);
    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects duplicate redeemKey without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarCoreRedeem({
      registrar,
      redeemEvents,
      message: {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      redeemKey: "redeem-1",
      amountBld: 11n,
      redeemedAt: 1100n
    });

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-2",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        redeemKey: "redeem-1",
        amountBld: 22n,
        redeemedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.has("message-2")).toBe(false);
    expect(registrar.processedMessages.size).toBe(1);
    expect(redeemEvents.usedRedeemEvents.size).toBe(1);
    expect(build.historyBld).toBe(11n);
    expect(build.availableBld).toBe(11n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects invalid BLD amount without marking message or redeemKey", () => {
    const registrar = createRegistrarState("registrar-1");
    const redeemEvents = createRedeemEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarCoreRedeem({
        registrar,
        redeemEvents,
        message: {
          messageId: "message-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        redeemKey: "redeem-1",
        amountBld: 0n,
        redeemedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(redeemEvents.usedRedeemEvents.size).toBe(0);
    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });
});

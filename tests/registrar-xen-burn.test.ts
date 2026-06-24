import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyRegistrarXenBurn,
  createBuild,
  createRegistrarState,
  createXenBurnEventState,
} from "../src/index.js";

describe("applyRegistrarXenBurn", () => {
  it("accepts XEN_BURN registrar message and applies XBP once", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1100n,
      },
      build,
      xenBurnKey: "xen-burn-1",
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(xenBurnEvents.usedXenBurnEvents.has("xen-burn-1")).toBe(true);
    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects non-XEN_BURN message without mutating state", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1100n,
        },
        build,
        xenBurnKey: "xen-burn-1",
        amountXbp: 100n,
        burnedAt: 1100n,
      }),
    ).toThrow(BuildError);

    try {
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-1",
          kind: "CORE_REDEEM",
          submittedBy: "registrar-1",
          createdAt: 1100n,
        },
        build,
        xenBurnKey: "xen-burn-1",
        amountXbp: 100n,
        burnedAt: 1100n,
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidRegistrarMessageKind,
      );
    }

    expect(registrar.processedMessages.size).toBe(0);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(build.historyXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects duplicate registrar message without applying second XBP", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1100n,
      },
      build,
      xenBurnKey: "xen-burn-1",
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(() =>
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1200n,
        },
        build,
        xenBurnKey: "xen-burn-2",
        amountXbp: 250n,
        burnedAt: 1200n,
      }),
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(1);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(1);
    expect(xenBurnEvents.usedXenBurnEvents.has("xen-burn-2")).toBe(false);
    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects duplicate xenBurnKey without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1100n,
      },
      build,
      xenBurnKey: "xen-burn-1",
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(() =>
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-2",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1200n,
        },
        build,
        xenBurnKey: "xen-burn-1",
        amountXbp: 250n,
        burnedAt: 1200n,
      }),
    ).toThrow(BuildError);

    expect(registrar.processedMessages.has("message-2")).toBe(false);
    expect(registrar.processedMessages.size).toBe(1);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(1);
    expect(build.historyXbp).toBe(100n);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects invalid XBP amount without marking message or xenBurnKey", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    expect(() =>
      applyRegistrarXenBurn({
        registrar,
        xenBurnEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n,
        },
        build,
        xenBurnKey: "xen-burn-1",
        amountXbp: 0n,
        burnedAt: 1100n,
      }),
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(xenBurnEvents.usedXenBurnEvents.size).toBe(0);
    expect(build.historyXbp).toBe(0n);
    expect(build.updatedAt).toBe(1000n);
  });

  it("does not create BLD or unrelated accounting values", () => {
    const registrar = createRegistrarState("registrar-1");
    const xenBurnEvents = createXenBurnEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n,
    });

    applyRegistrarXenBurn({
      registrar,
      xenBurnEvents,
      message: {
        messageId: "message-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 1100n,
      },
      build,
      xenBurnKey: "xen-burn-1",
      amountXbp: 100n,
      burnedAt: 1100n,
    });

    expect(build.historyBld).toBe(0n);
    expect(build.originBld).toBe(0n);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

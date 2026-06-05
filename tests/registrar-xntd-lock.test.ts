import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  applyCoreRedeemBld,
  applyRegistrarXntdLock,
  applyRegistrarXntdRelock,
  createBuild,
  createRegistrarState,
  createXntdCommitmentEventState
} from "../src/index.js";

describe("Registrar XNTD lock / relock integration", () => {
  it("accepts LOCK_XNTD registrar message and locks XNTD", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "registrar-xntd-commitment-1",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(
      xntdCommitmentEvents.usedXntdCommitmentEvents.has(
        "registrar-xntd-commitment-1"
      )
    ).toBe(true);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1100n);
  });

  it("accepts RELOCK_XNTD registrar message and relocks XNTD", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
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

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "registrar-xntd-commitment-2",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    applyRegistrarXntdRelock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-2",
        kind: "RELOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1200n
      },
      build,
      xntdCommitmentEventKey: "registrar-xntd-commitment-3",
      amountXntd: 250n,
      lockEpoch: 2,
      relockedAt: 1200n
    });

    expect(registrar.processedMessages.has("message-2")).toBe(true);
    expect(
      xntdCommitmentEvents.usedXntdCommitmentEvents.has(
        "registrar-xntd-commitment-2"
      )
    ).toBe(true);
    expect(
      xntdCommitmentEvents.usedXntdCommitmentEvents.has(
        "registrar-xntd-commitment-3"
      )
    ).toBe(true);
    expect(build.lockedXntd).toBe(250n);
    expect(build.requiredXntdLock).toBe(250n);
    expect(build.lockEpoch).toBe(2);
    expect(build.xcCommitmentActive).toBe(true);
    expect(build.updatedAt).toBe(1200n);
  });

  it("rejects wrong message kind without mutating lock state", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-4",
        amountXntd: 500n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "XEN_BURN",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-5",
        amountXntd: 500n,
        lockEpoch: 1,
        lockedAt: 1100n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidRegistrarMessageKind
      );
    }

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects unauthorized registrar without mutating lock state", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "LOCK_XNTD",
          submittedBy: "bad-registrar",
          createdAt: 1100n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-6",
        amountXntd: 500n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects duplicate registrar message without applying second lock", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "registrar-xntd-commitment-7",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(() =>
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "LOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-8",
        amountXntd: 250n,
        lockEpoch: 2,
        lockedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(1);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects duplicate XNTD commitment event key with a different messageId", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "shared-xntd-commitment-event",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(() =>
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-2",
          kind: "LOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        xntdCommitmentEventKey: "shared-xntd-commitment-event",
        amountXntd: 250n,
        lockEpoch: 2,
        lockedAt: 1200n
      })
    ).toThrow(BuildError);

    try {
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-2",
          kind: "LOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        xntdCommitmentEventKey: "shared-xntd-commitment-event",
        amountXntd: 250n,
        lockEpoch: 2,
        lockedAt: 1200n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateXntdCommitmentEvent
      );
    }

    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(registrar.processedMessages.has("message-2")).toBe(false);
    expect(registrar.processedMessages.size).toBe(1);
    expect(xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(1);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.updatedAt).toBe(1100n);
  });

  it("uses one commitment replay domain across lock and relock", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
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

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "shared-lock-relock-source-event",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(() =>
      applyRegistrarXntdRelock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-2",
          kind: "RELOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1200n
        },
        build,
        xntdCommitmentEventKey: "shared-lock-relock-source-event",
        amountXntd: 250n,
        lockEpoch: 2,
        relockedAt: 1200n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.has("message-2")).toBe(false);
    expect(xntdCommitmentEvents.usedXntdCommitmentEvents.size).toBe(1);
    expect(build.lockedXntd).toBe(500n);
    expect(build.requiredXntdLock).toBe(500n);
    expect(build.lockEpoch).toBe(1);
    expect(build.updatedAt).toBe(1100n);
  });

  it("rejects invalid lock amount without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarXntdLock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "LOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-9",
        amountXntd: 0n,
        lockEpoch: 1,
        lockedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("rejects invalid relock without marking registrar message", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    expect(() =>
      applyRegistrarXntdRelock({
        registrar,
        xntdCommitmentEvents,
        message: {
          messageId: "message-1",
          kind: "RELOCK_XNTD",
          submittedBy: "registrar-1",
          createdAt: 1100n
        },
        build,
        xntdCommitmentEventKey: "registrar-xntd-commitment-10",
        amountXntd: 250n,
        lockEpoch: 2,
        relockedAt: 1100n
      })
    ).toThrow(BuildError);

    expect(registrar.processedMessages.size).toBe(0);
    expect(build.lockedXntd).toBe(0n);
    expect(build.requiredXntdLock).toBe(0n);
    expect(build.lockEpoch).toBeNull();
    expect(build.xcCommitmentActive).toBe(false);
    expect(build.updatedAt).toBe(1000n);
  });

  it("does not create BLD, XBP, or X1 fee contribution", () => {
    const registrar = createRegistrarState("registrar-1");
    const xntdCommitmentEvents = createXntdCommitmentEventState();
    const build = createBuild({
      owner: "x1-user-1",
      buildId: "build-1",
      createdAt: 1000n
    });

    applyRegistrarXntdLock({
      registrar,
      xntdCommitmentEvents,
      message: {
        messageId: "message-1",
        kind: "LOCK_XNTD",
        submittedBy: "registrar-1",
        createdAt: 1100n
      },
      build,
      xntdCommitmentEventKey: "registrar-xntd-commitment-11",
      amountXntd: 500n,
      lockEpoch: 1,
      lockedAt: 1100n
    });

    expect(build.historyBld).toBe(0n);
    expect(build.availableBld).toBe(0n);
    expect(build.originBld).toBe(0n);
    expect(build.earnedXbp).toBe(0n);
    expect(build.availableXbp).toBe(0n);
    expect(build.x1FeeContribution).toBe(0n);
    expect(build.x1TxCount).toBe(0n);
  });
});

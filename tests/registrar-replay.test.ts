import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  acceptRegistrarMessage,
  createRegistrarState
} from "../src/index.js";

describe("registrar replay protection", () => {
  it("accepts a new message from registrar authority", () => {
    const registrar = createRegistrarState("registrar-1");

    acceptRegistrarMessage(registrar, {
      messageId: "message-1",
      kind: "CORE_REDEEM",
      submittedBy: "registrar-1",
      createdAt: 1000n
    });

    expect(registrar.processedMessages.has("message-1")).toBe(true);
  });

  it("rejects duplicate registrar message", () => {
    const registrar = createRegistrarState("registrar-1");

    acceptRegistrarMessage(registrar, {
      messageId: "message-1",
      kind: "CORE_REDEEM",
      submittedBy: "registrar-1",
      createdAt: 1000n
    });

    expect(() =>
      acceptRegistrarMessage(registrar, {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1001n
      })
    ).toThrow(BuildError);

    try {
      acceptRegistrarMessage(registrar, {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 1001n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateRegistrarMessage
      );
    }

    expect(registrar.processedMessages.size).toBe(1);
  });

  it("rejects unauthorized registrar before changing state", () => {
    const registrar = createRegistrarState("registrar-1");

    expect(() =>
      acceptRegistrarMessage(registrar, {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "attacker-1",
        createdAt: 1000n
      })
    ).toThrow(BuildError);

    try {
      acceptRegistrarMessage(registrar, {
        messageId: "message-1",
        kind: "CORE_REDEEM",
        submittedBy: "attacker-1",
        createdAt: 1000n
      });
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.UnauthorizedRegistrar
      );
    }

    expect(registrar.processedMessages.size).toBe(0);
    expect(registrar.processedMessages.has("message-1")).toBe(false);
  });

  it("accepts different message ids from registrar authority", () => {
    const registrar = createRegistrarState("registrar-1");

    acceptRegistrarMessage(registrar, {
      messageId: "message-1",
      kind: "CORE_REDEEM",
      submittedBy: "registrar-1",
      createdAt: 1000n
    });

    acceptRegistrarMessage(registrar, {
      messageId: "message-2",
      kind: "XEN_BURN",
      submittedBy: "registrar-1",
      createdAt: 1001n
    });

    expect(registrar.processedMessages.size).toBe(2);
    expect(registrar.processedMessages.has("message-1")).toBe(true);
    expect(registrar.processedMessages.has("message-2")).toBe(true);
  });
});

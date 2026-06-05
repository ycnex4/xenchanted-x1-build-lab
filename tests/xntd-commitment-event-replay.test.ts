import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  acceptXntdCommitmentEvent,
  createXntdCommitmentEventState
} from "../src/index.js";

describe("XNTD commitment event replay protection", () => {
  it("accepts a new XNTD commitment event and records event key", () => {
    const events = createXntdCommitmentEventState();

    const accepted = acceptXntdCommitmentEvent(events, "commitment-1");

    expect(accepted).toBe("commitment-1");
    expect(events.usedXntdCommitmentEvents.has("commitment-1")).toBe(true);
    expect(events.usedXntdCommitmentEvents.size).toBe(1);
  });

  it("rejects duplicate XNTD commitment event key", () => {
    const events = createXntdCommitmentEventState();

    acceptXntdCommitmentEvent(events, "commitment-1");

    expect(() =>
      acceptXntdCommitmentEvent(events, "commitment-1")
    ).toThrow(BuildError);

    try {
      acceptXntdCommitmentEvent(events, "commitment-1");
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.DuplicateXntdCommitmentEvent
      );
    }

    expect(events.usedXntdCommitmentEvents.size).toBe(1);
  });

  it("accepts different XNTD commitment event keys", () => {
    const events = createXntdCommitmentEventState();

    acceptXntdCommitmentEvent(events, "commitment-1");
    acceptXntdCommitmentEvent(events, "commitment-2");

    expect(events.usedXntdCommitmentEvents.has("commitment-1")).toBe(true);
    expect(events.usedXntdCommitmentEvents.has("commitment-2")).toBe(true);
    expect(events.usedXntdCommitmentEvents.size).toBe(2);
  });

  it("uses one replay domain for lock and relock source events", () => {
    const events = createXntdCommitmentEventState();

    const sharedSourceEventKey =
      "chain:1|contract:xc|kind:LOCK_XNTD|tx:0xabc|log:7";

    acceptXntdCommitmentEvent(events, sharedSourceEventKey);

    expect(() =>
      acceptXntdCommitmentEvent(events, sharedSourceEventKey)
    ).toThrow(BuildError);

    expect(events.usedXntdCommitmentEvents.size).toBe(1);
  });
});

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_PROCESSED_BURN_REGISTRY_ERROR,
  bytes32,
  checkStage1BurnNotProcessed,
  createStage1ProcessedBurnRegistry,
  markStage1BurnProcessed,
  stage1CanonicalEventKeyHex,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type Stage1GeneratedFixture = {
  validVector: {
    canonicalEventKey: string;
  };
};

function readFixture(): Stage1GeneratedFixture {
  return JSON.parse(
    readFileSync(VECTOR_PATH, "utf8"),
  ) as Stage1GeneratedFixture;
}

function canonicalEventKeyFromFixture(): Uint8Array {
  return bytes32(readFixture().validVector.canonicalEventKey, "canonicalEventKey");
}

describe("Stage 1 processed burn registry model", () => {
  it("creates an empty processed burn registry", () => {
    const registry = createStage1ProcessedBurnRegistry();

    expect(registry.processedCanonicalEventKeys.size).toBe(0);
  });

  it("checks an unprocessed canonicalEventKey without mutating state", () => {
    const registry = createStage1ProcessedBurnRegistry();
    const canonicalEventKey = canonicalEventKeyFromFixture();

    const result = checkStage1BurnNotProcessed(registry, canonicalEventKey);

    expect(result).toEqual({
      ok: true,
      canonicalEventKeyHex: stage1CanonicalEventKeyHex(canonicalEventKey),
      errors: [],
    });
    expect(registry.processedCanonicalEventKeys.size).toBe(0);
  });

  it("marks an unprocessed canonicalEventKey as processed", () => {
    const registry = createStage1ProcessedBurnRegistry();
    const canonicalEventKey = canonicalEventKeyFromFixture();

    const result = markStage1BurnProcessed(registry, canonicalEventKey);

    expect(result).toEqual({
      ok: true,
      canonicalEventKeyHex: stage1CanonicalEventKeyHex(canonicalEventKey),
      errors: [],
      marked: true,
    });
    expect(
      registry.processedCanonicalEventKeys.has(
        stage1CanonicalEventKeyHex(canonicalEventKey),
      ),
    ).toBe(true);
  });

  it("rejects duplicate canonicalEventKey processing", () => {
    const canonicalEventKey = canonicalEventKeyFromFixture();
    const registry = createStage1ProcessedBurnRegistry([
      stage1CanonicalEventKeyHex(canonicalEventKey),
    ]);

    const checkResult = checkStage1BurnNotProcessed(registry, canonicalEventKey);
    const markResult = markStage1BurnProcessed(registry, canonicalEventKey);

    expect(checkResult).toEqual({
      ok: false,
      canonicalEventKeyHex: stage1CanonicalEventKeyHex(canonicalEventKey),
      errors: [STAGE1_PROCESSED_BURN_REGISTRY_ERROR.AlreadyProcessed],
    });
    expect(markResult).toEqual({
      ...checkResult,
      marked: false,
    });
    expect(registry.processedCanonicalEventKeys.size).toBe(1);
  });

  it("normalizes preloaded canonicalEventKey hex values to lowercase", () => {
    const fixture = readFixture();
    const registry = createStage1ProcessedBurnRegistry([
      fixture.validVector.canonicalEventKey.toUpperCase(),
    ]);
    const canonicalEventKey = canonicalEventKeyFromFixture();

    expect(checkStage1BurnNotProcessed(registry, canonicalEventKey)).toEqual({
      ok: false,
      canonicalEventKeyHex: stage1CanonicalEventKeyHex(canonicalEventKey),
      errors: [STAGE1_PROCESSED_BURN_REGISTRY_ERROR.AlreadyProcessed],
    });
  });

  it("tracks different canonicalEventKeys independently", () => {
    const registry = createStage1ProcessedBurnRegistry();
    const first = canonicalEventKeyFromFixture();
    const second = new Uint8Array(first);
    second[31]! ^= 1;

    expect(markStage1BurnProcessed(registry, first).marked).toBe(true);
    expect(checkStage1BurnNotProcessed(registry, first).ok).toBe(false);
    expect(checkStage1BurnNotProcessed(registry, second).ok).toBe(true);
    expect(markStage1BurnProcessed(registry, second).marked).toBe(true);
    expect(registry.processedCanonicalEventKeys.size).toBe(2);
  });
});

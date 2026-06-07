import { bytesToHex } from "./stage-1-encoding.js";

export const STAGE1_PROCESSED_BURN_REGISTRY_ERROR = {
  AlreadyProcessed: "ALREADY_PROCESSED",
} as const;

export type Stage1ProcessedBurnRegistryErrorCode =
  (typeof STAGE1_PROCESSED_BURN_REGISTRY_ERROR)[keyof typeof STAGE1_PROCESSED_BURN_REGISTRY_ERROR];

export type Stage1ProcessedBurnRegistry = {
  processedCanonicalEventKeys: Set<string>;
};

export type Stage1ProcessedBurnRegistryCheckResult = {
  ok: boolean;
  canonicalEventKeyHex: string;
  errors: Stage1ProcessedBurnRegistryErrorCode[];
};

export type Stage1ProcessedBurnRegistryMarkResult =
  Stage1ProcessedBurnRegistryCheckResult & {
    marked: boolean;
  };

export function createStage1ProcessedBurnRegistry(
  processedCanonicalEventKeyHexes: string[] = [],
): Stage1ProcessedBurnRegistry {
  return {
    processedCanonicalEventKeys: new Set(
      processedCanonicalEventKeyHexes.map((canonicalEventKeyHex) =>
        canonicalEventKeyHex.toLowerCase(),
      ),
    ),
  };
}

export function stage1CanonicalEventKeyHex(
  canonicalEventKey: Uint8Array,
): string {
  return bytesToHex(canonicalEventKey).toLowerCase();
}

export function checkStage1BurnNotProcessed(
  registry: Stage1ProcessedBurnRegistry,
  canonicalEventKey: Uint8Array,
): Stage1ProcessedBurnRegistryCheckResult {
  const canonicalEventKeyHex = stage1CanonicalEventKeyHex(canonicalEventKey);

  if (registry.processedCanonicalEventKeys.has(canonicalEventKeyHex)) {
    return {
      ok: false,
      canonicalEventKeyHex,
      errors: [STAGE1_PROCESSED_BURN_REGISTRY_ERROR.AlreadyProcessed],
    };
  }

  return {
    ok: true,
    canonicalEventKeyHex,
    errors: [],
  };
}

export function markStage1BurnProcessed(
  registry: Stage1ProcessedBurnRegistry,
  canonicalEventKey: Uint8Array,
): Stage1ProcessedBurnRegistryMarkResult {
  const check = checkStage1BurnNotProcessed(registry, canonicalEventKey);

  if (!check.ok) {
    return {
      ...check,
      marked: false,
    };
  }

  registry.processedCanonicalEventKeys.add(check.canonicalEventKeyHex);

  return {
    ...check,
    marked: true,
  };
}

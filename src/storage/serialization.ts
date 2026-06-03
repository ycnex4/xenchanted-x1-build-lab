import { type BuildRegistry } from "../model/build-registry.js";
import { type BuildState } from "../model/build-state.js";
import { type RedeemEventState } from "../model/redeem-events.js";
import { type RegistrarState } from "../model/registrar.js";
import { type XenBurnEventState } from "../model/xen-burn-events.js";

export const STORAGE_SCHEMA_VERSION = 1 as const;

export interface SerializedBuildState {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "BuildState";
  owner: string;
  buildId: string;
  version: number;
  createdAt: string;
  updatedAt: string;
  ethereumIdentity: string | null;
  historyBld: string;
  availableBld: string;
  originBld: string;
  earnedXbp: string;
  availableXbp: string;
  lockedXntd: string;
  requiredXntdLock: string;
  lockEpoch: number | null;
  xcCommitmentActive: boolean;
  x1FeeContribution: string;
  x1TxCount: string;
  x1FeeCountedUntilSlot: string | null;
  lastFeeUpdateAt: string | null;
}

export interface SerializedRegistrarState {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "RegistrarState";
  registrarAuthority: string;
  processedMessages: string[];
}

export interface SerializedRedeemEventState {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "RedeemEventState";
  usedRedeemEvents: string[];
}

export interface SerializedXenBurnEventState {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "XenBurnEventState";
  usedXenBurnEvents: string[];
}

export interface SerializedBuildRegistry {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "BuildRegistry";
  builds: SerializedBuildState[];
  canonicalBuildByOwner: Array<[string, string]>;
  canonicalBuildByEthereumIdentity: Array<[string, string]>;
}

function requireRecord(value: unknown, field: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`${field} must be an object`);
  }

  return value as Record<string, unknown>;
}

function requireKind(
  value: Record<string, unknown>,
  expectedKind: string
): void {
  if (value.schemaVersion !== STORAGE_SCHEMA_VERSION) {
    throw new Error(`Unsupported schema version for ${expectedKind}`);
  }

  if (value.kind !== expectedKind) {
    throw new Error(`Invalid serialized kind for ${expectedKind}`);
  }
}

function requireString(value: unknown, field: string): string {
  if (typeof value !== "string") {
    throw new Error(`${field} must be a string`);
  }

  return value;
}

function requireNullableString(value: unknown, field: string): string | null {
  if (value === null) {
    return null;
  }

  return requireString(value, field);
}

function requireNumber(value: unknown, field: string): number {
  if (typeof value !== "number" || !Number.isInteger(value)) {
    throw new Error(`${field} must be an integer number`);
  }

  return value;
}

function requireNullableNumber(value: unknown, field: string): number | null {
  if (value === null) {
    return null;
  }

  return requireNumber(value, field);
}

function requireBoolean(value: unknown, field: string): boolean {
  if (typeof value !== "boolean") {
    throw new Error(`${field} must be a boolean`);
  }

  return value;
}

function serializeBigint(value: bigint): string {
  return value.toString(10);
}

function deserializeBigint(value: unknown, field: string): bigint {
  const text = requireString(value, field);

  if (!/^(0|[1-9][0-9]*)$/.test(text)) {
    throw new Error(`${field} must be a non-negative decimal bigint string`);
  }

  return BigInt(text);
}

function deserializeNullableBigint(
  value: unknown,
  field: string
): bigint | null {
  if (value === null) {
    return null;
  }

  return deserializeBigint(value, field);
}

function serializeStringSet(value: Set<string>): string[] {
  return [...value].sort((a, b) => a.localeCompare(b));
}

function deserializeStringSet(value: unknown, field: string): Set<string> {
  if (!Array.isArray(value)) {
    throw new Error(`${field} must be an array`);
  }

  const result = new Set<string>();

  for (const item of value) {
    const text = requireString(item, `${field} item`);

    if (result.has(text)) {
      throw new Error(`${field} contains duplicate value: ${text}`);
    }

    result.add(text);
  }

  return result;
}

function serializeStringMap(value: Map<string, string>): Array<[string, string]> {
  return [...value.entries()].sort(([a], [b]) => a.localeCompare(b));
}

function deserializeStringMap(
  value: unknown,
  field: string
): Map<string, string> {
  if (!Array.isArray(value)) {
    throw new Error(`${field} must be an array`);
  }

  const result = new Map<string, string>();

  for (const entry of value) {
    if (!Array.isArray(entry) || entry.length !== 2) {
      throw new Error(`${field} entries must be [key, value] pairs`);
    }

    const key = requireString(entry[0], `${field} key`);
    const mapValue = requireString(entry[1], `${field} value`);

    if (result.has(key)) {
      throw new Error(`${field} contains duplicate key: ${key}`);
    }

    result.set(key, mapValue);
  }

  return result;
}

export function serializeBuildState(
  build: BuildState
): SerializedBuildState {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "BuildState",
    owner: build.owner,
    buildId: build.buildId,
    version: build.version,
    createdAt: serializeBigint(build.createdAt),
    updatedAt: serializeBigint(build.updatedAt),
    ethereumIdentity: build.ethereumIdentity,
    historyBld: serializeBigint(build.historyBld),
    availableBld: serializeBigint(build.availableBld),
    originBld: serializeBigint(build.originBld),
    earnedXbp: serializeBigint(build.earnedXbp),
    availableXbp: serializeBigint(build.availableXbp),
    lockedXntd: serializeBigint(build.lockedXntd),
    requiredXntdLock: serializeBigint(build.requiredXntdLock),
    lockEpoch: build.lockEpoch,
    xcCommitmentActive: build.xcCommitmentActive,
    x1FeeContribution: serializeBigint(build.x1FeeContribution),
    x1TxCount: serializeBigint(build.x1TxCount),
    x1FeeCountedUntilSlot:
      build.x1FeeCountedUntilSlot === null
        ? null
        : serializeBigint(build.x1FeeCountedUntilSlot),
    lastFeeUpdateAt:
      build.lastFeeUpdateAt === null
        ? null
        : serializeBigint(build.lastFeeUpdateAt)
  };
}

export function deserializeBuildState(input: unknown): BuildState {
  const value = requireRecord(input, "BuildState");
  requireKind(value, "BuildState");

  return {
    owner: requireString(value.owner, "owner"),
    buildId: requireString(value.buildId, "buildId"),
    version: requireNumber(value.version, "version"),
    createdAt: deserializeBigint(value.createdAt, "createdAt"),
    updatedAt: deserializeBigint(value.updatedAt, "updatedAt"),
    ethereumIdentity: requireNullableString(
      value.ethereumIdentity,
      "ethereumIdentity"
    ),
    historyBld: deserializeBigint(value.historyBld, "historyBld"),
    availableBld: deserializeBigint(value.availableBld, "availableBld"),
    originBld: deserializeBigint(value.originBld, "originBld"),
    earnedXbp: deserializeBigint(value.earnedXbp, "earnedXbp"),
    availableXbp: deserializeBigint(value.availableXbp, "availableXbp"),
    lockedXntd: deserializeBigint(value.lockedXntd, "lockedXntd"),
    requiredXntdLock: deserializeBigint(
      value.requiredXntdLock,
      "requiredXntdLock"
    ),
    lockEpoch: requireNullableNumber(value.lockEpoch, "lockEpoch"),
    xcCommitmentActive: requireBoolean(
      value.xcCommitmentActive,
      "xcCommitmentActive"
    ),
    x1FeeContribution: deserializeBigint(
      value.x1FeeContribution,
      "x1FeeContribution"
    ),
    x1TxCount: deserializeBigint(value.x1TxCount, "x1TxCount"),
    x1FeeCountedUntilSlot: deserializeNullableBigint(
      value.x1FeeCountedUntilSlot,
      "x1FeeCountedUntilSlot"
    ),
    lastFeeUpdateAt: deserializeNullableBigint(
      value.lastFeeUpdateAt,
      "lastFeeUpdateAt"
    )
  };
}

export function serializeRegistrarState(
  state: RegistrarState
): SerializedRegistrarState {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "RegistrarState",
    registrarAuthority: state.registrarAuthority,
    processedMessages: serializeStringSet(state.processedMessages)
  };
}

export function deserializeRegistrarState(input: unknown): RegistrarState {
  const value = requireRecord(input, "RegistrarState");
  requireKind(value, "RegistrarState");

  return {
    registrarAuthority: requireString(
      value.registrarAuthority,
      "registrarAuthority"
    ),
    processedMessages: deserializeStringSet(
      value.processedMessages,
      "processedMessages"
    )
  };
}

export function serializeRedeemEventState(
  state: RedeemEventState
): SerializedRedeemEventState {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "RedeemEventState",
    usedRedeemEvents: serializeStringSet(state.usedRedeemEvents)
  };
}

export function deserializeRedeemEventState(
  input: unknown
): RedeemEventState {
  const value = requireRecord(input, "RedeemEventState");
  requireKind(value, "RedeemEventState");

  return {
    usedRedeemEvents: deserializeStringSet(
      value.usedRedeemEvents,
      "usedRedeemEvents"
    )
  };
}

export function serializeXenBurnEventState(
  state: XenBurnEventState
): SerializedXenBurnEventState {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "XenBurnEventState",
    usedXenBurnEvents: serializeStringSet(state.usedXenBurnEvents)
  };
}

export function deserializeXenBurnEventState(
  input: unknown
): XenBurnEventState {
  const value = requireRecord(input, "XenBurnEventState");
  requireKind(value, "XenBurnEventState");

  return {
    usedXenBurnEvents: deserializeStringSet(
      value.usedXenBurnEvents,
      "usedXenBurnEvents"
    )
  };
}

export function serializeBuildRegistry(
  registry: BuildRegistry
): SerializedBuildRegistry {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "BuildRegistry",
    builds: [...registry.buildsById.values()]
      .sort((a, b) => a.buildId.localeCompare(b.buildId))
      .map(serializeBuildState),
    canonicalBuildByOwner: serializeStringMap(registry.canonicalBuildByOwner),
    canonicalBuildByEthereumIdentity: serializeStringMap(
      registry.canonicalBuildByEthereumIdentity
    )
  };
}

export function deserializeBuildRegistry(input: unknown): BuildRegistry {
  const value = requireRecord(input, "BuildRegistry");
  requireKind(value, "BuildRegistry");

  if (!Array.isArray(value.builds)) {
    throw new Error("builds must be an array");
  }

  const buildsById = new Map<string, BuildState>();

  for (const item of value.builds) {
    const build = deserializeBuildState(item);

    if (buildsById.has(build.buildId)) {
      throw new Error(`builds contains duplicate buildId: ${build.buildId}`);
    }

    buildsById.set(build.buildId, build);
  }

  return {
    buildsById,
    canonicalBuildByOwner: deserializeStringMap(
      value.canonicalBuildByOwner,
      "canonicalBuildByOwner"
    ),
    canonicalBuildByEthereumIdentity: deserializeStringMap(
      value.canonicalBuildByEthereumIdentity,
      "canonicalBuildByEthereumIdentity"
    )
  };
}

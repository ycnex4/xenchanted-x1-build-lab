import { access, copyFile, mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { dirname } from "node:path";
import {
  type BuildApplicationState,
  createBuildApplicationState
} from "../app/build-service.js";
import {
  STORAGE_SCHEMA_VERSION,
  type SerializedBuildRegistry,
  type SerializedRedeemEventState,
  type SerializedRegistrarState,
  type SerializedXenBurnEventState,
  deserializeBuildRegistry,
  deserializeRedeemEventState,
  deserializeRegistrarState,
  deserializeXenBurnEventState,
  serializeBuildRegistry,
  serializeRedeemEventState,
  serializeRegistrarState,
  serializeXenBurnEventState
} from "./serialization.js";

export interface SerializedBuildApplicationSnapshot {
  schemaVersion: typeof STORAGE_SCHEMA_VERSION;
  kind: "BuildApplicationSnapshot";
  createdAt: string;
  registry: SerializedBuildRegistry;
  registrar: SerializedRegistrarState;
  redeemEvents: SerializedRedeemEventState;
  xenBurnEvents: SerializedXenBurnEventState;
}

function serializeBigint(value: bigint): string {
  return value.toString(10);
}

function deserializeBigint(value: unknown, field: string): bigint {
  if (typeof value !== "string") {
    throw new Error(`${field} must be a string`);
  }

  if (!/^(0|[1-9][0-9]*)$/.test(value)) {
    throw new Error(`${field} must be a non-negative decimal bigint string`);
  }

  return BigInt(value);
}

function requireRecord(value: unknown, field: string): Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`${field} must be an object`);
  }

  return value as Record<string, unknown>;
}

function isNodeError(error: unknown): error is NodeJS.ErrnoException {
  return typeof error === "object" && error !== null && "code" in error;
}

async function fileExists(filePath: string): Promise<boolean> {
  try {
    await access(filePath);
    return true;
  } catch (error) {
    if (isNodeError(error) && error.code === "ENOENT") {
      return false;
    }

    throw error;
  }
}

export function serializeBuildApplicationSnapshot(
  app: BuildApplicationState,
  createdAt: bigint
): SerializedBuildApplicationSnapshot {
  return {
    schemaVersion: STORAGE_SCHEMA_VERSION,
    kind: "BuildApplicationSnapshot",
    createdAt: serializeBigint(createdAt),
    registry: serializeBuildRegistry(app.registry),
    registrar: serializeRegistrarState(app.registrar),
    redeemEvents: serializeRedeemEventState(app.redeemEvents),
    xenBurnEvents: serializeXenBurnEventState(app.xenBurnEvents)
  };
}

export function deserializeBuildApplicationSnapshot(
  input: unknown
): {
  app: BuildApplicationState;
  createdAt: bigint;
} {
  const value = requireRecord(input, "BuildApplicationSnapshot");

  if (value.schemaVersion !== STORAGE_SCHEMA_VERSION) {
    throw new Error("Unsupported schema version for BuildApplicationSnapshot");
  }

  if (value.kind !== "BuildApplicationSnapshot") {
    throw new Error("Invalid serialized kind for BuildApplicationSnapshot");
  }

  const registrar = deserializeRegistrarState(value.registrar);
  const app = createBuildApplicationState(registrar.registrarAuthority);

  app.registry = deserializeBuildRegistry(value.registry);
  app.registrar = registrar;
  app.redeemEvents = deserializeRedeemEventState(value.redeemEvents);
  app.xenBurnEvents = deserializeXenBurnEventState(value.xenBurnEvents);

  return {
    app,
    createdAt: deserializeBigint(value.createdAt, "createdAt")
  };
}

export function encodeSnapshotJson(
  snapshot: SerializedBuildApplicationSnapshot
): string {
  return `${JSON.stringify(snapshot, null, 2)}\n`;
}

export function decodeSnapshotJson(
  json: string
): {
  app: BuildApplicationState;
  createdAt: bigint;
} {
  return deserializeBuildApplicationSnapshot(JSON.parse(json));
}

export function verifySnapshotJson(
  json: string
): {
  app: BuildApplicationState;
  createdAt: bigint;
} {
  return decodeSnapshotJson(json);
}

export async function verifySnapshotFile(
  filePath: string
): Promise<{
  app: BuildApplicationState;
  createdAt: bigint;
}> {
  const json = await readFile(filePath, "utf8");

  return verifySnapshotJson(json);
}

export interface SaveSnapshotFileWithBackupOptions {
  backupPath?: string;
}

export async function saveSnapshotFile(
  filePath: string,
  app: BuildApplicationState,
  createdAt: bigint
): Promise<void> {
  const snapshot = serializeBuildApplicationSnapshot(app, createdAt);
  const json = encodeSnapshotJson(snapshot);
  const tempPath = `${filePath}.tmp-${process.pid}-${Date.now()}`;

  await mkdir(dirname(filePath), { recursive: true });
  await writeFile(tempPath, json, "utf8");
  await rename(tempPath, filePath);
}

export async function loadSnapshotFile(
  filePath: string
): Promise<{
  app: BuildApplicationState;
  createdAt: bigint;
}> {
  const json = await readFile(filePath, "utf8");

  return decodeSnapshotJson(json);
}

export async function saveSnapshotFileWithBackup(
  filePath: string,
  app: BuildApplicationState,
  createdAt: bigint,
  options: SaveSnapshotFileWithBackupOptions = {}
): Promise<void> {
  const snapshot = serializeBuildApplicationSnapshot(app, createdAt);
  const json = encodeSnapshotJson(snapshot);
  const tempPath = `${filePath}.tmp-${process.pid}-${Date.now()}`;

  try {
    await mkdir(dirname(filePath), { recursive: true });
    await writeFile(tempPath, json, "utf8");
    await verifySnapshotFile(tempPath);

    if (await fileExists(filePath)) {
      await verifySnapshotFile(filePath);

      const backupPath = options.backupPath ?? `${filePath}.bak`;

      await mkdir(dirname(backupPath), { recursive: true });
      await copyFile(filePath, backupPath);
    }

    await rename(tempPath, filePath);
    await verifySnapshotFile(filePath);
  } catch (error) {
    await rm(tempPath, { force: true });
    throw error;
  }
}

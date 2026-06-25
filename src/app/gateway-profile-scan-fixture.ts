import { readFile } from "node:fs/promises";
import type {
  BuildId,
  EthereumAddress,
  X1Address,
} from "../model/build-state.js";
import {
  createCoreRedeemCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
} from "../watchers/watcher-candidates.js";
import {
  createStaticGatewayProfileScanner,
  type GatewayProfileScanner,
} from "./gateway-profile-scan.js";

type JsonRecord = Record<string, unknown>;

export interface GatewayProfilePreviewFixture {
  readonly buildId: BuildId;
  readonly owner: X1Address;
  readonly ethereumIdentity: EthereumAddress;
  readonly validatedAt: bigint;
  readonly buildExists: boolean;
  readonly existingBuildCreatedAt: bigint;
  readonly scanner: GatewayProfileScanner;
}

function asRecord(value: unknown, context: string): JsonRecord {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(`Fixture field ${context} must be an object`);
  }

  return value as JsonRecord;
}

function readRequiredString(
  record: JsonRecord,
  name: string,
  context: string,
): string {
  const value = record[name];

  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Fixture field ${context}.${name} must be a string`);
  }

  return value;
}

function readOptionalString(
  record: JsonRecord,
  name: string,
  fallback: string,
  context: string,
): string {
  const value = record[name];

  if (value === undefined) {
    return fallback;
  }

  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`Fixture field ${context}.${name} must be a string`);
  }

  return value;
}

function readOptionalBoolean(
  record: JsonRecord,
  name: string,
  fallback: boolean,
  context: string,
): boolean {
  const value = record[name];

  if (value === undefined) {
    return fallback;
  }

  if (typeof value !== "boolean") {
    throw new Error(`Fixture field ${context}.${name} must be a boolean`);
  }

  return value;
}

function readRequiredDecimalBigInt(
  record: JsonRecord,
  name: string,
  context: string,
): bigint {
  const value = record[name];

  if (typeof value !== "string" || !/^\d+$/.test(value)) {
    throw new Error(
      `Fixture field ${context}.${name} must be a decimal string`,
    );
  }

  return BigInt(value);
}

function readOptionalDecimalBigInt(
  record: JsonRecord,
  name: string,
  fallback: bigint,
  context: string,
): bigint {
  const value = record[name];

  if (value === undefined) {
    return fallback;
  }

  if (typeof value !== "string" || !/^\d+$/.test(value)) {
    throw new Error(
      `Fixture field ${context}.${name} must be a decimal string`,
    );
  }

  return BigInt(value);
}

function readOptionalNonNegativeNumber(
  record: JsonRecord,
  name: string,
  fallback: number,
  context: string,
): number {
  const value = record[name];

  if (value === undefined) {
    return fallback;
  }

  if (typeof value !== "number" || !Number.isInteger(value) || value < 0) {
    throw new Error(
      `Fixture field ${context}.${name} must be a non-negative integer`,
    );
  }

  return value;
}

function readOptionalArray(
  record: JsonRecord,
  name: string,
  context: string,
): unknown[] {
  const value = record[name];

  if (value === undefined) {
    return [];
  }

  if (!Array.isArray(value)) {
    throw new Error(`Fixture field ${context}.${name} must be an array`);
  }

  return value;
}

function parseCoreRedeemCandidates(
  fixture: JsonRecord,
  buildId: BuildId,
  owner: X1Address,
  scannedAt: bigint,
) {
  return readOptionalArray(fixture, "coreRedeemCandidates", "fixture").map(
    (value, index) => {
      const context = `coreRedeemCandidates[${index}]`;
      const record = asRecord(value, context);
      const observedAt = readOptionalDecimalBigInt(
        record,
        "observedAt",
        scannedAt,
        context,
      );

      return createCoreRedeemCandidate({
        sourceChainId: readOptionalString(
          record,
          "sourceChainId",
          "eip155-1",
          context,
        ),
        sourceAddress: readOptionalString(
          record,
          "sourceAddress",
          "fixture-core",
          context,
        ),
        eventKind: "CORE_REDEEM",
        transactionHash: readOptionalString(
          record,
          "transactionHash",
          `fixture-core-redeem-${index}`,
          context,
        ),
        eventIndex: readOptionalNonNegativeNumber(
          record,
          "eventIndex",
          index,
          context,
        ),
        observedAt,
        finalized: readOptionalBoolean(record, "finalized", true, context),
        buildId,
        owner,
        amountBld: readRequiredDecimalBigInt(record, "amountBld", context),
        redeemedAt: readOptionalDecimalBigInt(
          record,
          "redeemedAt",
          observedAt,
          context,
        ),
        coreTokenId: readOptionalString(
          record,
          "coreTokenId",
          `fixture-core-token-${index}`,
          context,
        ),
      });
    },
  );
}

function parseXenBurnCandidates(
  fixture: JsonRecord,
  buildId: BuildId,
  owner: X1Address,
  scannedAt: bigint,
) {
  return readOptionalArray(fixture, "xenBurnCandidates", "fixture").map(
    (value, index) => {
      const context = `xenBurnCandidates[${index}]`;
      const record = asRecord(value, context);
      const observedAt = readOptionalDecimalBigInt(
        record,
        "observedAt",
        scannedAt,
        context,
      );
      const amountXbp = readRequiredDecimalBigInt(record, "amountXbp", context);

      return createXenBurnCandidate({
        sourceChainId: readOptionalString(
          record,
          "sourceChainId",
          "eip155-1",
          context,
        ),
        sourceAddress: readOptionalString(
          record,
          "sourceAddress",
          "fixture-xen",
          context,
        ),
        eventKind: "XEN_BURN",
        transactionHash: readOptionalString(
          record,
          "transactionHash",
          `fixture-xen-burn-${index}`,
          context,
        ),
        eventIndex: readOptionalNonNegativeNumber(
          record,
          "eventIndex",
          index,
          context,
        ),
        observedAt,
        finalized: readOptionalBoolean(record, "finalized", true, context),
        buildId,
        owner,
        amountXbp,
        burnedAt: readOptionalDecimalBigInt(
          record,
          "burnedAt",
          observedAt,
          context,
        ),
        xenAmountBurned: readOptionalDecimalBigInt(
          record,
          "xenAmountBurned",
          amountXbp,
          context,
        ),
      });
    },
  );
}

function parseXntdLockCandidate(
  fixture: JsonRecord,
  buildId: BuildId,
  owner: X1Address,
  scannedAt: bigint,
) {
  const value = fixture.xntdLockCandidate;

  if (value === undefined || value === null) {
    return null;
  }

  const context = "xntdLockCandidate";
  const record = asRecord(value, context);
  const observedAt = readOptionalDecimalBigInt(
    record,
    "observedAt",
    scannedAt,
    context,
  );
  const amountXntd = readRequiredDecimalBigInt(record, "amountXntd", context);

  return createXntdLockCandidate({
    sourceChainId: readOptionalString(
      record,
      "sourceChainId",
      "eip155-1",
      context,
    ),
    sourceAddress: readOptionalString(
      record,
      "sourceAddress",
      "fixture-xntd-lock",
      context,
    ),
    eventKind: "XNTD_LOCK",
    transactionHash: readOptionalString(
      record,
      "transactionHash",
      "fixture-xntd-lock",
      context,
    ),
    eventIndex: readOptionalNonNegativeNumber(record, "eventIndex", 0, context),
    observedAt,
    finalized: readOptionalBoolean(record, "finalized", true, context),
    buildId,
    owner,
    amountXntd,
    observedRequiredXntdLock: readOptionalDecimalBigInt(
      record,
      "observedRequiredXntdLock",
      amountXntd,
      context,
    ),
    lockEpoch: readOptionalNonNegativeNumber(record, "lockEpoch", 0, context),
    lockedAt: readOptionalDecimalBigInt(
      record,
      "lockedAt",
      observedAt,
      context,
    ),
  });
}

export async function loadGatewayProfilePreviewFixtureFile(
  filePath: string,
): Promise<GatewayProfilePreviewFixture> {
  const raw = await readFile(filePath, "utf8");
  const parsed = JSON.parse(raw) as unknown;
  const fixture = asRecord(parsed, "fixture");

  const buildId = readRequiredString(fixture, "buildId", "fixture") as BuildId;
  const owner = readRequiredString(fixture, "owner", "fixture") as X1Address;
  const ethereumIdentity = readRequiredString(
    fixture,
    "ethereumIdentity",
    "fixture",
  ) as EthereumAddress;
  const scannedAt = readOptionalDecimalBigInt(
    fixture,
    "scannedAt",
    1000n,
    "fixture",
  );
  const validatedAt = readOptionalDecimalBigInt(
    fixture,
    "validatedAt",
    scannedAt,
    "fixture",
  );

  const buildExists = readOptionalBoolean(
    fixture,
    "buildExists",
    false,
    "fixture",
  );
  const existingBuildCreatedAt = readOptionalDecimalBigInt(
    fixture,
    "existingBuildCreatedAt",
    validatedAt,
    "fixture",
  );

  const scanner = createStaticGatewayProfileScanner({
    coreRedeemScanCompleted: readOptionalBoolean(
      fixture,
      "coreRedeemScanCompleted",
      true,
      "fixture",
    ),
    xenBurnScanCompleted: readOptionalBoolean(
      fixture,
      "xenBurnScanCompleted",
      true,
      "fixture",
    ),
    xntdLockScanCompleted: readOptionalBoolean(
      fixture,
      "xntdLockScanCompleted",
      true,
      "fixture",
    ),
    coreRedeemCandidates: parseCoreRedeemCandidates(
      fixture,
      buildId,
      owner,
      scannedAt,
    ),
    xenBurnCandidates: parseXenBurnCandidates(
      fixture,
      buildId,
      owner,
      scannedAt,
    ),
    xntdLockCandidate: parseXntdLockCandidate(
      fixture,
      buildId,
      owner,
      scannedAt,
    ),
    scannedAt,
  });

  return {
    buildId,
    owner,
    ethereumIdentity,
    validatedAt,
    buildExists,
    existingBuildCreatedAt,
    scanner,
  };
}

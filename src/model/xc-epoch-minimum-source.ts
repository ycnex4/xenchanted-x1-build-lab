import { BuildError, BuildErrorCode } from "../errors/build-error.js";

export interface XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

export interface XcEpochMinimumRecord {
  lockEpoch: number;
  minimumXntd: bigint;
  observedAt: bigint;
  sourceChainId?: string;
  sourceBlockNumber?: bigint;
  sourceBlockHash?: string;
}

function assertValidXcEpochMinimumRecord(record: XcEpochMinimumRecord): void {
  if (!Number.isInteger(record.lockEpoch) || record.lockEpoch < 0) {
    throw new BuildError(
      BuildErrorCode.InvalidXcEpochMinimumRecord,
      `Invalid XC epoch minimum record epoch: lockEpoch=${record.lockEpoch.toString()}`
    );
  }

  if (record.minimumXntd <= 0n) {
    throw new BuildError(
      BuildErrorCode.InvalidXcEpochMinimumRecord,
      `Invalid XC epoch minimum record amount: lockEpoch=${record.lockEpoch.toString()}, minimumXntd=${record.minimumXntd.toString()}`
    );
  }
}

export function createXcEpochMinimumSourceFromRecords(
  records: readonly XcEpochMinimumRecord[]
): XcEpochMinimumSource {
  const epochMinimums = new Map<number, bigint>();

  for (const record of records) {
    assertValidXcEpochMinimumRecord(record);

    const existingMinimum = epochMinimums.get(record.lockEpoch);

    if (
      existingMinimum !== undefined &&
      existingMinimum !== record.minimumXntd
    ) {
      throw new BuildError(
        BuildErrorCode.InvalidXcEpochMinimumRecord,
        `Conflicting XC epoch minimum records: lockEpoch=${record.lockEpoch.toString()}, existing=${existingMinimum.toString()}, next=${record.minimumXntd.toString()}`
      );
    }

    epochMinimums.set(record.lockEpoch, record.minimumXntd);
  }

  return createStaticXcEpochMinimumSource(epochMinimums);
}

export function createStaticXcEpochMinimumSource(
  epochMinimums: ReadonlyMap<number, bigint>
): XcEpochMinimumSource {
  return {
    authoritativeEpochMinimum(lockEpoch: number): bigint | null {
      return epochMinimums.get(lockEpoch) ?? null;
    }
  };
}

export function assertAuthoritativeXcEpochMinimum(
  source: XcEpochMinimumSource,
  lockEpoch: number,
  observedRequiredXntdLock: bigint
): void {
  const authoritativeMinimum = source.authoritativeEpochMinimum(lockEpoch);

  if (authoritativeMinimum === null) {
    throw new BuildError(
      BuildErrorCode.MissingAuthoritativeXcEpochMinimum,
      `Missing authoritative XC epoch minimum for lockEpoch=${lockEpoch.toString()}`
    );
  }

  if (observedRequiredXntdLock !== authoritativeMinimum) {
    throw new BuildError(
      BuildErrorCode.MismatchedAuthoritativeXcEpochMinimum,
      `Observed required XNTD lock does not match authoritative XC epoch minimum: lockEpoch=${lockEpoch.toString()}, observed=${observedRequiredXntdLock.toString()}, authoritative=${authoritativeMinimum.toString()}`
    );
  }
}

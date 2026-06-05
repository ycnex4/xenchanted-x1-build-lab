import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import {
  type XcEpochMinimumRecord,
  type XcEpochMinimumSource,
  createXcEpochMinimumSourceFromRecords
} from "./xc-epoch-minimum-source.js";

export interface EthereumXcLensEpochMinimumSnapshot {
  sourceChainId: string;
  sourceBlockNumber: bigint;
  sourceBlockHash: string;
  observedAt: bigint;
  finalizedPolicy: EthereumFinalityPolicy;
  epochMinimums: readonly EthereumXcEpochMinimumEntry[];
}

export interface EthereumXcEpochMinimumEntry {
  lockEpoch: number;
  minimumXntd: bigint;
}

export type EthereumFinalityPolicy =
  | { kind: "finalized" }
  | { kind: "safe" }
  | { kind: "confirmed"; confirmations: number };

function throwInvalidEthereumSnapshot(message: string): never {
  throw new BuildError(BuildErrorCode.InvalidXcEpochMinimumRecord, message);
}

function normalizeEthereumBlockHash(sourceBlockHash: string): string {
  if (!/^0x[0-9a-fA-F]{64}$/.test(sourceBlockHash)) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot sourceBlockHash: sourceBlockHash=${sourceBlockHash}`
    );
  }

  return sourceBlockHash.toLowerCase();
}

function assertValidEthereumFinalityPolicy(
  finalizedPolicy: EthereumFinalityPolicy
): void {
  const policy = finalizedPolicy as { kind?: unknown; confirmations?: unknown };

  if (
    policy.kind !== "finalized" &&
    policy.kind !== "safe" &&
    policy.kind !== "confirmed"
  ) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot finality policy: kind=${String(
        policy.kind
      )}`
    );
  }

  if (
    policy.kind === "confirmed" &&
    (!Number.isInteger(policy.confirmations) ||
      Number(policy.confirmations) <= 0)
  ) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot confirmations: confirmations=${String(
        policy.confirmations
      )}`
    );
  }
}

function assertValidEthereumSnapshotMetadata(
  snapshot: EthereumXcLensEpochMinimumSnapshot
): string {
  if (!/^eip155-\d+$/.test(snapshot.sourceChainId)) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot sourceChainId: sourceChainId=${snapshot.sourceChainId}`
    );
  }

  if (snapshot.sourceBlockNumber <= 0n) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot sourceBlockNumber: sourceBlockNumber=${snapshot.sourceBlockNumber.toString()}`
    );
  }

  const normalizedSourceBlockHash = normalizeEthereumBlockHash(
    snapshot.sourceBlockHash
  );

  if (snapshot.observedAt <= 0n) {
    throwInvalidEthereumSnapshot(
      `Invalid Ethereum XC epoch minimum snapshot observedAt: observedAt=${snapshot.observedAt.toString()}`
    );
  }

  assertValidEthereumFinalityPolicy(snapshot.finalizedPolicy);

  if (snapshot.epochMinimums.length === 0) {
    throwInvalidEthereumSnapshot(
      "Invalid Ethereum XC epoch minimum snapshot: epochMinimums is empty"
    );
  }

  return normalizedSourceBlockHash;
}

export function createXcEpochMinimumSourceFromEthereumLensSnapshot(
  snapshot: EthereumXcLensEpochMinimumSnapshot
): XcEpochMinimumSource {
  const normalizedSourceBlockHash = assertValidEthereumSnapshotMetadata(snapshot);

  const records: XcEpochMinimumRecord[] = snapshot.epochMinimums.map((entry) => ({
    lockEpoch: entry.lockEpoch,
    minimumXntd: entry.minimumXntd,
    observedAt: snapshot.observedAt,
    sourceChainId: snapshot.sourceChainId,
    sourceBlockNumber: snapshot.sourceBlockNumber,
    sourceBlockHash: normalizedSourceBlockHash
  }));

  return createXcEpochMinimumSourceFromRecords(records);
}

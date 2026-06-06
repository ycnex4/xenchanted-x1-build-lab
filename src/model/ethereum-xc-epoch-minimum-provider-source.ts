import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import {
  type EthereumFinalityPolicy,
  type EthereumXcEpochMinimumEntry,
  type EthereumXcLensEpochMinimumSnapshot,
  createXcEpochMinimumSourceFromEthereumLensSnapshot
} from "./ethereum-xc-epoch-minimum-source.js";
import type { XcEpochMinimumSource } from "./xc-epoch-minimum-source.js";

export interface EthereumReadProvider {
  getChainId(): Promise<bigint>;
  getBlock(
    input: EthereumBlockReadInput
  ): Promise<EthereumBlockSnapshot | null>;
  readContract(input: EthereumContractReadInput): Promise<unknown>;
}

export interface EthereumBlockReadInput {
  blockTag?: "finalized" | "safe";
  blockNumber?: bigint;
}

export interface EthereumBlockSnapshot {
  number: bigint;
  hash: string | null;
  timestamp: bigint;
}

export interface EthereumContractReadInput {
  address: string;
  abi: unknown;
  functionName: string;
  args: readonly unknown[];
  blockNumber: bigint;
}

export interface EthereumXcLensProviderAdapterInput {
  provider: EthereumReadProvider;
  chainId: string;
  lensAddress: string;
  finalityPolicy: EthereumFinalityPolicy;
  lockEpochs: readonly number[];
  epochMinimumFunctionName?: string;
  epochMinimumAbi?: unknown;
}

function throwInvalidProviderAdapterInput(message: string): never {
  throw new BuildError(BuildErrorCode.InvalidXcEpochMinimumRecord, message);
}

function assertValidEip155ChainId(chainId: string): void {
  if (!/^eip155-\d+$/.test(chainId)) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider chainId: chainId=${chainId}`
    );
  }
}

function toEip155ChainId(chainId: bigint): string {
  if (chainId <= 0n) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider chainId value: chainId=${chainId.toString()}`
    );
  }

  return `eip155-${chainId.toString()}`;
}

function assertValidEthereumAddress(address: string, label: string): string {
  if (!/^0x[0-9a-fA-F]{40}$/.test(address)) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider ${label}: ${label}=${address}`
    );
  }

  return address.toLowerCase();
}

function assertNonEmptyLockEpochs(lockEpochs: readonly number[]): void {
  if (lockEpochs.length === 0) {
    throwInvalidProviderAdapterInput(
      "Invalid Ethereum XC epoch minimum provider input: lockEpochs is empty"
    );
  }
}

function assertValidProviderFinalityPolicy(
  finalityPolicy: EthereumFinalityPolicy
): void {
  const policy = finalityPolicy as { kind?: unknown; confirmations?: unknown };

  if (
    policy.kind !== "finalized" &&
    policy.kind !== "safe" &&
    policy.kind !== "confirmed"
  ) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider finality policy: kind=${String(
        policy.kind
      )}`
    );
  }

  if (
    policy.kind === "confirmed" &&
    (!Number.isInteger(policy.confirmations) ||
      Number(policy.confirmations) <= 0)
  ) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider confirmations: confirmations=${String(
        policy.confirmations
      )}`
    );
  }
}

async function selectProvenanceBlock(
  provider: EthereumReadProvider,
  finalityPolicy: EthereumFinalityPolicy
): Promise<EthereumBlockSnapshot> {
  if (finalityPolicy.kind === "finalized") {
    return requireBlock(
      await provider.getBlock({ blockTag: "finalized" }),
      "finalized"
    );
  }

  if (finalityPolicy.kind === "safe") {
    return requireBlock(await provider.getBlock({ blockTag: "safe" }), "safe");
  }

  const headBlock = requireBlock(await provider.getBlock({}), "head");

  const confirmations = BigInt(finalityPolicy.confirmations);

  if (headBlock.number <= confirmations) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider confirmed block selection: head=${headBlock.number.toString()}, confirmations=${confirmations.toString()}`
    );
  }

  const confirmedBlockNumber = headBlock.number - confirmations;

  return requireBlock(
    await provider.getBlock({ blockNumber: confirmedBlockNumber }),
    "confirmed"
  );
}

function requireBlock(
  block: EthereumBlockSnapshot | null,
  label: string
): EthereumBlockSnapshot & { hash: string } {
  if (block === null) {
    throwInvalidProviderAdapterInput(
      `Missing Ethereum XC epoch minimum provider ${label} block`
    );
  }

  if (block.number <= 0n) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider ${label} block number: blockNumber=${block.number.toString()}`
    );
  }

  const hash = block.hash;

  if (hash === null || hash.length === 0) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider ${label} block hash`
    );
  }

  if (block.timestamp <= 0n) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider ${label} block timestamp: timestamp=${block.timestamp.toString()}`
    );
  }

  return {
    ...block,
    hash
  };
}

function decodeMinimumXntd(value: unknown, lockEpoch: number): bigint {
  if (typeof value !== "bigint") {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider read result: lockEpoch=${lockEpoch.toString()}`
    );
  }

  if (value <= 0n) {
    throwInvalidProviderAdapterInput(
      `Invalid Ethereum XC epoch minimum provider minimum: lockEpoch=${lockEpoch.toString()}, minimumXntd=${value.toString()}`
    );
  }

  return value;
}

export async function createXcEpochMinimumSourceFromEthereumLensProvider(
  input: EthereumXcLensProviderAdapterInput
): Promise<XcEpochMinimumSource> {
  assertValidEip155ChainId(input.chainId);
  assertValidProviderFinalityPolicy(input.finalityPolicy);
  assertNonEmptyLockEpochs(input.lockEpochs);

  const normalizedLensAddress = assertValidEthereumAddress(
    input.lensAddress,
    "lensAddress"
  );

  const actualChainId = toEip155ChainId(await input.provider.getChainId());

  if (actualChainId !== input.chainId) {
    throwInvalidProviderAdapterInput(
      `Ethereum XC epoch minimum provider chainId mismatch: expected=${input.chainId}, actual=${actualChainId}`
    );
  }

  const provenanceBlock = await selectProvenanceBlock(
    input.provider,
    input.finalityPolicy
  );

  const functionName = input.epochMinimumFunctionName ?? "epochMinimum";
  const abi = input.epochMinimumAbi ?? [];

  const epochMinimums: EthereumXcEpochMinimumEntry[] = [];

  for (const lockEpoch of input.lockEpochs) {
    const minimumXntd = decodeMinimumXntd(
      await input.provider.readContract({
        address: normalizedLensAddress,
        abi,
        functionName,
        args: [lockEpoch],
        blockNumber: provenanceBlock.number
      }),
      lockEpoch
    );

    epochMinimums.push({
      lockEpoch,
      minimumXntd
    });
  }

  const sourceBlockHash = provenanceBlock.hash;

  if (sourceBlockHash === null) {
    throwInvalidProviderAdapterInput(
      "Invalid Ethereum XC epoch minimum provider provenance block hash"
    );
  }

  const snapshot: EthereumXcLensEpochMinimumSnapshot = {
    sourceChainId: input.chainId,
    sourceBlockNumber: provenanceBlock.number,
    sourceBlockHash,
    observedAt: provenanceBlock.timestamp,
    finalizedPolicy: input.finalityPolicy,
    epochMinimums
  };

  return createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot);
}

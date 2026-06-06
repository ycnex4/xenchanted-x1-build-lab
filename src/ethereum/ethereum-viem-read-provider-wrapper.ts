import type {
  EthereumBlockReadInput,
  EthereumBlockSnapshot,
  EthereumContractReadInput,
  EthereumReadProvider
} from "../model/ethereum-xc-epoch-minimum-provider-source.js";

export interface ViemLikeBlock {
  number: bigint | null;
  hash: string | null;
  timestamp: bigint | number;
}

export interface ViemLikePublicClient {
  getChainId(): Promise<number>;
  getBlock(input: ViemLikeGetBlockInput): Promise<ViemLikeBlock | null>;
  readContract(input: ViemLikeReadContractInput): Promise<unknown>;
}

export interface ViemLikeGetBlockInput {
  blockTag?: "latest" | "finalized" | "safe";
  blockNumber?: bigint;
}

export interface ViemLikeReadContractInput {
  address: string;
  abi: unknown;
  functionName: string;
  args: readonly unknown[];
  blockNumber: bigint;
}

export function createEthereumReadProviderFromViemPublicClient(
  publicClient: ViemLikePublicClient
): EthereumReadProvider {
  return {
    async getChainId(): Promise<bigint> {
      return BigInt(await publicClient.getChainId());
    },

    async getBlock(
      input: EthereumBlockReadInput
    ): Promise<EthereumBlockSnapshot | null> {
      const block = await publicClient.getBlock(toViemGetBlockInput(input));

      if (block === null) {
        return null;
      }

      if (block.number === null) {
        return null;
      }

      return {
        number: block.number,
        hash: block.hash,
        timestamp: normalizeViemTimestamp(block.timestamp)
      };
    },

    async readContract(input: EthereumContractReadInput): Promise<unknown> {
      return publicClient.readContract({
        address: input.address,
        abi: input.abi,
        functionName: input.functionName,
        args: input.args,
        blockNumber: input.blockNumber
      });
    }
  };
}

function toViemGetBlockInput(
  input: EthereumBlockReadInput
): ViemLikeGetBlockInput {
  if (input.blockTag !== undefined) {
    return {
      blockTag: input.blockTag
    };
  }

  if (input.blockNumber !== undefined) {
    return {
      blockNumber: input.blockNumber
    };
  }

  return {
    blockTag: "latest"
  };
}

function normalizeViemTimestamp(timestamp: bigint | number): bigint {
  return typeof timestamp === "bigint" ? timestamp : BigInt(timestamp);
}

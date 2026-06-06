import type {
  EthereumBlockReadInput,
  EthereumBlockSnapshot,
  EthereumContractReadInput,
  EthereumReadProvider
} from "../model/ethereum-xc-epoch-minimum-provider-source.js";

export interface EthereumPublicClientBlock {
  number: bigint | null;
  hash: string | null;
  timestamp: bigint | number;
}

export interface EthereumPublicClientLike {
  getChainId(): Promise<number | bigint>;
  getBlock(input?: EthereumPublicClientGetBlockInput): Promise<EthereumPublicClientBlock | null>;
  readContract(input: EthereumPublicClientReadContractInput): Promise<unknown>;
}

export interface EthereumPublicClientGetBlockInput {
  blockTag?: "latest" | "finalized" | "safe";
  blockNumber?: bigint;
}

export interface EthereumPublicClientReadContractInput {
  address: string;
  abi: unknown;
  functionName: string;
  args: readonly unknown[];
  blockNumber: bigint;
}

export function createEthereumReadProviderFromPublicClient(
  publicClient: EthereumPublicClientLike
): EthereumReadProvider {
  return {
    async getChainId(): Promise<bigint> {
      const chainId = await publicClient.getChainId();

      return typeof chainId === "bigint" ? chainId : BigInt(chainId);
    },

    async getBlock(
      input: EthereumBlockReadInput
    ): Promise<EthereumBlockSnapshot | null> {
      const block = await publicClient.getBlock(toPublicClientBlockInput(input));

      if (block === null) {
        return null;
      }

      if (block.number === null) {
        return null;
      }

      return {
        number: block.number,
        hash: block.hash,
        timestamp:
          typeof block.timestamp === "bigint"
            ? block.timestamp
            : BigInt(block.timestamp)
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

function toPublicClientBlockInput(
  input: EthereumBlockReadInput
): EthereumPublicClientGetBlockInput {
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

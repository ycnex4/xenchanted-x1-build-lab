import type { EthereumFinalityPolicy } from "../model/ethereum-xc-epoch-minimum-source.js";
import {
  type EthereumReadProvider,
  createXcEpochMinimumSourceFromEthereumLensProvider
} from "../model/ethereum-xc-epoch-minimum-provider-source.js";
import type { XcEpochMinimumSource } from "../model/xc-epoch-minimum-source.js";
import {
  type ViemLikePublicClient,
  createEthereumReadProviderFromViemPublicClient
} from "./ethereum-viem-read-provider-wrapper.js";

export interface EthereumReadonlyRpcIntegrationInput {
  publicClient: ViemLikePublicClient;
  chainId: string;
  lensAddress: string;
  finalityPolicy: EthereumFinalityPolicy;
  lockEpochs: readonly number[];
  epochMinimumFunctionName?: string;
  epochMinimumAbi?: unknown;
}

export async function createXcEpochMinimumSourceFromReadonlyEthereumPublicClient(
  input: EthereumReadonlyRpcIntegrationInput
): Promise<XcEpochMinimumSource> {
  const provider = createEthereumReadProviderFromReadonlyEthereumPublicClient(
    input.publicClient
  );

  return createXcEpochMinimumSourceFromEthereumLensProvider({
    provider,
    chainId: input.chainId,
    lensAddress: input.lensAddress,
    finalityPolicy: input.finalityPolicy,
    lockEpochs: input.lockEpochs,
    ...(input.epochMinimumFunctionName !== undefined
      ? { epochMinimumFunctionName: input.epochMinimumFunctionName }
      : {}),
    ...(input.epochMinimumAbi !== undefined
      ? { epochMinimumAbi: input.epochMinimumAbi }
      : {})
  });
}

export function createEthereumReadProviderFromReadonlyEthereumPublicClient(
  publicClient: ViemLikePublicClient
): EthereumReadProvider {
  return createEthereumReadProviderFromViemPublicClient(publicClient);
}

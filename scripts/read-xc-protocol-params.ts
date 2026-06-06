import { createPublicClient, http, type Abi, type Address } from "viem";
import { mainnet, sepolia } from "viem/chains";
import {
  createXcProtocolParamsSourceFromEthereumReadProvider,
  parseEthereumScriptConfig,
  type EthereumScriptConfigEnv,
  type XcProtocolParamsReadProvider
} from "../src/index.js";

const SUPPORTED_CHAINS = {
  "eip155-1": mainnet,
  "eip155-11155111": sepolia
} as const;

type SupportedChainId = keyof typeof SUPPORTED_CHAINS;

function readEthereumScriptEnv(): EthereumScriptConfigEnv {
  return {
    XC_ETHEREUM_RPC_URL: process.env.XC_ETHEREUM_RPC_URL,
    XC_ETHEREUM_CHAIN_ID: process.env.XC_ETHEREUM_CHAIN_ID,
    XC_ETHEREUM_LENS_ADDRESS: process.env.XC_ETHEREUM_LENS_ADDRESS,
    XC_ETHEREUM_FINALITY: process.env.XC_ETHEREUM_FINALITY,
    XC_ETHEREUM_CONFIRMATIONS: process.env.XC_ETHEREUM_CONFIRMATIONS,
    XC_ETHEREUM_LOCK_EPOCHS: process.env.XC_ETHEREUM_LOCK_EPOCHS,
    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION:
      process.env.XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION,
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH:
      process.env.XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH,
    XC_ETHEREUM_REAL_RPC_CONFIRM:
      process.env.XC_ETHEREUM_REAL_RPC_CONFIRM
  };
}

function assertSupportedChainId(chainId: string): asserts chainId is SupportedChainId {
  if (!(chainId in SUPPORTED_CHAINS)) {
    throw new Error("Unsupported Ethereum script config: XC_ETHEREUM_CHAIN_ID");
  }
}

function expectedNumericChainId(chainId: SupportedChainId): number {
  return SUPPORTED_CHAINS[chainId].id;
}

function createReadonlyProtocolParamsProvider(config: {
  readonly chainId: SupportedChainId;
  readonly rpcUrl: string;
}): {
  readonly provider: XcProtocolParamsReadProvider;
  readonly getChainId: () => Promise<number>;
} {
  const publicClient = createPublicClient({
    chain: SUPPORTED_CHAINS[config.chainId],
    transport: http(config.rpcUrl)
  });

  return {
    async getChainId(): Promise<number> {
      return publicClient.getChainId();
    },

    provider: {
      async readContract(input) {
        return publicClient.readContract({
          address: input.address as Address,
          abi: input.abi as Abi,
          functionName: input.functionName,
          args: input.args,
          blockNumber: input.blockNumber
        });
      }
    }
  };
}

function sanitizeErrorMessage(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);

  if (
    message.startsWith("Missing required Ethereum script config:") ||
    message.startsWith("Missing required Ethereum script secret config:") ||
    message.startsWith("Missing required Ethereum script confirmation:") ||
    message.startsWith("Invalid Ethereum script config:") ||
    message.startsWith("Unsupported Ethereum script config:") ||
    message.startsWith("Ethereum script chain mismatch:")
  ) {
    return message;
  }

  return "Manual XC protocol params RPC smoke script failed with a sanitized runtime error.";
}

function writeParam(name: string, value: bigint | number): void {
  console.log(`${name}=${value.toString()}`);
}

async function main(): Promise<void> {
  const env = readEthereumScriptEnv();
  const config = parseEthereumScriptConfig(env);

  assertSupportedChainId(config.chainId);

  const readonlyClient = createReadonlyProtocolParamsProvider({
    chainId: config.chainId,
    rpcUrl: config.rpcUrl
  });

  const providerChainId = await readonlyClient.getChainId();
  const configuredChainId = expectedNumericChainId(config.chainId);

  if (providerChainId !== configuredChainId) {
    throw new Error(
      `Ethereum script chain mismatch: configured=${configuredChainId.toString()}, provider=${providerChainId.toString()}`
    );
  }

  const source = createXcProtocolParamsSourceFromEthereumReadProvider({
    provider: readonlyClient.provider,
    lensAddress: config.lensAddress
  });

  const params = await source.readProtocolParams();

  console.log("manualProtocolParamsSmoke=true");
  console.log(`providerChainId=${providerChainId.toString()}`);
  console.log("safeConfigSummary");
  console.log(`chainId=${config.chainId}`);
  console.log(`lensAddress=${config.lensAddress}`);
  console.log(`finality=${config.finalityPolicy.kind}`);
  console.log(`realRpcConfirmed=${String(config.realRpcConfirmed)}`);

  writeParam("genesisTs", params.genesisTs);
  writeParam("halvingInterval", params.halvingInterval);
  writeParam("xenBurnHalvingInterval", params.xenBurnHalvingInterval);
  writeParam("currentEpoch", params.currentEpoch);
  writeParam("nextHalvingTs", params.nextHalvingTs);
  writeParam("initialNominal", params.initialNominal);
  writeParam("currentBaseNominal", params.currentBaseNominal);
  writeParam("initialXenBurn", params.initialXenBurn);
  writeParam("currentXenBurnAmount", params.currentXenBurnAmount);
  writeParam("enchantMultiplier", params.enchantMultiplier);
  writeParam("maxLevel", params.maxLevel);
  writeParam("baseAprBpsNow", params.baseAprBpsNow);
  writeParam("bpsDenom", params.bpsDenom);
  writeParam("earlyPenaltyBps", params.earlyPenaltyBps);
  writeParam("maxWalletNfts", params.maxWalletNfts);
  console.log("completed=true");
}

main().catch((error: unknown) => {
  console.error(sanitizeErrorMessage(error));
  process.exitCode = 1;
});

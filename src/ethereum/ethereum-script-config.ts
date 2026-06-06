import type { EthereumFinalityPolicy } from "../model/ethereum-xc-epoch-minimum-source.js";

export interface EthereumScriptConfigEnv {
  readonly [name: string]: string | undefined;
}

export interface EthereumScriptConfig {
  readonly rpcUrl: string;
  readonly chainId: string;
  readonly lensAddress: string;
  readonly finalityPolicy: EthereumFinalityPolicy;
  readonly lockEpochs: readonly number[];
  readonly epochMinimumFunctionName: string;
  readonly epochMinimumAbiPath?: string;
  readonly realRpcConfirmed: true;
}

export interface EthereumScriptSafeConfigSummary {
  readonly chainId: string;
  readonly lensAddress: string;
  readonly finalityPolicy: EthereumFinalityPolicy;
  readonly lockEpochCount: number;
  readonly epochMinimumFunctionName: string;
  readonly hasEpochMinimumAbiPath: boolean;
  readonly realRpcConfirmed: true;
}

const REAL_RPC_CONFIRMATION = "I_UNDERSTAND_THIS_USES_REAL_RPC";
const DEFAULT_EPOCH_MINIMUM_FUNCTION_NAME = "epochMinimum";

export function parseEthereumScriptConfig(
  env: EthereumScriptConfigEnv
): EthereumScriptConfig {
  const rpcUrl = readRequiredSecret(env, "XC_ETHEREUM_RPC_URL");
  const chainId = parseChainId(readRequired(env, "XC_ETHEREUM_CHAIN_ID"));
  const lensAddress = parseLensAddress(
    readRequired(env, "XC_ETHEREUM_LENS_ADDRESS")
  );
  const finalityPolicy = parseFinalityPolicy(env);
  const lockEpochs = parseLockEpochs(readRequired(env, "XC_ETHEREUM_LOCK_EPOCHS"));
  const epochMinimumFunctionName = parseOptionalFunctionName(
    env.XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
  );
  const epochMinimumAbiPath = parseOptionalAbiPath(
    env.XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
  );

  assertRealRpcConfirmed(env.XC_ETHEREUM_REAL_RPC_CONFIRM);

  return {
    rpcUrl,
    chainId,
    lensAddress,
    finalityPolicy,
    lockEpochs,
    epochMinimumFunctionName,
    ...(epochMinimumAbiPath !== undefined ? { epochMinimumAbiPath } : {}),
    realRpcConfirmed: true
  };
}

export function summarizeEthereumScriptConfig(
  config: EthereumScriptConfig
): EthereumScriptSafeConfigSummary {
  return {
    chainId: config.chainId,
    lensAddress: config.lensAddress,
    finalityPolicy: config.finalityPolicy,
    lockEpochCount: config.lockEpochs.length,
    epochMinimumFunctionName: config.epochMinimumFunctionName,
    hasEpochMinimumAbiPath: config.epochMinimumAbiPath !== undefined,
    realRpcConfirmed: true
  };
}

function readRequired(env: EthereumScriptConfigEnv, name: string): string {
  const value = env[name];

  if (value === undefined || value.trim() === "") {
    throw new Error(`Missing required Ethereum script config: ${name}`);
  }

  return value.trim();
}

function readRequiredSecret(env: EthereumScriptConfigEnv, name: string): string {
  const value = env[name];

  if (value === undefined || value.trim() === "") {
    throw new Error(`Missing required Ethereum script secret config: ${name}`);
  }

  return value.trim();
}

function parseChainId(value: string): string {
  if (!/^eip155-[1-9]\d*$/.test(value)) {
    throw new Error("Invalid Ethereum script config: XC_ETHEREUM_CHAIN_ID");
  }

  return value;
}

function parseLensAddress(value: string): string {
  if (!/^0x[0-9a-fA-F]{40}$/.test(value)) {
    throw new Error("Invalid Ethereum script config: XC_ETHEREUM_LENS_ADDRESS");
  }

  return value.toLowerCase();
}

function parseFinalityPolicy(env: EthereumScriptConfigEnv): EthereumFinalityPolicy {
  const finality = readRequired(env, "XC_ETHEREUM_FINALITY");

  if (finality === "finalized") {
    return { kind: "finalized" };
  }

  if (finality === "safe") {
    return { kind: "safe" };
  }

  if (finality === "confirmed") {
    return {
      kind: "confirmed",
      confirmations: parseConfirmations(
        readRequired(env, "XC_ETHEREUM_CONFIRMATIONS")
      )
    };
  }

  throw new Error("Invalid Ethereum script config: XC_ETHEREUM_FINALITY");
}

function parseConfirmations(value: string): number {
  if (!/^[1-9]\d*$/.test(value)) {
    throw new Error("Invalid Ethereum script config: XC_ETHEREUM_CONFIRMATIONS");
  }

  return Number(value);
}

function parseLockEpochs(value: string): readonly number[] {
  const parts = value
    .split(",")
    .map((part) => part.trim())
    .filter((part) => part.length > 0);

  if (parts.length === 0) {
    throw new Error("Invalid Ethereum script config: XC_ETHEREUM_LOCK_EPOCHS");
  }

  return parts.map((part) => {
    if (!/^\d+$/.test(part)) {
      throw new Error("Invalid Ethereum script config: XC_ETHEREUM_LOCK_EPOCHS");
    }

    return Number(part);
  });
}

function parseOptionalFunctionName(value: string | undefined): string {
  if (value === undefined || value.trim() === "") {
    return DEFAULT_EPOCH_MINIMUM_FUNCTION_NAME;
  }

  const trimmed = value.trim();

  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(trimmed)) {
    throw new Error(
      "Invalid Ethereum script config: XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION"
    );
  }

  return trimmed;
}

function parseOptionalAbiPath(value: string | undefined): string | undefined {
  if (value === undefined || value.trim() === "") {
    return undefined;
  }

  return value.trim();
}

function assertRealRpcConfirmed(value: string | undefined): void {
  if (value !== REAL_RPC_CONFIRMATION) {
    throw new Error(
      "Missing required Ethereum script confirmation: XC_ETHEREUM_REAL_RPC_CONFIRM"
    );
  }
}

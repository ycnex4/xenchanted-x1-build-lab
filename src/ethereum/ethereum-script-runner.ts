import type {
  EthereumScriptConfigEnv,
  EthereumScriptSafeConfigSummary
} from "./ethereum-script-config.js";
import {
  parseEthereumScriptConfig,
  summarizeEthereumScriptConfig
} from "./ethereum-script-config.js";
import { createXcEpochMinimumSourceFromReadonlyEthereumPublicClient } from "./ethereum-readonly-rpc-integration.js";
import type { ViemLikePublicClient } from "./ethereum-viem-read-provider-wrapper.js";

export interface EthereumScriptRunnerOutput {
  writeLine(line: string): void;
}

export interface EthereumScriptRunnerInput {
  env: EthereumScriptConfigEnv;
  publicClient: ViemLikePublicClient;
  output: EthereumScriptRunnerOutput;
}

export interface EthereumScriptRunnerEpochMinimumResult {
  readonly lockEpoch: number;
  readonly minimumXntd: bigint | null;
}

export interface EthereumScriptRunnerResult {
  readonly safeConfigSummary: EthereumScriptSafeConfigSummary;
  readonly epochMinimums: readonly EthereumScriptRunnerEpochMinimumResult[];
  readonly completed: true;
}

export async function runEthereumXcEpochMinimumReadFromProvidedClient(
  input: EthereumScriptRunnerInput
): Promise<EthereumScriptRunnerResult> {
  const config = parseEthereumScriptConfig(input.env);
  const safeConfigSummary = summarizeEthereumScriptConfig(config);

  writeSafeConfigSummary(input.output, safeConfigSummary);

  const source = await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
    publicClient: input.publicClient,
    chainId: config.chainId,
    lensAddress: config.lensAddress,
    finalityPolicy: config.finalityPolicy,
    lockEpochs: config.lockEpochs,
    epochMinimumFunctionName: config.epochMinimumFunctionName
  });

  const epochMinimums = config.lockEpochs.map((lockEpoch) => ({
    lockEpoch,
    minimumXntd: source.authoritativeEpochMinimum(lockEpoch)
  }));

  for (const epochMinimum of epochMinimums) {
    input.output.writeLine(
      `epochMinimum lockEpoch=${epochMinimum.lockEpoch.toString()} minimumXntd=${formatMinimumXntd(epochMinimum.minimumXntd)}`
    );
  }

  input.output.writeLine("completed=true");

  return {
    safeConfigSummary,
    epochMinimums,
    completed: true
  };
}

function writeSafeConfigSummary(
  output: EthereumScriptRunnerOutput,
  summary: EthereumScriptSafeConfigSummary
): void {
  output.writeLine("safeConfigSummary");
  output.writeLine(`chainId=${summary.chainId}`);
  output.writeLine(`lensAddress=${summary.lensAddress}`);
  output.writeLine(`finality=${formatFinality(summary.finalityPolicy)}`);
  output.writeLine(`lockEpochCount=${summary.lockEpochCount.toString()}`);
  output.writeLine(
    `epochMinimumFunctionName=${summary.epochMinimumFunctionName}`
  );
  output.writeLine(
    `hasEpochMinimumAbiPath=${String(summary.hasEpochMinimumAbiPath)}`
  );
  output.writeLine(`realRpcConfirmed=${String(summary.realRpcConfirmed)}`);
}

function formatFinality(
  finalityPolicy: EthereumScriptSafeConfigSummary["finalityPolicy"]
): string {
  if (finalityPolicy.kind === "confirmed") {
    return `confirmed:${finalityPolicy.confirmations.toString()}`;
  }

  return finalityPolicy.kind;
}

function formatMinimumXntd(minimumXntd: bigint | null): string {
  return minimumXntd === null ? "null" : minimumXntd.toString();
}

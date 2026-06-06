import { describe, expect, it } from "vitest";
import {
  type EthereumScriptConfigEnv,
  type ViemLikeBlock,
  type ViemLikeGetBlockInput,
  type ViemLikePublicClient,
  type ViemLikeReadContractInput,
  runEthereumXcEpochMinimumReadFromProvidedClient
} from "../src/index.js";

const RPC_URL = "https://provider.example/SECRET_API_KEY";
const LENS_ADDRESS = "0x1111111111111111111111111111111111111111";
const FINALIZED_BLOCK_HASH =
  "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const SAFE_BLOCK_HASH =
  "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
const LATEST_BLOCK_HASH =
  "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const CONFIRMED_BLOCK_HASH =
  "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

class MockRunnerPublicClient implements ViemLikePublicClient {
  public readonly blockReads: ViemLikeGetBlockInput[] = [];
  public readonly contractReads: ViemLikeReadContractInput[] = [];

  public readonly blocks = new Map<string, ViemLikeBlock | null>([
    [
      "finalized",
      {
        number: 100n,
        hash: FINALIZED_BLOCK_HASH,
        timestamp: 1000n
      }
    ],
    [
      "safe",
      {
        number: 90n,
        hash: SAFE_BLOCK_HASH,
        timestamp: 900n
      }
    ],
    [
      "latest",
      {
        number: 120n,
        hash: LATEST_BLOCK_HASH,
        timestamp: 1200n
      }
    ],
    [
      "108",
      {
        number: 108n,
        hash: CONFIRMED_BLOCK_HASH,
        timestamp: 1080n
      }
    ]
  ]);

  public readonly minimumsByEpoch = new Map<number, bigint>([
    [0, 100n],
    [1, 50n],
    [2, 25n]
  ]);

  async getChainId(): Promise<number> {
    return 1;
  }

  async getBlock(input: ViemLikeGetBlockInput): Promise<ViemLikeBlock | null> {
    this.blockReads.push(input);

    if (input.blockNumber !== undefined) {
      return this.blocks.get(input.blockNumber.toString()) ?? null;
    }

    return this.blocks.get(input.blockTag ?? "latest") ?? null;
  }

  async readContract(input: ViemLikeReadContractInput): Promise<unknown> {
    this.contractReads.push(input);

    const lockEpoch = input.args[0];

    if (typeof lockEpoch !== "number") {
      return null;
    }

    return this.minimumsByEpoch.get(lockEpoch) ?? null;
  }
}

class CapturingOutput {
  public readonly lines: string[] = [];

  writeLine(line: string): void {
    this.lines.push(line);
  }

  text(): string {
    return this.lines.join("\n");
  }
}

function validEnv(overrides: EthereumScriptConfigEnv = {}): EthereumScriptConfigEnv {
  return {
    XC_ETHEREUM_RPC_URL: RPC_URL,
    XC_ETHEREUM_CHAIN_ID: "eip155-1",
    XC_ETHEREUM_LENS_ADDRESS: LENS_ADDRESS,
    XC_ETHEREUM_FINALITY: "finalized",
    XC_ETHEREUM_LOCK_EPOCHS: "0,1",
    XC_ETHEREUM_REAL_RPC_CONFIRM: "I_UNDERSTAND_THIS_USES_REAL_RPC",
    ...overrides
  };
}

function expectNoSecretOutput(outputText: string): void {
  expect(outputText).not.toContain(RPC_URL);
  expect(outputText).not.toContain("SECRET_API_KEY");
  expect(outputText).not.toContain("provider.example");
  expect(outputText).not.toContain("https://");
  expect(outputText).not.toContain("XC_ETHEREUM_RPC_URL");
  expect(outputText).not.toContain("raw env");
  expect(outputText).not.toContain("rpcUrl");
}

describe("Ethereum XC epoch minimum script runner", () => {
  it("runs with env-like input and a provided mocked public client", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    const result = await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv(),
      publicClient,
      output
    });

    expect(result.completed).toBe(true);
    expect(result.epochMinimums).toEqual([
      { lockEpoch: 0, minimumXntd: 100n },
      { lockEpoch: 1, minimumXntd: 50n }
    ]);
    expect(publicClient.blockReads).toEqual([{ blockTag: "finalized" }]);
    expect(publicClient.contractReads).toHaveLength(2);
  });

  it("writes safe config summary without RPC URL or API-key-like values", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv({
        XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH: "./abi/xc-lens.json"
      }),
      publicClient,
      output
    });

    expect(output.lines).toContain("safeConfigSummary");
    expect(output.lines).toContain("chainId=eip155-1");
    expect(output.lines).toContain(`lensAddress=${LENS_ADDRESS}`);
    expect(output.lines).toContain("finality=finalized");
    expect(output.lines).toContain("lockEpochCount=2");
    expect(output.lines).toContain("epochMinimumFunctionName=epochMinimum");
    expect(output.lines).toContain("hasEpochMinimumAbiPath=true");
    expect(output.lines).toContain("realRpcConfirmed=true");
    expectNoSecretOutput(output.text());
  });

  it("does not return the full parsed config object", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    const result = await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv(),
      publicClient,
      output
    });

    expect(Object.keys(result)).toEqual([
      "safeConfigSummary",
      "epochMinimums",
      "completed"
    ]);
    expect(Object.prototype.hasOwnProperty.call(result, "rpcUrl")).toBe(false);
    expect(Object.prototype.hasOwnProperty.call(result, "config")).toBe(false);
    expect(Object.prototype.hasOwnProperty.call(result, "env")).toBe(false);

    const safeSummaryText = JSON.stringify(result.safeConfigSummary);
    expect(safeSummaryText).not.toContain(RPC_URL);
    expect(safeSummaryText).not.toContain("SECRET_API_KEY");
    expect(safeSummaryText).not.toContain("rpcUrl");
  });

  it("passes confirmed finality policy through to the source helper", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    const result = await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv({
        XC_ETHEREUM_FINALITY: "confirmed",
        XC_ETHEREUM_CONFIRMATIONS: "12",
        XC_ETHEREUM_LOCK_EPOCHS: "2"
      }),
      publicClient,
      output
    });

    expect(result.safeConfigSummary.finalityPolicy).toEqual({
      kind: "confirmed",
      confirmations: 12
    });
    expect(result.epochMinimums).toEqual([{ lockEpoch: 2, minimumXntd: 25n }]);
    expect(publicClient.blockReads).toEqual([
      { blockTag: "latest" },
      { blockNumber: 108n }
    ]);
    expect(publicClient.contractReads.map((read) => read.blockNumber)).toEqual([
      108n
    ]);
    expect(output.lines).toContain("finality=confirmed:12");
  });

  it("passes lock epochs and function name through to contract reads", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv({
        XC_ETHEREUM_LOCK_EPOCHS: "0,2",
        XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION: "customEpochMinimum"
      }),
      publicClient,
      output
    });

    expect(publicClient.contractReads).toEqual([
      {
        address: LENS_ADDRESS,
        abi: [],
        functionName: "customEpochMinimum",
        args: [0],
        blockNumber: 100n
      },
      {
        address: LENS_ADDRESS,
        abi: [],
        functionName: "customEpochMinimum",
        args: [2],
        blockNumber: 100n
      }
    ]);
  });

  it("keeps ABI path as metadata only in the first runner milestone", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv({
        XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH: "./abi/xc-lens.json"
      }),
      publicClient,
      output
    });

    expect(output.lines).toContain("hasEpochMinimumAbiPath=true");
    expect(publicClient.contractReads.every((read) => read.abi === undefined)).toBe(
      false
    );
    expect(publicClient.contractReads.every((read) => Array.isArray(read.abi))).toBe(
      true
    );
  });

  it("propagates sanitized parser validation errors", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    await expect(
      runEthereumXcEpochMinimumReadFromProvidedClient({
        env: validEnv({ XC_ETHEREUM_CHAIN_ID: "invalid-chain" }),
        publicClient,
        output
      })
    ).rejects.toThrow("Invalid Ethereum script config: XC_ETHEREUM_CHAIN_ID");

    try {
      await runEthereumXcEpochMinimumReadFromProvidedClient({
        env: validEnv({ XC_ETHEREUM_FINALITY: "bad-finality" }),
        publicClient,
        output
      });
      throw new Error("expected runner to throw");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);

      expect(message).not.toContain(RPC_URL);
      expect(message).not.toContain("SECRET_API_KEY");
      expect(message).not.toContain("provider.example");
      expect(message).not.toContain("https://");
    }
  });

  it("uses the provided public client only", async () => {
    const publicClient = new MockRunnerPublicClient();
    const output = new CapturingOutput();

    await runEthereumXcEpochMinimumReadFromProvidedClient({
      env: validEnv({ XC_ETHEREUM_FINALITY: "safe" }),
      publicClient,
      output
    });

    expect(publicClient.blockReads).toEqual([{ blockTag: "safe" }]);
    expect(publicClient.contractReads).toHaveLength(2);
  });
});

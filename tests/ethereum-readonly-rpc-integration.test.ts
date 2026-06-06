import { describe, expect, it } from "vitest";
import {
  type ViemLikeBlock,
  type ViemLikeGetBlockInput,
  type ViemLikePublicClient,
  type ViemLikeReadContractInput,
  createEthereumReadProviderFromReadonlyEthereumPublicClient,
  createXcEpochMinimumSourceFromReadonlyEthereumPublicClient
} from "../src/index.js";

const LENS_ADDRESS = "0x1111111111111111111111111111111111111111";
const FINALIZED_BLOCK_HASH =
  "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const SAFE_BLOCK_HASH =
  "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
const LATEST_BLOCK_HASH =
  "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const CONFIRMED_BLOCK_HASH =
  "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

class MockReadonlyPublicClient implements ViemLikePublicClient {
  public readonly blockReads: ViemLikeGetBlockInput[] = [];
  public readonly contractReads: ViemLikeReadContractInput[] = [];

  public chainId = 1;

  public blocks = new Map<string, ViemLikeBlock | null>([
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

  public minimumsByEpoch = new Map<number, bigint>([
    [0, 100n],
    [1, 50n],
    [2, 25n]
  ]);

  async getChainId(): Promise<number> {
    return this.chainId;
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

describe("Ethereum read-only RPC integration helper", () => {
  it("creates an EthereumReadProvider from a provided public client", async () => {
    const publicClient = new MockReadonlyPublicClient();
    publicClient.chainId = 11155111;

    const provider =
      createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient);

    expect(await provider.getChainId()).toBe(11155111n);
    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toEqual({
      number: 100n,
      hash: FINALIZED_BLOCK_HASH,
      timestamp: 1000n
    });
  });

  it("constructs source from a provided public client without real RPC", async () => {
    const publicClient = new MockReadonlyPublicClient();

    const source = await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0, 1],
      epochMinimumFunctionName: "epochMinimum",
      epochMinimumAbi: []
    });

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(source.authoritativeEpochMinimum(1)).toBe(50n);
    expect(publicClient.blockReads).toEqual([{ blockTag: "finalized" }]);
    expect(publicClient.contractReads.map((read) => read.blockNumber)).toEqual([
      100n,
      100n
    ]);
  });

  it("preserves safe finality policy", async () => {
    const publicClient = new MockReadonlyPublicClient();

    const source = await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "safe" },
      lockEpochs: [0]
    });

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(publicClient.blockReads).toEqual([{ blockTag: "safe" }]);
    expect(publicClient.contractReads.map((read) => read.blockNumber)).toEqual([
      90n
    ]);
  });

  it("preserves confirmed finality policy", async () => {
    const publicClient = new MockReadonlyPublicClient();

    const source = await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "confirmed", confirmations: 12 },
      lockEpochs: [2]
    });

    expect(source.authoritativeEpochMinimum(2)).toBe(25n);
    expect(publicClient.blockReads).toEqual([
      { blockTag: "latest" },
      { blockNumber: 108n }
    ]);
    expect(publicClient.contractReads.map((read) => read.blockNumber)).toEqual([
      108n
    ]);
  });

  it("passes explicit function name and ABI through to contract reads", async () => {
    const publicClient = new MockReadonlyPublicClient();
    const abi = [{ type: "function", name: "customEpochMinimum" }];

    await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0],
      epochMinimumFunctionName: "customEpochMinimum",
      epochMinimumAbi: abi
    });

    expect(publicClient.contractReads).toEqual([
      {
        address: LENS_ADDRESS,
        abi,
        functionName: "customEpochMinimum",
        args: [0],
        blockNumber: 100n
      }
    ]);
  });

  it("uses provider adapter defaults when optional function metadata is omitted", async () => {
    const publicClient = new MockReadonlyPublicClient();

    await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0]
    });

    expect(publicClient.contractReads).toEqual([
      {
        address: LENS_ADDRESS,
        abi: [],
        functionName: "epochMinimum",
        args: [0],
        blockNumber: 100n
      }
    ]);
  });

  it("does not downgrade finalized to latest", async () => {
    const publicClient = new MockReadonlyPublicClient();

    await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0]
    });

    expect(publicClient.blockReads).toEqual([{ blockTag: "finalized" }]);
  });

  it("does not downgrade safe to latest", async () => {
    const publicClient = new MockReadonlyPublicClient();

    await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "safe" },
      lockEpochs: [0]
    });

    expect(publicClient.blockReads).toEqual([{ blockTag: "safe" }]);
  });

  it("propagates sanitized provider errors without adding RPC URL or API key", async () => {
    const publicClient = new MockReadonlyPublicClient();
    publicClient.getBlock = async () => {
      throw new Error("provider getBlock failed");
    };

    await expect(
      createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
        publicClient,
        chainId: "eip155-1",
        lensAddress: LENS_ADDRESS,
        finalityPolicy: { kind: "finalized" },
        lockEpochs: [0]
      })
    ).rejects.toThrow("provider getBlock failed");

    await expect(
      createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
        publicClient,
        chainId: "eip155-1",
        lensAddress: LENS_ADDRESS,
        finalityPolicy: { kind: "finalized" },
        lockEpochs: [0]
      })
    ).rejects.not.toThrow("https://");
  });

  it("does not expose RPC URL or API key in successful source state", async () => {
    const publicClient = new MockReadonlyPublicClient();

    const source = await createXcEpochMinimumSourceFromReadonlyEthereumPublicClient({
      publicClient,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0]
    });

    expect(JSON.stringify(source)).not.toContain("https://");
    expect(JSON.stringify(source)).not.toContain("API_KEY");
    expect(JSON.stringify(source)).not.toContain("RPC_URL");
  });
});

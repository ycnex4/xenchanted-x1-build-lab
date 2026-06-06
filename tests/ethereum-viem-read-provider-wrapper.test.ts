import { describe, expect, it } from "vitest";
import {
  type ViemLikeBlock,
  type ViemLikeGetBlockInput,
  type ViemLikePublicClient,
  type ViemLikeReadContractInput,
  createEthereumReadProviderFromViemPublicClient,
  createXcEpochMinimumSourceFromEthereumLensProvider
} from "../src/index.js";

const FINALIZED_BLOCK_HASH =
  "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const SAFE_BLOCK_HASH =
  "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
const LATEST_BLOCK_HASH =
  "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const NUMBERED_BLOCK_HASH =
  "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

const LENS_ADDRESS = "0x1111111111111111111111111111111111111111";

class MockViemPublicClient implements ViemLikePublicClient {
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
        timestamp: 900
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
        hash: NUMBERED_BLOCK_HASH,
        timestamp: 1080n
      }
    ]
  ]);

  public minimumsByEpoch = new Map<number, bigint>([
    [0, 100n],
    [1, 50n]
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

describe("Ethereum viem read provider wrapper", () => {
  it("maps viem getChainId number to bigint", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.chainId = 11155111;

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    expect(await provider.getChainId()).toBe(11155111n);
  });

  it("maps finalized block tag to viem getBlock", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const block = await provider.getBlock({ blockTag: "finalized" });

    expect(block).toEqual({
      number: 100n,
      hash: FINALIZED_BLOCK_HASH,
      timestamp: 1000n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "finalized" }]);
  });

  it("maps safe block tag and number timestamp to bigint timestamp", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const block = await provider.getBlock({ blockTag: "safe" });

    expect(block).toEqual({
      number: 90n,
      hash: SAFE_BLOCK_HASH,
      timestamp: 900n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "safe" }]);
  });

  it("maps blockNumber read to viem getBlock", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const block = await provider.getBlock({ blockNumber: 108n });

    expect(block).toEqual({
      number: 108n,
      hash: NUMBERED_BLOCK_HASH,
      timestamp: 1080n
    });
    expect(publicClient.blockReads).toEqual([{ blockNumber: 108n }]);
  });

  it("maps empty getBlock input to latest head block read", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const block = await provider.getBlock({});

    expect(block).toEqual({
      number: 120n,
      hash: LATEST_BLOCK_HASH,
      timestamp: 1200n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "latest" }]);
  });

  it("maps null block to null", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.blocks.set("finalized", null);

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toBeNull();
  });

  it("maps null block number to null", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.blocks.set("finalized", {
      number: null,
      hash: FINALIZED_BLOCK_HASH,
      timestamp: 1000n
    });

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toBeNull();
  });

  it("maps null block hash to hash null", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.blocks.set("finalized", {
      number: 100n,
      hash: null,
      timestamp: 1000n
    });

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toEqual({
      number: 100n,
      hash: null,
      timestamp: 1000n
    });
  });

  it("passes readContract input through unchanged", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const abi = [{ type: "function", name: "epochMinimum" }];

    await provider.readContract({
      address: LENS_ADDRESS,
      abi,
      functionName: "epochMinimum",
      args: [0],
      blockNumber: 100n
    });

    expect(publicClient.contractReads).toEqual([
      {
        address: LENS_ADDRESS,
        abi,
        functionName: "epochMinimum",
        args: [0],
        blockNumber: 100n
      }
    ]);
  });

  it("returns readContract result as unknown", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(
      provider.readContract({
        address: LENS_ADDRESS,
        abi: [],
        functionName: "epochMinimum",
        args: [1],
        blockNumber: 100n
      })
    ).resolves.toBe(50n);
  });

  it("propagates getBlock errors without adding secret-bearing config", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.getBlock = async () => {
      throw new Error("getBlock failed");
    };

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).rejects.toThrow(
      "getBlock failed"
    );
  });

  it("propagates readContract errors without adding secret-bearing config", async () => {
    const publicClient = new MockViemPublicClient();
    publicClient.readContract = async () => {
      throw new Error("readContract failed");
    };

    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    await expect(
      provider.readContract({
        address: LENS_ADDRESS,
        abi: [],
        functionName: "epochMinimum",
        args: [0],
        blockNumber: 100n
      })
    ).rejects.toThrow("readContract failed");
  });

  it("integrates with existing Ethereum Lens provider adapter without real RPC", async () => {
    const publicClient = new MockViemPublicClient();
    const provider = createEthereumReadProviderFromViemPublicClient(publicClient);

    const source = await createXcEpochMinimumSourceFromEthereumLensProvider({
      provider,
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
});

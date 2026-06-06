import { describe, expect, it } from "vitest";
import {
  type EthereumPublicClientGetBlockInput,
  type EthereumPublicClientLike,
  type EthereumPublicClientReadContractInput,
  createEthereumReadProviderFromPublicClient,
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

class MockPublicClient implements EthereumPublicClientLike {
  public readonly blockReads: (EthereumPublicClientGetBlockInput | undefined)[] =
    [];
  public readonly contractReads: EthereumPublicClientReadContractInput[] = [];

  public chainId: number | bigint = 1;

  public blocks = new Map<
    string,
    Awaited<ReturnType<EthereumPublicClientLike["getBlock"]>>
  >([
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

  async getChainId(): Promise<number | bigint> {
    return this.chainId;
  }

  async getBlock(
    input?: EthereumPublicClientGetBlockInput
  ): Promise<Awaited<ReturnType<EthereumPublicClientLike["getBlock"]>>> {
    this.blockReads.push(input);

    if (input?.blockNumber !== undefined) {
      return this.blocks.get(input.blockNumber.toString()) ?? null;
    }

    return this.blocks.get(input?.blockTag ?? "latest") ?? null;
  }

  async readContract(
    input: EthereumPublicClientReadContractInput
  ): Promise<unknown> {
    this.contractReads.push(input);

    const lockEpoch = input.args[0];

    if (typeof lockEpoch !== "number") {
      return null;
    }

    return this.minimumsByEpoch.get(lockEpoch) ?? null;
  }
}

describe("Ethereum read provider wrapper", () => {
  it("maps getChainId result to bigint", async () => {
    const publicClient = new MockPublicClient();
    publicClient.chainId = 11155111;

    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    expect(await provider.getChainId()).toBe(11155111n);
  });

  it("maps bigint getChainId result to bigint", async () => {
    const publicClient = new MockPublicClient();
    publicClient.chainId = 1n;

    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    expect(await provider.getChainId()).toBe(1n);
  });

  it("maps finalized block tag to public client getBlock", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    const block = await provider.getBlock({ blockTag: "finalized" });

    expect(block).toEqual({
      number: 100n,
      hash: FINALIZED_BLOCK_HASH,
      timestamp: 1000n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "finalized" }]);
  });

  it("maps safe block tag and number timestamp to bigint timestamp", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    const block = await provider.getBlock({ blockTag: "safe" });

    expect(block).toEqual({
      number: 90n,
      hash: SAFE_BLOCK_HASH,
      timestamp: 900n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "safe" }]);
  });

  it("maps blockNumber read to public client getBlock", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    const block = await provider.getBlock({ blockNumber: 108n });

    expect(block).toEqual({
      number: 108n,
      hash: NUMBERED_BLOCK_HASH,
      timestamp: 1080n
    });
    expect(publicClient.blockReads).toEqual([{ blockNumber: 108n }]);
  });

  it("maps empty getBlock input to latest head block read", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    const block = await provider.getBlock({});

    expect(block).toEqual({
      number: 120n,
      hash: LATEST_BLOCK_HASH,
      timestamp: 1200n
    });
    expect(publicClient.blockReads).toEqual([{ blockTag: "latest" }]);
  });

  it("maps missing block to null", async () => {
    const publicClient = new MockPublicClient();
    publicClient.blocks.delete("finalized");

    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toBeNull();
  });

  it("maps missing block number to null", async () => {
    const publicClient = new MockPublicClient();
    publicClient.blocks.set("finalized", {
      number: null,
      hash: FINALIZED_BLOCK_HASH,
      timestamp: 1000n
    });

    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toBeNull();
  });

  it("maps missing block hash to hash null", async () => {
    const publicClient = new MockPublicClient();
    publicClient.blocks.set("finalized", {
      number: 100n,
      hash: null,
      timestamp: 1000n
    });

    const provider = createEthereumReadProviderFromPublicClient(publicClient);

    await expect(provider.getBlock({ blockTag: "finalized" })).resolves.toEqual({
      number: 100n,
      hash: null,
      timestamp: 1000n
    });
  });

  it("passes readContract input through unchanged", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

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

  it("integrates with existing Ethereum Lens provider adapter without real RPC", async () => {
    const publicClient = new MockPublicClient();
    const provider = createEthereumReadProviderFromPublicClient(publicClient);

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

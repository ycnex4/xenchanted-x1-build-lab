import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  type EthereumBlockReadInput,
  type EthereumBlockSnapshot,
  type EthereumContractReadInput,
  type EthereumFinalityPolicy,
  type EthereumReadProvider,
  type EthereumXcLensProviderAdapterInput,
  createXcEpochMinimumSourceFromEthereumLensProvider
} from "../src/index.js";

const FINALIZED_BLOCK_HASH =
  "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const SAFE_BLOCK_HASH =
  "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HEAD_BLOCK_HASH =
  "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const CONFIRMED_BLOCK_HASH =
  "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";

const LENS_ADDRESS = "0x1111111111111111111111111111111111111111";

class MockEthereumReadProvider implements EthereumReadProvider {
  public readonly blockReads: EthereumBlockReadInput[] = [];
  public readonly contractReads: EthereumContractReadInput[] = [];

  public chainId = 1n;

  public finalizedBlock: EthereumBlockSnapshot | null = {
    number: 100n,
    hash: FINALIZED_BLOCK_HASH,
    timestamp: 1000n
  };

  public safeBlock: EthereumBlockSnapshot | null = {
    number: 90n,
    hash: SAFE_BLOCK_HASH,
    timestamp: 900n
  };

  public headBlock: EthereumBlockSnapshot | null = {
    number: 120n,
    hash: HEAD_BLOCK_HASH,
    timestamp: 1200n
  };

  public blocksByNumber = new Map<bigint, EthereumBlockSnapshot>([
    [
      108n,
      {
        number: 108n,
        hash: CONFIRMED_BLOCK_HASH,
        timestamp: 1080n
      }
    ]
  ]);

  public minimumsByEpoch = new Map<number, bigint>([
    [0, 100n],
    [1, 50n]
  ]);

  async getChainId(): Promise<bigint> {
    return this.chainId;
  }

  async getBlock(
    input: EthereumBlockReadInput
  ): Promise<EthereumBlockSnapshot | null> {
    this.blockReads.push(input);

    if (input.blockTag === "finalized") {
      return this.finalizedBlock;
    }

    if (input.blockTag === "safe") {
      return this.safeBlock;
    }

    if (input.blockNumber !== undefined) {
      return this.blocksByNumber.get(input.blockNumber) ?? null;
    }

    return this.headBlock;
  }

  async readContract(input: EthereumContractReadInput): Promise<unknown> {
    this.contractReads.push(input);

    const lockEpoch = input.args[0];

    if (typeof lockEpoch !== "number") {
      return null;
    }

    return this.minimumsByEpoch.get(lockEpoch) ?? null;
  }
}

function validInput(
  provider: EthereumReadProvider,
  overrides: Partial<EthereumXcLensProviderAdapterInput> = {}
): EthereumXcLensProviderAdapterInput {
  return {
    provider,
    chainId: "eip155-1",
    lensAddress: LENS_ADDRESS,
    finalityPolicy: { kind: "finalized" },
    lockEpochs: [0, 1],
    ...overrides
  };
}

async function expectInvalidProviderInput(
  input: EthereumXcLensProviderAdapterInput
): Promise<void> {
  await expect(
    createXcEpochMinimumSourceFromEthereumLensProvider(input)
  ).rejects.toThrow(BuildError);

  try {
    await createXcEpochMinimumSourceFromEthereumLensProvider(input);
  } catch (error) {
    expect(error).toBeInstanceOf(BuildError);
    expect((error as BuildError).code).toBe(
      BuildErrorCode.InvalidXcEpochMinimumRecord
    );
  }
}

describe("Ethereum XC epoch minimum provider source", () => {
  it("selects finalized block and builds source", async () => {
    const provider = new MockEthereumReadProvider();

    const source = await createXcEpochMinimumSourceFromEthereumLensProvider(
      validInput(provider)
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(source.authoritativeEpochMinimum(1)).toBe(50n);
    expect(provider.blockReads).toEqual([{ blockTag: "finalized" }]);
    expect(provider.contractReads.map((read) => read.blockNumber)).toEqual([
      100n,
      100n
    ]);
  });

  it("selects safe block and builds source", async () => {
    const provider = new MockEthereumReadProvider();

    const source = await createXcEpochMinimumSourceFromEthereumLensProvider(
      validInput(provider, { finalityPolicy: { kind: "safe" } })
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(provider.blockReads).toEqual([{ blockTag: "safe" }]);
    expect(provider.contractReads.map((read) => read.blockNumber)).toEqual([
      90n,
      90n
    ]);
  });

  it("selects confirmed block with positive confirmations", async () => {
    const provider = new MockEthereumReadProvider();

    const source = await createXcEpochMinimumSourceFromEthereumLensProvider(
      validInput(provider, {
        finalityPolicy: { kind: "confirmed", confirmations: 12 }
      })
    );

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(provider.blockReads).toEqual([{}, { blockNumber: 108n }]);
    expect(provider.contractReads.map((read) => read.blockNumber)).toEqual([
      108n,
      108n
    ]);
  });

  it("rejects latest finality policy", async () => {
    const provider = new MockEthereumReadProvider();

    await expectInvalidProviderInput(
      validInput(provider, {
        finalityPolicy: { kind: "latest" } as unknown as EthereumFinalityPolicy
      })
    );
  });

  it("rejects confirmed finality policy without positive confirmations", async () => {
    const provider = new MockEthereumReadProvider();

    await expectInvalidProviderInput(
      validInput(provider, {
        finalityPolicy: { kind: "confirmed", confirmations: 0 }
      })
    );
  });

  it("rejects provider chain ID mismatch", async () => {
    const provider = new MockEthereumReadProvider();
    provider.chainId = 11155111n;

    await expectInvalidProviderInput(validInput(provider));
  });

  it("rejects invalid configured chain ID", async () => {
    const provider = new MockEthereumReadProvider();

    await expectInvalidProviderInput(
      validInput(provider, { chainId: "ethereum-mainnet" })
    );
  });

  it("rejects invalid Lens address", async () => {
    const provider = new MockEthereumReadProvider();

    await expectInvalidProviderInput(
      validInput(provider, { lensAddress: "0xlens" })
    );
  });

  it("rejects selected block without hash", async () => {
    const provider = new MockEthereumReadProvider();
    provider.finalizedBlock = {
      number: 100n,
      hash: null,
      timestamp: 1000n
    };

    await expectInvalidProviderInput(validInput(provider));
  });

  it("rejects empty requested lock epochs", async () => {
    const provider = new MockEthereumReadProvider();

    await expectInvalidProviderInput(validInput(provider, { lockEpochs: [] }));
  });

  it("rejects invalid contract read result", async () => {
    const provider = new MockEthereumReadProvider();
    provider.minimumsByEpoch.set(1, 0n);

    await expectInvalidProviderInput(validInput(provider));
  });

  it("passes normalized Lens address and selected block number into reads", async () => {
    const provider = new MockEthereumReadProvider();

    await createXcEpochMinimumSourceFromEthereumLensProvider(
      validInput(provider, {
        lensAddress: "0xABCDEFabcdefABCDEFabcdefABCDEFabcdefABCD",
        lockEpochs: [0],
        epochMinimumFunctionName: "epochMinimum"
      })
    );

    expect(provider.contractReads).toEqual([
      {
        address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
        abi: [],
        functionName: "epochMinimum",
        args: [0],
        blockNumber: 100n
      }
    ]);
  });

  it("propagates snapshot validation through existing snapshot adapter", async () => {
    const provider = new MockEthereumReadProvider();
    provider.finalizedBlock = {
      number: 100n,
      hash: "0xblock",
      timestamp: 1000n
    };

    await expectInvalidProviderInput(validInput(provider));
  });

  it("returns null for missing epoch through resulting source", async () => {
    const provider = new MockEthereumReadProvider();

    const source = await createXcEpochMinimumSourceFromEthereumLensProvider(
      validInput(provider)
    );

    expect(source.authoritativeEpochMinimum(2)).toBeNull();
  });
});

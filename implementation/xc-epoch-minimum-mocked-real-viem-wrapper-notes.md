# XC epoch minimum mocked real viem wrapper notes

This milestone implements the real viem wrapper boundary with a structurally typed mocked viem PublicClient.

The implementation does not install viem, does not import viem, does not perform real RPC, does not read env, does not accept secrets, does not add an RPC URL factory, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Purpose

The previous real viem wrapper design review concluded that the first implementation should remain structurally typed and use mocked viem PublicClient objects.

This milestone adds a read-only wrapper that adapts a viem-like PublicClient shape into the already-reviewed EthereumReadProvider interface.

## Runtime additions

Added:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

Exported through:

    src/index.ts

New exports:

    ViemLikeBlock
    ViemLikePublicClient
    ViemLikeGetBlockInput
    ViemLikeReadContractInput
    createEthereumReadProviderFromViemPublicClient()

## Boundary

The wrapper lives outside src/model.

The model layer remains provider-library agnostic.

No viem dependency was installed.

No viem runtime import was added.

No ethers dependency or import was added.

The wrapper imports only model-facing EthereumReadProvider types and exposes an infrastructure adapter from viem-like client shape to EthereumReadProvider.

## Public client shape

The wrapper accepts a structurally typed read-only public client:

    ViemLikePublicClient {
      getChainId(): Promise<number>;
      getBlock(input): Promise<ViemLikeBlock | null>;
      readContract(input): Promise<unknown>;
    }

The wrapper does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet client
    account
    env config

## Mapping behavior

getChainId:

    number -> bigint

getBlock:

    { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
    { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
    { blockNumber } -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

The empty getBlock input maps to latest only for confirmed-policy head calculation.

The wrapper does not reinterpret empty input as finalized or safe.

The wrapper does not silently downgrade finalized or safe to latest.

Block result mapping:

    null block -> null
    null block number -> null
    null block hash -> hash: null
    timestamp number | bigint -> bigint

readContract:

    address passed unchanged
    abi passed unchanged
    functionName passed unchanged
    args passed unchanged
    blockNumber passed unchanged
    decoded result returned as unknown

The wrapper does not validate epoch minimum economics.

## Tests

Added:

    tests/ethereum-viem-read-provider-wrapper.test.ts

Covered:

    viem getChainId number maps to bigint
    finalized block tag maps to viem getBlock
    safe block tag maps to viem getBlock
    number timestamp maps to bigint timestamp
    blockNumber read maps to viem getBlock
    empty getBlock input maps to latest head block read
    null block maps to null
    null block number maps to null
    null block hash maps to hash null
    readContract input passes through unchanged
    readContract result returns as unknown
    getBlock errors propagate without adding secret-bearing config
    readContract errors propagate without adding secret-bearing config
    integration with existing Ethereum Lens provider adapter without real RPC

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    34 test files passed
    251 tests passed

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC
    viem dependency
    viem runtime imports
    ethers dependency
    env reads
    RPC URL factory
    private keys
    API keys
    mnemonic
    signer support
    wallet client support
    account support
    transaction sending
    CLI commands
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Conclusion

The mocked real viem wrapper now proves the intended real viem wrapper boundary while remaining structurally typed and dependency-free.

It adapts a viem-like PublicClient into EthereumReadProvider and integrates with the existing Ethereum Lens provider adapter without real network access.

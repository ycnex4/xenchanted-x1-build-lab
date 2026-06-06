# XC epoch minimum mocked Ethereum provider wrapper notes

This milestone implements the first mocked Ethereum provider wrapper against a viem-style public client shape.

The implementation does not perform real RPC, does not read env, does not accept secrets, does not accept private keys, does not accept signers, and does not add a direct RPC URL factory.

## Purpose

The previous provider wrapper design review concluded that the first implementation should use a mocked viem-style public client shape and adapt it into the already-reviewed EthereumReadProvider interface.

This milestone adds an infrastructure-style wrapper:

    mocked public client shape
    -> createEthereumReadProviderFromPublicClient(publicClient)
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()

## Runtime additions

Added:

    src/ethereum/ethereum-read-provider-wrapper.ts

Exported through:

    src/index.ts

New exports:

    EthereumPublicClientBlock
    EthereumPublicClientLike
    EthereumPublicClientGetBlockInput
    EthereumPublicClientReadContractInput
    createEthereumReadProviderFromPublicClient()

## Boundary

The wrapper lives outside src/model.

The model layer remains provider-library agnostic.

The wrapper imports the existing EthereumReadProvider model-facing types, but the model layer does not import the wrapper.

No viem or ethers dependency was introduced.

The public client interface is viem-style but mocked / dependency-free.

## Public client shape

The wrapper accepts:

    EthereumPublicClientLike {
      getChainId(): Promise<number | bigint>;
      getBlock(input?): Promise<EthereumPublicClientBlock | null>;
      readContract(input): Promise<unknown>;
    }

The wrapper does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet account
    env config

## Mapping behavior

getChainId:

    number | bigint -> bigint

getBlock:

    { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
    { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
    { blockNumber } -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

The empty getBlock input maps to latest only for confirmed-policy head calculation in the existing provider adapter.

The wrapper does not reinterpret empty input as finalized or safe.

Block result mapping:

    missing block -> null
    missing block number -> null
    hash preserved as string | null
    timestamp number | bigint -> bigint

readContract:

    address passed unchanged
    abi passed unchanged
    functionName passed unchanged
    args passed unchanged
    blockNumber passed unchanged
    raw decoded result returned as unknown

## Tests

Added:

    tests/ethereum-read-provider-wrapper.test.ts

Covered:

    getChainId number result maps to bigint
    getChainId bigint result maps to bigint
    finalized block tag maps to public client getBlock
    safe block tag maps to public client getBlock
    number timestamp maps to bigint timestamp
    blockNumber read maps to public client getBlock
    empty getBlock input maps to latest head block read
    missing block maps to null
    missing block number maps to null
    missing block hash maps to hash null
    readContract input passes through unchanged
    integration with existing Ethereum Lens provider adapter without real RPC

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    33 test files passed
    238 tests passed

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC
    env reads
    RPC URL factory
    private keys
    API keys
    mnemonic
    signer support
    wallet support
    transaction sending
    CLI commands
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Conclusion

The mocked Ethereum provider wrapper now adapts a viem-style public client shape into EthereumReadProvider without moving provider-library dependencies, RPC URLs, env, secrets, or signers into the model layer.

It proves the concrete wrapper boundary and integrates with the existing Ethereum Lens provider adapter without real network access.

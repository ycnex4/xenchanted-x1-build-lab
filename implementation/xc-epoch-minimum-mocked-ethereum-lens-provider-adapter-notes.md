# XC epoch minimum mocked Ethereum Lens provider adapter notes

This milestone implements the first mocked read-only provider adapter for Ethereum / XC Lens epoch minimum sources.

The implementation does not perform real RPC reads, does not read env, does not require secrets, does not accept private keys, does not accept direct RPC URLs, and does not add CLI commands.

## Purpose

The previous provider adapter design and review concluded that the first implementation should prove the provider boundary with mocked provider tests only.

This milestone adds a read-only provider abstraction that can produce an EthereumXcLensEpochMinimumSnapshot and then reuse the already-reviewed snapshot adapter:

    provider
    -> selected finalized / safe / confirmed provenance block
    -> read epoch minimums at selected block number
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

## Runtime additions

Added:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts

Exported through:

    src/index.ts

New exports:

    EthereumReadProvider
    EthereumBlockReadInput
    EthereumBlockSnapshot
    EthereumContractReadInput
    EthereumXcLensProviderAdapterInput
    createXcEpochMinimumSourceFromEthereumLensProvider()

## Provider interface

The provider adapter uses a custom read-only provider interface:

    EthereumReadProvider {
      getChainId(): Promise<bigint>;
      getBlock(input): Promise<EthereumBlockSnapshot | null>;
      readContract(input): Promise<unknown>;
    }

This keeps the model layer independent from concrete provider libraries.

No direct viem / ethers dependency was introduced.

## Input shape

The provider adapter accepts:

    provider
    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    optional epochMinimumFunctionName
    optional epochMinimumAbi

The adapter does not accept:

    RPC URLs
    private keys
    mnemonic
    API keys
    wallet / signer accounts
    env config

## Validation policy

The adapter validates:

    configured chainId must match eip155-<number>
    provider chain ID must match configured chain ID
    lensAddress must be 0x-prefixed 20-byte hex
    finalityPolicy must be finalized, safe, or confirmed
    confirmed finality requires positive integer confirmations
    lockEpochs must be non-empty
    selected provenance block must exist
    selected provenance block number must be > 0
    selected provenance block hash must be present
    selected provenance block timestamp must be > 0
    contract read result must decode to bigint
    minimumXntd must be > 0

Invalid provider input or read results use the existing source-record error:

    InvalidXcEpochMinimumRecord

No new error code was added.

## Finality behavior

finalized:

    getBlock({ blockTag: "finalized" })
    use finalized block number for all contract reads

safe:

    getBlock({ blockTag: "safe" })
    use safe block number for all contract reads

confirmed:

    getBlock({}) to read head only for block-number calculation
    confirmedBlockNumber = head.number - confirmations
    getBlock({ blockNumber: confirmedBlockNumber })
    use confirmed block number for all contract reads

latest is rejected.

The adapter does not use latest as the provenance block.

## Snapshot conversion

The adapter builds:

    EthereumXcLensEpochMinimumSnapshot

with:

    sourceChainId = configured chainId
    sourceBlockNumber = selected block number
    sourceBlockHash = selected block hash
    observedAt = selected block timestamp
    finalizedPolicy = input finality policy
    epochMinimums = provider read results

Then it calls:

    createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot)

This preserves the snapshot adapter as the validation boundary for Ethereum-shaped snapshots.

## Tests

Added:

    tests/ethereum-xc-epoch-minimum-provider-source.test.ts

Covered:

    finalized block selection and source build
    safe block selection and source build
    confirmed block selection with positive confirmations
    latest finality rejection
    confirmed finality without positive confirmations rejection
    provider chain ID mismatch rejection
    invalid configured chain ID rejection
    invalid Lens address rejection
    selected block without hash rejection
    empty requested lockEpochs rejection
    invalid contract read result rejection
    normalized Lens address and selected block number passed into reads
    snapshot validation propagation through existing snapshot adapter
    missing epoch returns null through resulting source

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    32 test files passed
    227 tests passed

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC
    env reads
    RPC URL config
    private keys
    API keys
    mnemonic
    signer support
    transaction sending
    CLI commands
    snapshot persistence migration
    bridge signer verification
    X1-native verification

## Conclusion

The mocked Ethereum Lens provider adapter is now implemented as a read-only provider layer.

It proves the provider boundary, finality block selection, chain/address validation, selected-block read consistency, and snapshot conversion without real network access.

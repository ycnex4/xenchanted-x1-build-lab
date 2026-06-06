# XC epoch minimum mocked Ethereum Lens provider adapter review notes

This review milestone checks the mocked Ethereum Lens provider adapter runtime boundary before any real provider wrapper design.

Reviewed implementation:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts

Reviewed tests:

    tests/ethereum-xc-epoch-minimum-provider-source.test.ts

Reviewed notes:

    implementation/xc-epoch-minimum-mocked-ethereum-lens-provider-adapter-notes.md

Reviewed commits:

    6c7414a Add mocked Ethereum XC epoch minimum provider adapter
    ffe0087 Add mocked Ethereum XC epoch minimum provider adapter notes
    1f9a411 Update checkpoint after mocked Ethereum XC epoch minimum provider adapter
    bcf1495 Merge branch 'xc-epoch-minimum-mocked-ethereum-lens-provider-adapter'

## Review conclusion

The mocked Ethereum Lens provider adapter runtime boundary is clean.

The implementation remains a mocked read-only provider layer.

It does not perform real Ethereum RPC and does not introduce env / secret / direct RPC URL coupling.

The adapter correctly produces an EthereumXcLensEpochMinimumSnapshot and delegates Ethereum-shaped snapshot validation to the existing snapshot adapter:

    provider
    -> selected finalized / safe / confirmed provenance block
    -> read epoch minimums at selected block number
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

## Runtime boundary review

The runtime adapter does not import or call:

    process.env
    fetch
    http / https
    viem
    ethers
    wallet APIs
    signer APIs

The provider adapter receives only a custom read-only provider object.

The adapter input does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet account
    env config

A targeted grep over the runtime file, test file, and implementation notes found RPC / secret / provider-library terms only in the notes file where they are documented as non-goals.

No secret-bearing files were inspected.

## Provider interface review

The runtime uses the intended custom read-only interface:

    EthereumReadProvider {
      getChainId()
      getBlock()
      readContract()
    }

This confirms the design decision:

    keep provider construction outside model code
    keep RPC URL / API key handling outside adapter input
    use mocked provider objects in tests
    avoid direct viem / ethers dependency in this layer

This is the correct boundary before any real provider wrapper.

## Finality behavior review

The implementation correctly supports:

    finalized
    safe
    confirmed

finalized:

    getBlock({ blockTag: "finalized" })
    all contract reads use finalized block number

safe:

    getBlock({ blockTag: "safe" })
    all contract reads use safe block number

confirmed:

    getBlock({}) is used only to get head block number
    confirmedBlockNumber = head.number - confirmations
    getBlock({ blockNumber: confirmedBlockNumber })
    all contract reads use confirmed block number

latest is not supported as a provenance policy.

Confirmed finality requires confirmations > 0.

## Selected block consistency review

The tests verify that all contract reads use the selected provenance block number.

This is essential because one snapshot must correspond to one provenance block.

The adapter also requires:

    selected block exists
    selected block number > 0
    selected block hash is present
    selected block timestamp > 0

## Chain and address validation review

The adapter validates:

    configured chainId matches eip155-<number>
    provider chain ID is converted to eip155-<number>
    provider chain ID must match configured chainId
    lensAddress must be 0x-prefixed 20-byte hex
    lensAddress is normalized to lowercase before contract reads

This validation remains Ethereum-specific and is not pushed into the generic source builder.

## Snapshot validation boundary review

The provider adapter intentionally does not duplicate all Ethereum snapshot validation.

It builds EthereumXcLensEpochMinimumSnapshot and then calls:

    createXcEpochMinimumSourceFromEthereumLensSnapshot(snapshot)

This preserves the already-reviewed snapshot adapter as the Ethereum-shaped validation boundary.

The test suite includes a snapshot-validation propagation case with an invalid block hash.

## Error model review

The implementation reuses:

    InvalidXcEpochMinimumRecord

for provider config / read-result / snapshot validation failures.

No new error code was added.

This remains acceptable for the mocked provider adapter stage.

A more specific adapter-config error can be considered later only if real provider wrapper implementation demonstrates a clear need.

## Test coverage review

The current provider adapter tests cover:

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

This is sufficient for the current mocked provider adapter milestone.

## Additional edge-case test decision

No additional tests are required before merging this review milestone.

Possible future tests for a real provider wrapper stage may include:

    concrete viem wrapper maps block responses into EthereumBlockSnapshot
    concrete provider wrapper never exposes RPC URL to model layer
    wrapper handles provider-specific finalized / safe support differences
    wrapper handles contract read decode errors
    wrapper handles provider unavailable errors

Those are not necessary in the current mocked provider adapter layer.

## Verification

After implementation and review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    32 test files passed
    227 tests passed

## Conclusion

The mocked Ethereum Lens provider adapter is safe to keep as the read-only provider boundary.

It proves finality block selection, selected-block read consistency, chain/address validation, and snapshot conversion without real network access.

Recommended next milestone:

    xc-epoch-minimum-ethereum-provider-wrapper-design

Suggested next scope:

    design concrete provider wrapper boundary only
    decide whether viem or ethers wrapper should be used externally
    keep RPC URLs / env / API keys outside model code
    define how wrapper maps provider block reads to EthereumBlockSnapshot
    define how wrapper maps contract reads to unknown results
    do not implement real RPC until wrapper design is reviewed

# XC epoch minimum Ethereum Lens provider adapter design review notes

This review milestone checks the Ethereum Lens provider / ABI adapter design before any implementation.

Reviewed design:

    implementation/xc-epoch-minimum-ethereum-lens-provider-adapter-design.md

Reviewed branch:

    xc-epoch-minimum-ethereum-lens-provider-adapter-design

Reviewed commits:

    90a543e Add XC epoch minimum Ethereum Lens provider adapter design
    c5e4fc1 Update checkpoint after XC epoch minimum Ethereum provider adapter design
    cf44d52 Merge branch 'xc-epoch-minimum-ethereum-lens-provider-adapter-design'

## Review conclusion

The provider adapter design boundary is clean.

The milestone is design-only.

No runtime implementation was added.

The document correctly keeps the future provider adapter as a thin read-only layer that produces the already-reviewed Ethereum snapshot shape:

    provider read at finalized / safe / confirmed block
    -> XC Lens / Core calls
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

The existing mocked snapshot adapter remains the deterministic validation boundary.

## Runtime scope review

The design does not add:

    runtime code
    real RPC reads
    provider configuration
    ABI calls
    CLI commands
    snapshot persistence
    env loading
    secrets

This is the correct boundary for the current stage.

## Secret / RPC / env coupling review

The design explicitly rejects secret-bearing coupling inside model code.

The future provider adapter should not:

    read process.env directly
    accept private keys
    accept RPC URLs directly
    accept mnemonics
    accept API keys
    log provider URLs
    log headers
    print raw config

The correct future pattern remains:

    outer integration layer constructs provider
    provider is passed into adapter
    adapter performs read-only calls through a narrow interface

A targeted grep over the design file found secret / RPC / ABI terms only in sections that define boundaries, non-goals, or future design constraints.

No secret-bearing files were inspected.

## Provider interface review

The design correctly recommends a custom read-only provider interface before binding to a concrete provider library.

Preferred future abstraction:

    EthereumReadProvider {
      getChainId()
      getBlock()
      readContract()
    }

This is better than immediately depending on viem or ethers because:

    tests can use mocked provider objects
    provider construction stays outside model code
    no RPC URL enters adapter input
    no private key / signer concept enters adapter input
    adapter remains deterministic except for explicit read calls
    implementation can later wrap viem / ethers behind this interface

Review decision:

    use a custom read-only provider interface first

Do not introduce a direct viem / ethers dependency in the first provider adapter implementation unless it is isolated behind this interface.

## Finality policy review

The design correctly rejects latest as a provenance source.

Allowed future policies remain:

    finalized
    safe
    confirmed

Finalized and safe policies should select a block by tag and use that same block number for all reads.

Confirmed policy may read head only to calculate an older confirmed block number, then must read the selected confirmed block by number and use that block for all contract reads.

Review decision:

    keep latest unsupported
    require confirmations > 0 for confirmed policy
    require selected block hash
    require all reads at selected block number

## Chain and address policy review

The provider adapter should validate configured chain ID against provider chain ID before producing records.

Expected configured format:

    eip155-<number>

The provider adapter should require explicit Lens / Core addresses and should not hardcode them.

Address validation should remain Ethereum-specific:

    0x-prefixed
    20-byte hex
    normalized lowercase or checksum-preserving comparison policy

Review decision:

    add address validation in provider adapter implementation
    do not add Ethereum address validation to generic source builder

## ABI / epoch minimum strategy review

The design intentionally does not choose the final ABI yet.

Possible strategies remain:

    direct Lens epoch minimum read
    Core protocol constants + local computation
    checkpointed Ethereum reads

Review decision:

    first implementation should support the read-only provider boundary and mocked provider tests
    do not perform real RPC yet
    do not lock final ABI in runtime until actual XC Lens/Core view source is confirmed

Preferred first real strategy after mocked provider implementation:

    direct Lens epoch minimum read if Lens exposes historical epoch minimums

Fallback strategy:

    protocol constants + local computation if direct historical minimums are not exposed

Reason:

    direct Lens read has less local economic logic
    protocol constants + local computation is viable but requires stronger contract-example tests

## observedAt review

The design recommends selected Ethereum block timestamp as the cleanest first observedAt value for provider-produced snapshots.

Review decision:

    use selected block timestamp for observedAt in the first mocked provider adapter implementation

This keeps source provenance tied to the same selected block used for reads.

## Requested lockEpochs review

The design requires explicit requested lockEpochs and rejects implicit unbounded epoch scans.

Review decision:

    provider adapter implementation should require non-empty lockEpochs

Reason:

    proof validation needs specific epochs
    bounded reads are easier to audit
    tests remain deterministic
    no unbounded historical scan behavior

## Error model review

The design currently avoids adding new error codes.

Review decision:

    reuse InvalidXcEpochMinimumRecord for first mocked provider adapter implementation unless implementation reveals a real need for a dedicated adapter-config error

Potential future dedicated error can be considered later if config failures become distinct enough from source record failures.

## Testing strategy review

The proposed implementation test strategy is appropriate.

First mocked provider adapter implementation should test:

    finalized block selection
    safe block selection
    confirmed block selection with positive confirmations
    latest policy rejection
    provider chain ID mismatch rejection
    invalid configured chain ID rejection
    invalid Lens address rejection
    missing block hash rejection
    all reads performed at selected block number
    empty requested lockEpochs rejection
    invalid read result rejection
    snapshot validation propagation
    no process.env reads
    no private keys
    no RPC URL in adapter input

## Next implementation recommendation

The next implementation milestone should be:

    xc-epoch-minimum-mocked-ethereum-lens-provider-adapter

Scope:

    implement mocked read-only provider interface
    no real RPC
    no env reads
    no secrets
    no private keys
    no direct RPC URL input
    no CLI command
    produce EthereumXcLensEpochMinimumSnapshot
    reuse createXcEpochMinimumSourceFromEthereumLensSnapshot()
    tests only with mocked provider

## Final conclusion

The Ethereum Lens provider / ABI adapter design is ready to proceed to a mocked provider implementation milestone.

The implementation should not perform real Ethereum RPC yet.

The first implementation should prove the provider boundary, finality block selection, chain/address validation, selected-block read consistency, and snapshot conversion using mocked provider tests only.

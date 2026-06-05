# XC epoch minimum mocked Ethereum Lens snapshot adapter review notes

This review milestone checks the mocked Ethereum Lens snapshot adapter boundary before any real RPC / ABI work.

The reviewed implementation was introduced by:

    462b786 Add mocked Ethereum XC epoch minimum source adapter
    c2033b9 Add mocked Ethereum XC epoch minimum adapter notes
    4400fc9 Update checkpoint after mocked Ethereum XC epoch minimum adapter
    29bb14c Merge branch 'xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter'

## Reviewed files

Runtime:

    src/model/ethereum-xc-epoch-minimum-source.ts

Tests:

    tests/ethereum-xc-epoch-minimum-source.test.ts

Notes:

    implementation/xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter-notes.md

Checkpoint:

    docs/checkpoints/current-design-checkpoint.md

## Boundary review

The adapter remains source-specific and deterministic.

It accepts an already prepared mocked Ethereum Lens snapshot and validates Ethereum-shaped metadata before producing generic records.

Current adapter flow:

    EthereumXcLensEpochMinimumSnapshot
    -> Ethereum-specific metadata validation
    -> XcEpochMinimumRecord[]
    -> createXcEpochMinimumSourceFromRecords()
    -> XcEpochMinimumSource

The generic source builder remains source-agnostic.

Ethereum-specific validation is not pushed into:

    src/model/xc-epoch-minimum-source.ts

This preserves the intended boundary:

    generic layer validates generic record invariants
    Ethereum adapter validates Ethereum-shaped provenance metadata

## Network / secret / ABI assumptions review

The implementation does not introduce:

    real RPC reads
    provider configuration
    RPC URLs
    private keys
    API keys
    ABIs
    fetch / HTTP calls
    viem / ethers dependencies
    process.env reads
    CLI integration
    snapshot persistence changes

A targeted grep over the new runtime, test, and notes files found network / secret / ABI terms only in the notes file where they are explicitly documented as non-goals.

No secret-bearing files were inspected.

## TypeScript / exact optional property review

The implementation is compatible with the current TypeScript settings.

The finality policy runtime guard intentionally treats the policy as an unknown-shaped object internally:

    const policy = finalizedPolicy as { kind?: unknown; confirmations?: unknown };

This allows tests to verify invalid runtime payload shapes while preserving the exported strict union type:

    finalized
    safe
    confirmed with confirmations

This is useful because external adapter input may be malformed even when TypeScript types describe the intended shape.

## Validation policy review

The implemented Ethereum-specific validation matches the planned mocked snapshot policy:

    sourceChainId must match eip155-<number>
    sourceBlockNumber must be > 0
    sourceBlockHash must be 0x-prefixed 32-byte hex
    observedAt must be > 0
    finalizedPolicy kind must be finalized, safe, or confirmed
    confirmed finality requires positive integer confirmations
    epochMinimums must be non-empty

The adapter lowercases valid Ethereum block hashes before mapping entries into records.

Generic epoch record validation remains delegated to the existing generic builder:

    lockEpoch integer and >= 0
    minimumXntd > 0
    observedAt > 0
    sourceBlockNumber > 0 when provided
    duplicate epoch records allowed only when minimumXntd matches
    conflicting duplicates rejected

## Test coverage review

The current test file covers the intended policy cases:

    valid mocked Ethereum Lens snapshot
    mixed-case block hash acceptance / normalization
    missing or empty sourceChainId rejection
    non-EIP-155 sourceChainId rejection
    non-positive sourceBlockNumber rejection
    missing / invalid sourceBlockHash rejection
    non-positive observedAt rejection
    safe finality acceptance
    confirmed finality with positive confirmations acceptance
    invalid finality kind rejection
    confirmed finality without positive confirmations rejection
    empty epochMinimums rejection
    conflicting duplicate epoch entries rejection
    missing epoch returns null

This is sufficient for the current mocked snapshot adapter stage.

## Additional invalid-shape tests

No additional tests are required before merging this review milestone.

Possible future tests, if the adapter begins accepting unknown JSON-like input directly, may include:

    missing finalizedPolicy object
    finalizedPolicy = null
    epochMinimums not an array
    sourceBlockNumber not bigint
    observedAt not bigint

Those cases are not necessary now because the current function accepts the typed `EthereumXcLensEpochMinimumSnapshot` shape, and this milestone is not a JSON parser or RPC response decoder.

## Conclusion

The mocked Ethereum Lens snapshot adapter boundary is clean.

The adapter is source-specific, deterministic, and production-shaped without becoming a real Ethereum integration.

It is safe to keep this milestone as the last mocked snapshot review before a separate real provider / ABI design milestone.

Recommended next milestone:

    xc-epoch-minimum-ethereum-lens-provider-adapter-design

Suggested next scope:

    design real provider / ABI integration only
    define required XC Core / Lens address inputs
    define finality source policy
    define finalized / safe / confirmed block handling
    define provider trust assumptions
    define no-secret config boundary
    do not implement real RPC yet unless design is reviewed first

# XC epoch minimum mocked Ethereum Lens snapshot adapter notes

This milestone implements the first source-specific XC epoch minimum adapter layer for Ethereum / XC Lens shaped data.

The adapter is intentionally mocked / snapshot-based. It does not perform real RPC reads, does not use provider configuration, does not require ABIs, and does not access secrets.

## Purpose

The authoritative XC epoch minimum runtime chain already validates:

    observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The generic source builder accepts validated XcEpochMinimumRecord[] and creates a deterministic XcEpochMinimumSource.

This milestone adds an Ethereum-specific snapshot adapter that validates Ethereum-shaped source metadata before passing records into the generic builder.

## Added runtime file

    src/model/ethereum-xc-epoch-minimum-source.ts

Exports:

    EthereumXcLensEpochMinimumSnapshot
    EthereumXcEpochMinimumEntry
    EthereumFinalityPolicy
    createXcEpochMinimumSourceFromEthereumLensSnapshot()

## Snapshot shape

    EthereumXcLensEpochMinimumSnapshot {
      sourceChainId: string;
      sourceBlockNumber: bigint;
      sourceBlockHash: string;
      observedAt: bigint;
      finalizedPolicy: EthereumFinalityPolicy;
      epochMinimums: readonly EthereumXcEpochMinimumEntry[];
    }

    EthereumFinalityPolicy =
      | { kind: "finalized" }
      | { kind: "safe" }
      | { kind: "confirmed"; confirmations: number };

## Validation policy

The adapter validates Ethereum-specific metadata:

    sourceChainId must match eip155-<number>
    sourceBlockNumber must be > 0
    sourceBlockHash must be 0x-prefixed 32-byte hex
    observedAt must be > 0
    finalizedPolicy kind must be finalized, safe, or confirmed
    confirmed finality requires positive integer confirmations
    epochMinimums must be non-empty

The adapter normalizes valid Ethereum block hashes to lowercase before building records.

Epoch entry validation remains delegated to the generic source builder:

    lockEpoch integer and >= 0
    minimumXntd > 0
    duplicate epoch records allowed only when minimumXntd matches
    conflicting duplicate epoch minimum records rejected

All invalid snapshot or entry cases use the existing source-record error:

    InvalidXcEpochMinimumRecord

No new error code was added.

## Produced records

Each snapshot entry is mapped to an XcEpochMinimumRecord:

    {
      lockEpoch: entry.lockEpoch,
      minimumXntd: entry.minimumXntd,
      observedAt: snapshot.observedAt,
      sourceChainId: snapshot.sourceChainId,
      sourceBlockNumber: snapshot.sourceBlockNumber,
      sourceBlockHash: normalizedSourceBlockHash
    }

Then the adapter calls:

    createXcEpochMinimumSourceFromRecords(records)

## Tests

Added:

    tests/ethereum-xc-epoch-minimum-source.test.ts

Covered:

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

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC reads
    provider config
    RPC URLs
    private keys
    API keys
    ABIs
    CLI commands
    snapshot persistence changes
    bridge signer verification
    X1-native source verification

The branch remains deterministic and suitable for production-shaped tests without network access.

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    31 test files passed
    213 tests passed

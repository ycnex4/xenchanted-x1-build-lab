# XC Epoch Minimum Source Block Hash Policy

## Branch

xc-epoch-minimum-source-block-hash-policy

## Purpose

This document defines the policy question around sourceBlockHash for XC epoch minimum records.

This is a design-only milestone.

It does not change runtime code.

## Current state

The mocked / production-shaped XC epoch minimum source adapter currently supports:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

Current validation enforces:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- observedAt must be > 0
- sourceBlockNumber must be > 0 when provided
- duplicate records for the same epoch may only repeat the same minimum

Current validation intentionally does not validate sourceBlockHash format.

## Why sourceBlockHash needs a separate policy

sourceBlockHash looks simple, but it depends on the source type.

Possible source types include:

1. Ethereum finalized RPC / XC Lens read
2. trusted integration source
3. checkpoint source
4. bridge-provided source
5. X1-native verified source
6. local deterministic test source

These may not all use the same block hash concept.

For Ethereum records, sourceBlockHash can mean an Ethereum block hash.

For X1 records, the equivalent may be a slot hash, state root, checkpoint hash, or signed message hash.

For trusted / mocked records, sourceBlockHash may be optional metadata.

## Policy question

Should sourceBlockHash be:

1. optional metadata,
2. required only when sourceBlockNumber is provided,
3. required only for Ethereum RPC / Lens records,
4. required for all production records,
5. validated by source-specific adapters instead of the generic record builder?

## Recommended initial policy

Keep sourceBlockHash optional in the generic record builder.

Do not enforce global hash format in createXcEpochMinimumSourceFromRecords().

Reason:

- the generic adapter is source-agnostic
- future sources may not use Ethereum-style hashes
- strict hash validation belongs in source-specific adapters
- tests should remain deterministic and network-free
- no need to reject valid checkpoint / trusted records just because no block hash exists

## Ethereum-specific policy

A future Ethereum / XC Lens source adapter should enforce Ethereum-specific rules.

Recommended Ethereum adapter rules:

- sourceChainId must be present
- sourceChainId should identify Ethereum mainnet / testnet source
- sourceBlockNumber must be present and > 0
- sourceBlockHash must be present
- sourceBlockHash should be a 0x-prefixed 32-byte hex string
- adapter should not read latest
- adapter should use finalized / safe / explicitly confirmed block policy

This belongs in the Ethereum adapter, not in the generic record builder.

## Checkpoint source policy

For checkpoint sources, sourceBlockHash may be replaced or supplemented by:

- checkpointId
- checkpointHash
- checkpointRoot
- signerSetId
- signedAt
- finalizedAt

Do not force checkpoint records into Ethereum block hash semantics.

## Bridge-provided source policy

For bridge-provided sources, the source may include:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- bridgeMessageId
- signerSetId
- attestationHash

Bridge records should be validated by bridge-specific policy.

Do not enforce these fields in the generic record builder yet.

## X1-native source policy

For X1-native verified sources, the equivalent may be:

- slot
- state root
- program-derived checkpoint account
- verified attestation
- canonical registry entry

Again, this should be source-specific.

## Generic adapter responsibility

The generic createXcEpochMinimumSourceFromRecords() should continue to validate only source-agnostic invariants:

- epoch validity
- positive minimum
- positive observation time
- positive sourceBlockNumber if provided
- no conflicting duplicate epoch minimums

It should not validate source-specific provenance.

## Future implementation direction

If source-specific adapters are added, they can create records only after enforcing their own provenance rules.

Example future layers:

1. Ethereum XC Lens adapter
   - validates sourceBlockHash as Ethereum block hash
   - validates finalized block policy
   - produces XcEpochMinimumRecord[]

2. Checkpoint adapter
   - validates checkpoint signatures / roots
   - produces XcEpochMinimumRecord[]

3. Bridge adapter
   - validates bridge signer policy
   - produces XcEpochMinimumRecord[]

The generic record builder remains the final deterministic map builder.

## Non-goals

This milestone does not implement:

- sourceBlockHash validation
- Ethereum RPC
- XC Core ABI
- XC Lens ABI
- checkpoint verification
- bridge signer verification
- X1-native verification
- snapshot schema changes
- CLI integration

## Conclusion

Do not add global sourceBlockHash validation to the generic source adapter.

Keep sourceBlockHash optional at the generic layer.

Add strict hash/provenance validation later only inside source-specific adapters.

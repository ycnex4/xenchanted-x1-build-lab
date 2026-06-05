# XC Epoch Minimum Ethereum Lens Adapter Design

## Branch

xc-epoch-minimum-ethereum-lens-adapter-design

## Purpose

This document designs a future Ethereum / XC Lens source-specific adapter for XC epoch minimum records.

This is a design-only milestone.

It does not change runtime code.

## Current completed foundation

The generic XC epoch minimum source layer is already complete enough for deterministic production-shaped testing.

Current generic flow:

XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource
-> appSubmitProof(..., xcEpochMinimumSource)
-> registrar authoritative validation
-> Build state

The generic layer validates source-agnostic invariants only:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- observedAt must be > 0
- sourceBlockNumber must be > 0 when provided
- duplicate records for the same epoch are allowed only when minimumXntd matches
- conflicting duplicate epoch minimum records are rejected

The generic layer intentionally does not validate sourceBlockHash format.

## Adapter responsibility

The Ethereum / XC Lens adapter should be source-specific.

Its responsibility is to read or receive Ethereum XC state, validate Ethereum-specific provenance, and produce XcEpochMinimumRecord[].

Conceptual flow:

Ethereum finalized/safe/confirmed XC state
-> Ethereum-specific validation
-> XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource

## Non-goal for first implementation

The first implementation should not perform real RPC reads.

The first implementation should use mocked Ethereum read results.

Reason:

- no RPC secrets
- no provider config
- deterministic tests
- no network flakiness
- no ABI/address hardcoding too early
- lets us validate policy before integration

## Candidate adapter name

Possible future name:

createEthereumXcLensEpochMinimumSource()

or:

createXcEpochMinimumSourceFromEthereumLensRead()

Recommended first mocked name:

createXcEpochMinimumSourceFromEthereumLensSnapshot()

Reason:

A snapshot-shaped input is deterministic and can be tested without network access.

## Proposed input shape

A future mocked adapter may accept:

EthereumXcLensEpochMinimumSnapshot {
  sourceChainId: string;
  sourceBlockNumber: bigint;
  sourceBlockHash: string;
  observedAt: bigint;
  finalizedPolicy: EthereumFinalityPolicy;
  epochMinimums: readonly EthereumXcEpochMinimumEntry[];
}

EthereumXcEpochMinimumEntry {
  lockEpoch: number;
  minimumXntd: bigint;
}

EthereumFinalityPolicy {
  kind: "finalized" | "safe" | "confirmed";
  confirmations?: number;
}

This is a design shape only.

Do not implement it in this milestone.

## Required Ethereum-specific fields

For Ethereum / XC Lens source records, the adapter should require:

- sourceChainId
- sourceBlockNumber
- sourceBlockHash
- observedAt
- finalizedPolicy
- epochMinimums

Unlike the generic record builder, the Ethereum-specific adapter should not treat sourceBlockHash as optional.

## sourceChainId policy

sourceChainId should identify the Ethereum source.

Recommended format:

- eip155-1 for Ethereum mainnet
- eip155-11155111 for Sepolia
- future EIP-155 chain IDs for other EVM chains if ever needed

The adapter should reject:

- empty sourceChainId
- non-EIP-155 chain ID format for Ethereum adapter
- unexpected chain ID when config restricts the adapter to one chain

## sourceBlockNumber policy

sourceBlockNumber must be present and > 0.

This is already validated generically if provided, but the Ethereum adapter should require it.

Reason:

Ethereum provenance should be tied to a specific block.

## sourceBlockHash policy

sourceBlockHash must be present.

Recommended format:

- string
- starts with 0x
- exactly 66 characters
- 32 bytes hex after 0x
- lowercase normalization may be applied by adapter, or comparison may be case-insensitive

The generic record builder should not enforce this.

The Ethereum adapter should enforce it.

## observedAt policy

observedAt must be > 0.

This is already enforced by the generic record builder.

The Ethereum adapter should also ensure observedAt is set from adapter observation time, not user payload.

## finalized / safe / confirmed block policy

The adapter should not read latest.

Acceptable future policies:

1. finalized
   - preferred when provider supports finalized block tag
   - strongest default for Ethereum mainnet

2. safe
   - acceptable when finalized is unavailable but safe is supported
   - weaker than finalized

3. confirmed
   - explicit block number with N confirmations
   - useful for providers or chains without finalized/safe support

Recommended initial production-like policy:

- support a mocked finalized snapshot first
- later support finalized block tag
- do not support latest
- reject unknown finality policy

## XC Core / Lens fields

The adapter must be able to derive:

authoritativeEpochMinimum(lockEpoch)

For the current protocol meaning, this should be the XC Core L1 minimum nominal for the relevant epoch.

Candidate XC fields:

- genesisTs
- halvingIntervalSec
- initialNominal
- currentEpoch
- currentBaseNominal
- epochAt(timestamp), if available
- base nominal by epoch, if exposed
- Lens-provided protocol parameters, if available

## Historical epoch policy

The adapter must answer for lockEpoch, not just current epoch.

Possible strategies:

1. Read direct epoch minimum from Lens if available.
2. Compute epoch minimum from initialNominal and halving rules.
3. Use checkpointed epoch minimum records generated from finalized Ethereum reads.
4. Use current epoch only for early controlled tests.

Recommended first design:

- support explicit epochMinimums in mocked snapshot
- later define whether production reads direct values or computes from protocol constants

Reason:

The current repository can test adapter shape without deciding final XC Lens ABI.

## Epoch boundary policy

Epoch boundaries are sensitive.

Questions for future implementation:

- Is lockEpoch derived from X1 event time?
- Is lockEpoch supplied by the proof payload?
- Should adapter cross-check lockEpoch against Ethereum timestamp?
- Should adapter use XC epochAt(timestamp)?
- What if the event happens near epoch boundary?

Current recommended policy:

- keep current runtime validation:
  observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

- do not add timestamp cross-check in this adapter design milestone
- document timestamp / epoch-boundary cross-check as future hardening

## Failure behavior

The Ethereum adapter should reject malformed snapshots before producing records.

Reject if:

- sourceChainId is invalid
- sourceBlockNumber is missing or <= 0
- sourceBlockHash is missing or invalid
- observedAt <= 0
- finality policy is invalid
- epochMinimums is empty when policy requires at least one epoch
- any epoch minimum entry has invalid epoch or minimum
- duplicate epoch entries conflict

Generic builder will also reject invalid entries.

## Error model

For malformed Ethereum adapter input, future implementation can use:

- InvalidXcEpochMinimumRecord

for invalid record/snapshot values.

If a source cannot answer an epoch, resulting source returns null and runtime will throw:

- MissingAuthoritativeXcEpochMinimum

If payload differs from authoritative source, runtime throws:

- MismatchedAuthoritativeXcEpochMinimum

## Test strategy for first implementation

First implementation should use mocked Ethereum snapshots.

Recommended tests:

1. builds source from valid mocked Ethereum snapshot
2. rejects missing sourceBlockHash
3. rejects invalid sourceBlockHash format
4. rejects sourceBlockNumber <= 0
5. rejects empty sourceChainId
6. rejects non-EIP-155 sourceChainId
7. rejects invalid finality policy
8. rejects conflicting duplicate epoch entries
9. returns null for missing epoch through resulting source
10. does not read network
11. does not require secrets

## Security boundary

This adapter improves provenance validation for Ethereum-shaped source data.

It does not make the source trustless by itself.

Actual production trust still depends on:

- RPC provider trust
- finality policy
- Lens/Core address correctness
- ABI correctness
- deployment configuration
- monitoring and replay/audit process

## Non-goals

This design milestone does not implement:

- real RPC reads
- viem / ethers provider integration
- XC Core ABI
- XC Lens ABI
- address config
- provider config
- RPC URLs
- private keys
- CLI integration
- snapshot schema migration
- bridge signer verification
- X1-native verification

## Recommended next implementation milestone

After this design milestone, the next safe implementation milestone can be:

xc-epoch-minimum-mocked-ethereum-lens-snapshot-adapter

Scope:

- mocked snapshot input
- Ethereum-specific source metadata validation
- sourceBlockHash validation
- finality policy validation
- produce XcEpochMinimumSource
- tests only
- no network access
- no provider config
- no ABI

## Conclusion

The Ethereum / XC Lens adapter should be source-specific and stricter than the generic record builder.

The generic builder remains source-agnostic.

Ethereum-specific provenance checks belong in the Ethereum adapter.

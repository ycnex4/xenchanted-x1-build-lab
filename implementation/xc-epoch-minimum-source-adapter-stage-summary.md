# XC Epoch Minimum Source Adapter Stage Summary

## Branch

xc-epoch-minimum-source-adapter-stage-summary

## Purpose

This document summarizes the completed XC epoch minimum source adapter stage.

This is a summary-only milestone.

It does not change runtime code.

## Completed adapter-stage line

This stage completed the following milestones:

1. production source adapter design
2. mocked / production-shaped source adapter
3. dedicated invalid source record error
4. record validation hardening
5. sourceBlockHash policy

## Current generic source layer

Current generic source flow:

XcEpochMinimumRecord[]
-> createXcEpochMinimumSourceFromRecords()
-> XcEpochMinimumSource
-> appSubmitProof(..., xcEpochMinimumSource)
-> registrar authoritative validation
-> Build state

## Current record shape

XcEpochMinimumRecord includes:

- lockEpoch
- minimumXntd
- observedAt
- sourceChainId
- sourceBlockNumber
- sourceBlockHash

## Current generic validation

The generic record builder validates source-agnostic invariants:

- lockEpoch must be an integer
- lockEpoch must be >= 0
- minimumXntd must be > 0
- observedAt must be > 0
- sourceBlockNumber must be > 0 when provided
- duplicate records for the same epoch are allowed only when minimumXntd matches
- conflicting duplicate epoch minimum records are rejected

## Current error model

Source-record construction errors use:

- InvalidXcEpochMinimumRecord

Authoritative source availability errors use:

- MissingAuthoritativeXcEpochMinimum

Authoritative mismatch errors use:

- MismatchedAuthoritativeXcEpochMinimum

XNTD lock amount relationship errors continue to use:

- InvalidXntdLockAmount

This separates:

- user lock/relock amount validation
- source availability validation
- source mismatch validation
- source record construction validation

## Current sourceBlockHash policy

The generic source adapter intentionally does not validate sourceBlockHash format.

Reason:

sourceBlockHash semantics depend on the source type.

Examples:

- Ethereum finalized RPC / XC Lens records may use Ethereum block hashes
- checkpoint records may use checkpoint hashes / roots
- bridge records may use attestation hashes or bridge message IDs
- X1-native records may use slots, state roots, or verified checkpoint accounts
- deterministic test records may not need sourceBlockHash at all

Therefore strict hash/provenance validation belongs in source-specific adapters, not in the generic record builder.

## Generic adapter boundary

The generic adapter should remain:

- deterministic
- source-agnostic
- network-free
- secret-free
- snapshot-free
- CLI-free

It should only turn validated source records into an XcEpochMinimumSource.

## Source-specific future adapters

Future adapters can validate provenance before producing XcEpochMinimumRecord[].

Possible future adapters:

1. Ethereum XC Lens adapter
   - validates Ethereum chain/source metadata
   - validates finalized/safe/confirmed block policy
   - validates sourceBlockHash as 0x-prefixed 32-byte hex
   - reads XC Core / Lens state

2. Checkpoint adapter
   - validates checkpoint records
   - validates checkpoint root/hash/signatures if applicable
   - produces deterministic epoch minimum records

3. Bridge-provided adapter
   - validates bridge signer / attestation policy
   - validates bridge message provenance
   - produces epoch minimum records

4. X1-native verified adapter
   - validates X1-native registry/checkpoint/proof source
   - produces epoch minimum records

## Current tests

Current test status after adapter-stage hardening:

- 30 test files passed
- 199 tests passed

Coverage includes:

- building source from production-shaped records
- missing epoch returns null
- duplicate matching records accepted
- conflicting duplicate records rejected
- invalid lockEpoch rejected
- invalid minimumXntd rejected
- invalid observedAt rejected
- invalid sourceBlockNumber rejected
- invalid records use InvalidXcEpochMinimumRecord
- authoritative missing epoch uses MissingAuthoritativeXcEpochMinimum
- authoritative mismatch uses MismatchedAuthoritativeXcEpochMinimum

## What this stage did not add

This stage intentionally did not add:

- real Ethereum RPC reads
- XC Core ABI integration
- XC Lens ABI integration
- provider config
- private keys
- RPC URLs
- checkpoint verification
- bridge signer verification
- X1-native verification
- snapshot schema changes
- CLI integration
- persistent app-state source ownership

## Current conclusion

The generic XC epoch minimum source adapter layer is now complete enough for deterministic production-shaped testing.

The next production-readiness step should not expand the generic adapter.

The next step should be source-specific design or implementation, starting with the safest production-like path.

## Recommended next stage

Recommended next stage:

xc-epoch-minimum-ethereum-lens-adapter-design

Scope:

- design only
- define XC Core / Lens fields
- define finalized / safe / confirmed block policy
- define sourceChainId policy
- define sourceBlockNumber/sourceBlockHash requirements for Ethereum records
- define mocked Ethereum read tests
- no real RPC yet
- no secrets
- no provider config
- no CLI wiring yet

Alternative next stage:

xc-epoch-minimum-checkpoint-adapter-design

Use this if checkpoint-based source ownership is preferred before live Ethereum reads.

# Watcher to Proof Conversion Notes

## Branch

watcher-to-proof-conversion

## Purpose

This branch adds watcher candidate to proof conversion for the xEnchanted X1 Build Lab.

The conversion layer connects normalized watcher observations to validated proof objects.

It does not validate raw chain data, does not build registrar payloads directly, and does not mutate BuildState.

## Implemented files

- src/watchers/proof-conversion.ts
- tests/watcher-proof-conversion.test.ts

Updated:

- src/index.ts

## Implemented conversion input

The branch adds WatcherProofConversionInput.

It includes:

- validatedAt

## Implemented conversion helpers

The branch adds:

- convertCoreRedeemCandidateToProof
- convertXenBurnCandidateToProof
- convertXntdLockCandidateToProof
- convertXntdRelockCandidateToProof
- convertX1FeeCheckpointCandidateToProof
- convertWatcherCandidateToProof

## Finality policy

All conversion helpers require finalized watcher candidates.

Non-finalized watcher candidates are rejected through assertFinalizedWatcherCandidate.

This keeps the boundary clear:

- watcher candidate: observed external event
- finalized watcher candidate: safe to convert
- proof object: validated protocol-facing fact

## Proof status policy

Converted proofs are created with:

- status: VALIDATED
- validatedAt: provided by conversion input
- rejectionReason: null

## Event key policy

Converted proofs preserve the candidate canonicalEventKey.

For replay-sensitive proof payloads:

- Core redeem proof redeemKey = candidate canonicalEventKey
- XEN burn proof xenBurnKey = candidate canonicalEventKey

## Pipeline compatibility

The tests cover the complete lightweight pipeline:

- watcher candidate
- validated proof
- registrar payload builder

This confirms that the watcher/proof/registrar layers connect without applying state transitions directly.

## Test coverage

Added test file:

- tests/watcher-proof-conversion.test.ts

Covered cases:

- Core redeem candidate to proof
- XEN burn candidate to proof
- XNTD lock candidate to proof
- XNTD relock candidate to proof
- X1 fee checkpoint candidate to proof
- generic watcher candidate conversion routing
- non-finalized candidate rejection
- watcher candidate to proof to registrar payload pipeline

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 25 test files passed
- 141 tests passed

## Current known exclusions

This milestone does not implement:

- raw RPC watcher code
- event ABI decoding
- cryptographic proof validation
- reorg handling
- finality depth calculation
- proof persistence
- automatic registrar submission
- CLI proof conversion command
- application service proof submission

## Main invariant

Watcher-to-proof conversion translates finalized observations into validated proof objects.

It must not mutate protocol accounting directly.

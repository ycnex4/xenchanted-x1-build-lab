# Watcher Candidate Types Notes

## Branch

watcher-candidate-types

## Purpose

This branch adds watcher candidate types for the xEnchanted X1 Build Lab.

Watcher candidates sit between raw external events and proof objects.

They describe normalized observations from external systems without validating proofs and without mutating BuildState.

## Implemented files

- src/watchers/watcher-candidates.ts
- tests/watcher-candidates.test.ts

Updated:

- src/index.ts

## Implemented candidate categories

The branch defines watcher candidate types for:

- CoreRedeemCandidate
- XenBurnCandidate
- XntdLockCandidate
- XntdRelockCandidate
- X1FeeCheckpointCandidate

These are grouped under:

- WatcherCandidate

## Implemented candidate base

WatcherCandidateBase includes:

- kind
- source
- canonicalEventKey
- observedAt

The source metadata reuses ProofSourceMetadata from the proof object layer.

## Implemented candidate creation helpers

The branch adds:

- createWatcherCandidateBase
- createCoreRedeemCandidate
- createXenBurnCandidate
- createXntdLockCandidate
- createXntdRelockCandidate
- createX1FeeCheckpointCandidate

Each helper derives a canonical event key using the existing createCanonicalEventKey helper.

## Implemented finality helpers

The branch adds:

- isFinalizedWatcherCandidate
- assertFinalizedWatcherCandidate

These helpers enforce the boundary that watcher observations may be candidates before they become validated proof or registrar input.

## Test coverage

Added test file:

- tests/watcher-candidates.test.ts

Covered cases:

- Core redeem candidate creation
- XEN burn candidate creation
- XNTD lock candidate creation
- XNTD relock candidate creation
- X1 fee checkpoint candidate creation
- canonical event key derivation
- source metadata propagation
- finalized candidate helper
- non-finalized candidate assertion rejection

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 21 test files passed
- 117 tests passed

## Current known exclusions

This milestone does not implement:

- real watcher code
- indexer polling
- RPC integration
- raw event decoding
- finality policy implementation
- proof validation
- proof object conversion
- registrar payload builders
- storage persistence
- API / CLI integration

## Main invariant

Watcher candidates describe external observations.

They must not mutate BuildState directly and must not become a second source of protocol accounting logic.

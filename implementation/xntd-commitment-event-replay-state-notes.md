# XNTD Commitment Event Replay State Notes

## Branch

xntd-commitment-event-replay-state

## Purpose

This milestone implements the first runtime foundation for XNTD lock / relock per-event replay protection.

It follows the design documented in:

- docs/registrar/xntd-lock-event-identity.md

## Scope

This milestone adds the low-level replay state, persists it through application snapshots, and wires it into XNTD lock / relock registrar handlers.

It does not yet integrate the state into:

- watcher candidates
- proof conversion
- CLI summaries

## Added model

New file:

- src/model/xntd-commitment-events.ts

Added types and helpers:

- XntdCommitmentEventKey
- XntdCommitmentEventState
- usedXntdCommitmentEvents
- createXntdCommitmentEventState()
- acceptXntdCommitmentEvent()

## Error model

Added error code:

- DuplicateXntdCommitmentEvent

This represents replay of a source commitment event, not replay of a registrar message.

## Replay domain

The replay domain is intentionally shared across XNTD lock and relock.

This follows the design decision that lock and relock update the same commitment-state domain.

The shared event key model prevents the same source event from being accepted again under a different registrar message ID.

## Application state and snapshot persistence

The replay state is now part of BuildApplicationState:

- xntdCommitmentEvents

Snapshot persistence now includes:

- SerializedXntdCommitmentEventState
- serializeXntdCommitmentEventState()
- deserializeXntdCommitmentEventState()
- xntdCommitmentEvents inside SerializedBuildApplicationSnapshot

The storage schema version was bumped to 2.

## Registrar integration

XNTD lock / relock registrar payloads now include:

- xntdCommitmentEventKey

For proof-submission flow, this key is derived from:

- proof.canonicalEventKey

The application service now passes:

- app.xntdCommitmentEvents

into:

- applyRegistrarXntdLock()
- applyRegistrarXntdRelock()

Successful registrar mutation order is now:

1. acceptRegistrarMessage()
2. acceptXntdCommitmentEvent()
3. lockXntd() / relockXntd()

Preconditions run before mutations.

Duplicate xntdCommitmentEventKey is rejected before:

- marking a new registrar message
- changing Build lock state
- changing usedXntdCommitmentEvents

This protects against replaying the same XNTD commitment source event under a different messageId.

## Tests

New test file:

- tests/xntd-commitment-event-replay.test.ts

Updated test file:

- tests/storage-snapshot.test.ts

Covered behavior:

- accepts a new XNTD commitment event key
- records the event key
- rejects duplicate event key
- returns DuplicateXntdCommitmentEvent
- accepts different commitment event keys
- uses one replay domain for lock and relock source events
- snapshot round-trip preserves usedXntdCommitmentEvents
- LOCK_XNTD records xntdCommitmentEventKey
- RELOCK_XNTD records xntdCommitmentEventKey
- duplicate xntdCommitmentEventKey is rejected even with a different messageId
- lock and relock share one commitment replay domain
- appSubmitProof uses canonicalEventKey as xntdCommitmentEventKey

## Validation result

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 177 tests passed

## Current boundary

This milestone changes registrar replay behavior for XNTD lock / relock.

The new replay state is wired into:

- application state
- snapshot persistence
- XNTD lock / relock registrar handlers
- proof-submission registrar payload flow

It is not yet wired into watcher candidate payload shape changes because proof-submission currently derives the commitment event key from proof.canonicalEventKey.

The next implementation step can either:

- update CLI summaries to show xntdCommitmentEventCount
- add explicit proof payload fields if desired
- add lockEpoch ordering guard

# XNTD Commitment Event Replay State Notes

## Branch

xntd-commitment-event-replay-state

## Purpose

This milestone implements the first runtime foundation for XNTD lock / relock per-event replay protection.

It follows the design documented in:

- docs/registrar/xntd-lock-event-identity.md

## Scope

This milestone adds the low-level replay state and persists it through application snapshots.

It does not yet integrate the state into:

- registrar XNTD lock / relock handlers
- proof payloads
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

## Validation result

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 175 tests passed

## Current boundary

This milestone does not change registrar behavior yet.

The new replay state is now wired into application state and snapshot persistence.

It is not yet used by XNTD lock / relock registrar handlers.

The next implementation step should wire registrar XNTD lock / relock handlers to this replay state.

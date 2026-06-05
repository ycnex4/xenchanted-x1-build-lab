# XNTD Lock Epoch Ordering Guard Notes

## Branch

xntd-lock-epoch-ordering-guard

## Purpose

This milestone adds a registrar-layer ordering guard for XNTD lock / relock commitment events.

The previous XNTD commitment event replay milestone protected against replaying the same source event under a different registrar message ID.

This milestone protects against stale-but-unique commitment events.

A stale-but-unique event is not a replay because it has a different source-event key, but it can still regress commitment state if accepted after a newer event.

## Scope

This milestone changes only XNTD lock / relock registrar acceptance rules.

It does not change:

- low-level lockXntd() / relockXntd() primitives
- proof payload shape
- watcher payload shape
- snapshot serialization
- CLI output
- XNTD amount / epoch minimum validation

## Added error code

- NonIncreasingXntdLockEpoch
- NON_INCREASING_XNTD_LOCK_EPOCH

## Ordering rule

Registrar XNTD lock / relock now requires monotonic lockEpoch.

Rule:

- if build.lockEpoch is null, accept any incoming lockEpoch
- if build.lockEpoch is not null, incoming lockEpoch must be greater than current build.lockEpoch

Conceptually:

incomingLockEpoch > currentLockEpoch

If incomingLockEpoch is less than or equal to currentLockEpoch, the registrar handler rejects the event.

## Mutation order

The ordering guard runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

This ensures stale-but-unique events do not mark:

- registrar message ID
- XNTD commitment event key

and do not mutate Build commitment state.

## Why registrar layer

The guard is intentionally implemented in:

- src/instructions/registrar-xntd-lock.ts

It is not implemented in the low-level primitives:

- lockXntd()
- relockXntd()

Reason:

- lockXntd() and relockXntd() remain simple state transition primitives
- registrar acceptance policy handles ordering / source-event safety
- this keeps low-level model tests and registrar policy tests separated

## Tests

Updated:

- tests/registrar-xntd-lock.test.ts

Covered behavior:

- stale unique LOCK_XNTD event is rejected
- stale unique RELOCK_XNTD event is rejected
- stale event does not mark new registrar message
- stale event does not mark new xntdCommitmentEventKey
- stale event does not mutate Build lock state
- accepted newer state remains unchanged

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

## Remaining future work

Production may still choose a stricter ordering source later, such as:

- source block number
- finalized slot / block height
- event timestamp
- monotonic commitment version

For the MVP, monotonic lockEpoch is the accepted ordering guard.

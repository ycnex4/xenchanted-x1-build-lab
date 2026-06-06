# XC Build active status model review

This document reviews the XC Build active status model runtime implementation milestone.

Reviewed branch:

    xc-build-active-status-model-review

Reviewed implementation milestone:

    xc-build-active-status-model

Reviewed files:

- src/model/build-active-status.ts
- tests/build-active-status.test.ts
- src/index.ts

## Review summary

The XC Build active status model implementation is accepted.

The implementation adds a read-only `getBuildActiveStatus()` helper.

The implementation exports the helper and related types through `src/index.ts`.

The implementation adds focused unit tests for active, inactive, unknown, below-required, and non-mutating behavior.

The implementation does not change app proof submission.

The implementation does not change watcher behavior.

The implementation does not change registrar behavior.

The implementation does not change proof payload behavior.

The implementation does not introduce real RPC.

The implementation does not introduce Forge requirements.

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/index.ts
- src/model/build-active-status.ts
- tests/build-active-status.test.ts

No other runtime files changed.

No app/service files changed.

No watcher files changed.

No registrar files changed.

No proof files changed.

No ethereum/RPC files changed.

No package dependency changed.

No script changed.

## Model review

The implementation adds:

- BuildActiveStatusValue
- BuildActiveStatusReason
- BuildActiveStatus
- GetBuildActiveStatusInput
- getBuildActiveStatus()

Accepted status values:

    ACTIVE
    INACTIVE
    UNKNOWN

Accepted reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

The model fields match the accepted design:

    isActive
    status
    reason
    historyBld
    availableBld
    lockedXntd
    requiredXntdLock
    lockEpoch
    currentEpoch
    needsRelock

## Behavior review

The helper returns `INACTIVE_NO_HISTORY` when `historyBld == 0`.

The helper returns `INACTIVE_NO_LOCK` when historical contribution exists but there is no active XNTD lock.

The helper returns `ACTIVE_LOCK_CURRENT` when historical contribution and sufficient lock exist.

The helper returns `INACTIVE_LOCK_BELOW_REQUIRED` when a provided current requirement exceeds locked XNTD.

The helper returns `UNKNOWN_NO_CURRENT_CONTEXT` when strict current context is required but missing.

The helper sets `needsRelock = true` only for below-required lock in the current implementation.

## Non-mutating review

The implementation is read-only and non-mutating.

The test suite verifies that calling `getBuildActiveStatus()` does not mutate:

- historyBld
- availableBld
- originBld
- lockedXntd
- requiredXntdLock
- lockEpoch

This matches the accepted design boundary.

## Epoch / relock review

The implementation follows the accepted conservative direction:

    Use amount-based current requirement first.
    Add strict epoch freshness only if there is a clear product reason.

The implementation does not mark a Build inactive merely because `lockEpoch < currentEpoch`.

It only uses `currentRequiredXntdLock` to determine below-required status.

This keeps active status as a signal, not punishment.

## Boundary review

The implementation does not add:

- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- active status enforcement
- external project policy
- inactive Build history erasure
- Forge requirements
- unlock mechanics
- BLD transfer/sale rule changes
- CLI commands
- real RPC reads
- process.env reads
- wallet client usage
- write transactions

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 41 test files, 323 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Review decision

The XC Build active status model implementation is accepted.

No implementation changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-active-status-model-completion-checkpoint

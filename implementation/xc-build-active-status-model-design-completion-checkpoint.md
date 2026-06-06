# XC Build active status model design completion checkpoint

This document closes the XC Build active status model design milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build active status model design milestone completed the full progression:

1. active status model design
2. active status model design review
3. merge to main

## Current main status

Latest completed main milestone:

    main -> 2ea40c0 Merge branch 'xc-build-active-status-model-design-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Completed documents

Design document:

    implementation/xc-build-active-status-model-design.md

Review document:

    implementation/xc-build-active-status-model-design-review-notes.md

Checkpoint update:

    docs/checkpoints/current-design-checkpoint.md

## Accepted active status model

The accepted helper name is:

    getBuildActiveStatus()

The accepted status values are:

    ACTIVE
    INACTIVE
    UNKNOWN

The accepted reason values are:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

The accepted model fields are:

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

## Core boundary

The active status model is read-only and non-mutating.

Future status calculation must not mutate:

- history_bld
- available_bld
- origin_bld
- lockedXntd
- requiredXntdLock
- lockEpoch
- replay protection state
- registrar state
- proof state

## Historical safety

Inactive status does not erase historical contribution.

Inactive status does not reject Core redeem proof.

Inactive status does not reduce history_bld.

Unknown status does not mean invalid history.

currentEpoch may affect active status, but must not invalidate Core redeem history.

## Relock direction

The accepted design avoids strict epoch expiration by default.

Recommended implementation direction:

    Use amount-based current requirement first.
    Add strict epoch freshness only if there is a clear product reason.

This keeps active status as a signal, not punishment.

## Forge scope

Forge participation is out of scope for MVP active status.

Forge is not an active status requirement.

Forge is not an implicit Build activation requirement.

If Forge participation is ever reintroduced, it must be handled in a separate future design milestone.

## Boundary

This milestone does not add:

- runtime code
- tests
- dependencies
- real RPC execution
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
- getBuildActiveStatus implementation
- CLI commands

## Recommended next milestone

Recommended next implementation milestone:

    xc-build-active-status-model

Purpose:

- add `BuildActiveStatus` type
- add status and reason type unions
- implement `getBuildActiveStatus()`
- keep implementation read-only and non-mutating
- add focused unit tests
- avoid appSubmitProof / watcher / registrar / proof payload changes
- avoid real RPC
- keep Forge out of MVP active status

## Decision

The XC Build active status model design milestone is complete.

Next step may be runtime implementation:

    xc-build-active-status-model

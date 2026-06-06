# XC Build active status model completion checkpoint

This document closes the XC Build active status model runtime milestone.

This checkpoint is documentation-only.

No runtime code is changed in this checkpoint.

No dependencies are changed in this checkpoint.

No real RPC is executed in this checkpoint.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build active status model milestone completed the full progression:

1. active status model design
2. active status model design review
3. active status model design completion checkpoint
4. active status model runtime implementation
5. active status model runtime review
6. merge to main

## Current main status

Latest completed main milestone:

    main -> f5b3913 Merge branch 'xc-build-active-status-model-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 41 test files, 323 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Runtime files added

The runtime implementation added:

- `src/model/build-active-status.ts`
- `tests/build-active-status.test.ts`

The implementation also exports the model through:

- `src/index.ts`

## Implemented helper

The accepted helper is:

    getBuildActiveStatus()

The helper is read-only and non-mutating.

It reads Build state and optional current context, then returns status interpretation.

## Implemented status values

Implemented status values:

    ACTIVE
    INACTIVE
    UNKNOWN

## Implemented reason values

Implemented reason values:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

## Implemented model fields

Implemented model fields:

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

## Implemented behavior

The helper returns:

- `INACTIVE_NO_HISTORY` when `historyBld == 0`
- `INACTIVE_NO_LOCK` when history exists but no XNTD lock exists
- `ACTIVE_LOCK_CURRENT` when history and sufficient lock exist
- `INACTIVE_LOCK_BELOW_REQUIRED` when a provided current requirement exceeds locked XNTD
- `UNKNOWN_NO_CURRENT_CONTEXT` when strict current context is required but missing

The helper sets `needsRelock = true` only for below-required lock in the current implementation.

## Non-mutating guarantee

The implementation test suite verifies that calling `getBuildActiveStatus()` does not mutate:

- historyBld
- availableBld
- originBld
- lockedXntd
- requiredXntdLock
- lockEpoch

## Boundary preserved

The milestone did not change:

- appSubmitProof behavior
- watcher behavior
- registrar behavior
- proof payload behavior
- ethereum/RPC code
- scripts
- dependencies
- CLI commands
- BLD transfer/sale rules
- Forge requirements
- unlock mechanics

## Epoch / relock policy

The implementation follows the accepted conservative direction:

    Use amount-based current requirement first.
    Add strict epoch freshness only if there is a clear product reason.

The implementation does not mark a Build inactive merely because `lockEpoch < currentEpoch`.

## Forge scope

Forge participation remains out of scope for MVP active status.

Forge is not an active status requirement.

Forge is not an implicit Build activation requirement.

## Recommended next milestone

Recommended next milestone:

    xc-build-active-status-model-completion-review

or, if no separate review is needed:

    xc-build-active-status-app-integration-design

Purpose of the next design milestone:

- decide how the app/service layer should optionally expose active status
- avoid turning active status into mandatory enforcement too early
- keep external X1 project usage optional
- preserve historical contribution regardless of active status

## Decision

The XC Build active status model runtime milestone is complete.

Next recommended design step:

    xc-build-active-status-app-integration-design

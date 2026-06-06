# XC Build active status model design review

This document reviews the XC Build active status model design milestone.

Reviewed branch:

    xc-build-active-status-model-design-review

Reviewed design milestone:

    xc-build-active-status-model-design

Reviewed files:

- implementation/xc-build-active-status-model-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build active status model design is accepted.

The design correctly defines a read-only active status model.

The design correctly keeps active status calculation non-mutating.

The design correctly recommends `getBuildActiveStatus()` as the helper name.

The design correctly defines `ACTIVE`, `INACTIVE`, and `UNKNOWN` status values.

The design correctly defines reason codes for active, inactive, relock-required, and unknown-current-context cases.

The design correctly states that inactive status does not erase historical contribution.

The design correctly states that currentEpoch may affect active status but must not invalidate Core redeem history.

The design correctly keeps Forge out of MVP active validity.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-active-status-model-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Model shape review

The recommended output shape is accepted:

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

This shape is sufficient for:

- project-owned active status display
- optional eligibility checks
- external X1 project interpretation
- future app/service integration
- read-only diagnostics

## Status value review

The accepted status values are:

    ACTIVE
    INACTIVE
    UNKNOWN

This is the correct minimal set.

`UNKNOWN` is useful because some callers may require current context, while others may only need stored Build state interpretation.

## Reason code review

The accepted reason values are:

    ACTIVE_LOCK_CURRENT
    INACTIVE_NO_HISTORY
    INACTIVE_NO_LOCK
    INACTIVE_LOCK_BELOW_REQUIRED
    INACTIVE_RELOCK_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

These reason codes are enough for the first implementation.

They explain status without mutating state or enforcing policy globally.

## Non-mutating review

The design correctly states that active status calculation must not mutate:

- history_bld
- available_bld
- origin_bld
- lockedXntd
- requiredXntdLock
- lockEpoch
- replay protection state
- registrar state
- proof state

This preserves the intended boundary between interpretation and state transition.

## Epoch and relock review

The design correctly states that currentEpoch can affect active status, but must not erase history or reject Core redeem proof.

The design also correctly avoids locking in strict epoch expiration too early.

The recommended direction is accepted:

    Use amount-based current requirement first.
    Add strict epoch freshness only if there is a clear product reason.

This keeps active status as a signal, not punishment.

## External project policy review

The design correctly lets external X1 projects use or ignore any part of the status model.

The model exposes context without forcing policy.

This preserves optionality.

## Forge scope review

Forge is correctly out of scope for MVP active status.

The design does not reintroduce Forge participation as an activation requirement.

Forge should remain outside the MVP active status path unless explicitly reintroduced in a separate future design milestone.

## Boundary review

The design does not add:

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

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Review decision

The XC Build active status model design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-active-status-model

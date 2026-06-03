# XNTD Lock / Relock Notes

## Branch

xntd-lock-relock

## Purpose

This branch implements the XNTD lock / relock model for the TypeScript MVP model layer.

The lock model represents XC commitment activation and epoch-based relock requirements.

## Scope

Included:

- LockXntdInput type
- RelockXntdInput type
- lockXntd transition
- relockXntd transition
- positive XNTD lock amount validation
- active commitment requirement for relock
- relock BLD integrity rule
- InvalidXntdLockAmount error
- XntdCommitmentNotActive error
- InsufficientAvailableBldForRelock error
- tests for lock and relock behavior
- tests proving Genesis Origin BLD does not block relock when availableBld >= historyBld
- tests proving unrelated accounting values are not created

Excluded:

- registrar LOCK_XNTD / RELOCK_XNTD message integration
- unlock flow
- lock proof validation
- external XNTD escrow / custody mechanics
- epoch parameter source
- XNTD amount calculation policy
- BLD transfer / burn mechanics
- X1 Fee Contribution checkpoints

## Implemented behavior

lockXntd:

1. validates amountXntd is positive
2. sets lockedXntd
3. sets requiredXntdLock
4. sets lockEpoch
5. sets xcCommitmentActive to true
6. updates updatedAt from lockedAt

relockXntd:

1. validates amountXntd is positive
2. requires xcCommitmentActive
3. requires availableBld >= historyBld
4. updates lockedXntd
5. updates requiredXntdLock
6. updates lockEpoch
7. keeps xcCommitmentActive true
8. updates updatedAt from relockedAt

## Relock integrity rule

Relock requires:

availableBld >= historyBld

This means the Build must still have enough available BLD to cover the historical XC contribution that supports the relock.

Genesis Origin BLD can make availableBld greater than historyBld and does not block relock.

## Failure behavior

Invalid lock amount:

- must not change lockedXntd
- must not change requiredXntdLock
- must not change lockEpoch
- must not activate commitment
- must not update updatedAt

Relock without active commitment:

- must not change lock state
- must not update updatedAt

Relock with insufficient availableBld:

- must not change lockedXntd
- must not change requiredXntdLock
- must not change lockEpoch
- must not update updatedAt

## Errors

Added BuildErrorCode values:

- InvalidXntdLockAmount
- XntdCommitmentNotActive
- InsufficientAvailableBldForRelock

## Tests

Current XNTD lock / relock tests verify:

- locks XNTD and activates XC commitment
- rejects zero XNTD lock amount
- relocks active commitment when availableBld covers historyBld
- allows relock when Genesis Origin makes availableBld greater than historyBld
- rejects relock when commitment is not active
- rejects relock when availableBld is below historyBld
- does not create BLD, XBP, or X1 fee contribution

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 13 test files passed
- 73 tests passed

## Main invariant

XNTD lock / relock controls XC commitment state only.

It must not create BLD, XBP, Origin BLD, or X1 fee contribution.

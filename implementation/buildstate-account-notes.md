# BuildState Account / Object Notes

## Branch

buildstate-account

## Purpose

This branch implements the initial BuildState model structure for the TypeScript MVP model layer.

This branch must not implement real accounting transitions.

## Scope

Included:

- BuildState field structure
- BuildState version constant
- createEmptyBuildState factory
- tests for initial default values

Excluded:

- create_build instruction behavior
- Core redeem accounting
- Genesis Origin BLD accounting
- XEN Burn Power accounting
- XNTD lock / unlock / relock logic
- X1 Fee Contribution checkpoint logic
- registrar message processing
- replay protection logic

## Implemented fields

Identity / metadata:

- owner
- buildId
- version
- createdAt
- updatedAt
- ethereumIdentity

BLD:

- historyBld
- availableBld
- originBld

XBP:

- earnedXbp
- availableXbp

XNTD commitment:

- lockedXntd
- requiredXntdLock
- lockEpoch
- xcCommitmentActive

X1 Fee Contribution:

- x1FeeContribution
- x1TxCount
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

## Factory

createEmptyBuildState initializes:

- identity fields from input
- version from BUILD_STATE_VERSION
- updatedAt equal to createdAt
- all numeric accounting fields to 0n
- optional identity / checkpoint fields to null
- xcCommitmentActive to false

## Test status

Current tests:

- creates an empty BuildState with identity fields
- initializes BLD fields to zero
- initializes XBP fields to zero
- initializes XNTD commitment fields as inactive
- initializes X1 fee fields to zero or null

Validation:

- npm run typecheck: passed
- npm test: passed
- 2 test files passed
- 6 tests passed

## Main invariant

This branch defines state shape only.

No transition in this branch should create, move, burn, lock, unlock, or account for any value.

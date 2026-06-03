# X1 Fee Contribution Checkpoint Notes

## Branch

x1-fee-contribution

## Purpose

This branch implements the X1 Fee Contribution checkpoint model for the TypeScript MVP model layer.

The checkpoint model records X1 fee contribution totals and processed slot progress.

## Scope

Included:

- ApplyX1FeeContributionCheckpointInput type
- applyX1FeeContributionCheckpoint transition
- positive fee amount validation
- positive tx count validation
- increasing countedUntilSlot validation
- x1FeeContribution accumulation
- x1TxCount accumulation
- x1FeeCountedUntilSlot update
- lastFeeUpdateAt update
- updatedAt update
- tests for valid and rejected checkpoints
- tests proving BLD, XBP, and XNTD commitment are not created or changed

Excluded:

- registrar X1_FEE_CHECKPOINT message integration
- source transaction proof validation
- external X1 fee indexing
- slot finality policy
- fee normalization policy
- bridge proof logic
- BLD minting from fees
- XNTD lock / unlock / relock mechanics

## Implemented behavior

applyX1FeeContributionCheckpoint:

1. validates feeAmount is positive
2. validates txCount is positive
3. validates countedUntilSlot increases
4. adds feeAmount to x1FeeContribution
5. adds txCount to x1TxCount
6. sets x1FeeCountedUntilSlot
7. sets lastFeeUpdateAt
8. sets updatedAt

## Checkpoint slot rule

If x1FeeCountedUntilSlot already exists, the next countedUntilSlot must be strictly greater.

Rejected cases:

- same slot
- lower slot

## Failure behavior

Invalid fee amount:

- must not change fee contribution
- must not change tx count
- must not set counted slot
- must not set last fee update
- must not update updatedAt

Invalid tx count:

- must not change fee contribution
- must not change tx count
- must not set counted slot
- must not set last fee update
- must not update updatedAt

Non-increasing checkpoint slot:

- must preserve previous fee totals
- must preserve previous tx count
- must preserve previous counted slot
- must preserve previous last fee update
- must preserve previous updatedAt

## Errors

Added BuildErrorCode values:

- InvalidFeeContributionAmount
- InvalidFeeContributionTxCount
- NonIncreasingFeeCheckpointSlot

## Tests

Current X1 Fee Contribution tests verify:

- applies first fee contribution checkpoint
- accumulates fee amount and tx count with increasing slot
- rejects zero fee amount without mutating state
- rejects zero tx count without mutating state
- rejects non-increasing checkpoint slot without mutating state
- rejects lower checkpoint slot without mutating state
- does not create or change BLD, XBP, or XNTD commitment

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 14 test files passed
- 80 tests passed

## Main invariant

X1 Fee Contribution checkpoints record fee participation only.

They must not create BLD, XBP, Origin BLD, or XNTD commitment state.

# Core Redeem BLD Transition Notes

## Branch

core-redeem-bld

## Purpose

This branch implements the Core redeem -> history_bld transition model for the TypeScript MVP model layer.

The transition applies already accepted Core redeem BLD value to a BuildState.

This branch does not implement source-event replay protection yet.

## Scope

Included:

- applyCoreRedeemBld input type
- applyCoreRedeemBld transition
- positive BLD amount validation
- InvalidBldAmount error
- tests for history_bld and available_bld accumulation
- tests proving unrelated layers do not change

Excluded:

- used_redeem_events replay protection
- source redeem key validation
- registrar message integration
- Core NFT proof validation
- Genesis Origin BLD
- XEN Burn Power
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Implemented behavior

applyCoreRedeemBld accepts:

- build
- amountBld
- redeemedAt

It updates:

- historyBld += amountBld
- availableBld += amountBld
- updatedAt = redeemedAt

## Validation rule

amountBld must be positive.

Rejected values:

- 0n
- negative bigint values

Rejected amount must not mutate the BuildState.

## Unchanged fields

The transition must not change:

- originBld
- earnedXbp
- availableXbp
- lockedXntd
- requiredXntdLock
- lockEpoch
- xcCommitmentActive
- x1FeeContribution
- x1TxCount
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

## Tests

Current Core redeem BLD tests verify:

- adds redeemed Core history to historyBld and availableBld
- accumulates multiple accepted Core redeem amounts
- does not change originBld
- does not create XBP
- does not change XNTD commitment fields
- does not create X1 fee contribution
- rejects zero BLD amount
- rejects negative BLD amount

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 6 test files passed
- 30 tests passed

## Main invariant

Core redeem BLD creates historical BLD and matching available BLD only.

It must not create XBP, Origin BLD, XNTD commitment, or X1 fee contribution.

# XEN Burn Power Transition Notes

## Branch

xen-burn-power

## Purpose

This branch implements the XEN Burn Power transition model for the TypeScript MVP model layer.

The transition applies already accepted XEN burn power value to a BuildState.

This branch does not implement used_xen_burn_events replay protection yet.

## Scope

Included:

- ApplyXenBurnPowerInput type
- applyXenBurnPower transition
- positive XBP amount validation
- InvalidXbpAmount error
- earnedXbp accumulation
- availableXbp accumulation
- updatedAt update from burnedAt
- tests proving unrelated layers do not change

Excluded:

- used_xen_burn_events replay protection
- XEN burn event key validation
- registrar XEN_BURN message integration
- Ethereum XEN.burn log proof validation
- XEN burn amount normalization policy
- BLD accounting
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Implemented behavior

applyXenBurnPower accepts:

- build
- amountXbp
- burnedAt

It updates:

- earnedXbp += amountXbp
- availableXbp += amountXbp
- updatedAt = burnedAt

## Validation rule

amountXbp must be positive.

Rejected values:

- 0n
- negative bigint values

Rejected amount must not mutate the BuildState.

## Unchanged fields

The transition must not change:

- historyBld
- availableBld
- originBld
- lockedXntd
- requiredXntdLock
- lockEpoch
- xcCommitmentActive
- x1FeeContribution
- x1TxCount
- x1FeeCountedUntilSlot
- lastFeeUpdateAt

## Tests

Current XEN Burn Power tests verify:

- adds XEN Burn Power to earnedXbp and availableXbp
- accumulates multiple accepted XBP amounts
- does not create BLD
- does not change XNTD commitment fields
- does not create X1 fee contribution
- rejects zero XBP amount
- rejects negative XBP amount

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 9 test files passed
- 47 tests passed

## Main invariant

XEN Burn Power creates XBP only.

It must not create BLD, Origin BLD, XNTD commitment, or X1 fee contribution.

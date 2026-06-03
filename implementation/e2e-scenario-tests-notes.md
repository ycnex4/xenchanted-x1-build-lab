# End-to-End Scenario Tests Notes

## Branch

e2e-scenario-tests

## Purpose

This branch adds end-to-end scenario tests for the current MVP Build lifecycle.

The tests connect already implemented model layers into complete flows and verify that they work together as expected.

## Scope

Included:

- full MVP Build lifecycle scenario
- registrar CORE_REDEEM flow
- registrar XEN_BURN flow
- Genesis Origin BLD claim
- registrar LOCK_XNTD flow
- registrar RELOCK_XNTD flow
- registrar X1_FEE_CHECKPOINT flow
- final BuildState accounting checks
- registrar processedMessages checks
- Core redeem event replay checks
- XEN burn event replay checks
- duplicate registrar message rejection
- duplicate Core redeem event rejection
- duplicate XEN burn event rejection

Excluded:

- external proof validation
- storage persistence
- API / CLI orchestration
- watcher / indexer integration
- serialization round-trip
- real chain event parsing

## Implemented test file

- tests/e2e-scenario.test.ts

## Covered scenario

The first e2e test runs this flow:

1. create Build
2. apply registrar Core redeem
3. apply registrar XEN burn
4. claim Genesis Origin BLD
5. apply registrar XNTD lock
6. apply registrar XNTD relock
7. apply registrar X1 fee checkpoint
8. verify final accounting state
9. verify registrar and event replay sets

Expected final state:

- historyBld = 121
- originBld = 55
- availableBld = 176
- earnedXbp = 1000
- availableXbp = 1000
- lockedXntd = 250
- requiredXntdLock = 250
- lockEpoch = 2
- xcCommitmentActive = true
- x1FeeContribution = 777
- x1TxCount = 11
- x1FeeCountedUntilSlot = 9000
- lastFeeUpdateAt = 1600
- updatedAt = 1600

## Replay scenario

The second e2e test verifies:

- duplicate Core redeem event key is rejected
- duplicate XEN burn event key is rejected
- duplicate registrar message id is rejected
- failed duplicate attempts do not increase accounting values
- replay sets keep expected sizes

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 17 test files passed
- 98 tests passed

## Main invariant

The full MVP lifecycle can compose multiple registrar and direct model transitions without mixing accounting layers or weakening replay protection.

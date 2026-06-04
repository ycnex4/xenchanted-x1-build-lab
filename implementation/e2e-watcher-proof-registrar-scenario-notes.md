# End-to-End Watcher Proof Registrar Scenario Notes

## Branch

e2e-watcher-proof-registrar-scenario

## Purpose

This milestone adds an end-to-end scenario test that connects the implemented watcher, proof, registrar payload, application service, and replay protection layers.

The tested path:

watcher candidate
  -> validated proof
  -> appSubmitProof
  -> registrar application service
  -> BuildState update
  -> replay protection state update

## Added files

- tests/e2e-watcher-proof-registrar-scenario.test.ts

## Scenario coverage

The new test creates a registered Build through the application service layer and then submits finalized watcher candidates through the full proof submission path.

Covered watcher candidate categories:

- Core redeem candidate
- XEN burn candidate
- XNTD lock candidate
- XNTD relock candidate
- X1 fee checkpoint candidate

Each candidate is converted into a validated proof through convertWatcherCandidateToProof.

Each proof is submitted through appSubmitProof.

## Verified state transitions

The scenario verifies final Build state fields:

- historyBld
- availableBld
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

## Verified replay protection

The scenario verifies:

- registrar processedMessages
- Core redeem replay state through usedRedeemEvents
- XEN burn replay state through usedXenBurnEvents
- duplicate proof submission rejection through existing registrar replay protection

## Architectural boundary

This milestone does not add new model logic.

It does not change accounting rules.

It does not add new watcher validation rules.

It only proves that existing layers compose correctly in the intended order:

watcher -> proof -> application proof submission -> registrar transition

## Genesis Origin note

Genesis Origin proof is intentionally not included in this e2e watcher-proof-registrar path.

Genesis Origin eligibility proof does not map to registrar payload submission and remains a separate application action.

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 153 tests passed

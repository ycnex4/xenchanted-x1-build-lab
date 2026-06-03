# create_build Instruction Notes

## Branch

create-build

## Purpose

This branch implements the first create_build instruction model for the TypeScript MVP model layer.

The instruction creates an empty BuildState from user identity input.

This branch must not implement duplicate Build prevention yet unless a registry model is introduced in a later milestone.

## Scope

Included:

- createBuild input type
- createBuild function
- creation of empty BuildState through createEmptyBuildState
- tests proving createBuild does not create contribution or commitment fields

Excluded:

- canonical_build_by_identity registry
- duplicate Build prevention
- registrar message processing
- replay protection
- Core redeem accounting
- Genesis Origin BLD
- XEN Burn Power
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Implemented behavior

createBuild accepts:

- owner
- buildId
- createdAt
- optional ethereumIdentity

createBuild returns a BuildState initialized through createEmptyBuildState.

## Required zero-value behavior

createBuild must not create:

- historyBld
- availableBld
- originBld
- earnedXbp
- availableXbp
- lockedXntd
- requiredXntdLock
- xcCommitmentActive
- x1FeeContribution
- x1TxCount

## Tests

Current createBuild tests verify:

- creates a canonical empty BuildState from input identity
- allows creating a BuildState without Ethereum identity
- does not create BLD balances
- does not create XBP balances
- does not activate XNTD commitment
- does not create X1 fee contribution

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 3 test files passed
- 12 tests passed

## Main invariant

createBuild creates the Build object only.

It must not create history, contribution, commitment, reward, or fee accounting value.

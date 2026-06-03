# Registrar X1_FEE_CHECKPOINT Integration Notes

## Branch

registrar-x1-fee-checkpoint

## Purpose

This branch integrates registrar X1_FEE_CHECKPOINT messages with the X1 Fee Contribution checkpoint model.

The integration connects already implemented layers:

- registrar authority / processedMessages
- X1 Fee Contribution checkpoint transition
- countedUntilSlot monotonic checkpoint rule

## Scope

Included:

- ApplyRegistrarX1FeeCheckpointInput type
- applyRegistrarX1FeeCheckpoint helper
- X1_FEE_CHECKPOINT message kind support
- X1_FEE_CHECKPOINT message kind validation
- registrar authority validation
- processedMessages duplicate check
- fee checkpoint application
- registrar message recording after successful checkpoint application
- tests for non-mutating failure paths
- tests proving BLD, XBP, and XNTD commitment are not created or changed

Excluded:

- source transaction proof validation
- external X1 fee indexing
- slot finality policy
- fee normalization policy
- registrar signature validation
- Merkle proof logic
- bridge proof logic
- BLD minting from fees

## Implemented behavior

applyRegistrarX1FeeCheckpoint validates:

1. message.kind must be X1_FEE_CHECKPOINT
2. submittedBy must match registrarAuthority
3. messageId must not already be processed
4. feeAmount must be positive through fee checkpoint transition
5. txCount must be positive through fee checkpoint transition
6. countedUntilSlot must increase through fee checkpoint transition

Then it applies:

- applyX1FeeContributionCheckpoint
- acceptRegistrarMessage

## Atomicity model

Failure paths must not partially mutate state.

Invalid message kind:

- does not mark registrar message
- does not change fee contribution state
- does not update BuildState

Unauthorized registrar:

- does not mark registrar message
- does not change fee contribution state
- does not update BuildState

Duplicate registrar message:

- does not apply a second checkpoint
- does not update BuildState

Invalid fee amount:

- does not mark registrar message
- does not change fee contribution state
- does not update BuildState

Non-increasing checkpoint slot:

- does not mark the new registrar message
- preserves previous fee totals
- preserves previous counted slot
- preserves previous updatedAt

## Errors

Uses existing errors:

- InvalidRegistrarMessageKind
- UnauthorizedRegistrar
- DuplicateRegistrarMessage
- InvalidFeeContributionAmount
- InvalidFeeContributionTxCount
- NonIncreasingFeeCheckpointSlot

## Tests

Current registrar X1_FEE_CHECKPOINT integration tests verify:

- accepts X1_FEE_CHECKPOINT registrar message and applies fee checkpoint
- accumulates multiple valid registrar fee checkpoints
- rejects wrong message kind without mutating state
- rejects unauthorized registrar without mutating state
- rejects duplicate registrar message without applying second checkpoint
- rejects invalid fee amount without marking registrar message
- rejects non-increasing slot without marking registrar message
- does not create or change BLD, XBP, or XNTD commitment

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Main invariant

Registrar X1_FEE_CHECKPOINT messages can update X1 fee contribution state only after registrar replay protection passes.

They must not create BLD, XBP, Origin BLD, or XNTD commitment state.

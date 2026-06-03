# Registrar CORE_REDEEM Integration Notes

## Branch

registrar-core-redeem

## Purpose

This branch integrates registrar CORE_REDEEM messages with Core redeem BLD accounting and used_redeem_events replay protection.

The integration connects already implemented layers:

- registrar authority / processedMessages
- Core redeem BLD transition
- usedRedeemEvents replay protection

## Scope

Included:

- ApplyRegistrarCoreRedeemInput type
- applyRegistrarCoreRedeem helper
- CORE_REDEEM message kind validation
- registrar authority validation
- processedMessages duplicate check
- usedRedeemEvents duplicate check
- Core redeem BLD application
- registrar message recording after successful redeem application
- tests for non-mutating failure paths

Excluded:

- source redeem key derivation
- Ethereum log proof validation
- Core NFT proof validation
- registrar signature validation
- Merkle proof logic
- bridge proof logic
- XEN burn accounting
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Implemented behavior

applyRegistrarCoreRedeem validates:

1. message.kind must be CORE_REDEEM
2. submittedBy must match registrarAuthority
3. messageId must not already be processed
4. redeemKey must not already be used
5. amountBld must be positive through Core redeem transition

Then it applies:

- acceptCoreRedeemEvent
- acceptRegistrarMessage

## Atomicity model

Failure paths must not partially mutate state.

Invalid message kind:

- does not mark registrar message
- does not mark redeemKey
- does not change BuildState

Duplicate registrar message:

- does not mark redeemKey
- does not apply BLD again

Duplicate redeemKey:

- does not mark the new registrar message
- does not apply BLD again

Invalid BLD amount:

- does not mark registrar message
- does not mark redeemKey
- does not change BuildState

## Errors

Added BuildErrorCode value:

- InvalidRegistrarMessageKind

Used existing errors:

- UnauthorizedRegistrar
- DuplicateRegistrarMessage
- DuplicateRedeemEvent
- InvalidBldAmount

## Tests

Current registrar CORE_REDEEM integration tests verify:

- accepts CORE_REDEEM registrar message and applies BLD once
- rejects non-CORE_REDEEM message without mutating state
- rejects duplicate registrar message without applying second redeem
- rejects duplicate redeemKey without marking registrar message
- rejects invalid BLD amount without marking message or redeemKey

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 8 test files passed
- 40 tests passed

## Main invariant

A registrar CORE_REDEEM message can create history_bld / available_bld only when both registrar replay protection and redeem event replay protection pass.

No unrelated accounting layer may be changed by this integration.

# Registrar XEN_BURN Integration Notes

## Branch

registrar-xen-burn

## Purpose

This branch integrates registrar XEN_BURN messages with XEN Burn Power accounting and used_xen_burn_events replay protection.

The integration connects already implemented layers:

- registrar authority / processedMessages
- XEN Burn Power transition
- usedXenBurnEvents replay protection

## Scope

Included:

- ApplyRegistrarXenBurnInput type
- applyRegistrarXenBurn helper
- XEN_BURN message kind validation
- registrar authority validation
- processedMessages duplicate check
- usedXenBurnEvents duplicate check
- XEN Burn Power application through acceptXenBurnEvent
- registrar message recording after successful XEN burn application
- tests for non-mutating failure paths
- tests proving BLD and unrelated accounting values are not created

Excluded:

- source XEN burn key derivation
- Ethereum XEN.burn log proof validation
- XEN burn amount normalization policy
- registrar signature validation
- Merkle proof logic
- bridge proof logic
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Implemented behavior

applyRegistrarXenBurn validates:

1. message.kind must be XEN_BURN
2. submittedBy must match registrarAuthority
3. messageId must not already be processed
4. xenBurnKey must not already be used
5. amountXbp must be positive through XEN Burn Power transition

Then it applies:

- acceptXenBurnEvent
- acceptRegistrarMessage

## Atomicity model

Failure paths must not partially mutate state.

Invalid message kind:

- does not mark registrar message
- does not mark xenBurnKey
- does not change BuildState

Duplicate registrar message:

- does not mark xenBurnKey
- does not apply XBP again

Duplicate xenBurnKey:

- does not mark the new registrar message
- does not apply XBP again

Invalid XBP amount:

- does not mark registrar message
- does not mark xenBurnKey
- does not change BuildState

## Errors

Uses existing errors:

- InvalidRegistrarMessageKind
- UnauthorizedRegistrar
- DuplicateRegistrarMessage
- DuplicateXenBurnEvent
- InvalidXbpAmount

## Tests

Current registrar XEN_BURN integration tests verify:

- accepts XEN_BURN registrar message and applies XBP once
- rejects non-XEN_BURN message without mutating state
- rejects duplicate registrar message without applying second XBP
- rejects duplicate xenBurnKey without marking registrar message
- rejects invalid XBP amount without marking message or xenBurnKey
- does not create BLD or unrelated accounting values

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 11 test files passed
- 58 tests passed

## Main invariant

A registrar XEN_BURN message can create earned_xbp / available_xbp only when both registrar replay protection and XEN burn event replay protection pass.

No BLD or unrelated accounting layer may be changed by this integration.

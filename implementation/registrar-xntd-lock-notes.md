# Registrar XNTD Lock / Relock Integration Notes

## Branch

registrar-xntd-lock

## Purpose

This branch integrates registrar LOCK_XNTD and RELOCK_XNTD messages with the XNTD lock / relock model.

The integration connects already implemented layers:

- registrar authority / processedMessages
- XNTD lock transition
- XNTD relock transition
- relock BLD integrity rule

## Scope

Included:

- ApplyRegistrarXntdLockInput type
- ApplyRegistrarXntdRelockInput type
- applyRegistrarXntdLock helper
- applyRegistrarXntdRelock helper
- LOCK_XNTD message kind support
- RELOCK_XNTD message kind support
- registrar authority validation
- processedMessages duplicate check
- lock / relock application
- registrar message recording after successful lock / relock
- tests for non-mutating failure paths
- tests proving unrelated accounting values are not created

Excluded:

- external XNTD escrow proof validation
- registrar signature validation
- Merkle proof logic
- bridge proof logic
- unlock flow
- epoch parameter source
- XNTD amount calculation policy
- BLD transfer / burn mechanics
- X1 Fee Contribution integration

## Implemented behavior

applyRegistrarXntdLock validates:

1. message.kind must be LOCK_XNTD
2. submittedBy must match registrarAuthority
3. messageId must not already be processed
4. amountXntd must be positive through lock transition

Then it applies:

- lockXntd
- acceptRegistrarMessage

applyRegistrarXntdRelock validates:

1. message.kind must be RELOCK_XNTD
2. submittedBy must match registrarAuthority
3. messageId must not already be processed
4. amountXntd must be positive through relock transition
5. commitment must already be active
6. availableBld must be >= historyBld

Then it applies:

- relockXntd
- acceptRegistrarMessage

## Atomicity model

Failure paths must not partially mutate state.

Invalid message kind:

- does not mark registrar message
- does not change lock state
- does not update BuildState

Unauthorized registrar:

- does not mark registrar message
- does not change lock state
- does not update BuildState

Duplicate registrar message:

- does not apply a second lock / relock
- does not update BuildState

Invalid lock amount:

- does not mark registrar message
- does not change lock state
- does not update BuildState

Invalid relock:

- does not mark registrar message
- does not change lock state
- does not update BuildState

## Errors

Uses existing errors:

- InvalidRegistrarMessageKind
- UnauthorizedRegistrar
- DuplicateRegistrarMessage
- InvalidXntdLockAmount
- XntdCommitmentNotActive
- InsufficientAvailableBldForRelock

## Tests

Current registrar XNTD lock / relock integration tests verify:

- accepts LOCK_XNTD registrar message and locks XNTD
- accepts RELOCK_XNTD registrar message and relocks XNTD
- rejects wrong message kind without mutating lock state
- rejects unauthorized registrar without mutating lock state
- rejects duplicate registrar message without applying second lock
- rejects invalid lock amount without marking registrar message
- rejects invalid relock without marking registrar message
- does not create BLD, XBP, or X1 fee contribution

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 15 test files passed
- 88 tests passed

## Main invariant

Registrar LOCK_XNTD / RELOCK_XNTD messages can change XC commitment lock state only after registrar replay protection passes.

They must not create BLD, XBP, Origin BLD, or X1 fee contribution.

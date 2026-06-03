# XEN Burn Event Replay Protection Notes

## Branch

xen-burn-event-replay

## Purpose

This branch implements used_xen_burn_events replay protection for the TypeScript MVP model layer.

The model accepts XEN burn events by xenBurnKey and prevents the same XEN burn event from being applied twice.

This branch does not integrate registrar XEN_BURN messages yet.

## Scope

Included:

- XenBurnEventKey type
- XenBurnEvent type
- XenBurnEventState structure
- createXenBurnEventState factory
- acceptXenBurnEvent helper
- usedXenBurnEvents replay protection
- DuplicateXenBurnEvent error
- tests proving duplicate xenBurnKey cannot apply XBP twice
- tests proving invalid XBP amount does not mark xenBurnKey as used

Excluded:

- registrar XEN_BURN message integration
- source XEN burn key derivation
- Ethereum XEN.burn log proof validation
- XEN burn amount normalization policy
- BLD accounting
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## XEN burn event state

XenBurnEventState contains:

- usedXenBurnEvents

## XEN burn event fields

XenBurnEvent contains:

- xenBurnKey
- build
- amountXbp
- burnedAt

## Implemented behavior

acceptXenBurnEvent:

1. rejects duplicate xenBurnKey
2. applies XEN Burn Power transition
3. records xenBurnKey only after successful transition
4. returns the updated BuildState

## Failure behavior

If xenBurnKey is duplicate:

- BuildState must not change
- usedXenBurnEvents size must not change

If amountXbp is invalid:

- BuildState must not change
- xenBurnKey must not be recorded

## Errors

Added BuildErrorCode value:

- DuplicateXenBurnEvent

## Tests

Current XEN burn event replay tests verify:

- accepts a new XEN burn event and records xenBurnKey
- rejects duplicate xenBurnKey before applying XBP twice
- accepts different xenBurnKeys and accumulates XBP
- does not mark xenBurnKey when XBP amount is invalid
- does not create unrelated accounting values

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 10 test files passed
- 52 tests passed

## Main invariant

A XEN burn event can affect BuildState only once.

Replay protection must prevent duplicate earned_xbp / available_xbp accounting.

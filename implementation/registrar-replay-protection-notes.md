# Registrar Replay Protection Notes

## Branch

registrar-replay-protection

## Purpose

This branch implements the basic registrar message replay protection model for the TypeScript MVP model layer.

The registrar model accepts source-based messages only from the configured registrar authority and records processed message IDs.

This branch must not implement source accounting transitions.

## Scope

Included:

- RegistrarState structure
- RegistrarMessage type
- RegistrarMessageKind type
- createRegistrarState factory
- acceptRegistrarMessage helper
- registrar authority check
- processedMessages replay protection
- tests proving rejected messages do not mutate state

Excluded:

- Core redeem accounting
- used_redeem_events
- XEN burn accounting
- used_xen_burn_events
- Genesis Origin BLD accounting
- XNTD lock / unlock / relock accounting
- X1 Fee Contribution checkpoint accounting
- message payload validation
- signature validation
- Merkle proof logic
- bridge proof logic

## Registrar state

RegistrarState contains:

- registrarAuthority
- processedMessages

## Message fields

RegistrarMessage contains:

- messageId
- kind
- submittedBy
- createdAt

## Message kinds

Current message kind union includes:

- CORE_REDEEM
- XEN_BURN
- GENESIS_ORIGIN
- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD
- FEE_CHECKPOINT

These are only message categories at this milestone.

No accounting behavior is attached to them yet.

## Errors

Added BuildErrorCode values:

- UnauthorizedRegistrar
- DuplicateRegistrarMessage

## Tests

Current registrar replay tests verify:

- accepts a new message from registrar authority
- rejects duplicate registrar message
- rejects unauthorized registrar before changing state
- accepts different message IDs from registrar authority

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 5 test files passed
- 22 tests passed

## Main invariant

Registrar replay protection only records accepted message IDs.

It must not create BLD, XBP, XNTD commitment, X1 fee contribution, or source-event accounting value.

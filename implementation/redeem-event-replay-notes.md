# Redeem Event Replay Protection Notes

## Branch

redeem-event-replay

## Purpose

This branch implements used_redeem_events replay protection for the TypeScript MVP model layer.

The model accepts Core redeem events by redeemKey and prevents the same Core redeem event from being applied twice.

This branch does not integrate registrar CORE_REDEEM messages yet.

## Scope

Included:

- RedeemEventKey type
- CoreRedeemEvent type
- RedeemEventState structure
- createRedeemEventState factory
- acceptCoreRedeemEvent helper
- usedRedeemEvents replay protection
- DuplicateRedeemEvent error
- tests proving duplicate redeemKey cannot apply BLD twice
- tests proving invalid BLD amount does not mark redeemKey as used

Excluded:

- registrar CORE_REDEEM message integration
- source redeem key derivation
- Ethereum log proof validation
- Core NFT proof validation
- XEN burn accounting
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Redeem event state

RedeemEventState contains:

- usedRedeemEvents

## Core redeem event fields

CoreRedeemEvent contains:

- redeemKey
- build
- amountBld
- redeemedAt

## Implemented behavior

acceptCoreRedeemEvent:

1. rejects duplicate redeemKey
2. applies Core redeem BLD transition
3. records redeemKey only after successful transition
4. returns the updated BuildState

## Failure behavior

If redeemKey is duplicate:

- BuildState must not change
- usedRedeemEvents size must not change

If amountBld is invalid:

- BuildState must not change
- redeemKey must not be recorded

## Errors

Added BuildErrorCode value:

- DuplicateRedeemEvent

## Tests

Current redeem event replay tests verify:

- accepts a new Core redeem event and records redeemKey
- rejects duplicate redeemKey before applying BLD twice
- accepts different redeemKeys and accumulates BLD
- does not mark redeemKey when BLD amount is invalid
- does not create unrelated accounting values

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 7 test files passed
- 35 tests passed

## Main invariant

A Core redeem event can affect BuildState only once.

Replay protection must prevent duplicate history_bld / available_bld accounting.

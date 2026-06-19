# Stage 2.14 Relayer Input Preflight Guard Evidence

This document records Stage 2.14 relayer input preflight guard evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-14-relayer-event-input-preflight-guard

Runtime commit:

    3480662 Add Stage 2.14 relayer input preflight guard

Base runtime commit:

    b338be3 Add Stage 2.13 relayer operational state machine

## Scope

Stage 2.14 adds local relayer input preflight validation.

It does not change the on-chain runtime.

It is not a replacement for on-chain validation.

It is a relayer-side guard that rejects obviously invalid mint tasks before building or submitting a transaction.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_preflight_validation.test.ts

## Preflight model

The relayer preflight helper validates:

- canonical event key
- message nonce
- recipient
- minted amount
- deadline / finality block
- minimum quorum
- guardian signer count
- duplicate guardian signers

## Failure reasons

The preflight guard can reject inputs with:

    invalid_canonical_event_key
    invalid_message_nonce
    invalid_recipient
    invalid_minted_amount
    invalid_deadline_or_finality_block
    invalid_min_quorum
    insufficient_guardian_signers
    duplicate_guardian_signer

## Live Stage 2.14 test

Command:

    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_preflight_validation.test.ts

Result:

    Stage 2.14 relayer event input preflight guard
      ✔ accepts a valid relayer mint input
      ✔ rejects invalid canonical event keys
      ✔ rejects invalid message nonces
      ✔ rejects invalid recipients
      ✔ rejects zero minted amount
      ✔ rejects expired deadline or finality block
      ✔ rejects invalid quorum
      ✔ rejects insufficient guardian signers
      ✔ rejects duplicate guardian signers

    9 passing

## Regression checks

Stage 2.13 operational state machine remained green:

    Stage 2.13 relayer operational state machine
      ✔ maps recovery states to completed, retry-candidate, and manual-review decisions

    1 passing

Stage 2.12 inconsistent recovery remained green:

    Stage 2.12 relayer inconsistent recovery state handling
      ✔ classifies processed burn with unexpected balance delta as inconsistent and does not retry blindly

    1 passing

Stage 2.11 ambiguous recovery remained green:

    Stage 2.11 relayer ambiguous confirmation recovery
      ✔ recovers a completed mint after an ambiguous send result by checking processed burn and balance

    1 passing

Stage 2.10 idempotency / retry remained green:

    Stage 2.10 relayer idempotency / retry prototype
      ✔ stops safely when processed burn already exists and does not mint twice

    1 passing

Stage 2.9 relayer prototype remained green:

    Stage 2.9 TypeScript relayer prototype
      ✔ builds and submits the relayer transaction shape for a direct mint

    1 passing

Stage 2.6 rollback matrix remained green:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, mints tokens, rejects replay, and rolls back failed CPI
      ✔ rejects missing guardian signature instruction
      ✔ rejects wrong xxxl mint and leaves no processed burn
      ✔ rejects recipient token account with wrong mint and leaves no processed burn
      ✔ rejects recipient token account with wrong owner and leaves no processed burn
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    6 passing

Rust / Anchor checks remained green:

    cargo test -p hello-x1 binding_
    cargo test -p hello-x1 parser_
    anchor build

## Current conclusion

Stage 2.14 adds relayer-side preflight validation before transaction construction and submission.

The relayer can now reject malformed or unsafe input tasks before attempting to build a transaction.

The on-chain runtime remains unchanged.

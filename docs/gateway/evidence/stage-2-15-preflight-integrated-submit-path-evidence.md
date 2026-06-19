# Stage 2.15 Preflight Integrated Submit Path Evidence

This document records Stage 2.15 preflight-integrated submit path evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-15-preflight-integrated-submit-path

Runtime commit:

    a1bed6d Add Stage 2.15 preflight integrated submit path

Base runtime commit:

    3480662 Add Stage 2.14 relayer input preflight guard

## Scope

Stage 2.15 integrates the Stage 2.14 relayer preflight guard into the relayer submit path.

It does not change the on-chain runtime.

The relayer submit helper now performs preflight validation before:

- processed_burn lookup
- transaction construction
- transaction submission

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_preflight_integrated_submit.test.ts

## Submit result model

The relayer submit result now supports:

    submitted
    already_processed
    preflight_rejected

The new result is:

    preflight_rejected

with:

    signature = null
    reason = preflight failure reason

## Integrated behavior

Invalid input is rejected before transaction construction or submission.

The Stage 2.15 test verifies this with:

    mintedAmount = 0

Expected result:

    preflight_rejected
    reason = invalid_minted_amount
    signature = null

State preservation after rejected input:

- processed_burn remains absent
- recipient token balance remains unchanged

## Valid path

The integrated submit path still accepts valid input.

The Stage 2.15 test verifies that a valid relayer mint input still returns:

    submitted

and that:

- processed_burn exists
- recipient token balance increases by the expected minted amount

## Live Stage 2.15 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_preflight_integrated_submit.test.ts

Result:

    Stage 2.15 relayer preflight-integrated submit path
      ✔ rejects invalid input before building/submitting a transaction and preserves state
      ✔ still submits valid input through the integrated path

    2 passing

## Regression checks

Stage 2.14 preflight validation remained green:

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

Stage 2.15 confirms that relayer preflight is now part of the submit path.

Invalid relayer input can be rejected before transaction construction/submission, with no processed_burn entry and no token balance change.

The valid submit path remains functional.

The on-chain runtime remains unchanged.

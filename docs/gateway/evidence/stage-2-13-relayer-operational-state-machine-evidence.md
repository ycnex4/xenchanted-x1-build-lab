# Stage 2.13 Relayer Operational State Machine Evidence

This document records Stage 2.13 relayer operational state machine evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-13-relayer-operational-state-machine

Runtime commit:

    b338be3 Add Stage 2.13 relayer operational state machine

Base runtime commit:

    cc07651 Add Stage 2.12 inconsistent recovery state handling test

## Scope

Stage 2.13 adds a small operational decision layer to the TypeScript relayer prototype.

It does not change the on-chain runtime.

It formalizes how the relayer should map recovery results into operational actions after an ambiguous confirmation result.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_operational_state_machine.test.ts

## Operational decision model

The relayer recovery states are mapped to operational decisions.

Recovery state:

    confirmed_after_ambiguous_result

maps to:

    completed_no_retry

Meaning:

- processed_burn exists
- recipient token balance delta matches expected minted amount
- relayer treats the mint as completed
- no retry is needed

Recovery state:

    not_processed_after_ambiguous_result

maps to:

    safe_retry_candidate

Meaning:

- processed_burn does not exist
- recipient token balance did not change
- relayer may consider a safe retry path

Recovery state:

    inconsistent_after_ambiguous_result

maps to:

    stop_manual_review

Meaning:

- processed state and token balance delta disagree with the expected result
- relayer must not retry blindly
- operator/manual review is required

## Live Stage 2.13 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_operational_state_machine.test.ts

Result:

    Stage 2.13 relayer operational state machine
      ✔ maps recovery states to completed, retry-candidate, and manual-review decisions

    1 passing

## Regression checks

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

Stage 2.13 connects the Stage 2.10 through Stage 2.12 relayer behaviors into a clear operational state machine.

The relayer can now classify recovery results into:

- completed_no_retry
- safe_retry_candidate
- stop_manual_review

The on-chain runtime remains unchanged.

# Stage 2.11 Ambiguous Confirmation Recovery Evidence

This document records Stage 2.11 ambiguous confirmation recovery evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-11-ambiguous-confirmation-recovery

Runtime commit:

    3a0e1e0 Add Stage 2.11 ambiguous confirmation recovery prototype

Base runtime commit:

    5db96ed Add Stage 2.10 relayer idempotency retry prototype

## Scope

Stage 2.11 adds ambiguous confirmation recovery behavior to the TypeScript relayer prototype.

It does not change the on-chain runtime.

It models the case where the relayer has submitted a transaction but the confirmation result is ambiguous, for example because of:

- RPC timeout
- dropped RPC response
- unknown confirmation state
- client-side uncertainty after send

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_ambiguous_recovery.test.ts

## Recovery model

The relayer prototype now inspects on-chain state after an ambiguous send result.

It checks:

- whether processed_burn exists
- recipient token account balance delta
- expected minted amount

## Recovery statuses

The relayer recovery helper can return:

    confirmed_after_ambiguous_result

when:

- processed_burn exists
- recipient token balance increased by the expected minted amount

It can return:

    not_processed_after_ambiguous_result

when:

- processed_burn does not exist
- recipient token balance did not change

It can return:

    inconsistent_after_ambiguous_result

when processed state and balance delta do not match the expected completed or not-completed states.

## Live Stage 2.11 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_ambiguous_recovery.test.ts

Result:

    Stage 2.11 relayer ambiguous confirmation recovery
      ✔ recovers a completed mint after an ambiguous send result by checking processed burn and balance

    1 passing

## Regression checks

Stage 2.10 relayer idempotency / retry remained green:

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

Stage 2.11 confirms that the relayer can recover from an ambiguous confirmation result by inspecting protocol state.

If processed_burn exists and the recipient token balance increased by the expected minted amount, the relayer can classify the mint as completed and avoid blind resubmission.

The on-chain runtime remains unchanged.

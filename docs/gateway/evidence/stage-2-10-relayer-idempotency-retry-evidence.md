# Stage 2.10 Relayer Idempotency / Retry Prototype Evidence

This document records Stage 2.10 relayer idempotency / retry prototype evidence for the X1 direct mint gateway.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-10-relayer-idempotency-retry-prototype

Runtime commit:

    5db96ed Add Stage 2.10 relayer idempotency retry prototype

Base runtime commit:

    f0dbb4f Add Stage 2.9 TypeScript relayer prototype

## Scope

Stage 2.10 adds the first relayer idempotency / retry behavior.

It does not change the on-chain runtime.

It extends the Stage 2.9 TypeScript relayer prototype so the relayer checks whether processed_burn already exists before submitting a mint transaction.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_idempotency_retry.test.ts

## Added relayer behavior

The relayer prototype now returns:

    submitted

when it builds and sends the transaction successfully.

It returns:

    already_processed

when the processed_burn PDA already exists.

In the already_processed case:

- no new transaction is submitted
- no second mint happens
- the relayer stops safely

## Idempotency anchor

The idempotency anchor is the processed_burn PDA.

If processed_burn exists, the relayer treats the event as already completed and stops.

This matches the Stage 2.8 retry policy.

## Live Stage 2.10 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_idempotency_retry.test.ts

Result:

    Stage 2.10 relayer idempotency / retry prototype
      ✔ stops safely when processed burn already exists and does not mint twice

    1 passing

## Regression checks

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

Stage 2.10 confirms the first safe relayer retry behavior.

A repeated relayer run for an already processed event stops at processed_burn detection and does not mint twice.

The on-chain runtime remains unchanged.

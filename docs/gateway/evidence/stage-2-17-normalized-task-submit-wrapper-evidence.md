# Stage 2.17 Normalized Task Submit Wrapper Evidence

This document records Stage 2.17 normalized task submit wrapper evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-17-normalized-task-submit-wrapper

Runtime commit:

    4d6ecd7 Add Stage 2.17 normalized task submit wrapper

Base runtime commit:

    31560c9 Add Stage 2.16 relayer task normalization

## Scope

Stage 2.17 connects the normalized relayer task object from Stage 2.16 to the integrated submit path from Stage 2.15.

It adds a wrapper that accepts a normalized task and submits it through the existing guarded relayer submit helper.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_normalized_task_submit.test.ts

## New wrapper

The new wrapper is:

    submitStage2RelayerNormalizedMintTask

It accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- task: Stage2RelayerNormalizedMintTask

It forwards the normalized task into:

    submitStage2RelayerMintPrototype

## Submit path

The Stage 2.17 path is:

    watcher-style input
      -> normalizeStage2RelayerMintTask
      -> submitStage2RelayerNormalizedMintTask
      -> submitStage2RelayerMintPrototype
      -> preflight / idempotency / transaction build / submit

## Confirmed behavior

The Stage 2.17 test confirms that a normalized task can be submitted through the integrated submit path.

For a valid normalized task:

- submit result is submitted
- signature is a string
- processed_burn is created
- recipient token balance increases by the expected minted amount

The test also confirms that invalid watcher input remains outside the normalized submit wrapper:

- invalid minted amount is rejected during normalization
- no normalized task is produced
- submit wrapper is not called for invalid watcher input

## Live Stage 2.17 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_normalized_task_submit.test.ts

Result:

    Stage 2.17 normalized task submit wrapper
      ✔ submits a normalized relayer mint task through the integrated submit path
      ✔ keeps invalid watcher input outside the normalized submit wrapper

    2 passing

## Regression checks

Stage 2.16 task normalization remained green:

    Stage 2.16 relayer task normalization
      ✔ normalizes a valid watcher task into deterministic relayer submit fields
      ✔ rejects invalid watcher task input before normalization
      ✔ copies byte arrays so normalized task is stable after source mutation

    3 passing

Stage 2.15 preflight-integrated submit path remained green:

    Stage 2.15 relayer preflight-integrated submit path
      ✔ rejects invalid input before building/submitting a transaction and preserves state
      ✔ still submits valid input through the integrated path

    2 passing

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

Stage 2.17 connects normalized relayer tasks to the integrated submit path.

The relayer can now accept a normalized task object and submit it through the same protected path that already includes preflight, idempotency, transaction construction, and send/confirm behavior.

Invalid watcher-style input remains rejected before a normalized task is produced.

The on-chain runtime remains unchanged.

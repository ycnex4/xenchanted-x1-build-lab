# Stage 2.16 Relayer Task Normalization Evidence

This document records Stage 2.16 relayer task normalization evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-16-relayer-task-normalization

Runtime commit:

    31560c9 Add Stage 2.16 relayer task normalization

Base runtime commit:

    a1bed6d Add Stage 2.15 preflight integrated submit path

## Scope

Stage 2.16 adds a normalized relayer mint task object.

This creates a clearer boundary between watcher output and relayer submit logic.

It does not change the on-chain runtime.

The normalization layer converts watcher-style input into stable relayer submit fields.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_task_normalization.test.ts

## Normalized task model

The new normalization helper is:

    normalizeStage2RelayerMintTask

Input type:

    Stage2RelayerMintTaskInput

Output type:

    Stage2RelayerTaskNormalizationResult

Successful normalized task type:

    Stage2RelayerNormalizedMintTask

## Normalized fields

The normalized task derives or stabilizes:

- gatewayConfig PDA
- guardianSet PDA
- processedBurn PDA
- mintAuthority PDA
- canonicalEventKey byte array
- recipient
- mintedAmount
- guardianSetVersion
- deadlineOrFinalityBlock
- messageNonce byte array
- messageHash
- guardianSigners
- minQuorum
- currentFinalityBlock

## Preflight integration

The normalization helper calls Stage 2.14 preflight validation before returning a normalized task.

Invalid watcher-style input is rejected before normalization.

For invalid input, the result is:

    ok = false
    reason = preflight failure reason

## Stability behavior

The normalized task copies source byte arrays.

The Stage 2.16 test verifies that mutating the source input after normalization does not mutate:

- normalized canonicalEventKey
- normalized messageNonce
- normalized messageHash

## Stage 2.16 test

Command:

    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_task_normalization.test.ts

Result:

    Stage 2.16 relayer task normalization
      ✔ normalizes a valid watcher task into deterministic relayer submit fields
      ✔ rejects invalid watcher task input before normalization
      ✔ copies byte arrays so normalized task is stable after source mutation

    3 passing

## Regression checks

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

Stage 2.16 creates a stable normalization boundary between watcher-style input and relayer submit logic.

The relayer can now normalize an incoming task into deterministic PDAs, copied byte arrays, and a derived message hash before submit.

Invalid input is rejected through the existing preflight model before a normalized task is produced.

The on-chain runtime remains unchanged.

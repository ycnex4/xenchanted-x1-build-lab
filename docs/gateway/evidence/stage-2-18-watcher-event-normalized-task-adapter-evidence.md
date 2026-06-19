# Stage 2.18 Watcher Event Normalized Task Adapter Evidence

This document records Stage 2.18 watcher event to normalized task adapter evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-18-watcher-event-normalized-task-adapter

Runtime commit:

    ed52794 Add Stage 2.18 watcher event normalized task adapter

Base runtime commit:

    4d6ecd7 Add Stage 2.17 normalized task submit wrapper

## Scope

Stage 2.18 adds an adapter from watcher-style event input to a normalized relayer mint task.

It creates a clearer boundary between watcher output and the protected relayer pipeline.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_watcher_event_normalized_task_adapter.test.ts

## New adapter

The new adapter is:

    adaptStage2WatcherMintEventToNormalizedTask

It accepts watcher-style fields:

- canonicalEventKeyHex
- recipientBase58
- mintedAmount
- guardianSetVersion
- deadlineOrFinalityBlock
- messageNonceHex
- guardianSigners
- minQuorum
- currentFinalityBlock

It converts them into a normalized task through:

    normalizeStage2RelayerMintTask

## Adapter path

The Stage 2.18 path is:

    watcher event / candidate
      -> adaptStage2WatcherMintEventToNormalizedTask
      -> normalizeStage2RelayerMintTask
      -> submitStage2RelayerNormalizedMintTask
      -> submitStage2RelayerMintPrototype
      -> preflight / idempotency / transaction build / submit

## Parsing and validation

The adapter validates and parses:

- bytes32 hex canonical event key
- bytes32 hex message nonce
- base58 recipient public key
- decimal minted amount
- decimal guardian set version
- decimal deadline / finality block
- optional decimal current finality block

Malformed watcher fields are rejected before normalization.

After parsing, existing preflight validation still applies.

## Failure reasons

Adapter-level failures include:

    invalid_canonical_event_key_hex
    invalid_message_nonce_hex
    invalid_recipient_pubkey
    invalid_minted_amount_decimal
    invalid_guardian_set_version_decimal
    invalid_deadline_or_finality_block_decimal
    invalid_current_finality_block_decimal

Preflight failures can also be propagated, for example:

    invalid_minted_amount

## Confirmed behavior

The Stage 2.18 test confirms that a valid watcher event is adapted into a deterministic normalized relayer task.

It also confirms that:

- malformed canonical event key hex is rejected
- malformed recipient public key is rejected
- malformed decimal fields are rejected before normalization
- parsed but invalid economic input propagates preflight rejection

## Stage 2.18 test

Command:

    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_event_normalized_task_adapter.test.ts

Result:

    Stage 2.18 watcher event to normalized task adapter
      ✔ adapts a watcher event into a deterministic normalized relayer task
      ✔ rejects malformed canonical event key hex
      ✔ rejects malformed recipient public key
      ✔ rejects malformed decimal fields before normalization
      ✔ propagates preflight rejection after parsing watcher event fields

    5 passing

## Regression checks

Stage 2.17 normalized task submit wrapper remained green:

    Stage 2.17 normalized task submit wrapper
      ✔ submits a normalized relayer mint task through the integrated submit path
      ✔ keeps invalid watcher input outside the normalized submit wrapper

    2 passing

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

Stage 2.18 adds a watcher-event adapter in front of the normalized relayer task pipeline.

The relayer can now accept watcher-style event fields, parse and validate them, produce a normalized task, and keep malformed watcher output away from the submit path.

The on-chain runtime remains unchanged.

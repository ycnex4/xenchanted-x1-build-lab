# Stage 2.20 Watcher Event Submit Idempotency Retry Evidence

This document records Stage 2.20 watcher event submit idempotency / retry evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-20-watcher-event-submit-idempotency-retry

Runtime commit:

    5832233 Add Stage 2.20 watcher event submit idempotency retry

Base runtime commit:

    d1dbe96 Add Stage 2.19 watcher event full submit pipeline

## Scope

Stage 2.20 proves that the high-level watcher-event submit API from Stage 2.19 preserves idempotency / retry safety.

It does not add new runtime helper code.

It adds a live regression test for submitting the same watcher event twice through:

    submitStage2WatcherMintEventPrototype

It does not change the on-chain runtime.

## Runtime changes

Runtime test added:

    tests/stage2_watcher_event_submit_idempotency_retry.test.ts

No runtime helper changes were required.

## Tested path

The tested path is:

    watcher event
      -> submitStage2WatcherMintEventPrototype
      -> adaptStage2WatcherMintEventToNormalizedTask
      -> submitStage2RelayerNormalizedMintTask
      -> submitStage2RelayerMintPrototype
      -> processed_burn idempotency check
      -> protected submit path

## Confirmed behavior

The Stage 2.20 test submits the same watcher event twice.

First submit:

- returns status = submitted
- returns a signature
- creates processed_burn
- increases recipient token balance by expected minted amount

Second submit of the same watcher event:

- returns status = already_processed
- returns signature = null
- does not mint again
- does not change recipient token balance
- leaves processed_burn present

## Stage 2.20 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_event_submit_idempotency_retry.test.ts

Result:

    Stage 2.20 watcher event submit idempotency / retry
      ✔ stops safely when the same watcher event is submitted twice

    1 passing

## Regression checks

Stage 2.19 watcher event full submit pipeline remained green:

    Stage 2.19 watcher event full submit pipeline
      ✔ submits a watcher event through adapter, normalization, and protected submit path
      ✔ rejects malformed watcher event before submit
      ✔ propagates parsed watcher event preflight rejection before submit

    3 passing

Stage 2.18 watcher event adapter remained green:

    Stage 2.18 watcher event to normalized task adapter
      ✔ adapts a watcher event into a deterministic normalized relayer task
      ✔ rejects malformed canonical event key hex
      ✔ rejects malformed recipient public key
      ✔ rejects malformed decimal fields before normalization
      ✔ propagates preflight rejection after parsing watcher event fields

    5 passing

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

Stage 2.20 proves that the high-level watcher-event submit API is idempotent for repeated watcher events.

A repeated watcher event does not mint twice.

The second submit stops safely with already_processed, no signature, unchanged balance, and the processed_burn marker remains present.

The on-chain runtime remains unchanged.

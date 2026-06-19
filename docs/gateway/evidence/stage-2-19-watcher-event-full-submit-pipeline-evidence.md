# Stage 2.19 Watcher Event Full Submit Pipeline Evidence

This document records Stage 2.19 watcher event full submit pipeline evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-19-watcher-event-full-submit-pipeline

Runtime commit:

    d1dbe96 Add Stage 2.19 watcher event full submit pipeline

Base runtime commit:

    ed52794 Add Stage 2.18 watcher event normalized task adapter

## Scope

Stage 2.19 adds a high-level watcher event submit helper.

It connects the Stage 2.18 watcher-event adapter to the Stage 2.17 normalized task submit wrapper.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_watcher_event_full_submit_pipeline.test.ts

## New helper

The new helper is:

    submitStage2WatcherMintEventPrototype

It accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- event: Stage2RelayerWatcherMintEventInput

It returns either:

- an existing relayer submit result
- watcher_event_rejected with a parsing or preflight failure reason

## Full submit path

The Stage 2.19 path is:

    watcher event
      -> adaptStage2WatcherMintEventToNormalizedTask
      -> submitStage2RelayerNormalizedMintTask
      -> submitStage2RelayerMintPrototype
      -> preflight / idempotency / transaction build / submit

## Confirmed behavior

The Stage 2.19 test confirms that a valid watcher event can be submitted through the full relayer pipeline.

For a valid watcher event:

- watcher-style fields are parsed
- normalized task is produced
- protected submit path is used
- submit result is submitted
- signature is produced
- processed_burn is created
- recipient token balance increases by the expected minted amount

The test also confirms rejection behavior:

- malformed watcher event is rejected before submit
- parsed watcher event with preflight failure is rejected before submit
- rejected watcher events return signature = null

## Stage 2.19 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_event_full_submit_pipeline.test.ts

Result:

    Stage 2.19 watcher event full submit pipeline
      ✔ submits a watcher event through adapter, normalization, and protected submit path
      ✔ rejects malformed watcher event before submit
      ✔ propagates parsed watcher event preflight rejection before submit

    3 passing

## Regression checks

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

Stage 2.19 adds the first high-level watcher-event submit API for the relayer prototype.

The relayer can now accept a watcher-style event, parse it, normalize it, and submit it through the existing protected path.

Malformed watcher events and parsed preflight failures are rejected before submit.

The on-chain runtime remains unchanged.

# Stage 2.23 Watcher Event Batch Queue Processing Evidence

This document records Stage 2.23 watcher event batch / queue processing evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-23-watcher-event-batch-queue-processing

Runtime commit:

    d763177 Add Stage 2.23 watcher event batch queue processing

Base runtime commit:

    1a75653 Add Stage 2.22 watcher event operational submit wrapper

## Scope

Stage 2.23 adds a prototype batch / queue processing helper for watcher events.

It builds on the Stage 2.22 operational submit wrapper and processes watcher-event items sequentially.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_watcher_event_batch_queue_processing.test.ts

Stage 2.22 live-test hygiene was also improved:

    tests/stage2_watcher_event_operational_submit_wrapper.test.ts

The Stage 2.22 test now places the unique prefix at the beginning of the bytes32 input, so live tests do not accidentally reuse an old canonicalEventKey when long labels are truncated to 32 bytes.

## New helper

The new helper is:

    processStage2WatcherEventOperationalBatchPrototype

It accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- items

Each batch item contains:

- id
- event
- optional mode
- optional balanceBeforeOverride
- optional expectedMintedAmountOverride

Each batch result contains:

- id
- index
- result

The batch helper processes items sequentially and preserves item order.

## Tested queue

The Stage 2.23 test processes a mixed watcher-event queue with these outcomes:

    submitted
    watcher_event_rejected
    submitted
    already_processed
    safe_retry_candidate
    completed_no_retry
    stop_manual_review

The tested queue includes:

- fresh submit
- malformed watcher event
- repeated event first submit
- repeated event second submit
- ambiguous recovery before submit
- submit then ambiguous recovery
- inconsistent submit then ambiguous recovery

## Confirmed behavior

The batch helper confirms:

- item ids are preserved
- item indexes are preserved
- result order matches processing order
- malformed watcher event does not stop the queue
- repeated event becomes already_processed on second occurrence
- safe_retry_candidate can appear in a batch without submit
- completed_no_retry can appear after submit-then-recover
- stop_manual_review can appear for inconsistent recovery
- total balance delta equals only actually minted amounts

Expected minted total:

    10101 + 20202 + 40404 + 50505 = 121212

Confirmed balance delta:

    121212

## Stage 2.23 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_event_batch_queue_processing.test.ts

Result:

    Stage 2.23 watcher event batch / queue processing
      ✔ processes a watcher-event queue and returns mixed operational outcomes

    1 passing

## Regression checks

Stage 2.22 watcher event operational submit wrapper remained green after the canonicalEventKey live-test hygiene fix:

    Stage 2.22 watcher event operational submit wrapper
      ✔ returns submitted for normal watcher-event operational submit
      ✔ returns already_processed for repeated watcher-event operational submit
      ✔ returns safe_retry_candidate for ambiguous recovery before submit
      ✔ returns completed_no_retry for ambiguous recovery after submitted watcher event
      ✔ returns stop_manual_review for inconsistent ambiguous watcher-event recovery
      ✔ returns watcher_event_rejected for malformed watcher event

    6 passing

Stage 2.21 watcher event ambiguous recovery remained green:

    Stage 2.21 watcher event ambiguous recovery
      ✔ recovers ambiguous watcher-event submit results without blind retry

    1 passing

Stage 2.20 watcher event submit idempotency / retry remained green:

    Stage 2.20 watcher event submit idempotency / retry
      ✔ stops safely when the same watcher event is submitted twice

    1 passing

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

Stage 2.23 creates a prototype batch / queue processing model for watcher events.

The relayer can now process a sequence of watcher-event operational items and receive ordered per-item outcomes.

The queue can contain successful submits, malformed watcher events, retries, ambiguous recovery candidates, completed recoveries, and manual-review cases without collapsing the whole batch.

The on-chain runtime remains unchanged.

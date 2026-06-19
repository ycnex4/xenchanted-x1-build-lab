# Stage 2.24 Durable Relayer Journal Model Evidence

This document records Stage 2.24 durable relayer journal model evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-24-durable-relayer-journal-model

Runtime commit:

    4c8d930 Add Stage 2.24 durable relayer journal model

Base runtime commit:

    d763177 Add Stage 2.23 watcher event batch queue processing

## Scope

Stage 2.24 adds a prototype durable journal model for the watcher-event relayer path.

It builds on Stage 2.23 batch / queue processing and records relayer progress across watcher events.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_durable_relayer_journal_model.test.ts

## New journal record model

New journal record kinds:

    event_received
    operational_result
    final_outcome
    retry_candidate
    manual_review_required

Each journal record contains:

- sequence
- itemId
- itemIndex
- kind
- optional result summary

Operational result summaries include:

- status
- signature
- optional reason
- optional processedBurnExists
- optional balanceDelta

BigInt balance deltas are serialized as decimal strings.

## New helpers

New helpers:

    createStage2RelayerJournalPrototype
    serializeStage2RelayerJournalPrototype
    deserializeStage2RelayerJournalPrototype
    processStage2WatcherEventOperationalBatchWithJournalPrototype

The journaled batch helper:

- accepts an optional existing journal
- appends records sequentially
- records event receipt
- records operational result
- records final classification
- returns both journal and batch results

## Tested flow

The Stage 2.24 test confirms:

    watcher-event batch
      -> journal records created
      -> journal serialized
      -> journal deserialized
      -> processing continues after reload
      -> repeated submitted event becomes already_processed
      -> no second mint occurs

## First pass tested outcomes

The first journaled batch contains:

    submitted
    watcher_event_rejected
    safe_retry_candidate
    completed_no_retry
    stop_manual_review

Expected journal record kinds:

    event_received
    operational_result
    final_outcome
    event_received
    operational_result
    final_outcome
    event_received
    operational_result
    retry_candidate
    event_received
    operational_result
    final_outcome
    event_received
    operational_result
    manual_review_required

Expected sequence:

    0..14

## Reload / restart behavior

After serialization and deserialization, the journal is reloaded and reused.

A repeated previously submitted watcher event is processed after reload.

Expected result:

    already_processed

Expected second-pass journal append:

    event_received
    operational_result
    final_outcome

Expected appended sequence:

    15, 16, 17

Expected balance behavior:

- first pass mints only actual successful amounts
- second pass does not mint again
- balance after second pass equals balance after first pass

## Malformed journal checks

The deserializer rejects malformed journals.

Rejected cases:

- records is not an array
- sequence does not match record index

Expected errors:

    invalid_stage2_relayer_journal_records
    invalid_stage2_relayer_journal_sequence

## Stage 2.24 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_durable_relayer_journal_model.test.ts

Result:

    Stage 2.24 durable relayer journal model
      ✔ records, serializes, reloads, and continues watcher-event batch processing safely
      ✔ rejects malformed serialized journals

    2 passing

## Regression checks

Stage 2.23 watcher event batch / queue processing remained green:

    Stage 2.23 watcher event batch / queue processing
      ✔ processes a watcher-event queue and returns mixed operational outcomes

    1 passing

Stage 2.22 watcher event operational submit wrapper remained green:

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

Stage 2.24 creates a durable relayer journal prototype for watcher-event processing.

The relayer can now record event receipt, operational results, final outcomes, retry candidates, and manual-review cases.

The journal can be serialized, reloaded, validated, and reused after restart.

After reload, a previously submitted event is safely classified as already_processed and does not mint again.

The on-chain runtime remains unchanged.

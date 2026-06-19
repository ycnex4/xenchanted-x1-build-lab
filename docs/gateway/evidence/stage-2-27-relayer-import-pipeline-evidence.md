# Stage 2.27 Relayer Import Pipeline Evidence

This document records Stage 2.27 relayer import pipeline evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-27-relayer-import-pipeline

Runtime commit:

    9e028ec Add Stage 2.27 relayer import pipeline

Base runtime commit:

    1bd9743 Add Stage 2.26 relayer dedupe journal replay guard

## Scope

Stage 2.27 adds a high-level watcher contract import pipeline.

It combines:

- Stage 2.25 watcher-to-relayer contract boundary
- Stage 2.26 relayer dedupe / journal replay guard
- Stage 2.24 durable journaled relayer processing

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_import_pipeline.test.ts

## New import summary model

New summary type:

    Stage2WatcherContractImportSummary

Summary fields:

- accepted
- duplicates
- manualReview
- invalid
- processed
- submitted
- alreadyProcessed
- watcherEventRejected
- retryCandidates
- completedNoRetry
- manualReviewOutcomes

## New import pipeline result

New result type:

    Stage2WatcherContractImportPipelineResult

The result contains:

- journal
- importResults
- acceptedBatchItems
- batchResults
- summary

## New helper

New helper:

    importStage2WatcherContractsWithJournalPrototype

The helper accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- journal
- contracts

The helper returns a full import pipeline result.

## Pipeline behavior

For each watcher contract, the import pipeline:

1. runs watcher-to-relayer contract validation
2. runs dedupe / replay guard
3. records import result
4. counts accepted / duplicate / manual review / invalid outcomes
5. converts accepted contracts into operational batch items
6. processes accepted batch items through the journaled relayer path
7. counts operational batch outcomes
8. returns import results, accepted items, batch results, journal, and summary

## Confirmed mixed import behavior

The Stage 2.27 mixed import test includes:

- valid contract A
- duplicate of contract A
- same dedupeKey with different payload
- same canonicalEventKey with different dedupeKey
- invalid metadata
- invalid watcher event payload
- valid contract B

Expected import statuses:

    accepted
    duplicate_existing
    manual_review_required
    manual_review_required
    invalid_contract
    invalid_contract
    accepted

Expected summary:

    accepted: 2
    duplicates: 1
    manualReview: 2
    invalid: 2
    processed: 2
    submitted: 2
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReviewOutcomes: 0

Accepted batch item ids:

- contractA.eventId
- contractB.eventId

Batch result statuses:

    submitted
    submitted

Confirmed balance delta:

    33333

This equals the sum of accepted submitted amounts:

    11111 + 22222 = 33333

## Journal behavior

The import pipeline creates journal records only for accepted processed items.

Expected journal record kinds:

    event_received
    operational_result
    final_outcome
    event_received
    operational_result
    final_outcome

Expected journal item ids:

- contractA.eventId
- contractA.eventId
- contractA.eventId
- contractB.eventId
- contractB.eventId
- contractB.eventId

Expected dedupe records:

    contractRecords.length = 2

## Conflict behavior

Payload conflict result:

    manual_review_required

Payload conflict reason:

    dedupe_key_payload_mismatch

Canonical event conflict result:

    manual_review_required

Canonical event conflict reason:

    canonical_event_key_dedupe_mismatch

## Invalid input behavior

Invalid watcher event result:

    invalid_contract

Invalid watcher event reason:

    invalid_watcher_event

Propagated watcher event reason:

    invalid_canonical_event_key_hex

## Reload / duplicate import behavior

After serialization and deserialization of the journal, contractA and contractB are imported again.

Expected statuses:

    duplicate_existing
    duplicate_existing

Expected accepted batch items:

    []

Expected batch results:

    []

Expected duplicate import summary:

    accepted: 0
    duplicates: 2
    manualReview: 0
    invalid: 0
    processed: 0
    submitted: 0
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReviewOutcomes: 0

Confirmed duplicate import balance behavior:

- balance after duplicate import equals balance after first import
- no replay mint occurs

## Invalid-only import behavior

The pipeline also handles a batch containing only invalid input.

For invalid eventId:

    eventId = ""

Expected import result:

    invalid_contract

Expected reason:

    invalid_event_id

Expected accepted batch items:

    []

Expected batch results:

    []

Expected journal records:

    []

Expected contractRecords:

    undefined

Expected summary:

    accepted: 0
    duplicates: 0
    manualReview: 0
    invalid: 1
    processed: 0
    submitted: 0
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReviewOutcomes: 0

## Stage 2.27 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_import_pipeline.test.ts

Result:

    Stage 2.27 relayer import pipeline
      ✔ imports watcher contracts, processes accepted items, and skips duplicates/conflicts/invalid input
      ✔ returns only invalid import results without relayer processing

    2 passing

## Regression checks

Stage 2.26 relayer dedupe journal replay guard remained green:

    Stage 2.26 relayer dedupe journal replay guard
      ✔ accepts a new watcher contract and records its dedupe entry
      ✔ classifies the same watcher contract as duplicate_existing after journal reload
      ✔ requires manual review for the same dedupeKey with a different payload
      ✔ requires manual review for the same canonicalEventKey with a different dedupeKey
      ✔ returns invalid_contract for malformed watcher-to-relayer contracts
      ✔ processes accepted dedupe-guard output through the journaled relayer path and blocks replay import

    6 passing

Stage 2.25 watcher-to-relayer contract boundary remained green:

    Stage 2.25 watcher-to-relayer contract boundary
      ✔ accepts a valid watcher-to-relayer contract and converts it into a batch item
      ✔ rejects malformed watcher-to-relayer contract metadata before relayer submit
      ✔ rejects malformed watcher event payload at the watcher-to-relayer boundary
      ✔ processes an accepted watcher-to-relayer contract through the journaled relayer path

    4 passing

Stage 2.24 durable relayer journal model remained green:

    Stage 2.24 durable relayer journal model
      ✔ records, serializes, reloads, and continues watcher-event batch processing safely
      ✔ rejects malformed serialized journals

    2 passing

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

Stage 2.27 creates a high-level relayer import pipeline for watcher contracts.

The relayer can now import a batch of watcher-provided contracts, validate each contract, apply dedupe / replay guard, convert accepted contracts into operational batch items, and process accepted items through the durable journaled relayer path.

Duplicates are skipped.

Conflicts are routed to manual review.

Invalid input is rejected before relayer submit.

Duplicate import after journal reload does not mint again.

Invalid-only imports do not create journal records or dedupe records.

The on-chain runtime remains unchanged.

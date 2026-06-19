# Stage 2.28 Import Pipeline Durable Resume Plan Evidence

This document records Stage 2.28 import pipeline durable resume plan evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-28-import-pipeline-durable-resume-plan

Runtime commit:

    52e984e Add Stage 2.28 import pipeline durable resume plan

Base runtime commit:

    9e028ec Add Stage 2.27 relayer import pipeline

## Scope

Stage 2.28 adds a durable resume planning layer for watcher contract import.

It builds on:

- Stage 2.27 relayer import pipeline
- Stage 2.26 relayer dedupe / journal replay guard
- Stage 2.24 durable relayer journal model

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_import_pipeline_durable_resume_plan.test.ts

## New resume action model

New action type:

    Stage2WatcherContractResumeAction

Supported actions:

    ready_to_import
    skip_duplicate
    skip_completed
    retry_candidate
    manual_review_required
    rejected_invalid

## New resume reason model

New reason type:

    Stage2WatcherContractResumeReason

Supported reasons include:

    new_contract
    duplicate_without_final_outcome
    already_completed
    retry_candidate_recorded
    manual_review_recorded
    dedupe_key_payload_mismatch
    canonical_event_key_dedupe_mismatch

It also supports existing watcher-to-relayer contract failure reasons.

## New plan item model

New plan item type:

    Stage2WatcherContractResumePlanItem

Each plan item may include:

- index
- action
- reason
- eventId
- journalId
- dedupeKey
- canonicalEventKeyHex
- importResultStatus
- latestJournalKind
- watcherEventReason

## New plan summary model

New summary type:

    Stage2WatcherContractResumePlanSummary

Summary fields:

- readyToImport
- skipDuplicate
- skipCompleted
- retryCandidates
- manualReview
- rejectedInvalid

## New helper

New helper:

    planStage2WatcherContractImportResumePrototype

The helper accepts:

- durable relayer dedupe journal
- watcher-to-relayer contracts

The helper returns:

- plans
- summary

## Shadow journal policy

The resume planner uses a serialized/deserialized shadow journal when checking new contracts.

This means planning can classify new contracts and intra-plan duplicates without mutating the original journal.

Confirmed behavior:

- original journal contractRecords remain unchanged
- original journal records remain unchanged

## Completed / new / conflict / invalid planning

The Stage 2.28 test first imports two contracts through the Stage 2.27 import pipeline.

Accepted submitted contracts:

- contractA
- contractB

Confirmed submitted count:

    2

Confirmed balance delta:

    33333

The journal is then serialized and deserialized.

The resume planner receives:

- completed contractA
- completed contractB
- new contract
- same dedupeKey with different payload
- same canonicalEventKey with different dedupeKey
- invalid contract

Expected actions:

    skip_completed
    skip_completed
    ready_to_import
    manual_review_required
    manual_review_required
    rejected_invalid

Expected reasons:

    already_completed
    already_completed
    new_contract
    dedupe_key_payload_mismatch
    canonical_event_key_dedupe_mismatch
    invalid_event_id

Expected summary:

    readyToImport: 1
    skipDuplicate: 0
    skipCompleted: 2
    retryCandidates: 0
    manualReview: 2
    rejectedInvalid: 1

Expected dedupe record count after planning:

    contractRecords.length = 2

## Retry candidate planning

The test creates a journal with an accepted watcher contract and then appends a retry candidate journal record:

    retry_candidate

Expected resume action:

    retry_candidate

Expected reason:

    retry_candidate_recorded

Expected import result status:

    duplicate_existing

Expected latest journal kind:

    retry_candidate

Expected summary:

    readyToImport: 0
    skipDuplicate: 0
    skipCompleted: 0
    retryCandidates: 1
    manualReview: 0
    rejectedInvalid: 0

## Manual review journal planning

The test creates a journal with an accepted watcher contract and then appends a manual review journal record:

    manual_review_required

Expected resume action:

    manual_review_required

Expected reason:

    manual_review_recorded

Expected import result status:

    duplicate_existing

Expected latest journal kind:

    manual_review_required

Expected summary:

    readyToImport: 0
    skipDuplicate: 0
    skipCompleted: 0
    retryCandidates: 0
    manualReview: 1
    rejectedInvalid: 0

## Non-mutating planning behavior

The test plans:

- new contract A
- new contract B
- duplicate of contract A

Expected actions:

    ready_to_import
    ready_to_import
    skip_duplicate

Expected summary:

    readyToImport: 2
    skipDuplicate: 1
    skipCompleted: 0
    retryCandidates: 0
    manualReview: 0
    rejectedInvalid: 0

Confirmed original journal behavior:

- contractRecords remains undefined
- records remains empty

## Stage 2.28 test

Initial full run had one network/RPC fetch failure:

    TypeError: fetch failed

The test was rerun successfully.

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_import_pipeline_durable_resume_plan.test.ts

Successful result:

    Stage 2.28 import pipeline durable resume plan
      ✔ plans completed, new, conflict, and invalid watcher contracts after journal reload
      ✔ plans retry candidates and manual-review journal states without submitting
      ✔ does not mutate the original journal while planning new contracts

    3 passing

## Regression checks

The full Stage 2.28 run continued through regressions after the initial network failure.

Stage 2.27 relayer import pipeline remained green:

    Stage 2.27 relayer import pipeline
      ✔ imports watcher contracts, processes accepted items, and skips duplicates/conflicts/invalid input
      ✔ returns only invalid import results without relayer processing

    2 passing

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

Stage 2.28 creates a durable resume planning layer for the Stage 2.27 relayer import pipeline.

The relayer can now inspect watcher contracts against a durable journal and produce a plan before submit.

Completed contracts are skipped.

New contracts are marked ready_to_import.

Conflicts are routed to manual_review_required.

Invalid contracts are marked rejected_invalid.

Retry candidate journal states are surfaced as retry_candidate.

Manual-review journal states are surfaced as manual_review_required.

The planner uses a shadow journal and does not mutate the original journal during planning.

The on-chain runtime remains unchanged.

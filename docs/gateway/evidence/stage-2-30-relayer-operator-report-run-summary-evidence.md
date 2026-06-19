# Stage 2.30 Relayer Operator Report / Run Summary Evidence

This document records Stage 2.30 relayer operator report / run summary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-30-relayer-operator-report-run-summary

Runtime commit:

    f3ea200 Add Stage 2.30 relayer operator report run summary

Base runtime commit:

    c029ff3 Add Stage 2.29 resume plan execution model

## Scope

Stage 2.30 adds an operator-facing report layer above Stage 2.29 resume plan execution.

It builds on:

- Stage 2.29 resume plan execution model
- Stage 2.28 import pipeline durable resume plan
- Stage 2.27 relayer import pipeline
- Stage 2.26 relayer dedupe / journal replay guard
- Stage 2.24 durable relayer journal model

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_operator_report_run_summary.test.ts

## New operator report decision model

New type:

    Stage2RelayerOperatorReportDecision

Each decision may include:

- index
- planAction
- status
- reason
- eventId
- journalId
- dedupeKey
- canonicalEventKeyHex
- importResultStatus
- latestJournalKind
- watcherEventReason
- batchResultStatus

## New operator run report model

New type:

    Stage2RelayerOperatorRunReport

Report fields:

- runId
- startedAtIso
- completedAtIso
- contractsReceived
- journalRecordsBefore
- journalRecordsAfter
- contractRecordsBefore
- contractRecordsAfter
- balanceBefore
- balanceAfter
- balanceDelta
- executionSummary
- decisions

## New report result model

New type:

    Stage2RelayerOperatorRunReportResult

Result fields:

- execution
- report

## New helper

New helper:

    executeStage2WatcherContractResumePlanWithOperatorReportPrototype

The helper accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- journal
- contracts
- runId
- optional startedAtIso
- optional completedAtIso

The helper returns:

- execution result from Stage 2.29
- operator report

## Run id validation

The helper rejects empty operator run ids.

Confirmed error:

    runId must not be empty

This prevents ambiguous or unnamed operator reports.

## Mixed resume execution report

The Stage 2.30 test first imports a completed contract through the Stage 2.27 import pipeline.

Then it reloads the journal and runs an operator report over a mixed contract set:

- completed contract
- ready contract
- payload conflict
- invalid contract

Run metadata:

    runId: stage-2-30-run-001
    startedAtIso: 2026-01-01T00:00:00.000Z
    completedAtIso: 2026-01-01T00:00:10.000Z

Expected contracts received:

    4

Expected journal deltas:

    journalRecordsBefore: 3
    journalRecordsAfter: 6

Expected contract record deltas:

    contractRecordsBefore: 1
    contractRecordsAfter: 2

Expected balance delta:

    22222

Expected execution summary:

    readyToImport: 1
    submitted: 1
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReview: 1
    rejectedInvalid: 1
    skippedCompleted: 1
    skippedDuplicate: 0

Expected decision statuses:

    skipped_completed
    submitted
    manual_review_queued
    rejected_invalid

Expected plan actions:

    skip_completed
    ready_to_import
    manual_review_required
    rejected_invalid

Expected submitted decision fields for the ready contract:

    index: 1
    planAction: ready_to_import
    status: submitted
    reason: new_contract
    importResultStatus: accepted
    batchResultStatus: submitted

Expected accepted batch item:

- ready.eventId

## Retry/manual-review queued report behavior

The test creates:

- a retry candidate journal
- a manual review journal

Retry candidate setup:

- accepted watcher contract
- journal record kind: retry_candidate

Expected retry report:

    importResult: undefined
    balanceDelta: 0
    journalRecordsBefore: 1
    journalRecordsAfter: 1
    contractRecordsBefore: 1
    contractRecordsAfter: 1

Expected retry decision status:

    retry_queued

Expected retry summary:

    readyToImport: 0
    submitted: 0
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 1
    completedNoRetry: 0
    manualReview: 0
    rejectedInvalid: 0
    skippedCompleted: 0
    skippedDuplicate: 0

Manual review setup:

- accepted watcher contract
- journal record kind: manual_review_required

Expected manual review report:

    importResult: undefined
    balanceDelta: 0
    journalRecordsBefore: 1
    journalRecordsAfter: 1
    contractRecordsBefore: 1
    contractRecordsAfter: 1

Expected manual review decision status:

    manual_review_queued

Expected manual review summary:

    readyToImport: 0
    submitted: 0
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReview: 1
    rejectedInvalid: 0
    skippedCompleted: 0
    skippedDuplicate: 0

## Stage 2.30 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_operator_report_run_summary.test.ts

Result:

    Stage 2.30 relayer operator report / run summary
      ✔ reports a mixed resume execution run with journal and balance deltas
      ✔ reports retry/manual-review queued runs with zero balance delta and no import result
      ✔ rejects empty operator run ids

    3 passing

## Regression checks

Stage 2.29 resume plan execution model remained green:

    Stage 2.29 resume plan execution model
      ✔ executes only ready_to_import contracts and leaves completed/conflict/invalid plans unsubmitted
      ✔ queues retry/manual-review plans without blind submit
      ✔ handles duplicate-in-plan execution without submitting the duplicate twice

    3 passing

Stage 2.28 import pipeline durable resume plan remained green:

    Stage 2.28 import pipeline durable resume plan
      ✔ plans completed, new, conflict, and invalid watcher contracts after journal reload
      ✔ plans retry candidates and manual-review journal states without submitting
      ✔ does not mutate the original journal while planning new contracts

    3 passing

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

Stage 2.30 creates an operator-facing report layer above Stage 2.29 resume plan execution.

The relayer can now produce a run report containing run id, timestamps, received contract count, journal record deltas, dedupe contract record deltas, token balance delta, execution summary, and per-contract decisions.

Retry/manual-review queued runs produce zero balance delta and no import result.

Empty run ids are rejected.

The on-chain runtime remains unchanged.

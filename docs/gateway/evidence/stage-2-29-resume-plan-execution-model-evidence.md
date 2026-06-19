# Stage 2.29 Resume Plan Execution Model Evidence

This document records Stage 2.29 resume plan execution model evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-29-resume-plan-execution-model

Runtime commit:

    c029ff3 Add Stage 2.29 resume plan execution model

Base runtime commit:

    52e984e Add Stage 2.28 import pipeline durable resume plan

## Scope

Stage 2.29 adds an execution model for the Stage 2.28 durable resume plan.

It builds on:

- Stage 2.28 import pipeline durable resume plan
- Stage 2.27 relayer import pipeline
- Stage 2.26 relayer dedupe / journal replay guard
- Stage 2.24 durable relayer journal model

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_resume_plan_execution_model.test.ts

## New execution status model

New status type:

    Stage2WatcherContractResumeExecutionStatus

Supported statuses include existing operational batch result statuses plus:

    skipped_completed
    skipped_duplicate
    retry_queued
    manual_review_queued
    rejected_invalid

## New execution item model

New item type:

    Stage2WatcherContractResumeExecutionItem

Each execution item may include:

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

## New execution summary model

New summary type:

    Stage2WatcherContractResumeExecutionSummary

Summary fields:

- readyToImport
- submitted
- alreadyProcessed
- watcherEventRejected
- retryCandidates
- completedNoRetry
- manualReview
- rejectedInvalid
- skippedCompleted
- skippedDuplicate

## New helper

New helper:

    executeStage2WatcherContractResumePlanPrototype

The helper accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- journal
- contracts

The helper returns:

- plan
- importResult
- executions
- summary

## Execution policy

The helper first builds a Stage 2.28 resume plan.

Then it executes only contracts with action:

    ready_to_import

Execution behavior:

    ready_to_import -> submitted through Stage 2.27 import pipeline
    skip_completed -> skipped_completed
    skip_duplicate -> skipped_duplicate
    retry_candidate -> retry_queued
    manual_review_required -> manual_review_queued
    rejected_invalid -> rejected_invalid

Retry candidate and manual review plans do not blindly submit.

Invalid contracts do not submit.

Completed and duplicate contracts do not submit.

## Ready / completed / conflict / invalid execution

The Stage 2.29 test first imports two completed contracts through the Stage 2.27 import pipeline:

- completedA
- completedB

Then it reloads the journal and executes a mixed resume plan containing:

- completedA
- ready contract
- payload conflict
- invalid contract
- completedB

Expected plan actions:

    skip_completed
    ready_to_import
    manual_review_required
    rejected_invalid
    skip_completed

Expected execution statuses:

    skipped_completed
    submitted
    manual_review_queued
    rejected_invalid
    skipped_completed

Expected summary:

    readyToImport: 1
    submitted: 1
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReview: 1
    rejectedInvalid: 1
    skippedCompleted: 2
    skippedDuplicate: 0

Expected importResult acceptedBatchItems:

- ready.eventId

Expected importResult batchResults:

- ready.eventId

Expected batch result status:

    submitted

Confirmed balance delta:

    33333

Confirmed dedupe records after execution:

    contractRecords.length = 3

This means only the new ready contract was submitted. Completed, conflict, and invalid items were not submitted.

## Retry/manual-review no blind submit behavior

The test creates one retry candidate journal and one manual review journal.

Retry candidate setup:

- accepted watcher contract
- journal record kind: retry_candidate

Expected retry execution:

    retry_queued

Expected retry importResult:

    undefined

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

Confirmed retry balance behavior:

- balance after retry execution equals balance before retry execution

Manual review setup:

- accepted watcher contract
- journal record kind: manual_review_required

Expected manual review execution:

    manual_review_queued

Expected manual review importResult:

    undefined

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

Confirmed manual review balance behavior:

- balance after manual review execution equals balance before manual review execution

## Duplicate-in-plan execution behavior

The test executes a plan containing:

- readyA
- readyB
- duplicate of readyA

Expected plan actions:

    ready_to_import
    ready_to_import
    skip_duplicate

Expected execution statuses:

    submitted
    submitted
    skipped_duplicate

Expected summary:

    readyToImport: 2
    submitted: 2
    alreadyProcessed: 0
    watcherEventRejected: 0
    retryCandidates: 0
    completedNoRetry: 0
    manualReview: 0
    rejectedInvalid: 0
    skippedCompleted: 0
    skippedDuplicate: 1

Expected acceptedBatchItems:

- readyA.eventId
- readyB.eventId

Confirmed balance delta:

    188887

This equals:

    88888 + 99999 = 188887

Confirmed dedupe records:

    contractRecords.length = 2

This confirms the duplicate item is not submitted twice.

## Stage 2.29 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_resume_plan_execution_model.test.ts

Result:

    Stage 2.29 resume plan execution model
      ✔ executes only ready_to_import contracts and leaves completed/conflict/invalid plans unsubmitted
      ✔ queues retry/manual-review plans without blind submit
      ✔ handles duplicate-in-plan execution without submitting the duplicate twice

    3 passing

## Regression checks

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

Stage 2.29 creates a safe resume plan execution model.

The relayer can now take a Stage 2.28 durable resume plan and execute only ready_to_import contracts through the Stage 2.27 import pipeline.

Completed contracts are skipped.

Duplicates are skipped.

Retry candidates are queued and not blindly submitted.

Manual-review plans are queued and not submitted.

Invalid contracts are rejected and not submitted.

Duplicate-in-plan entries are not submitted twice.

The balance delta equals only the sum of actually submitted contracts.

The on-chain runtime remains unchanged.

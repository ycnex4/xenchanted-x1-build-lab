# Stage 2.32 Operator Report Audit Log Append Model Evidence

This document records Stage 2.32 operator report audit log append model evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-32-operator-report-audit-log-append-model

Runtime commit:

    9daa74f Add Stage 2.32 operator report audit log append model

Base runtime commit:

    b711f62 Add Stage 2.31 operator report serialization log artifact

## Scope

Stage 2.32 adds an append-only audit log model above Stage 2.31 report artifacts.

It turns individual stable operator report artifacts into an ordered audit log artifact that can be serialized, reloaded, validated, and checked for duplicate run ids.

It builds on:

- Stage 2.31 operator report serialization / stable log artifact
- Stage 2.30 relayer operator report / run summary
- Stage 2.29 resume plan execution model
- Stage 2.28 import pipeline durable resume plan
- Stage 2.27 relayer import pipeline
- Stage 2.26 relayer dedupe / journal replay guard

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_operator_report_audit_log_append_model.test.ts

## New audit log artifact model

New type:

    Stage2RelayerOperatorAuditLogArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_audit_log"
    schemaVersion: 1
    reports: Stage2RelayerOperatorRunReportLogArtifact[]

## New append result model

New type:

    Stage2RelayerOperatorAuditLogAppendResult

Append result statuses:

    appended
    duplicate_run_id

Successful append result contains:

- status
- reportCount
- runId

Duplicate run id result contains:

- status
- reportCount
- runId

## New validation model

New validation reason type:

    Stage2RelayerOperatorAuditLogValidationReason

Validation reasons:

- invalid_audit_log
- invalid_artifact_type
- invalid_schema_version
- invalid_reports
- duplicate_run_id
- invalid_report_artifact

New validation result type:

    Stage2RelayerOperatorAuditLogValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason, index?, runId? }

## New helpers

New audit log creation helper:

    createStage2RelayerOperatorAuditLogPrototype

New append helper:

    appendStage2RelayerOperatorAuditLogPrototype

New validation helper:

    validateStage2RelayerOperatorAuditLogPrototype

New serialization helper:

    serializeStage2RelayerOperatorAuditLogPrototype

New deserialization helper:

    deserializeStage2RelayerOperatorAuditLogPrototype

## Audit log creation behavior

A new audit log starts as:

    artifactType: stage2_relayer_operator_audit_log
    schemaVersion: 1
    reports: []

This creates an empty, schema-versioned audit trail container.

## Append behavior

The append helper validates the audit log before appending.

It validates the incoming report artifact before appending.

It rejects duplicate run ids without mutating the audit log.

Successful append returns:

    status: appended
    reportCount
    runId

Duplicate run id returns:

    status: duplicate_run_id
    reportCount
    runId

## Serialization behavior

The audit log is serialized as deterministic pretty JSON:

    JSON.stringify(auditLog, null, 2)

Serializing the same audit log twice produces the same string.

The serialized audit log contains:

    artifactType: stage2_relayer_operator_audit_log
    schemaVersion: 1
    reports

## Deserialization behavior

Deserialization rejects malformed JSON.

Confirmed error:

    invalid operator audit log: invalid_json

Deserialization rejects wrong artifact type.

Confirmed error:

    invalid operator audit log: invalid_artifact_type

Deserialization rejects unsupported schema version.

Confirmed error:

    invalid operator audit log: invalid_schema_version

Deserialization rejects non-array reports.

Confirmed error:

    invalid operator audit log: invalid_reports

Deserialization validates every embedded report artifact.

Malformed embedded report artifacts are rejected.

## Ordered append / reload test

The Stage 2.32 test creates two live Stage 2.31 report artifacts through the Stage 2.30 report helper.

First run:

    runId: stage-2-32-run-001
    mintedAmount: 10101

Second run:

    runId: stage-2-32-run-002
    mintedAmount: 20202

Expected first append result:

    status: appended
    reportCount: 1
    runId: stage-2-32-run-001

Expected second append result:

    status: appended
    reportCount: 2
    runId: stage-2-32-run-002

After serialization and reload, expected run id order:

    stage-2-32-run-001
    stage-2-32-run-002

After serialization and reload, expected balance delta order:

    10101
    20202

Expected validation result:

    { ok: true }

## Duplicate run id test

The Stage 2.32 duplicate test creates two report artifacts with the same run id:

    stage-2-32-duplicate-run

First minted amount:

    30303

Duplicate minted amount:

    40404

The first append succeeds.

The duplicate append returns:

    status: duplicate_run_id
    reportCount: 1
    runId: stage-2-32-duplicate-run

The audit log remains length 1.

The preserved report balanceDelta remains:

    30303

This confirms duplicate run ids do not overwrite or append a second report.

## Malformed persisted log test

The Stage 2.32 validation test confirms a newly created empty audit log is valid.

Confirmed malformed persisted logs rejected:

- invalid JSON
- wrong artifactType
- wrong schemaVersion
- reports is not an array
- invalid embedded report artifact

Confirmed invalid embedded report artifact validation result:

    ok: false
    reason: invalid_report_artifact
    index: 0

## Stage 2.32 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_operator_report_audit_log_append_model.test.ts

Result:

    Stage 2.32 operator report audit log append model
      ✔ appends serialized operator reports to an audit log and preserves order after reload
      ✔ rejects duplicate run ids without appending a second report
      ✔ validates audit log artifacts and rejects malformed persisted logs

    3 passing

## Regression checks

Stage 2.31 operator report serialization / stable log artifact remained green:

    Stage 2.31 operator report serialization / stable log artifact
      ✔ serializes and deserializes an operator report as a stable JSON artifact
      ✔ keeps the serialized report free of secret-bearing fields
      ✔ validates report shape and rejects malformed artifacts

    3 passing

Stage 2.30 relayer operator report / run summary remained green:

    Stage 2.30 relayer operator report / run summary
      ✔ reports a mixed resume execution run with journal and balance deltas
      ✔ reports retry/manual-review queued runs with zero balance delta and no import result
      ✔ rejects empty operator run ids

    3 passing

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

Stage 2.32 creates an append-only audit log model above Stage 2.31 operator report artifacts.

The relayer can now create a schema-versioned operator audit log, append stable report artifacts, preserve report order across serialization and reload, reject duplicate run ids without mutation, and reject malformed persisted logs.

The on-chain runtime remains unchanged.

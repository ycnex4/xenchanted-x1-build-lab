# Stage 2.31 Operator Report Serialization / Stable Log Artifact Evidence

This document records Stage 2.31 operator report serialization / stable log artifact evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-31-operator-report-serialization-log-artifact

Runtime commit:

    b711f62 Add Stage 2.31 operator report serialization log artifact

Base runtime commit:

    f3ea200 Add Stage 2.30 relayer operator report run summary

## Scope

Stage 2.31 adds stable serialization and validation for the Stage 2.30 operator report.

It turns the in-memory operator report into a deterministic JSON log artifact suitable for files, logs, audit trails, and later validation.

It builds on:

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

    tests/stage2_operator_report_serialization_log_artifact.test.ts

## New log artifact model

New type:

    Stage2RelayerOperatorRunReportLogArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_run_report"
    schemaVersion: 1
    report: Stage2RelayerOperatorRunReport

## New validation model

New validation reason type:

    Stage2RelayerOperatorRunReportValidationReason

Validation reasons:

- invalid_report
- invalid_run_id
- invalid_started_at_iso
- invalid_completed_at_iso
- invalid_contracts_received
- invalid_journal_records_before
- invalid_journal_records_after
- invalid_contract_records_before
- invalid_contract_records_after
- invalid_balance_before
- invalid_balance_after
- invalid_balance_delta
- invalid_execution_summary
- invalid_decisions

New validation result type:

    Stage2RelayerOperatorRunReportValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason }

## New helpers

New validation helper:

    validateStage2RelayerOperatorRunReportPrototype

New serialization helper:

    serializeStage2RelayerOperatorRunReportPrototype

New deserialization helper:

    deserializeStage2RelayerOperatorRunReportPrototype

## Serialization behavior

Serialization validates the report before writing the artifact.

The artifact is serialized as deterministic pretty JSON:

    JSON.stringify(artifact, null, 2)

The serialized artifact contains:

    artifactType: stage2_relayer_operator_run_report
    schemaVersion: 1
    report

Serializing the same report twice produces the same string.

## Deserialization behavior

Deserialization rejects malformed JSON.

Confirmed error:

    invalid operator report artifact: invalid_json

Deserialization rejects non-object artifacts.

Confirmed error:

    invalid operator report artifact: invalid_artifact

Deserialization rejects wrong artifact type.

Confirmed error:

    invalid operator report artifact: invalid_artifact_type

Deserialization rejects unsupported schema version.

Confirmed error:

    invalid operator report artifact: invalid_schema_version

Deserialization validates the embedded report and rejects malformed report fields.

## JSON roundtrip behavior

The test explicitly accounts for JSON behavior where fields with value undefined are omitted during:

    JSON.stringify / JSON.parse

Therefore, the deserialized report is compared against a JSON-roundtrippable version of the original in-memory report.

This confirms the persisted artifact shape rather than the transient in-memory object shape.

## Stable artifact test

The Stage 2.31 test creates a live operator report through the Stage 2.30 helper.

Run id:

    stage-2-31-run-001

Minted amount:

    12345

The report is serialized twice.

Confirmed:

- serializedA equals serializedB
- artifactType is present
- schemaVersion is present
- artifactType equals stage2_relayer_operator_run_report
- schemaVersion equals 1
- parsed report balanceDelta equals 12345
- parsed report executionSummary.submitted equals 1
- parsed report decision status equals submitted

## Secret-safety test

The Stage 2.31 test confirms serialized operator reports do not contain secret-bearing fields.

Checked absent strings:

- secretKey
- guardianSigners
- privateKey
- ANCHOR_WALLET
- wallet.json

This keeps operator report artifacts suitable for logs and audit files without leaking local wallet paths, private key material, or guardian signer internals.

## Validation test

The Stage 2.31 validation test confirms a minimal valid report passes validation.

The test confirms malformed artifacts are rejected:

- invalid JSON
- wrong artifactType
- wrong schemaVersion
- empty runId
- invalid balanceDelta

Confirmed validation result for valid report:

    { ok: true }

Confirmed empty runId error:

    invalid operator report: invalid_run_id

Confirmed invalid balanceDelta error:

    invalid operator report: invalid_balance_delta

## Stage 2.31 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_operator_report_serialization_log_artifact.test.ts

Result after JSON-roundtrip assertion fix:

    Stage 2.31 operator report serialization / stable log artifact
      ✔ serializes and deserializes an operator report as a stable JSON artifact
      ✔ keeps the serialized report free of secret-bearing fields
      ✔ validates report shape and rejects malformed artifacts

    3 passing

## Regression checks

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

Stage 2.31 creates a stable JSON log artifact layer for Stage 2.30 operator reports.

Operator reports can now be validated, serialized, deserialized, schema-versioned, and checked for secret-bearing field leakage.

The artifact has deterministic JSON output for the same report, rejects malformed input, rejects unsupported schema versions, and preserves only JSON-compatible persisted fields.

The on-chain runtime remains unchanged.

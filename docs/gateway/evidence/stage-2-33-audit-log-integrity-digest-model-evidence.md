# Stage 2.33 Audit Log Integrity Digest / Tamper-Evidence Model Evidence

This document records Stage 2.33 audit log integrity digest / tamper-evidence model evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-33-audit-log-integrity-digest-model

Runtime commit:

    0878885 Add Stage 2.33 audit log integrity digest model

Base runtime commit:

    9daa74f Add Stage 2.32 operator report audit log append model

## Scope

Stage 2.33 adds an integrity digest layer above Stage 2.32 operator audit logs.

It turns a validated operator audit log artifact into a compact tamper-evidence digest artifact.

It builds on:

- Stage 2.32 operator report audit log append model
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

    tests/stage2_operator_audit_log_integrity_digest_model.test.ts

## New digest artifact model

New type:

    Stage2RelayerOperatorAuditLogDigestArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_audit_log_digest"
    schemaVersion: 1
    algorithm: "sha256"
    digestHex: string
    reportCount: number
    firstRunId?: string
    lastRunId?: string

## New digest validation model

New validation reason type:

    Stage2RelayerOperatorAuditLogDigestValidationReason

Validation reasons:

- invalid_digest_artifact
- invalid_artifact_type
- invalid_schema_version
- invalid_algorithm
- invalid_digest_hex
- invalid_report_count
- invalid_first_run_id
- invalid_last_run_id

New validation result type:

    Stage2RelayerOperatorAuditLogDigestValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason }

## New helpers

New digest computation helper:

    computeStage2RelayerOperatorAuditLogDigestPrototype

New digest validation helper:

    validateStage2RelayerOperatorAuditLogDigestPrototype

New digest serialization helper:

    serializeStage2RelayerOperatorAuditLogDigestPrototype

New digest deserialization helper:

    deserializeStage2RelayerOperatorAuditLogDigestPrototype

New digest verification helper:

    verifyStage2RelayerOperatorAuditLogDigestPrototype

## Digest computation behavior

Digest computation validates the audit log before digesting.

Invalid audit logs cannot be digested as valid.

The digest is computed from the deterministic serialized audit log:

    serializeStage2RelayerOperatorAuditLogPrototype(auditLog)

Digest algorithm:

    sha256

Digest encoding:

    lowercase hex

Expected digest length:

    64 hex characters

## Digest metadata behavior

The digest artifact records:

- reportCount
- firstRunId
- lastRunId

For an empty audit log:

- reportCount: 0
- firstRunId: undefined
- lastRunId: undefined

For a non-empty audit log:

- firstRunId equals the first report run id
- lastRunId equals the last report run id

## Stable digest test

The Stage 2.33 test creates two live Stage 2.31 report artifacts and appends them into a Stage 2.32 audit log.

First run:

    runId: stage-2-33-run-001
    mintedAmount: 11111

Second run:

    runId: stage-2-33-run-002
    mintedAmount: 22222

Confirmed digest behavior:

- computing digest twice for the same audit log returns the same artifact
- artifactType equals stage2_relayer_operator_audit_log_digest
- schemaVersion equals 1
- algorithm equals sha256
- digestHex matches 64 lowercase hex characters
- reportCount equals 2
- firstRunId equals stage-2-33-run-001
- lastRunId equals stage-2-33-run-002
- digest artifact serializes and deserializes
- digest validation returns ok: true
- digest verification returns true for matching audit log and digest

## Tamper-evidence test

The Stage 2.33 test creates:

- an ordered audit log
- a reversed-order audit log
- a changed-content audit log

Ordered run ids:

    stage-2-33-order-001
    stage-2-33-order-002

Reversed run ids:

    stage-2-33-order-002
    stage-2-33-order-001

Changed content case:

    second report balanceDelta changed from 44444 to 55555

Confirmed tamper-evidence behavior:

- reversed order changes digestHex
- changed report content changes digestHex
- verifying ordered digest against reversed log returns false
- verifying ordered digest against changed-content log returns false

## Malformed digest artifact test

The Stage 2.33 validation test rejects malformed digest artifacts.

Confirmed rejections:

- invalid JSON
- wrong artifactType
- wrong schemaVersion
- wrong algorithm
- invalid digestHex

Confirmed errors:

    invalid operator audit log digest artifact: invalid_json
    invalid operator audit log digest artifact: invalid_artifact_type
    invalid operator audit log digest artifact: invalid_schema_version
    invalid operator audit log digest artifact: invalid_algorithm
    invalid operator audit log digest artifact: invalid_digest_hex

## Invalid audit log digest input test

The Stage 2.33 test confirms invalid audit logs cannot be digested as valid.

Duplicate run id audit log input is rejected before digesting.

Confirmed error:

    invalid operator audit log digest input: duplicate_run_id

## Stage 2.33 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_operator_audit_log_integrity_digest_model.test.ts

Result:

    Stage 2.33 operator audit log integrity digest model
      ✔ computes a stable sha256 digest for the same audit log
      ✔ changes digest when audit log order or report content changes
      ✔ rejects malformed digest artifacts and invalid audit logs before digesting

    3 passing

## Regression checks

Stage 2.32 operator report audit log append model remained green:

    Stage 2.32 operator report audit log append model
      ✔ appends serialized operator reports to an audit log and preserves order after reload
      ✔ rejects duplicate run ids without appending a second report
      ✔ validates audit log artifacts and rejects malformed persisted logs

    3 passing

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

Stage 2.33 creates a tamper-evidence digest layer above Stage 2.32 operator audit logs.

The relayer can now compute a schema-versioned sha256 digest artifact for a validated audit log, verify that a digest matches a log, detect order/content changes, reject malformed digest artifacts, and reject invalid audit logs before digesting.

The on-chain runtime remains unchanged.

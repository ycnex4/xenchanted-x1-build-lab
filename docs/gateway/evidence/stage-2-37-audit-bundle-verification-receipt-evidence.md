# Stage 2.37 Audit Bundle Verification Receipt Evidence

This document records Stage 2.37 audit bundle verification receipt boundary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-37-audit-bundle-verification-receipt-boundary

Runtime commit:

    fac1a1d Add Stage 2.37 audit bundle verification receipt boundary

Base runtime commit:

    b4a5b00 Add Stage 2.36 audit bundle verifier boundary

## Scope

Stage 2.37 adds a stable verification receipt artifact above the Stage 2.36 external bundle verifier.

Stage 2.35 creates a portable export bundle.

Stage 2.36 verifies a serialized export bundle as an external consumer and returns a compact verification result.

Stage 2.37 converts a successful verification result into a durable receipt artifact.

It does not verify the bundle directly.

It does not create a bundle.

It does not mutate an audit log.

It does not change the on-chain runtime.

It does not introduce new live X1 behavior.

It is an offline / zero-SOL receipt boundary.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts

## New receipt artifact model

New artifact type:

    Stage2RelayerOperatorAuditBundleVerificationReceiptArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_audit_bundle_verification_receipt"
    schemaVersion: 1
    createdAtIso: string
    verifierId: string
    verificationResultArtifactType: "stage2_relayer_operator_audit_bundle_verification_result"
    verificationResultSchemaVersion: 1
    bundleArtifactType: "stage2_relayer_operator_audit_export_bundle"
    stageRange: "2.31-2.35"
    runtimeCommit: string
    digestHex: string
    reportCount: number
    firstRunId?: string
    lastRunId?: string
    checkpointCreatedAtIso: string
    bundleCreatedAtIso: string
    verifiedAtIso: string

## New receipt validation model

New validation reason type:

    Stage2RelayerOperatorAuditBundleVerificationReceiptValidationReason

Validation reasons:

- invalid_receipt_artifact
- invalid_artifact_type
- invalid_schema_version
- invalid_created_at_iso
- invalid_verifier_id
- invalid_verification_result_artifact_type
- invalid_verification_result_schema_version
- invalid_bundle_artifact_type
- invalid_stage_range
- invalid_runtime_commit
- invalid_digest_hex
- invalid_report_count
- invalid_first_run_id
- invalid_last_run_id
- invalid_checkpoint_created_at_iso
- invalid_bundle_created_at_iso
- invalid_verified_at_iso

New validation result type:

    Stage2RelayerOperatorAuditBundleVerificationReceiptValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason }

## New receipt helpers

New receipt creation helper:

    createStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

New receipt validation helper:

    validateStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

New receipt serialization helper:

    serializeStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

New receipt deserialization helper:

    deserializeStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

New receipt verification helper:

    verifyStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

## Receipt creation behavior

Receipt creation requires a successful Stage 2.36 verification result.

Receipt creation rejects failed verification results.

Receipt creation validates createdAtIso.

Receipt creation validates non-empty verifierId.

Receipt creation preserves the important verified bundle summary:

- runtimeCommit
- digestHex
- reportCount
- firstRunId
- lastRunId
- checkpointCreatedAtIso
- bundleCreatedAtIso
- verifiedAtIso
- stageRange
- bundleArtifactType

## Successful receipt test

The Stage 2.37 test creates a Stage 2.35 export bundle, verifies it through Stage 2.36, and then creates a Stage 2.37 receipt from the successful verification result.

Input bundle data:

    runId: stage-2-37-run-001
    balanceDelta: 11111

    runId: stage-2-37-run-002
    balanceDelta: 22222

Checkpoint timestamp:

    2026-01-01T00:37:00.000Z

Bundle timestamp:

    2026-01-01T00:38:00.000Z

Verifier timestamp:

    2026-01-01T00:39:00.000Z

Receipt timestamp:

    2026-01-01T00:40:00.000Z

Verifier id:

    stage-2-37-verifier-001

Runtime commit metadata:

    b4a5b00

Confirmed receipt behavior:

- artifactType equals stage2_relayer_operator_audit_bundle_verification_receipt
- schemaVersion equals 1
- createdAtIso equals 2026-01-01T00:40:00.000Z
- verifierId equals stage-2-37-verifier-001
- verificationResultArtifactType equals stage2_relayer_operator_audit_bundle_verification_result
- verificationResultSchemaVersion equals 1
- bundleArtifactType equals stage2_relayer_operator_audit_export_bundle
- stageRange equals 2.31-2.35
- runtimeCommit equals b4a5b00
- digestHex is a 64-character lowercase hex digest
- reportCount equals 2
- firstRunId equals stage-2-37-run-001
- lastRunId equals stage-2-37-run-002
- checkpointCreatedAtIso equals 2026-01-01T00:37:00.000Z
- bundleCreatedAtIso equals 2026-01-01T00:38:00.000Z
- verifiedAtIso equals 2026-01-01T00:39:00.000Z
- receipt validation returns ok: true
- receipt verification returns true
- receipt serializes and deserializes
- deserialized receipt verifies as true

## Failed result and metadata rejection test

Confirmed rejection behavior:

- failed verification result is rejected
- invalid receipt createdAtIso is rejected
- empty verifierId is rejected

Confirmed errors:

    invalid operator audit bundle verification receipt result: digest_mismatch
    invalid operator audit bundle verification receipt: invalid_created_at_iso
    invalid operator audit bundle verification receipt: invalid_verifier_id

## Malformed receipt validation and deserialization test

Confirmed validation rejections:

- wrong artifactType returns invalid_artifact_type
- wrong schemaVersion returns invalid_schema_version
- invalid digestHex returns invalid_digest_hex
- negative reportCount returns invalid_report_count
- wrong verificationResultArtifactType returns invalid_verification_result_artifact_type
- invalid verifiedAtIso returns invalid_verified_at_iso

Confirmed deserialization errors:

    invalid operator audit bundle verification receipt: invalid_json
    invalid operator audit bundle verification receipt: invalid_bundle_artifact_type

## Stage 2.37 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts

Result:

    Stage 2.37 operator audit bundle verification receipt boundary
      ✔ creates a stable verification receipt from a successful bundle verification result
      ✔ rejects failed verification results and malformed receipt metadata
      ✔ rejects malformed receipt artifacts during validation and deserialization

    3 passing (10ms)

## Stage 2.31 through Stage 2.37 artifact / verifier / receipt chain check

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_report_serialization_log_artifact.test.ts \
      tests/stage2_operator_report_audit_log_append_model.test.ts \
      tests/stage2_operator_audit_log_integrity_digest_model.test.ts \
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts \
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts \
      tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts

Result:

    21 passing (37ms)

## Stage 2.22 through Stage 2.37 optimized regression

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx mocha -r ts-node/register \
      tests/stage2_watcher_event_operational_submit_wrapper.test.ts \
      tests/stage2_watcher_event_batch_queue_processing.test.ts \
      tests/stage2_durable_relayer_journal_model.test.ts \
      tests/stage2_watcher_to_relayer_contract_boundary.test.ts \
      tests/stage2_relayer_dedupe_journal_replay_guard.test.ts \
      tests/stage2_relayer_import_pipeline.test.ts \
      tests/stage2_import_pipeline_durable_resume_plan.test.ts \
      tests/stage2_resume_plan_execution_model.test.ts \
      tests/stage2_relayer_operator_report_run_summary.test.ts \
      tests/stage2_operator_report_serialization_log_artifact.test.ts \
      tests/stage2_operator_report_audit_log_append_model.test.ts \
      tests/stage2_operator_audit_log_integrity_digest_model.test.ts \
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts \
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts \
      tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts

Result:

    51 passing (18s)

## Checks passed

Runtime checks passed:

- Stage 2.37 audit bundle verification receipt boundary: 3 passing
- Stage 2.31 through Stage 2.37 artifact / verifier / receipt chain: 21 passing
- Stage 2.22 through Stage 2.37 full optimized regression: 51 passing (18s)

Build-lab checks to run after this evidence document:

- npm run typecheck
- npm test
- npm run build

## Current conclusion

Stage 2.37 adds a durable verification receipt artifact above the Stage 2.36 external bundle verifier. The operator can export a portable evidence bundle, an external verifier can verify that bundle and return a compact verification result, and then the successful result can be converted into a stable receipt artifact.

This completes the transition from operator-side evidence production into consumer-side evidence verification and receipt creation while preserving the offline / zero-SOL artifact boundary.

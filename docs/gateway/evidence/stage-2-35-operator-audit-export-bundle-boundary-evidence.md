# Stage 2.35 Operator Audit Export Bundle Boundary Evidence

This document records Stage 2.35 operator audit export bundle boundary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-35-operator-audit-export-bundle-boundary

Runtime commit:

    dc47baf Add Stage 2.35 operator audit export bundle boundary

Base runtime commit:

    702573b Add Stage 2.34 audit log checkpoint boundary

## Scope

Stage 2.35 adds an operator audit export bundle boundary above the Stage 2.31 through Stage 2.34 artifact chain.

It packages the full operator audit evidence chain into one schema-versioned export artifact:

- Stage 2.31 stable operator report artifacts
- Stage 2.32 append-only operator audit log
- Stage 2.33 audit log digest / tamper-evidence artifact
- Stage 2.34 audit log checkpoint summary artifact
- Stage 2.35 export bundle metadata

It does not change the on-chain runtime.

It does not introduce new live X1 behavior.

It is an offline / zero-SOL artifact boundary.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_operator_audit_export_bundle_boundary.test.ts

## New export bundle artifact model

New type:

    Stage2RelayerOperatorAuditExportBundleArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_audit_export_bundle"
    schemaVersion: 1
    createdAtIso: string
    stageRange: "2.31-2.35"
    runtimeCommit: string
    sourceArtifactType: "stage2_relayer_operator_audit_log"
    digestArtifactType: "stage2_relayer_operator_audit_log_digest"
    checkpointArtifactType: "stage2_relayer_operator_audit_log_checkpoint"
    auditLog: Stage2RelayerOperatorAuditLogArtifact
    digest: Stage2RelayerOperatorAuditLogDigestArtifact
    checkpoint: Stage2RelayerOperatorAuditLogCheckpointArtifact

## New export bundle validation model

New validation reason type:

    Stage2RelayerOperatorAuditExportBundleValidationReason

Validation reasons:

- invalid_export_bundle_artifact
- invalid_artifact_type
- invalid_schema_version
- invalid_created_at_iso
- invalid_stage_range
- invalid_runtime_commit
- invalid_source_artifact_type
- invalid_digest_artifact_type
- invalid_checkpoint_artifact_type
- invalid_audit_log
- invalid_digest
- invalid_checkpoint
- digest_mismatch
- checkpoint_mismatch

New validation result type:

    Stage2RelayerOperatorAuditExportBundleValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason }

## New helpers

New export bundle creation helper:

    createStage2RelayerOperatorAuditExportBundlePrototype

New export bundle validation helper:

    validateStage2RelayerOperatorAuditExportBundlePrototype

New export bundle serialization helper:

    serializeStage2RelayerOperatorAuditExportBundlePrototype

New export bundle deserialization helper:

    deserializeStage2RelayerOperatorAuditExportBundlePrototype

New export bundle verification helper:

    verifyStage2RelayerOperatorAuditExportBundlePrototype

## Export bundle creation behavior

Export bundle creation validates the source audit log before creating a bundle.

Export bundle creation validates the digest artifact before creating a bundle.

Export bundle creation validates the checkpoint artifact before creating a bundle.

Export bundle creation verifies that the digest matches the source audit log.

Export bundle creation verifies that the checkpoint matches the source audit log and digest.

Export bundle creation validates createdAtIso as an exact ISO timestamp.

Export bundle creation rejects an empty runtimeCommit.

The created bundle binds:

- audit log artifact
- digest artifact
- checkpoint artifact
- stageRange
- runtimeCommit
- createdAtIso
- sourceArtifactType
- digestArtifactType
- checkpointArtifactType

## Export bundle verification behavior

Export bundle verification validates:

- export bundle artifact shape
- createdAtIso
- stageRange
- runtimeCommit
- sourceArtifactType
- digestArtifactType
- checkpointArtifactType
- nested audit log artifact
- nested digest artifact
- nested checkpoint artifact
- digest against audit log
- checkpoint against audit log and digest

Verification returns false when nested artifacts or metadata no longer match.

## Stable export bundle test

The Stage 2.35 test creates an audit log with two stable operator report artifacts, computes a digest, creates a checkpoint, and then creates an export bundle.

First run:

    runId: stage-2-35-run-001
    balanceDelta: 11111

Second run:

    runId: stage-2-35-run-002
    balanceDelta: 22222

Checkpoint timestamp:

    2026-01-01T00:35:00.000Z

Bundle timestamp:

    2026-01-01T00:36:00.000Z

Runtime commit metadata:

    702573b

Confirmed export bundle behavior:

- artifactType equals stage2_relayer_operator_audit_export_bundle
- schemaVersion equals 1
- createdAtIso equals 2026-01-01T00:36:00.000Z
- stageRange equals 2.31-2.35
- runtimeCommit equals 702573b
- sourceArtifactType equals stage2_relayer_operator_audit_log
- digestArtifactType equals stage2_relayer_operator_audit_log_digest
- checkpointArtifactType equals stage2_relayer_operator_audit_log_checkpoint
- nested auditLog equals the source audit log
- nested digest equals the computed digest
- nested checkpoint equals the created checkpoint
- export bundle validation returns ok: true
- export bundle verification returns true
- export bundle serializes and deserializes
- deserialized export bundle verifies as true

## Mismatch / tamper-evidence boundary test

The Stage 2.35 mismatch test confirms export bundle validation rejects mismatched nested artifacts or metadata.

Confirmed mismatch behavior:

- changed audit log returns digest_mismatch
- changed digest digestHex returns digest_mismatch
- changed checkpoint digestHex returns checkpoint_mismatch
- changed stageRange returns invalid_stage_range
- empty runtimeCommit returns invalid_runtime_commit
- wrong sourceArtifactType returns invalid_source_artifact_type
- wrong digestArtifactType returns invalid_digest_artifact_type
- wrong checkpointArtifactType returns invalid_checkpoint_artifact_type
- invalid createdAtIso returns invalid_created_at_iso
- changed checkpoint reportCount makes bundle verification return false

## Malformed export bundle artifact test

The Stage 2.35 validation test rejects malformed export bundle artifacts.

Confirmed rejections:

- invalid JSON
- wrong artifactType
- wrong schemaVersion
- invalid nested auditLog
- invalid nested digest
- invalid nested checkpoint
- digest mismatch during creation
- checkpoint mismatch during creation
- invalid createdAtIso during creation
- empty runtimeCommit during creation

Confirmed errors:

    invalid operator audit export bundle artifact: invalid_json
    invalid operator audit export bundle artifact: invalid_artifact_type
    invalid operator audit export bundle artifact: invalid_schema_version
    invalid operator audit export bundle artifact: invalid_audit_log
    invalid operator audit export bundle artifact: invalid_digest
    invalid operator audit export bundle artifact: invalid_checkpoint
    invalid operator audit export bundle digest: digest_mismatch
    invalid operator audit export bundle checkpoint: checkpoint_mismatch
    invalid operator audit export bundle: invalid_created_at_iso
    invalid operator audit export bundle: invalid_runtime_commit

## Stage 2.35 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts

Result:

    Stage 2.35 operator audit export bundle boundary
      ✔ creates a stable export bundle from audit log, digest, checkpoint, and metadata
      ✔ rejects export bundle verification when nested artifacts or metadata change
      ✔ rejects malformed export bundle artifacts and mismatched bundle inputs

    3 passing (11ms)

## Stage 2.31 through Stage 2.35 artifact chain check

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_report_serialization_log_artifact.test.ts \
      tests/stage2_operator_report_audit_log_append_model.test.ts \
      tests/stage2_operator_audit_log_integrity_digest_model.test.ts \
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts

Result:

    15 passing (23ms)

## Stage 2.22 through Stage 2.35 optimized regression

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
      tests/stage2_operator_audit_export_bundle_boundary.test.ts

Result:

    45 passing (14s)

## Checks passed

Runtime checks passed:

- Stage 2.35 operator audit export bundle boundary: 3 passing
- Stage 2.31 through Stage 2.35 artifact chain: 15 passing
- Stage 2.22 through Stage 2.35 full optimized regression: 45 passing (14s)

Build-lab checks to run after this evidence document:

- npm run typecheck
- npm test
- npm run build

## Current conclusion

Stage 2.35 creates an operator audit export bundle boundary above the Stage 2.31 through Stage 2.34 artifact chain. The relayer can now package a validated audit log, matching digest, matching checkpoint, and bundle metadata into one schema-versioned export artifact.

The bundle can be serialized, deserialized, validated, and verified offline. It rejects malformed nested artifacts, digest mismatches, checkpoint mismatches, and invalid metadata. This gives the operator a complete portable evidence package while preserving the existing split between cheap deterministic artifact tests and minimal live X1 runtime smoke coverage.

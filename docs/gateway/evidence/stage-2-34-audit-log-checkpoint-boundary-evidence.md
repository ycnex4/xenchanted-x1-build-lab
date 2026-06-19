# Stage 2.34 Audit Log Checkpoint Summary Boundary Evidence

This document records Stage 2.34 audit log checkpoint / summary boundary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-34-audit-log-checkpoint-summary-boundary

Runtime commit:

    702573b Add Stage 2.34 audit log checkpoint boundary

Base runtime commit:

    0878885 Add Stage 2.33 audit log integrity digest model

## Scope

Stage 2.34 adds a compact checkpoint summary boundary above Stage 2.33 audit log digest artifacts.

It turns a validated Stage 2.32 audit log and a matching Stage 2.33 digest artifact into a compact checkpoint artifact that can be stored, copied, compared, or published without carrying the full audit log.

It builds on:

- Stage 2.33 audit log integrity digest / tamper-evidence model
- Stage 2.32 operator report audit log append model
- Stage 2.31 operator report serialization / stable log artifact
- Stage 2.30 relayer operator report / run summary
- Stage 2.29 resume plan execution model
- Stage 2.28 import pipeline durable resume plan
- Stage 2.27 relayer import pipeline
- Stage 2.26 relayer dedupe / journal replay guard
- Stage 2.25 watcher-to-relayer contract boundary

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts

Runtime test-cost optimization also updated:

    tests/stage2_watcher_to_relayer_contract_boundary.test.ts
    tests/stage2_relayer_dedupe_journal_replay_guard.test.ts
    tests/stage2_import_pipeline_durable_resume_plan.test.ts
    tests/stage2_resume_plan_execution_model.test.ts
    tests/stage2_relayer_operator_report_run_summary.test.ts
    tests/stage2_operator_report_serialization_log_artifact.test.ts
    tests/stage2_operator_report_audit_log_append_model.test.ts
    tests/stage2_operator_audit_log_integrity_digest_model.test.ts

Timeout-only runtime test updates:

    tests/stage2_watcher_event_operational_submit_wrapper.test.ts
    tests/stage2_watcher_event_batch_queue_processing.test.ts
    tests/stage2_durable_relayer_journal_model.test.ts
    tests/stage2_relayer_import_pipeline.test.ts

## New checkpoint artifact model

New type:

    Stage2RelayerOperatorAuditLogCheckpointArtifact

Artifact fields:

    artifactType: "stage2_relayer_operator_audit_log_checkpoint"
    schemaVersion: 1
    digestAlgorithm: "sha256"
    digestHex: string
    reportCount: number
    firstRunId?: string
    lastRunId?: string
    createdAtIso: string
    sourceArtifactType: "stage2_relayer_operator_audit_log"
    digestArtifactType: "stage2_relayer_operator_audit_log_digest"

## New checkpoint validation model

New validation reason type:

    Stage2RelayerOperatorAuditLogCheckpointValidationReason

Validation reasons:

- invalid_checkpoint_artifact
- invalid_artifact_type
- invalid_schema_version
- invalid_digest_algorithm
- invalid_digest_hex
- invalid_report_count
- invalid_first_run_id
- invalid_last_run_id
- invalid_created_at_iso
- invalid_source_artifact_type
- invalid_digest_artifact_type

New validation result type:

    Stage2RelayerOperatorAuditLogCheckpointValidationResult

Result shapes:

    { ok: true }

or:

    { ok: false, reason }

## New helpers

New checkpoint creation helper:

    createStage2RelayerOperatorAuditLogCheckpointPrototype

New checkpoint validation helper:

    validateStage2RelayerOperatorAuditLogCheckpointPrototype

New checkpoint serialization helper:

    serializeStage2RelayerOperatorAuditLogCheckpointPrototype

New checkpoint deserialization helper:

    deserializeStage2RelayerOperatorAuditLogCheckpointPrototype

New checkpoint verification helper:

    verifyStage2RelayerOperatorAuditLogCheckpointPrototype

Supporting ISO timestamp helper:

    isStage2IsoTimestamp

## Checkpoint creation behavior

Checkpoint creation validates the source audit log before creating a checkpoint.

Checkpoint creation validates the digest artifact before creating a checkpoint.

Checkpoint creation verifies that the digest matches the audit log before creating a checkpoint.

Checkpoint creation validates createdAtIso as an exact ISO timestamp.

Checkpoint creation binds the checkpoint to:

- digestAlgorithm
- digestHex
- reportCount
- firstRunId
- lastRunId
- createdAtIso
- sourceArtifactType
- digestArtifactType

## Checkpoint verification behavior

Checkpoint verification validates:

- checkpoint artifact shape
- digest artifact shape
- source audit log shape
- digest matches source audit log
- checkpoint digestAlgorithm matches digest algorithm
- checkpoint digestHex matches digest digestHex
- checkpoint reportCount matches digest reportCount
- checkpoint firstRunId matches digest firstRunId
- checkpoint lastRunId matches digest lastRunId
- checkpoint sourceArtifactType matches stage2_relayer_operator_audit_log
- checkpoint digestArtifactType matches stage2_relayer_operator_audit_log_digest

Verification returns false when the audit log, digest artifact, or checkpoint fields no longer match.

## Stable checkpoint test

The Stage 2.34 test creates two stable operator report artifacts, appends them into an audit log, computes a digest, and creates a checkpoint.

First run:

    runId: stage-2-34-run-001

Second run:

    runId: stage-2-34-run-002

Checkpoint timestamp:

    2026-01-01T00:10:00.000Z

Confirmed checkpoint behavior:

- artifactType equals stage2_relayer_operator_audit_log_checkpoint
- schemaVersion equals 1
- digestAlgorithm equals sha256
- digestHex equals digest.digestHex
- reportCount equals 2
- firstRunId equals stage-2-34-run-001
- lastRunId equals stage-2-34-run-002
- createdAtIso equals 2026-01-01T00:10:00.000Z
- sourceArtifactType equals stage2_relayer_operator_audit_log
- digestArtifactType equals stage2_relayer_operator_audit_log_digest
- checkpoint validation returns ok: true
- checkpoint serializes and deserializes
- checkpoint verification returns true for matching audit log, digest, and checkpoint

## Mismatch / tamper-evidence boundary test

The Stage 2.34 mismatch test confirms checkpoint verification rejects mismatched inputs.

Confirmed mismatch behavior:

- changed audit log makes checkpoint verification return false
- changed digest artifact makes checkpoint verification return false
- changed checkpoint digestHex makes checkpoint verification return false
- changed checkpoint reportCount makes checkpoint verification return false
- changed checkpoint firstRunId makes checkpoint verification return false
- changed checkpoint lastRunId makes checkpoint verification return false
- changed checkpoint sourceArtifactType makes checkpoint verification return false
- changed checkpoint digestArtifactType makes checkpoint verification return false

## Malformed checkpoint artifact test

The Stage 2.34 validation test rejects malformed checkpoint artifacts.

Confirmed rejections:

- invalid checkpoint artifact
- wrong artifactType
- wrong schemaVersion
- wrong digestAlgorithm
- invalid digestHex
- invalid reportCount
- invalid firstRunId
- invalid lastRunId
- invalid createdAtIso
- wrong sourceArtifactType
- wrong digestArtifactType

Confirmed errors:

    invalid operator audit log checkpoint artifact: invalid_json
    invalid operator audit log checkpoint artifact: invalid_artifact_type
    invalid operator audit log checkpoint artifact: invalid_schema_version
    invalid operator audit log checkpoint artifact: invalid_digest_algorithm
    invalid operator audit log checkpoint artifact: invalid_digest_hex
    invalid operator audit log checkpoint artifact: invalid_report_count
    invalid operator audit log checkpoint artifact: invalid_created_at_iso
    invalid operator audit log checkpoint artifact: invalid_source_artifact_type
    invalid operator audit log checkpoint artifact: invalid_digest_artifact_type

## Invalid checkpoint input tests

The Stage 2.34 test confirms invalid inputs cannot create valid checkpoints.

Confirmed invalid input behavior:

- invalid source audit log is rejected before checkpoint creation
- invalid digest artifact is rejected before checkpoint creation
- digest mismatch is rejected before checkpoint creation
- invalid createdAtIso is rejected before checkpoint creation

Confirmed errors:

    invalid operator audit log checkpoint input: duplicate_run_id
    invalid operator audit log checkpoint digest: invalid_digest_hex
    invalid operator audit log checkpoint digest: digest_mismatch
    invalid operator audit log checkpoint: invalid_created_at_iso

## Test-cost optimization

Stage 2.34 runtime commit also reduces testnet cost for the Stage 2.25 through Stage 2.34 relayer prototype test range.

The optimized split is:

Offline / zero-SOL tests:

- Stage 2.25 watcher-to-relayer contract boundary
- Stage 2.26 relayer dedupe journal replay guard
- Stage 2.28 import pipeline durable resume plan
- Stage 2.29 resume plan execution model
- Stage 2.30 retry/manual-review and empty run id checks
- Stage 2.31 operator report serialization / stable log artifact
- Stage 2.32 operator report audit log append model
- Stage 2.33 operator audit log integrity digest model
- Stage 2.34 operator audit log checkpoint summary boundary

Runtime / X1 testnet tests retained:

- Stage 2.22 watcher event operational submit wrapper
- Stage 2.23 watcher event batch / queue processing
- Stage 2.24 durable relayer journal model
- Stage 2.27 relayer import pipeline
- Stage 2.30 first live operator report smoke test

This keeps live X1 coverage where deployed-runtime proof is still valuable while moving upper artifact, planner, and checkpoint layers into deterministic offline tests.

## Stage 2.34 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts

Result:

    Stage 2.34 operator audit log checkpoint summary boundary
      ✔ creates a stable checkpoint from an audit log digest
      ✔ rejects checkpoint verification when audit log, digest, or checkpoint fields change
      ✔ rejects malformed checkpoint artifacts and mismatched checkpoint inputs

    3 passing

## Stage 2.22 through Stage 2.34 optimized regression

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
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts

Result:

    42 passing (12s)

Earlier comparable full regression results before optimization:

    42 passing (41s)
    42 passing (26s)

Optimized result:

    42 passing (12s)

## Checks passed

Runtime checks passed:

- Stage 2.34 operator audit log checkpoint summary boundary: 3 passing
- Stage 2.25 through Stage 2.34 mixed check: 33 passing
- Stage 2.22 through Stage 2.34 full optimized regression: 42 passing

Build-lab checks to run after this evidence document:

- npm run typecheck
- npm test
- npm run build

## Current conclusion

Stage 2.34 creates a compact checkpoint summary boundary above Stage 2.33 audit log digest artifacts. The relayer can now create a schema-versioned checkpoint artifact for a validated audit log and matching digest, serialize and deserialize that checkpoint, validate checkpoint shape, and verify that the checkpoint still matches the source audit log and digest artifact.

The same runtime commit also improves the test architecture: upper artifact, planner, digest, and checkpoint layers now run mostly offline as deterministic model tests, while live X1 testnet coverage remains where it proves deployed-runtime behavior. The full Stage 2.22 through Stage 2.34 regression remains green at 42 passing and now completes in 12 seconds.

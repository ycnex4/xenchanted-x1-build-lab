# Stage 2.36 Audit Bundle Verifier Boundary Evidence

This document records Stage 2.36 audit bundle verifier boundary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-36-operator-audit-bundle-verifier-boundary

Runtime commit:

    b4a5b00 Add Stage 2.36 audit bundle verifier boundary

Base runtime commit:

    dc47baf Add Stage 2.35 operator audit export bundle boundary

## Scope

Stage 2.36 adds a consumer-side verifier boundary for serialized operator audit export bundles.

Stage 2.35 creates a portable export bundle.

Stage 2.36 consumes a serialized export bundle and returns a compact verification result.

It does not create a bundle.

It does not mutate an audit log.

It does not change the on-chain runtime.

It does not introduce new live X1 behavior.

It is an offline / zero-SOL verifier boundary.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_operator_audit_bundle_verifier_boundary.test.ts

Runtime stability fix:

    tests/stage2_relayer_operator_report_run_summary.test.ts

The Stage 2.30 live smoke suite now uses a 30000ms Mocha timeout because live X1 RPC can occasionally exceed Mocha's default 2000ms timeout even when the test behavior is correct.

## New verifier result model

New reason type:

    Stage2RelayerOperatorAuditBundleVerificationReason

Reason values:

- invalid_json
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

New result type:

    Stage2RelayerOperatorAuditBundleVerificationResult

Success result shape:

    {
      ok: true,
      artifactType: "stage2_relayer_operator_audit_bundle_verification_result",
      schemaVersion: 1,
      verifiedAtIso,
      bundleArtifactType,
      stageRange,
      runtimeCommit,
      digestHex,
      reportCount,
      firstRunId,
      lastRunId,
      checkpointCreatedAtIso,
      bundleCreatedAtIso
    }

Failure result shape:

    {
      ok: false,
      artifactType: "stage2_relayer_operator_audit_bundle_verification_result",
      schemaVersion: 1,
      verifiedAtIso,
      reason
    }

## New verifier helper

New helper:

    verifySerializedStage2RelayerOperatorAuditBundlePrototype

Input:

    serializedBundle: string
    verifiedAtIso: string

Behavior:

- validates verifiedAtIso before parsing bundle input
- returns invalid_json for malformed JSON
- validates the parsed bundle through Stage 2.35 export bundle validation
- returns Stage 2.35 validation reason for invalid or tampered bundles
- returns a compact success summary for valid bundles
- does not recreate the export bundle
- does not require live RPC
- does not require wallet access
- does not expose secret-bearing fields

## Successful verification test

The Stage 2.36 test creates a Stage 2.35 export bundle, serializes it, and then verifies it as an external consumer.

Input bundle data:

    runId: stage-2-36-run-001
    balanceDelta: 11111

    runId: stage-2-36-run-002
    balanceDelta: 22222

Checkpoint timestamp:

    2026-01-01T00:36:00.000Z

Bundle timestamp:

    2026-01-01T00:37:00.000Z

Verifier timestamp:

    2026-01-01T00:38:00.000Z

Runtime commit metadata:

    dc47baf

Confirmed success summary:

- ok equals true
- artifactType equals stage2_relayer_operator_audit_bundle_verification_result
- schemaVersion equals 1
- verifiedAtIso equals 2026-01-01T00:38:00.000Z
- bundleArtifactType equals stage2_relayer_operator_audit_export_bundle
- stageRange equals 2.31-2.35
- runtimeCommit equals dc47baf
- digestHex equals the bundle digest
- reportCount equals 2
- firstRunId equals stage-2-36-run-001
- lastRunId equals stage-2-36-run-002
- checkpointCreatedAtIso equals 2026-01-01T00:36:00.000Z
- bundleCreatedAtIso equals 2026-01-01T00:37:00.000Z

## Failure verification test

Confirmed failure behavior:

- malformed JSON returns invalid_json
- wrong bundle artifactType returns invalid_artifact_type
- tampered digest returns digest_mismatch
- tampered checkpoint digest returns checkpoint_mismatch

All failure results preserve:

- artifactType: stage2_relayer_operator_audit_bundle_verification_result
- schemaVersion: 1
- verifiedAtIso
- reason

## Invalid verifier timestamp test

The verifier rejects invalid verifier timestamps before parsing bundle input.

Confirmed error:

    invalid operator audit bundle verification: invalid_verified_at_iso

## Stage 2.36 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts

Result:

    Stage 2.36 operator audit bundle verifier boundary
      ✔ verifies a serialized operator audit export bundle and returns compact summary
      ✔ returns verifier failure results for invalid or tampered serialized bundles
      ✔ rejects invalid verifier timestamps before parsing bundle input

    3 passing (7ms)

## Stage 2.31 through Stage 2.36 artifact / verifier chain check

Command:

    TS_NODE_TRANSPILE_ONLY=1 \
    npx mocha -r ts-node/register \
      tests/stage2_operator_report_serialization_log_artifact.test.ts \
      tests/stage2_operator_report_audit_log_append_model.test.ts \
      tests/stage2_operator_audit_log_integrity_digest_model.test.ts \
      tests/stage2_operator_audit_log_checkpoint_summary_boundary.test.ts \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts \
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts

Result:

    18 passing (21ms)

## Stage 2.22 through Stage 2.36 optimized regression

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
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts

Result:

    48 passing (13s)

## Checks passed

Runtime checks passed:

- Stage 2.36 audit bundle verifier boundary: 3 passing
- Stage 2.31 through Stage 2.36 artifact / verifier chain: 18 passing
- Stage 2.22 through Stage 2.36 full optimized regression: 48 passing (13s)

Build-lab checks to run after this evidence document:

- npm run typecheck
- npm test
- npm run build

## Current conclusion

Stage 2.36 adds a consumer-side verification boundary above the Stage 2.35 export bundle. The operator can now export a portable evidence bundle, and an external verifier can consume the serialized bundle and receive a compact verification result.

This separates evidence production from evidence consumption. It keeps the full artifact chain portable, validates tamper-evidence through digest and checkpoint checks, and preserves the project goal of keeping high-level artifact and verifier tests offline while limiting live X1 RPC usage to minimal runtime smoke coverage.

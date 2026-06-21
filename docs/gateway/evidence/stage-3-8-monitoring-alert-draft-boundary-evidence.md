# Stage 3.8 Monitoring Alert Draft Boundary Evidence

This document records Stage 3.8 monitoring alert draft boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-8-monitoring-alert-draft-boundary

Runtime commit:

    8129896 Add Stage 3.8 monitoring alert draft boundary

Base runtime commit:

    fd62222 Add Stage 3.7 operator workflow config boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 established the audit bundle export boundary.

Stage 3.4 established the audit bundle verifier boundary.

Stage 3.5 established the verification receipt boundary.

Stage 3.6 established the offline operator workflow script boundary.

Stage 3.7 established the operator workflow config/env boundary.

Stage 3.8 adds the offline monitoring alert draft boundary.

## Scope

Stage 3.8 adds a deterministic monitoring alert draft layer.

It connects:

    Stage 3.6 / Stage 3.7 workflow result
    -> workflow summary inspection
    -> deterministic alert draft generation
    -> ok / warning / critical status
    -> source summary

It is not live monitoring.

It is not production alerting.

It does not send notifications.

It does not call RPC.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

It produces a draft monitoring signal only.

## Runtime changes

New helper:

    tests/helpers/stage3MonitoringAlertDraftPrototype.ts

New test:

    tests/stage3_monitoring_alert_draft_boundary.test.ts

## Stage 3.6 / 3.7 dependency

Stage 3.8 depends on the Stage 3.6 operator workflow script result shape.

The Stage 3.8 tests build the workflow result through the Stage 3.7 config/env boundary and Stage 3.6 workflow helper.

Stage 3.8 does not alter workflow execution.

It only inspects the resulting deterministic workflow object and produces an offline alert draft.

## New artifact type

New artifact type:

    stage3_monitoring_alert_draft

Schema version:

    1

Stage:

    3.8

Execution mode:

    offline_zero_sol

## New status model

New status type:

    Stage3MonitoringAlertStatus

Statuses:

    ok
    warning
    critical

Status computation:

- critical if any critical alert exists
- warning if no critical alert exists but at least one warning exists
- ok if no alerts exist

## New alert severity model

New severity type:

    Stage3MonitoringAlertSeverity

Severities:

    warning
    critical

## New alert code model

New alert code type:

    Stage3MonitoringAlertCode

Codes:

    verification_failed
    workflow_receipt_invalid
    digest_invalid
    runtime_commit_mismatch
    minimum_report_count_not_met

## New source summary type

New type:

    Stage3MonitoringAlertDraftSourceSummary

Fields:

    runtimeCommit
    digestHex
    reportCount
    firstRunId
    lastRunId
    verifierId
    verifiedAtIso
    receiptCreatedAtIso
    verificationOk
    receiptValid

## New config type

New type:

    Stage3MonitoringAlertDraftConfig

Fields:

    observedAtIso
    expectedRuntimeCommit
    minimumReportCount

The expectedRuntimeCommit and minimumReportCount fields are optional.

## New error type

New class:

    Stage3MonitoringAlertDraftError

New reason type:

    Stage3MonitoringAlertDraftErrorReason

Reasons:

    invalid_workflow_result
    invalid_observed_at_iso
    invalid_expected_runtime_commit
    invalid_minimum_report_count
    forbidden_config_value
    forbidden_workflow_value

## New helpers

Alert draft creation helper:

    createStage3MonitoringAlertDraftPrototype

Alert draft validation helper:

    checkStage3MonitoringAlertDraftPrototype

## Validation rules

Stage 3.8 validates:

- workflow result object exists
- workflow artifactType is stage3_operator_workflow_script_result
- workflow schemaVersion is 1
- workflow executionMode is offline_zero_sol
- workflow summary exists
- runtimeCommit exists
- digestHex exists
- reportCount exists
- verifierId exists
- verifiedAtIso exists
- receiptCreatedAtIso exists
- receiptValid is boolean
- verification result ok flag is boolean
- observedAtIso is a valid ISO timestamp
- expectedRuntimeCommit, if present, is non-empty
- minimumReportCount, if present, is an integer >= 1

## Live / secret material rejection

Stage 3.8 rejects forbidden config or workflow values containing markers such as:

- secretKey
- guardianSigners
- privateKey
- ANCHOR_WALLET
- wallet.json
- PRIVATE_KEY
- MNEMONIC
- seed phrase

These are defensive markers only.

No real secret values are introduced.

## Successful ok draft test

Confirmed behavior:

- creates an ok offline monitoring alert draft from a successful workflow result
- artifactType is stage3_monitoring_alert_draft
- schemaVersion is 1
- stage is 3.8
- executionMode is offline_zero_sol
- sourceArtifactType is stage3_operator_workflow_script_result
- sourceSchemaVersion is 1
- status is ok
- alerts array is empty
- runtimeCommit is preserved
- reportCount is preserved
- firstRunId is preserved
- lastRunId is preserved
- verifierId is preserved
- verifiedAtIso is preserved
- receiptCreatedAtIso is preserved
- verificationOk is true
- receiptValid is true
- digestHex is a 64-character lowercase hex string
- checkStage3MonitoringAlertDraftPrototype returns true

## Warning and critical draft test

Confirmed warning behavior:

- expectedRuntimeCommit mismatch produces runtime_commit_mismatch
- minimumReportCount above actual reportCount produces minimum_report_count_not_met
- both alerts are warning severity
- resulting status is warning

Confirmed critical behavior:

- receiptValid false produces workflow_receipt_invalid
- malformed digestHex produces digest_invalid
- both alerts are critical severity
- resulting status is critical

## Malformed config and forbidden value rejection test

Confirmed behavior:

- bad observedAtIso is rejected as invalid_observed_at_iso
- minimumReportCount 0 is rejected as invalid_minimum_report_count
- expectedRuntimeCommit containing privateKey marker is rejected as forbidden_config_value
- malformed workflow artifactType is rejected as invalid_workflow_result
- workflow verifierId containing privateKey marker is rejected as forbidden_workflow_value
- successful draft stable JSON does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent
- PRIVATE_KEY is absent
- MNEMONIC is absent

## Stage 3.8 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_monitoring_alert_draft_boundary.test.ts

Result:

    Stage 3.8 monitoring alert draft boundary
      ✔ creates an ok offline monitoring alert draft from a successful workflow result
      ✔ creates warning and critical drafts for deterministic workflow anomalies
      ✔ rejects malformed monitoring config and forbidden secret-bearing values

    3 passing

## Stage 3.7 plus Stage 3.8 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_workflow_config_boundary.test.ts \
      tests/stage3_monitoring_alert_draft_boundary.test.ts

Result:

    Stage 3.7 operator workflow config/env boundary
      ✔ parses operator workflow env config and runs the offline workflow
      ✔ rejects malformed config and env values before workflow execution
      ✔ rejects forbidden live or secret-bearing config/env material

    Stage 3.8 monitoring alert draft boundary
      ✔ creates an ok offline monitoring alert draft from a successful workflow result
      ✔ creates warning and critical drafts for deterministic workflow anomalies
      ✔ rejects malformed monitoring config and forbidden secret-bearing values

    6 passing

## Stage 3.1 through Stage 3.8 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts \
      tests/stage3_operator_workflow_script_boundary.test.ts \
      tests/stage3_operator_workflow_config_boundary.test.ts \
      tests/stage3_monitoring_alert_draft_boundary.test.ts

Result:

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    Stage 3.2 operator report export boundary
      ✔ exports and reads a Stage 2 operator report artifact through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for operator report exports
      ✔ rejects malformed operator report artifacts and invalid files

    Stage 3.3 audit bundle export boundary
      ✔ exports and reads a Stage 2 audit export bundle through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for audit bundle exports
      ✔ rejects malformed audit bundles and invalid files

    Stage 3.4 audit bundle verifier boundary
      ✔ verifies an exported Stage 2 audit bundle file through Stage 3 file IO
      ✔ returns Stage 2 verifier failure results for tampered bundle files
      ✔ rejects invalid files and invalid verifier timestamps

    Stage 3.5 verification receipt boundary
      ✔ creates, exports, reads, and verifies a receipt from a successful Stage 3.4 verification result
      ✔ rejects failed verification results and malformed receipt metadata
      ✔ rejects malformed receipt artifacts, invalid JSON, path escapes, and accidental overwrite

    Stage 3.6 operator workflow script boundary
      ✔ runs the offline operator workflow from reports to audit bundle, verification, and receipt
      ✔ rejects invalid workflow inputs before producing a full workflow result
      ✔ inherits Stage 3 file IO overwrite and path safety across the workflow

    Stage 3.7 operator workflow config/env boundary
      ✔ parses operator workflow env config and runs the offline workflow
      ✔ rejects malformed config and env values before workflow execution
      ✔ rejects forbidden live or secret-bearing config/env material

    Stage 3.8 monitoring alert draft boundary
      ✔ creates an ok offline monitoring alert draft from a successful workflow result
      ✔ creates warning and critical drafts for deterministic workflow anomalies
      ✔ rejects malformed monitoring config and forbidden secret-bearing values

    24 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

## Zero-SOL boundary

Stage 3.8 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution
- notification delivery
- production monitoring transport

Stage 3.8 uses only:

- Stage 3.6 workflow result shape
- Stage 3.7 config/env workflow setup in tests
- local deterministic status computation
- local alert draft generation
- local validation
- local temporary test directories

Therefore Stage 3.8 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.8 establishes the monitoring alert draft boundary.

It proves that an offline workflow result can be converted into a deterministic ok / warning / critical monitoring draft without live RPC, notification transport, transaction submission, or secret-bearing material.

This becomes the foundation for later production runbooks, monitoring adapters, and alert delivery layers.

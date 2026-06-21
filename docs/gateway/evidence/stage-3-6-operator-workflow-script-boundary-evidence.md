# Stage 3.6 Operator Workflow Script Boundary Evidence

This document records Stage 3.6 operator workflow script boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-6-operator-workflow-script-boundary

Runtime commit:

    97cc765 Add Stage 3.6 operator workflow script boundary

Base runtime commit:

    c926ffe Add Stage 3.5 verification receipt boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 established the audit bundle export boundary.

Stage 3.4 established the audit bundle verifier boundary.

Stage 3.5 established the verification receipt boundary.

Stage 3.6 adds an offline operator workflow script boundary that connects the already-proven Stage 3.2 through Stage 3.5 layers into one deterministic operator workflow.

## Scope

Stage 3.6 adds an operator workflow script boundary.

It connects:

    operator reports
    -> report artifact exports
    -> audit log construction
    -> digest
    -> checkpoint
    -> audit bundle export
    -> audit bundle verification
    -> verification receipt creation
    -> verification receipt export
    -> receipt verification
    -> compact workflow summary

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

It does not introduce a new verifier schema.

It does not introduce a new receipt schema.

It composes already-proven Stage 2 and Stage 3 helper models.

## Runtime changes

New helper:

    tests/helpers/stage3OperatorWorkflowScriptPrototype.ts

New test:

    tests/stage3_operator_workflow_script_boundary.test.ts

## Stage 2 dependency

Stage 3.6 depends on Stage 2 audit and report models:

    createStage2RelayerOperatorAuditLogPrototype
    appendStage2RelayerOperatorAuditLogPrototype
    computeStage2RelayerOperatorAuditLogDigestPrototype
    createStage2RelayerOperatorAuditLogCheckpointPrototype
    createStage2RelayerOperatorAuditExportBundlePrototype
    Stage2RelayerOperatorRunReportLogArtifact

Stage 3.6 does not alter Stage 2 evidence semantics.

It uses Stage 2 models to construct the same audit log, digest, checkpoint, and bundle shape that earlier Stage 2 and Stage 3 layers already proved.

## Stage 3 dependencies

Stage 3.6 composes the following Stage 3 helpers:

    exportStage3OperatorReportArtifactPrototype
    exportStage3AuditBundleArtifactPrototype
    verifyStage3AuditBundleFilePrototype
    createStage3VerificationReceiptArtifactPrototype
    exportStage3VerificationReceiptArtifactPrototype
    verifyStage3VerificationReceiptArtifactPrototype

This preserves the intended Stage 3 evidence chain:

    Stage 3.2 operator report export
    -> Stage 3.3 audit bundle export
    -> Stage 3.4 audit bundle verification
    -> Stage 3.5 verification receipt export / verification

## New workflow result

New result type:

    Stage3OperatorWorkflowScriptResult

Fields:

    artifactType: "stage3_operator_workflow_script_result"
    schemaVersion: 1
    executionMode: "offline_zero_sol"
    reportExports
    auditBundleExport
    verification
    receiptExport
    receiptValid
    summary

Summary fields:

    reportCount
    runtimeCommit
    digestHex
    firstRunId
    lastRunId
    verifierId
    auditBundleRelativePath
    receiptRelativePath
    verifiedAtIso
    receiptCreatedAtIso

## New helper

Workflow helper:

    runStage3OperatorWorkflowScriptPrototype

The helper validates:

- at least one report exists
- report count equals report path count
- Stage 3.1 overwrite rules are preserved
- Stage 3.1 path safety is preserved
- Stage 3.4 verification must succeed before receipt creation
- Stage 3.5 receipt verification must succeed for receiptValid to be true

## Successful workflow test

Confirmed behavior:

- exports two operator report artifacts
- constructs an audit log from exported report artifacts
- computes audit log digest
- creates audit log checkpoint
- creates audit export bundle
- exports audit bundle through Stage 3.3
- verifies audit bundle through Stage 3.4
- creates verification receipt through Stage 3.5
- exports receipt through Stage 3.5
- verifies receipt through Stage 3.5
- returns receiptValid: true
- returns executionMode: offline_zero_sol
- preserves reportCount
- preserves runtimeCommit
- preserves digestHex
- preserves firstRunId
- preserves lastRunId
- preserves verifierId
- preserves auditBundleRelativePath
- preserves receiptRelativePath
- preserves verifiedAtIso
- preserves receiptCreatedAtIso
- written report artifact can be read back
- written audit bundle can be read back
- written receipt can be read back
- combined stable JSON does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent

## Invalid workflow input test

Confirmed behavior:

- empty report list is rejected before producing a full workflow result
- report path count mismatch is rejected before producing a full workflow result
- blank verifierId is rejected through Stage 2.37 receipt validation

Failure modes covered:

- no_reports
- report_path_count_mismatch
- invalid_verifier_id

## File IO safety test

Confirmed behavior:

- accidental overwrite is rejected by default
- explicit overwrite succeeds with overwrite: true
- overwrite preserves the same digest when inputs are unchanged
- report path escape is rejected
- audit bundle path escape is rejected
- receipt path escape is rejected

Failure modes covered:

- artifact_exists
- path_escape

## Stage 3.6 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_workflow_script_boundary.test.ts

Result:

    Stage 3.6 operator workflow script boundary
      ✔ runs the offline operator workflow from reports to audit bundle, verification, and receipt
      ✔ rejects invalid workflow inputs before producing a full workflow result
      ✔ inherits Stage 3 file IO overwrite and path safety across the workflow

    3 passing

## Stage 3.2 plus Stage 3.3 plus Stage 3.4 plus Stage 3.5 plus Stage 3.6 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts \
      tests/stage3_operator_workflow_script_boundary.test.ts

Result:

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

    15 passing

## Stage 3.1 through Stage 3.6 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts \
      tests/stage3_operator_workflow_script_boundary.test.ts

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

    18 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected secret-safety assertion lines exist only as negative assertions in the test.

No secret-like material was introduced.

## Zero-SOL boundary

Stage 3.6 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.6 uses only:

- Stage 2 report / audit / digest / checkpoint / bundle models
- Stage 3.1 local file IO
- Stage 3.2 operator report export helper
- Stage 3.3 audit bundle export helper
- Stage 3.4 audit bundle verifier helper
- Stage 3.5 verification receipt helper
- local temporary test directories
- local JSON serialization / deserialization / verification

Therefore Stage 3.6 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.6 establishes the offline operator workflow script boundary.

It proves that the existing Stage 3.2 through Stage 3.5 tooling layers can be composed into one deterministic operator workflow:

    reports -> audit bundle -> verification -> receipt

This becomes the foundation for later real CLI commands, config/env boundaries, monitoring, and production runbooks.

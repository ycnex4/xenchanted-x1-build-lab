# Stage 3.7 Operator Workflow Config Boundary Evidence

This document records Stage 3.7 operator workflow config/env boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-7-operator-workflow-config-boundary

Runtime commit:

    fd62222 Add Stage 3.7 operator workflow config boundary

Base runtime commit:

    97cc765 Add Stage 3.6 operator workflow script boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 established the audit bundle export boundary.

Stage 3.4 established the audit bundle verifier boundary.

Stage 3.5 established the verification receipt boundary.

Stage 3.6 established the offline operator workflow script boundary.

Stage 3.7 adds the config/env boundary for that offline workflow.

## Scope

Stage 3.7 adds a config/env validation boundary.

It connects:

    operator env/config input
    -> config parsing
    -> config validation
    -> forbidden live/secret material rejection
    -> Stage 3.6 workflow execution

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

It does not introduce a new artifact schema.

It does not introduce a CLI command yet.

It prepares the workflow for a later CLI surface by proving that config/env input can be normalized and rejected safely before workflow execution.

## Runtime changes

New helper:

    tests/helpers/stage3OperatorWorkflowConfigPrototype.ts

New test:

    tests/stage3_operator_workflow_config_boundary.test.ts

## Stage 3.6 dependency

Stage 3.7 depends on the Stage 3.6 operator workflow script boundary.

It uses:

    runStage3OperatorWorkflowScriptPrototype

Stage 3.7 does not alter the Stage 3.6 workflow semantics.

It only validates config/env input before passing normalized values into the existing Stage 3.6 workflow.

## New config type

New type:

    Stage3OperatorWorkflowConfig

Fields:

    rootDir
    reportRelativePaths
    auditBundleRelativePath
    receiptRelativePath
    checkpointCreatedAtIso
    bundleCreatedAtIso
    verifiedAtIso
    receiptCreatedAtIso
    runtimeCommit
    verifierId
    overwrite

## New env type

New type:

    Stage3OperatorWorkflowConfigEnv

This is a string map representing environment-style input.

Required env keys:

    STAGE3_OPERATOR_WORKFLOW_ROOT_DIR
    STAGE3_OPERATOR_WORKFLOW_REPORT_PATHS
    STAGE3_OPERATOR_WORKFLOW_AUDIT_BUNDLE_PATH
    STAGE3_OPERATOR_WORKFLOW_RECEIPT_PATH
    STAGE3_OPERATOR_WORKFLOW_CHECKPOINT_CREATED_AT_ISO
    STAGE3_OPERATOR_WORKFLOW_BUNDLE_CREATED_AT_ISO
    STAGE3_OPERATOR_WORKFLOW_VERIFIED_AT_ISO
    STAGE3_OPERATOR_WORKFLOW_RECEIPT_CREATED_AT_ISO
    STAGE3_OPERATOR_WORKFLOW_RUNTIME_COMMIT
    STAGE3_OPERATOR_WORKFLOW_VERIFIER_ID

Optional env key:

    STAGE3_OPERATOR_WORKFLOW_OVERWRITE

Overwrite accepts only:

    true
    false
    empty / absent as false

## New error type

New class:

    Stage3OperatorWorkflowConfigError

New reason type:

    Stage3OperatorWorkflowConfigErrorReason

Reasons:

    invalid_config
    missing_env_key
    invalid_root_dir
    invalid_report_paths
    invalid_relative_path
    invalid_iso_timestamp
    invalid_runtime_commit
    invalid_verifier_id
    invalid_overwrite
    forbidden_env_key
    forbidden_config_value

## New helpers

Config creation helper:

    createStage3OperatorWorkflowConfigPrototype

Env parser helper:

    parseStage3OperatorWorkflowConfigEnvPrototype

Workflow-from-config helper:

    runStage3OperatorWorkflowFromConfigPrototype

## Validation rules

Stage 3.7 validates:

- config object exists
- rootDir is present
- reportRelativePaths is a non-empty array
- report paths are relative paths
- audit bundle path is a relative path
- receipt path is a relative path
- timestamps are ISO timestamps
- runtimeCommit is present
- verifierId is present
- overwrite is boolean
- env overwrite accepts only true / false
- required env keys are present
- path escapes are rejected
- absolute paths for artifact-relative fields are rejected

## Live / secret material rejection

Stage 3.7 rejects forbidden env keys containing markers such as:

- ANCHOR_WALLET
- ANCHOR_PROVIDER_URL
- PRIVATE_KEY
- MNEMONIC
- SEED
- WALLET_JSON
- RPC_SECRET

Stage 3.7 rejects forbidden config values containing markers such as:

- secretKey
- guardianSigners
- privateKey
- ANCHOR_WALLET
- wallet.json
- MNEMONIC
- seed phrase

These are defensive markers only.

No real secret values are introduced.

## Successful config/env test

Confirmed behavior:

- parses env-style operator workflow config
- trims and normalizes values
- parses comma-separated report paths
- parses overwrite as false
- runs the offline Stage 3.6 workflow from config
- returns stage3_operator_workflow_script_result
- returns executionMode: offline_zero_sol
- returns receiptValid: true
- preserves reportCount
- preserves runtimeCommit
- preserves firstRunId
- preserves lastRunId
- preserves verifierId
- preserves auditBundleRelativePath
- preserves receiptRelativePath
- preserves verifiedAtIso
- preserves receiptCreatedAtIso
- returns digestHex as 64 lowercase hex characters

## Malformed config/env rejection test

Confirmed behavior:

- missing rootDir env value is rejected as missing_env_key
- blank report paths env value is rejected as missing_env_key
- direct config with reportRelativePaths: [] is rejected as invalid_report_paths
- report path escape is rejected as invalid_relative_path
- bad verifiedAtIso is rejected as invalid_iso_timestamp
- invalid overwrite value is rejected as invalid_overwrite
- blank verifierId is rejected as invalid_verifier_id

Important correction:

A blank env value for STAGE3_OPERATOR_WORKFLOW_REPORT_PATHS is treated as missing_env_key because env parsing rejects empty values before config construction.

The invalid_report_paths case is covered separately through direct config construction with an empty reportRelativePaths array.

## Forbidden material rejection test

Confirmed behavior:

- ANCHOR_WALLET env key is rejected as forbidden_env_key
- PRIVATE_KEY env key is rejected as forbidden_env_key
- verifierId containing privateKey marker is rejected as forbidden_config_value
- successful workflow stable JSON output does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent

## Stage 3.7 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_workflow_config_boundary.test.ts

Result:

    Stage 3.7 operator workflow config/env boundary
      ✔ parses operator workflow env config and runs the offline workflow
      ✔ rejects malformed config and env values before workflow execution
      ✔ rejects forbidden live or secret-bearing config/env material

    3 passing

## Stage 3.6 plus Stage 3.7 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_workflow_script_boundary.test.ts \
      tests/stage3_operator_workflow_config_boundary.test.ts

Result:

    Stage 3.6 operator workflow script boundary
      ✔ runs the offline operator workflow from reports to audit bundle, verification, and receipt
      ✔ rejects invalid workflow inputs before producing a full workflow result
      ✔ inherits Stage 3 file IO overwrite and path safety across the workflow

    Stage 3.7 operator workflow config/env boundary
      ✔ parses operator workflow env config and runs the offline workflow
      ✔ rejects malformed config and env values before workflow execution
      ✔ rejects forbidden live or secret-bearing config/env material

    6 passing

## Stage 3.1 through Stage 3.7 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts \
      tests/stage3_operator_workflow_script_boundary.test.ts \
      tests/stage3_operator_workflow_config_boundary.test.ts

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

    21 passing

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

Stage 3.7 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.7 uses only:

- local config/env parsing
- local validation
- forbidden marker rejection
- Stage 3.6 offline workflow helper
- local temporary test directories
- local JSON serialization / deserialization / verification through prior Stage 3 layers

Therefore Stage 3.7 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.7 establishes the operator workflow config/env boundary.

It proves that offline operator workflow configuration can be parsed, normalized, validated, rejected safely when malformed or secret-bearing, and then used to run the already-proven Stage 3.6 workflow.

This becomes the foundation for later CLI command boundaries, monitoring, and production runbooks.

# Stage 3.2 Operator Report Export Boundary Evidence

This document records Stage 3.2 operator report export boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-2-operator-report-export-boundary

Runtime commit:

    a3d021d Add Stage 3.2 operator report export boundary

Base runtime commit:

    c307ffd Add Stage 3.1 artifact file IO boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 builds on Stage 3.1 by exporting an already-proven Stage 2 operator report artifact through the Stage 3 file IO layer.

## Scope

Stage 3.2 adds an operator report export boundary.

It connects:

    Stage 2.31 operator report artifact
    -> Stage 3.1 artifact file IO
    -> local JSON export
    -> local JSON read
    -> Stage 2 operator report deserialization / validation
    -> stable export round-trip verification

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

## Runtime changes

New helper:

    tests/helpers/stage3OperatorReportExportPrototype.ts

New test:

    tests/stage3_operator_report_export_boundary.test.ts

## Stage 2 dependency

Stage 3.2 depends on the Stage 2.31 operator report artifact model:

    stage2_relayer_operator_run_report

It uses the existing Stage 2 helper:

    deserializeStage2RelayerOperatorRunReportPrototype

It uses the existing Stage 2 fixture helper:

    createStage2RelayerOperatorRunReportFixturePrototype

The Stage 3.2 boundary does not redefine the operator report schema.

It reuses the Stage 2 artifact validation model and only adds the file export/read boundary around it.

## Stage 3.1 dependency

Stage 3.2 depends on the Stage 3.1 artifact file IO model.

It uses:

    writeStage3ArtifactFilePrototype
    readStage3ArtifactFilePrototype
    serializeStage3ArtifactFileJsonPrototype

Therefore Stage 3.2 inherits Stage 3.1 file safety properties:

- stable pretty JSON
- trailing newline
- parent directory creation
- accidental overwrite protection
- explicit overwrite only
- path escape protection
- invalid JSON rejection
- local offline file system execution

## New operator report export result

New result type:

    Stage3OperatorReportExportResult

Fields inherited from Stage 3 file write result:

    rootDir
    relativePath
    artifactPath
    bytesWritten
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_run_report"
    runId
    artifact

## New operator report read result

New result type:

    Stage3OperatorReportReadResult

Fields inherited from Stage 3 file read result:

    rootDir
    relativePath
    artifactPath
    bytesRead
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_run_report"
    runId
    artifact

## New helpers

Export helper:

    exportStage3OperatorReportArtifactPrototype

Read helper:

    readStage3OperatorReportArtifactPrototype

Verification helper:

    verifyStage3OperatorReportArtifactExportPrototype

Internal normalization helper:

    normalizeStage3OperatorReportArtifactPrototype

## Successful export / read test

Confirmed behavior:

- creates a valid Stage 2 operator report artifact fixture
- exports the operator report artifact through Stage 3.1 file IO
- writes stable pretty JSON to disk
- records bytesWritten
- reads the same JSON artifact back from disk
- records bytesRead
- preserves artifactType
- preserves runId
- parsed artifact equals original artifact
- Stage 2 deserializer accepts the exported JSON
- stable export round-trip verification returns true
- exported stableJson does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent

## Overwrite behavior test

Confirmed behavior:

- first operator report export succeeds
- second export to the same path without overwrite fails
- failure reason is artifact_exists
- explicit overwrite succeeds when overwrite: true is passed
- overwritten report reads back correctly
- overwritten runId is preserved

## Malformed artifact / invalid file rejection test

Confirmed behavior:

- non-.json export path is rejected
- wrong artifactType is rejected by Stage 2 operator report deserializer
- wrong artifactType written through generic Stage 3.1 file IO is rejected by Stage 3.2 operator report reader
- malformed operator report with empty runId is rejected
- invalid JSON file is rejected during read
- verification returns false for invalid JSON

## Stage 3.2 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_operator_report_export_boundary.test.ts

Result:

    Stage 3.2 operator report export boundary
      ✔ exports and reads a Stage 2 operator report artifact through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for operator report exports
      ✔ rejects malformed operator report artifacts and invalid files

    3 passing

## Stage 3.1 plus Stage 3.2 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts

Result:

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    Stage 3.2 operator report export boundary
      ✔ exports and reads a Stage 2 operator report artifact through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for operator report exports
      ✔ rejects malformed operator report artifacts and invalid files

    6 passing

## Stage 2.31 plus Stage 3.1 plus Stage 3.2 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage2_operator_report_serialization_log_artifact.test.ts \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts

Result:

    Stage 2.31 operator report serialization / stable log artifact
      ✔ serializes and deserializes an operator report as a stable JSON artifact
      ✔ keeps the serialized report free of secret-bearing fields
      ✔ validates report shape and rejects malformed artifacts

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    Stage 3.2 operator report export boundary
      ✔ exports and reads a Stage 2 operator report artifact through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for operator report exports
      ✔ rejects malformed operator report artifacts and invalid files

    9 passing

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

Stage 3.2 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.2 uses only:

- Stage 2 operator report fixture / deserializer
- Stage 3.1 local file IO
- local temporary test directories
- local JSON serialization / deserialization

Therefore Stage 3.2 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.2 establishes the operator report export boundary.

It proves that a Stage 2 operator report artifact can be exported to disk, read back, validated by the Stage 2 artifact deserializer, and verified as a stable Stage 3 file IO round trip.

This becomes the foundation for later operator export commands and production workflow packaging.

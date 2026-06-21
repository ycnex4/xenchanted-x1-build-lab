# Stage 3.3 Audit Bundle Export Boundary Evidence

This document records Stage 3.3 audit bundle export boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-3-audit-bundle-export-boundary

Runtime commit:

    e2f0fd6 Add Stage 3.3 audit bundle export boundary

Base runtime commit:

    a3d021d Add Stage 3.2 operator report export boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 builds on Stage 3.1 by exporting an already-proven Stage 2.35 audit export bundle artifact through the Stage 3 file IO layer.

## Scope

Stage 3.3 adds an audit bundle export boundary.

It connects:

    Stage 2.35 audit export bundle
    -> Stage 3.1 artifact file IO
    -> local JSON export
    -> local JSON read
    -> Stage 2 audit bundle deserialization / validation
    -> Stage 2 audit bundle verification
    -> stable export round-trip verification

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

## Runtime changes

New helper:

    tests/helpers/stage3AuditBundleExportPrototype.ts

New test:

    tests/stage3_audit_bundle_export_boundary.test.ts

## Stage 2 dependency

Stage 3.3 depends on the Stage 2.35 audit export bundle artifact model:

    stage2_relayer_operator_audit_export_bundle

It uses the existing Stage 2 helpers:

    deserializeStage2RelayerOperatorAuditExportBundlePrototype
    verifyStage2RelayerOperatorAuditExportBundlePrototype
    createStage2RelayerOperatorAuditExportBundlePrototype
    createStage2RelayerOperatorAuditLogPrototype
    appendStage2RelayerOperatorAuditLogPrototype
    computeStage2RelayerOperatorAuditLogDigestPrototype
    createStage2RelayerOperatorAuditLogCheckpointPrototype
    createStage2RelayerOperatorRunReportFixturePrototype

The Stage 3.3 boundary does not redefine the audit bundle schema.

It reuses the Stage 2 artifact validation / verification model and only adds the file export/read boundary around it.

## Stage 3.1 dependency

Stage 3.3 depends on the Stage 3.1 artifact file IO model.

It uses:

    writeStage3ArtifactFilePrototype
    readStage3ArtifactFilePrototype
    serializeStage3ArtifactFileJsonPrototype

Therefore Stage 3.3 inherits Stage 3.1 file safety properties:

- stable pretty JSON
- trailing newline
- parent directory creation
- accidental overwrite protection
- explicit overwrite only
- path escape protection
- invalid JSON rejection
- local offline file system execution

## New audit bundle export result

New result type:

    Stage3AuditBundleExportResult

Fields inherited from Stage 3 file write result:

    rootDir
    relativePath
    artifactPath
    bytesWritten
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_audit_export_bundle"
    stageRange: "2.31-2.35"
    runtimeCommit
    reportCount
    artifact

## New audit bundle read result

New result type:

    Stage3AuditBundleReadResult

Fields inherited from Stage 3 file read result:

    rootDir
    relativePath
    artifactPath
    bytesRead
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_audit_export_bundle"
    stageRange: "2.31-2.35"
    runtimeCommit
    reportCount
    artifact

## New helpers

Export helper:

    exportStage3AuditBundleArtifactPrototype

Read helper:

    readStage3AuditBundleArtifactPrototype

Verification helper:

    verifyStage3AuditBundleArtifactExportPrototype

Internal normalization helper:

    normalizeStage3AuditBundleArtifactPrototype

## Successful export / read test

Confirmed behavior:

- creates a valid Stage 2.35 audit export bundle fixture
- exports the audit bundle through Stage 3.1 file IO
- writes stable pretty JSON to disk
- records bytesWritten
- reads the same JSON artifact back from disk
- records bytesRead
- preserves artifactType
- preserves stageRange
- preserves runtimeCommit
- preserves reportCount
- parsed artifact equals original artifact
- Stage 2 deserializer accepts the exported JSON
- Stage 2 verifier accepts the reloaded bundle
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

- first audit bundle export succeeds
- second export to the same path without overwrite fails
- failure reason is artifact_exists
- explicit overwrite succeeds when overwrite: true is passed
- overwritten bundle reads back correctly
- overwritten bundle run data is preserved

## Malformed artifact / invalid file rejection test

Confirmed behavior:

- non-.json export path is rejected
- wrong artifactType is rejected by Stage 2 audit bundle deserializer
- wrong artifactType written through generic Stage 3.1 file IO is rejected by Stage 3.3 audit bundle reader
- malformed audit bundle with blank runtimeCommit is rejected
- digest mismatch is rejected
- invalid JSON file is rejected during read
- verification returns false for invalid JSON

## Stage 3.3 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_audit_bundle_export_boundary.test.ts

Result:

    Stage 3.3 audit bundle export boundary
      ✔ exports and reads a Stage 2 audit export bundle through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for audit bundle exports
      ✔ rejects malformed audit bundles and invalid files

    3 passing

## Stage 3.1 plus Stage 3.2 plus Stage 3.3 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts

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

    9 passing

## Stage 2.35 plus Stage 3.1 plus Stage 3.3 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage2_operator_audit_export_bundle_boundary.test.ts \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts

Result:

    Stage 2.35 operator audit export bundle boundary
      ✔ creates a stable export bundle from audit log, digest, checkpoint, and metadata
      ✔ rejects export bundle verification when nested artifacts or metadata change
      ✔ rejects malformed export bundle artifacts and mismatched bundle inputs

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    Stage 3.3 audit bundle export boundary
      ✔ exports and reads a Stage 2 audit export bundle through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for audit bundle exports
      ✔ rejects malformed audit bundles and invalid files

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

Stage 3.3 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.3 uses only:

- Stage 2.35 audit export bundle fixture / deserializer / verifier
- Stage 3.1 local file IO
- local temporary test directories
- local JSON serialization / deserialization

Therefore Stage 3.3 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.3 establishes the audit bundle export boundary.

It proves that a Stage 2.35 audit export bundle can be exported to disk, read back, validated by the Stage 2 artifact deserializer, verified by the Stage 2 bundle verifier, and verified as a stable Stage 3 file IO round trip.

This becomes the foundation for later audit bundle export commands, verifier commands, verification receipts, and production workflow packaging.

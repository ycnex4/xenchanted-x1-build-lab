# Stage 3.5 Verification Receipt Boundary Evidence

This document records Stage 3.5 verification receipt boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-5-verification-receipt-boundary

Runtime commit:

    c926ffe Add Stage 3.5 verification receipt boundary

Base runtime commit:

    e624a43 Add Stage 3.4 audit bundle verifier boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 established the audit bundle export boundary.

Stage 3.4 established the audit bundle verifier boundary.

Stage 3.5 builds on Stage 3.4 by taking a successful Stage 3.4 audit bundle verification result and creating a Stage 2.37 verification receipt artifact from it.

## Scope

Stage 3.5 adds a verification receipt boundary.

It connects:

    Stage 3.4 audit bundle verification result
    -> Stage 2.37 verification receipt model
    -> Stage 3.1 artifact file IO
    -> local JSON export
    -> local JSON read
    -> Stage 2 receipt deserialization / validation
    -> stable receipt round-trip verification

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

It does not introduce a new receipt schema.

It reuses the Stage 2.37 verification receipt artifact model.

## Runtime changes

New helper:

    tests/helpers/stage3VerificationReceiptPrototype.ts

New test:

    tests/stage3_verification_receipt_boundary.test.ts

## Exact Stage 2.37 test file

The exact Stage 2.37 receipt test file used for smoke coverage is:

    tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts

This filename was discovered by `find` / `grep` before adding Stage 3.5 smoke checks.

## Stage 2 dependency

Stage 3.5 depends on the Stage 2.37 verification receipt model.

It uses:

    createStage2RelayerOperatorAuditBundleVerificationReceiptPrototype
    deserializeStage2RelayerOperatorAuditBundleVerificationReceiptPrototype
    verifyStage2RelayerOperatorAuditBundleVerificationReceiptPrototype

The receipt artifact type is:

    stage2_relayer_operator_audit_bundle_verification_receipt

A receipt can be created only from a successful verification result.

Failed verification results are rejected.

## Stage 3.1 dependency

Stage 3.5 depends on the Stage 3.1 artifact file IO model.

It uses:

    writeStage3ArtifactFilePrototype
    readStage3ArtifactFilePrototype
    serializeStage3ArtifactFileJsonPrototype

Therefore Stage 3.5 inherits Stage 3.1 file safety properties:

- stable pretty JSON
- trailing newline
- parent directory creation
- accidental overwrite protection
- explicit overwrite only
- path escape protection
- invalid JSON rejection
- local offline file system execution

## Stage 3.4 dependency

Stage 3.5 depends on the Stage 3.4 audit bundle verifier boundary.

It uses a successful Stage 3.4 verification result as the input for receipt creation.

This preserves the intended evidence chain:

    exported audit bundle
    -> verifier result
    -> verification receipt

## New receipt export result

New result type:

    Stage3VerificationReceiptExportResult

Fields inherited from Stage 3 file write result:

    rootDir
    relativePath
    artifactPath
    bytesWritten
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_audit_bundle_verification_receipt"
    verifierId
    runtimeCommit
    digestHex
    reportCount
    artifact

## New receipt read result

New result type:

    Stage3VerificationReceiptReadResult

Fields inherited from Stage 3 file read result:

    rootDir
    relativePath
    artifactPath
    bytesRead
    stableJson

Additional fields:

    artifactType: "stage2_relayer_operator_audit_bundle_verification_receipt"
    verifierId
    runtimeCommit
    digestHex
    reportCount
    artifact

## New helpers

Receipt creation helper:

    createStage3VerificationReceiptArtifactPrototype

Receipt export helper:

    exportStage3VerificationReceiptArtifactPrototype

Receipt read helper:

    readStage3VerificationReceiptArtifactPrototype

Receipt verification helper:

    verifyStage3VerificationReceiptArtifactPrototype

Internal normalization helper:

    normalizeStage3VerificationReceiptArtifactPrototype

## Successful receipt test

Confirmed behavior:

- creates a valid Stage 2.35 audit export bundle
- exports the bundle through Stage 3.3
- verifies the bundle through Stage 3.4
- creates a receipt from the successful Stage 3.4 verification result
- preserves artifactType
- preserves schemaVersion
- preserves createdAtIso
- preserves verifierId
- preserves verificationResultArtifactType
- preserves verificationResultSchemaVersion
- preserves bundleArtifactType
- preserves stageRange
- preserves runtimeCommit
- preserves digestHex
- preserves reportCount
- preserves firstRunId
- preserves lastRunId
- preserves checkpointCreatedAtIso
- preserves bundleCreatedAtIso
- preserves verifiedAtIso
- exports receipt as stable pretty JSON
- reads receipt JSON back from disk
- Stage 2 receipt deserializer accepts exported JSON
- Stage 2 receipt verifier accepts reloaded receipt
- stable receipt round-trip verification returns true
- exported stableJson does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent

## Failed verification / malformed metadata rejection test

Confirmed behavior:

- failed verification result is rejected
- invalid receipt createdAtIso is rejected
- blank verifierId is rejected

Failure modes covered:

- digest_mismatch as failed verification result reason
- invalid_created_at_iso
- invalid_verifier_id

## Malformed artifact / file rejection test

Confirmed behavior:

- accidental overwrite is rejected by default
- explicit overwrite succeeds when overwrite: true is passed
- wrong receipt artifactType is rejected
- malformed digestHex is rejected
- invalid JSON file is rejected during read
- invalid JSON verification returns false
- path escape is rejected

Failure modes covered:

- artifact_exists
- invalid_artifact_type
- invalid_digest_hex
- invalid_json
- path_escape

## Stage 3.5 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_verification_receipt_boundary.test.ts

Result:

    Stage 3.5 verification receipt boundary
      ✔ creates, exports, reads, and verifies a receipt from a successful Stage 3.4 verification result
      ✔ rejects failed verification results and malformed receipt metadata
      ✔ rejects malformed receipt artifacts, invalid JSON, path escapes, and accidental overwrite

    3 passing

## Stage 2.37 plus Stage 3.5 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage2_operator_audit_bundle_verification_receipt_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts

Result:

    Stage 2.37 operator audit bundle verification receipt boundary
      ✔ creates a stable verification receipt from a successful bundle verification result
      ✔ rejects failed verification results and malformed receipt metadata
      ✔ rejects malformed receipt artifacts during validation and deserialization

    Stage 3.5 verification receipt boundary
      ✔ creates, exports, reads, and verifies a receipt from a successful Stage 3.4 verification result
      ✔ rejects failed verification results and malformed receipt metadata
      ✔ rejects malformed receipt artifacts, invalid JSON, path escapes, and accidental overwrite

    6 passing

## Stage 3.3 plus Stage 3.4 plus Stage 3.5 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts

Result:

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

    9 passing

## Stage 3.1 plus Stage 3.2 plus Stage 3.3 plus Stage 3.4 plus Stage 3.5 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_verification_receipt_boundary.test.ts

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

    15 passing

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

Stage 3.5 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.5 uses only:

- Stage 3.3 audit bundle export helper for setup
- Stage 3.4 audit bundle verifier helper
- Stage 2.37 verification receipt model
- Stage 3.1 local file IO
- local temporary test directories
- local JSON serialization / deserialization / verification

Therefore Stage 3.5 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.5 establishes the verification receipt boundary.

It proves that a successful Stage 3.4 audit bundle verification result can be converted into a Stage 2.37 verification receipt, exported to disk, read back, validated, and verified as a stable Stage 3 file IO round trip.

This becomes the foundation for later receipt CLI commands and production workflow packaging.

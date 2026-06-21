# Stage 3.4 Audit Bundle Verifier Boundary Evidence

This document records Stage 3.4 audit bundle verifier boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-4-audit-bundle-verifier-boundary

Runtime commit:

    e624a43 Add Stage 3.4 audit bundle verifier boundary

Base runtime commit:

    e2f0fd6 Add Stage 3.3 audit bundle export boundary

## Stage position

Stage 2 is closed.

Stage 3 is the tooling / production surface.

Stage 3.1 established the local deterministic artifact file IO boundary.

Stage 3.2 established the operator report export boundary.

Stage 3.3 established the audit bundle export boundary.

Stage 3.4 builds on Stage 3.1 and Stage 3.3 by reading an exported audit bundle file and passing its stable JSON into the already-proven Stage 2.36 audit bundle verifier model.

## Scope

Stage 3.4 adds an audit bundle verifier boundary.

It connects:

    Stage 3.3 exported audit bundle file
    -> Stage 3.1 artifact file IO read
    -> stable JSON
    -> Stage 2.36 serialized audit bundle verifier
    -> compact verification result

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

It does not introduce a new audit bundle verification schema.

It reuses the Stage 2.36 verifier result model.

## Runtime changes

New helper:

    tests/helpers/stage3AuditBundleVerifierPrototype.ts

New test:

    tests/stage3_audit_bundle_verifier_boundary.test.ts

## Stage 2 dependency

Stage 3.4 depends on the Stage 2.36 serialized audit bundle verifier model.

It uses:

    verifySerializedStage2RelayerOperatorAuditBundlePrototype

The Stage 2.36 verifier accepts:

    serializedBundle
    verifiedAtIso

It returns:

    stage2_relayer_operator_audit_bundle_verification_result

Successful result fields include:

    ok
    artifactType
    schemaVersion
    verifiedAtIso
    bundleArtifactType
    stageRange
    runtimeCommit
    digestHex
    reportCount
    firstRunId
    lastRunId
    checkpointCreatedAtIso
    bundleCreatedAtIso

Failure result fields include:

    ok
    artifactType
    schemaVersion
    verifiedAtIso
    reason

Stage 3.4 does not redefine these fields.

## Stage 3.1 dependency

Stage 3.4 depends on the Stage 3.1 artifact file IO model.

It uses:

    readStage3ArtifactFilePrototype

Therefore Stage 3.4 inherits Stage 3.1 file read safety properties:

- path escape protection
- invalid artifact path rejection
- invalid JSON rejection
- stable JSON read model
- local offline file system execution

## Stage 3.3 dependency

Stage 3.4 depends on the Stage 3.3 audit bundle export model for test setup and production flow continuity.

It uses:

    exportStage3AuditBundleArtifactPrototype

This allows Stage 3.4 tests to verify a bundle file produced by the same Stage 3 export surface that later operator workflows will use.

## New file verification result

New result type:

    Stage3AuditBundleFileVerificationResult

Fields inherited from Stage 3 file read result:

    rootDir
    relativePath
    artifactPath
    bytesRead
    stableJson

Additional field:

    verificationResult

The verificationResult is the Stage 2.36 audit bundle verification result.

## New helpers

Verifier helper:

    verifyStage3AuditBundleFilePrototype

Boolean check helper:

    checkStage3AuditBundleFileVerificationPrototype

## Successful verification test

Confirmed behavior:

- creates a valid Stage 2.35 audit export bundle
- exports the bundle through Stage 3.3
- reads the bundle file through Stage 3.1 file IO
- passes stable JSON into the Stage 2.36 verifier
- returns the same compact verification result as direct Stage 2.36 verifier usage
- preserves relativePath
- preserves stableJson
- records bytesRead
- returns ok: true
- preserves bundleArtifactType
- preserves stageRange
- preserves runtimeCommit
- preserves digestHex
- preserves reportCount
- preserves firstRunId
- preserves lastRunId
- preserves checkpointCreatedAtIso
- preserves bundleCreatedAtIso
- boolean verification check returns true
- stableJson does not contain secret-bearing fields

Secret-bearing field checks:

- secretKey is absent
- guardianSigners is absent
- privateKey is absent
- ANCHOR_WALLET is absent
- wallet.json is absent

## Tampered bundle verification test

Confirmed behavior:

- wrong artifactType returns Stage 2 verifier failure result
- digest mismatch returns Stage 2 verifier failure result
- checkpoint mismatch returns Stage 2 verifier failure result
- boolean verification check returns false for failed verifier result

Failure reasons covered:

- invalid_artifact_type
- digest_mismatch
- checkpoint_mismatch

## Invalid file / invalid verifier timestamp test

Confirmed behavior:

- invalid verifiedAtIso is rejected before verification result creation
- invalid JSON file is rejected by Stage 3.1 file IO read
- boolean verification check returns false for invalid JSON
- path escape is rejected by Stage 3.1 file IO read

Failure modes covered:

- invalid_verified_at_iso
- invalid_json
- path_escape

## Stage 3.4 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_audit_bundle_verifier_boundary.test.ts

Result:

    Stage 3.4 audit bundle verifier boundary
      ✔ verifies an exported Stage 2 audit bundle file through Stage 3 file IO
      ✔ returns Stage 2 verifier failure results for tampered bundle files
      ✔ rejects invalid files and invalid verifier timestamps

    3 passing

## Stage 3.1 plus Stage 3.3 plus Stage 3.4 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts

Result:

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    Stage 3.3 audit bundle export boundary
      ✔ exports and reads a Stage 2 audit export bundle through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for audit bundle exports
      ✔ rejects malformed audit bundles and invalid files

    Stage 3.4 audit bundle verifier boundary
      ✔ verifies an exported Stage 2 audit bundle file through Stage 3 file IO
      ✔ returns Stage 2 verifier failure results for tampered bundle files
      ✔ rejects invalid files and invalid verifier timestamps

    9 passing

## Stage 2.36 plus Stage 3.3 plus Stage 3.4 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage2_operator_audit_bundle_verifier_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts

Result:

    Stage 2.36 operator audit bundle verifier boundary
      ✔ verifies a serialized operator audit export bundle and returns compact summary
      ✔ returns verifier failure results for invalid or tampered serialized bundles
      ✔ rejects invalid verifier timestamps before parsing bundle input

    Stage 3.3 audit bundle export boundary
      ✔ exports and reads a Stage 2 audit export bundle through Stage 3 file IO
      ✔ rejects accidental overwrite and allows explicit overwrite for audit bundle exports
      ✔ rejects malformed audit bundles and invalid files

    Stage 3.4 audit bundle verifier boundary
      ✔ verifies an exported Stage 2 audit bundle file through Stage 3 file IO
      ✔ returns Stage 2 verifier failure results for tampered bundle files
      ✔ rejects invalid files and invalid verifier timestamps

    9 passing

## Stage 3.1 plus Stage 3.2 plus Stage 3.3 plus Stage 3.4 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts \
      tests/stage3_operator_report_export_boundary.test.ts \
      tests/stage3_audit_bundle_export_boundary.test.ts \
      tests/stage3_audit_bundle_verifier_boundary.test.ts

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

    12 passing

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

Stage 3.4 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.4 uses only:

- Stage 3.1 local file IO read
- Stage 3.3 audit bundle export helper for test setup
- Stage 2.36 serialized audit bundle verifier
- local temporary test directories
- local JSON serialization / verification

Therefore Stage 3.4 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.4 establishes the audit bundle verifier boundary.

It proves that an exported audit bundle JSON file can be read through Stage 3.1 file IO and verified through the already-proven Stage 2.36 serialized audit bundle verifier model.

This becomes the foundation for later verifier CLI commands, verification receipt generation, and production workflow packaging.

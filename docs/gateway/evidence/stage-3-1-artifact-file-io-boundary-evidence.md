# Stage 3.1 Artifact File IO Boundary Evidence

This document records Stage 3.1 artifact file IO boundary evidence for the X1 direct mint gateway tooling / production surface.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-3-1-artifact-file-io-boundary

Runtime commit:

    c307ffd Add Stage 3.1 artifact file IO boundary

Base runtime commit:

    7cbbeb3 Add Stage 2.38 final evidence index closure boundary

## Stage transition

Stage 2 is closed.

Stage 2 covered the model / evidence layer:

- watcher
- relayer
- operator report
- audit log
- digest
- checkpoint
- export bundle
- verifier
- verification receipt
- final evidence index closure

Stage 3 begins the tooling / production surface.

Stage 3.1 is the first Stage 3 boundary.

## Scope

Stage 3.1 adds a local artifact file IO boundary.

It defines how tooling can safely and deterministically write and read JSON artifacts from disk.

It is not a CLI command yet.

It is not a live RPC workflow.

It does not submit transactions.

It does not spend gas or SOL.

It is fully offline / zero-SOL.

## Runtime changes

New helper:

    tests/helpers/stage3ArtifactFileIoPrototype.ts

New test:

    tests/stage3_artifact_file_io_boundary.test.ts

## New artifact file IO model

New JSON object type:

    Stage3JsonObject

New JSON value type:

    Stage3JsonValue

Supported values:

- null
- string
- finite number
- boolean
- JSON-compatible arrays
- JSON-compatible objects

Unsupported values are rejected, including:

- undefined
- non-finite numbers
- functions
- non-object artifact roots

## New error model

New error class:

    Stage3ArtifactFileIoError

New error reason type:

    Stage3ArtifactFileIoErrorReason

Error reasons:

- invalid_root_dir
- invalid_artifact_path
- path_escape
- invalid_artifact
- artifact_exists
- invalid_json

## New path model

New path resolution result:

    Stage3ArtifactFilePathResolution

Fields:

    rootDir
    relativePath
    artifactPath

Path rules:

- rootDir must be a non-empty string
- relativePath must be a non-empty relative path
- absolute artifact paths are rejected
- path escapes outside the root directory are rejected
- artifact path must point to a .json file
- normalized relative path is returned

## New write model

New write result:

    Stage3ArtifactFileWriteResult

Fields:

    rootDir
    relativePath
    artifactPath
    bytesWritten
    stableJson

Write behavior:

- creates parent directories recursively
- writes stable pretty JSON
- appends a trailing newline
- rejects accidental overwrite by default
- allows overwrite only when overwrite: true is explicitly passed

## New read model

New read result:

    Stage3ArtifactFileReadResult

Fields:

    rootDir
    relativePath
    artifactPath
    bytesRead
    stableJson
    artifact

Read behavior:

- reads UTF-8 JSON from disk
- rejects invalid JSON
- rejects non-object artifact root
- rejects non-JSON-compatible values
- returns parsed artifact and original stableJson bytes

## New helpers

Path resolution helper:

    resolveStage3ArtifactFilePathPrototype

Serialization helper:

    serializeStage3ArtifactFileJsonPrototype

Write helper:

    writeStage3ArtifactFilePrototype

Read helper:

    readStage3ArtifactFilePrototype

Round-trip verifier:

    verifyStage3ArtifactFileRoundTripPrototype

## Stable JSON format

Stage 3.1 uses stable pretty JSON formatting:

    JSON.stringify(artifact, null, 2) + "\n"

This creates deterministic artifact files for later CLI / export / verify workflows.

## Successful write / read test

Confirmed behavior:

- writes a JSON-compatible artifact to disk
- creates nested artifact directories
- preserves stable pretty JSON
- appends trailing newline
- records bytesWritten
- reads the same artifact back
- records bytesRead
- parsed artifact equals original artifact
- round-trip verification returns true

## Accidental overwrite rejection test

Confirmed behavior:

- first write succeeds
- second write to the same path without overwrite fails
- failure reason is artifact_exists
- explicit overwrite succeeds when overwrite: true is passed
- overwritten artifact reads back correctly

## Invalid path / invalid artifact / invalid JSON rejection test

Confirmed behavior:

- path escape using ../escape.json is rejected
- absolute path is rejected
- non-.json artifact path is rejected
- artifact containing undefined is rejected
- invalid JSON file is rejected during read
- round-trip verification returns false for invalid JSON

## Stage 3.1 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_artifact_file_io_boundary.test.ts

Result:

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    3 passing

## Stage 2.38 plus Stage 3.1 boundary smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage2_final_evidence_index_closure_boundary.test.ts \
      tests/stage3_artifact_file_io_boundary.test.ts

Result:

    Stage 2.38 final evidence index closure boundary
      ✔ creates a stable final evidence index covering Stage 2.22 through Stage 2.38
      ✔ rejects missing or duplicate stages in the final evidence index
      ✔ rejects malformed final evidence index metadata and entries

    Stage 3.1 artifact file IO boundary
      ✔ writes and reads a stable JSON artifact without live RPC
      ✔ rejects accidental overwrite unless overwrite is explicit
      ✔ rejects path escapes, invalid artifact paths, invalid artifacts, and invalid JSON

    6 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Suspicious pasted terminal fragment check:

    clean

No secret-like material was found in the new Stage 3.1 files.

## Zero-SOL boundary

Stage 3.1 does not use:

- ANCHOR_PROVIDER_URL
- ANCHOR_WALLET
- RPC calls
- X1 program calls
- Solana transactions
- mint operations
- submit operations
- import operations
- live relayer execution

Stage 3.1 uses only local file system operations:

- mkdtemp
- writeFile
- readFile
- rm
- mkdir

Therefore Stage 3.1 is offline / zero-SOL and cannot spend gas or SOL.

## Current conclusion

Stage 3.1 establishes the first production/tooling boundary after Stage 2 closure.

It provides a safe deterministic file IO layer for future Stage 3 commands such as artifact export, verifier CLI, receipt creation, and operator workflow packaging.

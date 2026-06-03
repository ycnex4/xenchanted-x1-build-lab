# File Snapshot Storage Notes

## Branch

file-snapshot-storage

## Purpose

This branch adds the first real file snapshot storage adapter for the xEnchanted X1 Build Lab.

It builds on the existing storage serialization layer and stores the full application state as a JSON snapshot.

## Implemented files

- src/storage/snapshot.ts
- tests/storage-snapshot.test.ts

Updated:

- src/index.ts

## Implemented snapshot type

The branch adds SerializedBuildApplicationSnapshot.

The snapshot includes:

- schemaVersion
- kind
- createdAt
- registry
- registrar
- redeemEvents
- xenBurnEvents

## Implemented snapshot helpers

The branch adds:

- serializeBuildApplicationSnapshot
- deserializeBuildApplicationSnapshot
- encodeSnapshotJson
- decodeSnapshotJson
- saveSnapshotFile
- loadSnapshotFile

## File write policy

saveSnapshotFile writes through a temporary file and then renames it to the target path.

This provides a simple atomic-write pattern for local filesystem snapshots.

The helper also creates the target directory recursively.

## JSON policy

Snapshots are written as pretty JSON with a trailing newline.

BigInt values remain encoded through the existing decimal string serialization policy.

## Test coverage

Added test file:

- tests/storage-snapshot.test.ts

Covered cases:

- full application snapshot round-trip
- snapshot JSON encode / decode
- snapshot file save / load
- snapshot kind validation
- snapshot schema version validation

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 22 test files passed
- 121 tests passed

## Current known exclusions

This milestone does not implement:

- database storage
- concurrent write locks
- backup rotation
- snapshot migration framework
- corrupted file recovery
- production encryption
- API / CLI snapshot commands
- watcher persistence integration

## Main invariant

Snapshot storage must preserve the full application state without changing accounting meaning.

The storage layer must not create, remove, or reinterpret protocol state.

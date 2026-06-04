# Snapshot Verification / Backup Implementation Notes

## Branch

snapshot-verification-backup

## Purpose

This milestone implements the first snapshot verification and backup-enabled save helpers.

It follows the previously documented snapshot migration / backup policy.

## Added / updated files

Updated:

- `src/storage/snapshot.ts`
- `tests/storage-snapshot.test.ts`

Added notes:

- `implementation/snapshot-verification-backup-notes.md`

## Implemented helpers

The snapshot storage layer now includes:

- `verifySnapshotJson(json)`
- `verifySnapshotFile(filePath)`
- `saveSnapshotFileWithBackup(filePath, app, createdAt, options?)`

## Verification behavior

`verifySnapshotJson` decodes and validates snapshot JSON using the existing snapshot decoding path.

It rejects:

- invalid JSON
- invalid snapshot kind
- unsupported schema version
- invalid bigint strings
- invalid nested serialized state
- duplicate replay / registry entries through existing deserialization checks

`verifySnapshotFile` reads a file and runs the same JSON verification path.

## Backup-enabled save behavior

`saveSnapshotFileWithBackup`:

1. serializes the new application snapshot
2. writes it to a temporary file
3. verifies the temporary file
4. verifies the existing canonical snapshot if it exists
5. copies the existing canonical snapshot to a backup path
6. renames the verified temporary file to the canonical path
7. verifies the new canonical snapshot
8. removes the temporary file if an error occurs

Default backup path:

- `<snapshotPath>.bak`

Custom backup path can be passed through:

- `SaveSnapshotFileWithBackupOptions.backupPath`

## Corrupted canonical policy

If an existing canonical snapshot is corrupted, backup-enabled save fails before replacement.

This preserves the corrupted canonical file for investigation and prevents turning a corrupted canonical snapshot into a trusted backup.

## Test coverage

New tests cover:

- snapshot JSON verification
- snapshot file verification
- invalid JSON rejection
- invalid schema rejection
- backup creation when canonical snapshot exists
- no backup creation when canonical snapshot does not exist
- corrupted canonical snapshot rejection before replacement

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 159 tests passed

## Architectural boundary

This milestone does not add migration functions.

This milestone does not add restore helpers.

This milestone does not add CLI mutation commands.

The storage layer remains accounting-preserving only and does not create, remove, or reinterpret protocol state.

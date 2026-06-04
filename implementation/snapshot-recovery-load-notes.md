# Snapshot Recovery Load Notes

## Branch

snapshot-recovery-load

## Purpose

This milestone adds a recovery-oriented snapshot load helper.

It follows the previously documented snapshot migration / backup policy and builds on the snapshot verification / backup implementation milestone.

## Added / updated files

Updated:

- `src/storage/snapshot.ts`
- `tests/storage-snapshot.test.ts`

Added notes:

- `implementation/snapshot-recovery-load-notes.md`

## Implemented helper

The snapshot storage layer now includes:

- `loadSnapshotFileWithRecovery(filePath, options?)`

Related types:

- `LoadSnapshotFileWithRecoveryOptions`
- `LoadSnapshotFileWithRecoveryResult`

## Recovery behavior

The helper first tries to load the canonical snapshot.

If the canonical snapshot loads successfully, the result includes:

- `source: "canonical"`
- `filePath: <canonicalPath>`

If the canonical snapshot is missing or invalid, the helper tries the backup snapshot.

Default backup path:

- `<snapshotPath>.bak`

A custom backup path can be passed through:

- `LoadSnapshotFileWithRecoveryOptions.backupPath`

If the backup snapshot loads successfully, the result includes:

- `source: "backup"`
- `filePath: <backupPath>`

If both canonical and backup snapshots fail to load, the helper throws an error containing both failure contexts.

## Important boundary

This helper does not repair the canonical snapshot.

It does not overwrite the canonical file from backup.

It does not delete corrupted files.

It only performs recovery loading and reports which snapshot source was used.

## Test coverage

New tests cover:

- loading canonical snapshot when canonical is valid
- loading backup snapshot when canonical is corrupted
- custom backup path support
- rejection when canonical and backup are both invalid

The backup-load test also verifies that the corrupted canonical file remains unchanged.

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 163 tests passed

## Architectural boundary

This milestone does not add migration functions.

This milestone does not add automatic restore / repair behavior.

This milestone does not add CLI snapshot recovery commands.

The storage layer remains accounting-preserving only and does not create, remove, or reinterpret protocol state.

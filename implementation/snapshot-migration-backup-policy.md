# Snapshot Migration / Backup Policy

## Branch

snapshot-migration-backup-policy

## Purpose

This milestone defines the migration, backup, restore, and recovery policy for Build application snapshots.

This is a documentation-only milestone.

No TypeScript model logic is changed in this branch.

## Current implementation baseline

The current snapshot implementation already supports:

- full `BuildApplicationState` snapshot serialization
- snapshot schema version validation
- snapshot kind validation
- JSON encode / decode helpers
- local file save / load helpers
- temporary file write before final rename
- pretty JSON output with trailing newline

Current snapshot files include:

- `src/storage/snapshot.ts`
- `tests/storage-snapshot.test.ts`

Current snapshot shape:

- `schemaVersion`
- `kind`
- `createdAt`
- `registry`
- `registrar`
- `redeemEvents`
- `xenBurnEvents`

## Core invariant

Snapshot storage must preserve application state.

Snapshot storage must not create, remove, or reinterpret accounting values.

Snapshot migration must not bypass:

- Build uniqueness rules
- registrar replay protection
- Core redeem replay protection
- XEN burn replay protection

A loaded or migrated snapshot must preserve the same accounting meaning as the original state.

## Schema version policy

Every persisted snapshot must include:

- `schemaVersion`
- `kind`

The current schema version is `1`.

Unsupported schema versions must be rejected by default.

A future migration framework may explicitly accept older versions only through named migration functions.

There should be no silent fallback for unknown versions.

## Migration policy

Any storage schema change must include:

- old schema description
- new schema description
- migration function
- migration tests
- rollback notes where practical
- compatibility notes for existing snapshots

Migration functions must be explicit.

Example future naming:

- `migrateSnapshotV1ToV2`
- `migrateSnapshotV2ToV3`

A migration must not silently drop replay protection fields.

If a field cannot be migrated safely, the migration must fail.

## Migration test policy

Each migration must test:

- successful migration from the old schema
- rejection of invalid old schema data
- replay protection preservation
- bigint decimal string preservation
- registry uniqueness preservation
- idempotent behavior expectations, if applicable

Migration tests should include at least one non-empty application state with:

- one Build
- one processed registrar message
- one used Core redeem event
- one used XEN burn event

## Backup policy

Before replacing the canonical snapshot, the storage layer should preserve a backup of the previous valid snapshot.

Recommended local-file layout:

- `snapshot.json` — canonical snapshot
- `snapshot.json.bak` — previous canonical snapshot
- optional `snapshots/archive/<timestamp>.json` — timestamped snapshots

For MVP, one previous backup is enough.

For production-like use, timestamped backup rotation should be considered.

## Backup creation policy

A backup should be created only from the current canonical snapshot.

A corrupted temporary file must never become the backup.

Recommended write flow:

1. encode new snapshot JSON
2. write new snapshot to temp file
3. validate temp file by decoding it
4. if canonical snapshot exists, copy or rename it to backup
5. rename temp file to canonical path
6. optionally verify canonical snapshot after rename

## Atomic write policy

The existing `saveSnapshotFile` already writes to a temporary file and renames it to the target path.

Future backup-enabled save helpers should preserve this property.

A partial write must not become canonical state.

A failed write should leave either:

- the previous canonical snapshot available, or
- a valid backup available

## Restore policy

Default restore should load the canonical snapshot.

If canonical snapshot loading fails, recovery tooling may try a backup snapshot.

Automatic production restore from backup should be conservative.

Recommended behavior:

- normal load: canonical only
- recovery load: canonical first, backup second
- if both fail: return structured failure

Recovery must report which file was loaded.

Silent backup fallback can hide corruption and should be avoided in normal operation.

## Corrupted snapshot policy

Corrupted snapshots should be rejected.

Examples:

- invalid JSON
- unsupported schema version
- invalid kind
- invalid bigint decimal strings
- duplicate set entries
- duplicate registry keys
- missing replay protection fields

Recovery tooling may keep corrupted files for investigation.

It should not delete corrupted files automatically unless explicitly requested.

## Replay protection policy

The following fields are critical and must be preserved across save, load, migration, backup, and restore:

- `registrar.processedMessages`
- `redeemEvents.usedRedeemEvents`
- `xenBurnEvents.usedXenBurnEvents`
- `registry.canonicalBuildByOwner`
- `registry.canonicalBuildByEthereumIdentity`
- `registry.buildsById`

Losing replay protection is a correctness failure.

## Snapshot timestamp policy

`createdAt` is snapshot metadata.

It is not the same as Build creation time or event time.

It should be encoded as a bigint decimal string under the current storage policy.

Future backup filenames may use wall-clock timestamps, but snapshot metadata should remain deterministic and explicit.

## Concurrency policy

The current MVP does not support concurrent writers.

Future production-like storage should add a write lock or single-writer process policy before concurrent commands are enabled.

Until then, snapshot write operations should be treated as single-writer only.

## CLI / API policy

Future CLI snapshot commands should be explicit and safe.

Potential future commands:

- `snapshot:show`
- `snapshot:verify`
- `snapshot:save`
- `snapshot:backup`
- `snapshot:restore`
- `snapshot:migrate`

Commands that replace canonical state should require explicit file paths and should report backup behavior.

## Security policy

Snapshots may contain user identities and protocol state.

Snapshot commands should not print secrets.

If future snapshots include private keys, tokens, RPC credentials, or signer material, this would be a design error.

Such secrets should not be stored in application snapshots.

## Current known exclusions

This milestone does not implement:

- migration functions
- backup-enabled save helper
- restore helper
- corrupted snapshot recovery helper
- concurrent write lock
- CLI mutation commands
- database storage
- encryption

## Recommended next implementation order

1. Add snapshot verification helper.
2. Add backup-enabled local snapshot save helper.
3. Add recovery load helper for canonical + backup.
4. Add migration function framework when schema version 2 is needed.
5. Add CLI commands only after storage behavior is tested.

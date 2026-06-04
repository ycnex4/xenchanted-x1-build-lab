# CLI Snapshot Recovery Command Notes

## Branch

cli-snapshot-recovery-command

## Purpose

This milestone adds a read-only CLI command for snapshot recovery loading.

It builds on the existing recovery helper:

- `loadSnapshotFileWithRecovery`

## Added / updated files

Updated:

- `src/cli/commands.ts`
- `tests/cli-commands.test.ts`

Added notes:

- `implementation/cli-snapshot-recovery-command-notes.md`

## Implemented command

The CLI now supports:

- `snapshot:recover --file <path> [--backup <path>]`

## Behavior

The command first tries to load the canonical snapshot from `--file`.

If the canonical snapshot is valid, the command returns exit code `0` and writes a JSON summary with:

- `recovered: true`
- `source: "canonical"`
- `filePath: <canonicalPath>`

If the canonical snapshot is missing or invalid, the command tries the backup snapshot.

Default backup path:

- `<snapshotPath>.bak`

Custom backup path:

- `--backup <path>`

If the backup snapshot is valid, the command returns exit code `0` and writes a JSON summary with:

- `recovered: true`
- `source: "backup"`
- `filePath: <backupPath>`

If both canonical and backup snapshots fail, the command returns exit code `1` and writes the recovery error to stderr.

## JSON summary fields

Successful recovery output includes:

- `recovered`
- `source`
- `filePath`
- `createdAt`
- `buildCount`
- `registrarAuthority`
- `processedMessageCount`
- `usedRedeemEventCount`
- `usedXenBurnEventCount`

## Help output

CLI help now includes:

- `snapshot:recover --file <path> [--backup <path>]`

## Test coverage

New tests cover:

- canonical snapshot recovery
- backup snapshot recovery when canonical is invalid
- custom backup path recovery
- missing `--file` structured failure
- recovery failure when canonical and backup are both invalid
- help output includes the new command

The backup recovery test also verifies that the corrupted canonical file remains unchanged.

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- node ./dist/src/cli/main.js help: passed
- 28 test files passed
- 171 tests passed

## Architectural boundary

This command is read-only.

It does not repair the canonical snapshot.

It does not copy backup into canonical.

It does not delete corrupted files.

It does not create backups.

It does not migrate snapshot files.

It only attempts recovery loading and reports which source was used.

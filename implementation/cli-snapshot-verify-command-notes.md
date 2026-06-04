# CLI Snapshot Verify Command Notes

## Branch

cli-snapshot-verify-command

## Purpose

This milestone adds a read-only CLI command for snapshot verification.

It builds on the existing snapshot verification helper:

- `verifySnapshotFile`

## Added / updated files

Updated:

- `src/cli/commands.ts`
- `tests/cli-commands.test.ts`

Added notes:

- `implementation/cli-snapshot-verify-command-notes.md`

## Implemented command

The CLI now supports:

- `snapshot:verify --file <path>`

## Behavior

For a valid snapshot file, the command returns exit code `0` and writes a JSON summary to stdout.

Summary fields:

- `valid`
- `createdAt`
- `buildCount`
- `registrarAuthority`
- `processedMessageCount`
- `usedRedeemEventCount`
- `usedXenBurnEventCount`

For a missing `--file` flag, the command returns exit code `1` and writes a structured error to stderr.

For an invalid snapshot file, the command returns exit code `1` and writes the verification error to stderr.

## Help output

CLI help now includes:

- `snapshot:verify --file <path>`

## Test coverage

New tests cover:

- successful snapshot verification
- missing `--file` flag
- invalid snapshot verification failure
- help output includes the new command

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 28 test files passed
- 166 tests passed

## Architectural boundary

This command is read-only.

It does not mutate snapshots.

It does not create backups.

It does not recover from backups.

It does not migrate snapshot files.

It only validates an existing snapshot and reports a summary.

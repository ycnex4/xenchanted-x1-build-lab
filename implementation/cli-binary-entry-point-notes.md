# CLI Binary Entry Point Notes

## Branch

cli-binary-entry-point

## Purpose

This milestone adds a real executable CLI entry point for the existing read-only CLI command layer.

The CLI command layer already supported:

- `help`
- `version`
- `snapshot:show --file <path>`

This milestone connects that command layer to a Node.js binary entry point.

## Added files

- `src/cli/main.ts`
- `tests/cli-main.test.ts`

## Updated files

- `package.json`

## Implemented entry point

The new `src/cli/main.ts` file adds:

- executable shebang
- `runCliMain(args, io)` helper
- stdout / stderr writing based on `CliCommandResult`
- process argv handling for direct execution
- process exit code assignment
- top-level failure handling for unexpected CLI errors

## Package configuration

`package.json` now includes:

- `build` script
- `cli` script
- `bin` entry

Current binary name:

- `x1-build-lab`

Current compiled binary target:

- `./dist/src/cli/main.js`

This path matches the current TypeScript output layout.

## Testing policy

The binary entry layer is tested without spawning a shell process.

Tests call `runCliMain(args, io)` with memory-backed stdout and stderr writers.

Covered cases:

- help output writes to stdout and returns exit code 0
- version output writes to stdout and returns exit code 0
- unknown command writes to stderr and returns exit code 1

## Manual verification

The compiled CLI was manually verified with:

- `npm run build`
- `node ./dist/src/cli/main.js version`
- `node ./dist/src/cli/main.js help`

## Current validation result

After the code commit:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- 27 test files passed
- 152 tests passed

## Architectural boundary

This milestone does not add new mutable commands.

The CLI remains intentionally minimal and read-only.

State mutation commands should be added only after the corresponding storage, backup, restore, and safety policies are implemented and tested.

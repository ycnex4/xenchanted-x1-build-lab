# CLI Command Implementation Notes

## Branch

cli-command-implementation

## Purpose

This branch adds the first minimal CLI command layer for the xEnchanted X1 Build Lab.

The CLI layer is intentionally small and dependency-free.

It currently supports read-only / utility commands and does not mutate protocol state.

## Implemented files

- src/cli/parse.ts
- src/cli/commands.ts
- tests/cli-commands.test.ts

Updated:

- src/index.ts

## Implemented CLI parser

The branch adds parseCliArgs.

It supports:

- command name
- positional arguments
- boolean flags
- string flags using --flag value
- string flags using --flag=value

## Implemented CLI helpers

The branch adds:

- getStringFlag
- renderCliHelp
- runCliCommand

## Implemented commands

The branch adds support for:

- help
- version
- snapshot:show --file <path>

## Snapshot summary command

snapshot:show reads a local snapshot file using the file snapshot storage adapter and prints a JSON summary.

The summary includes:

- createdAt
- buildCount
- registrarAuthority
- processedMessageCount
- usedRedeemEventCount
- usedXenBurnEventCount

## Error behavior

CLI commands return structured command results:

- exitCode
- stdout
- stderr

Unknown commands and missing required flags return exitCode 1.

## State mutation policy

This milestone does not add any mutation commands.

The CLI does not create Builds, apply registrar messages, or change snapshots yet.

This keeps the first CLI implementation safe and read-only.

## Test coverage

Added test file:

- tests/cli-commands.test.ts

Covered cases:

- command / positional / flag parsing
- help command
- version command
- snapshot:show command
- missing --file failure
- unknown command failure

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 23 test files passed
- 127 tests passed

## Current known exclusions

This milestone does not implement:

- CLI binary entry point
- package.json bin field
- mutation commands
- snapshot write commands
- registrar command submission
- proof command submission
- watcher commands
- authentication
- interactive prompts

## Main invariant

The CLI layer should expose existing application and storage behavior.

It must not become a second source of protocol logic.

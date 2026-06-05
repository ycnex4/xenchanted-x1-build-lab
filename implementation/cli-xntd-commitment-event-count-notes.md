# CLI XNTD Commitment Event Count Notes

## Branch

cli-xntd-commitment-event-count

## Purpose

This milestone updates the read-only CLI snapshot summaries to expose the XNTD commitment event replay count.

## Scope

This is a CLI visibility-only change.

It does not change:

- protocol state transitions
- registrar behavior
- proof submission behavior
- watcher behavior
- snapshot serialization
- recovery behavior

## Updated CLI commands

The following commands now include:

- usedXntdCommitmentEventCount

Updated commands:

- snapshot:show
- snapshot:verify
- snapshot:recover

## Reason

The application snapshot now persists:

- xntdCommitmentEvents
- usedXntdCommitmentEvents

The CLI already exposed:

- processedMessageCount
- usedRedeemEventCount
- usedXenBurnEventCount

Adding usedXntdCommitmentEventCount keeps CLI snapshot visibility aligned with the runtime replay-state model.

## Tests

Updated:

- tests/cli-commands.test.ts

Covered behavior:

- snapshot:show includes usedXntdCommitmentEventCount
- snapshot:verify includes usedXntdCommitmentEventCount
- snapshot:recover includes usedXntdCommitmentEventCount

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 177 tests passed

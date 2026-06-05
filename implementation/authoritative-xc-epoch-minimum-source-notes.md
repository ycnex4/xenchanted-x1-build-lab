# Authoritative XC Epoch Minimum Source Notes

## Branch

authoritative-xc-epoch-minimum-source

## Purpose

This milestone introduces a deterministic local source / helper for authoritative XC epoch minimum validation.

It is the first runtime building block for validating:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

This milestone does not connect the validator to the registrar handler yet.

## Runtime additions

Added:

- src/model/xc-epoch-minimum-source.ts

The file defines:

- XcEpochMinimumSource
- createStaticXcEpochMinimumSource()
- assertAuthoritativeXcEpochMinimum()

## Error codes added

Added:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

These errors are intentionally separate from InvalidXntdLockAmount.

Reason:

- InvalidXntdLockAmount covers invalid amount relationships
- MissingAuthoritativeXcEpochMinimum covers unknown / unavailable authoritative source state
- MismatchedAuthoritativeXcEpochMinimum covers economically incorrect observed required lock value

## Helper behavior

assertAuthoritativeXcEpochMinimum(source, lockEpoch, observedRequiredXntdLock) does:

1. Reads authoritativeEpochMinimum(lockEpoch) from the source.
2. Rejects if the source returns null.
3. Rejects if observedRequiredXntdLock does not equal the authoritative minimum.
4. Accepts if the observed value matches the authoritative minimum.

## Test coverage

Added:

- tests/xc-epoch-minimum-source.test.ts

Covered cases:

- matching observed required XNTD lock is accepted
- missing authoritative epoch minimum is rejected
- mismatched observed required XNTD lock is rejected

## Scope boundary

This milestone does not change:

- registrar XNTD lock / relock handlers
- app proof submission
- app service
- watcher candidates
- proof payloads
- registrar payloads
- snapshot schema
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

## Next step

The next runtime layer should connect assertAuthoritativeXcEpochMinimum() to the registrar XNTD lock / relock boundary.

The validator should run before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Rejected authoritative validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 189 tests passed

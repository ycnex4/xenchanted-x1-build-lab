# Authoritative XC Epoch Minimum Registrar Validation Notes

## Branch

authoritative-xc-epoch-minimum-registrar-validation

## Purpose

This milestone connects authoritative XC epoch minimum validation to the registrar XNTD lock / relock boundary.

It is the first registrar-level runtime integration of:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Runtime change

Updated:

- src/instructions/registrar-xntd-lock.ts

Added optional input field to:

- ApplyRegistrarXntdLockInput
- ApplyRegistrarXntdRelockInput

Field:

- xcEpochMinimumSource?: XcEpochMinimumSource

## Validation behavior

When xcEpochMinimumSource is provided, applyRegistrarXntdLock() and applyRegistrarXntdRelock() call:

assertAuthoritativeXcEpochMinimum(
  xcEpochMinimumSource,
  lockEpoch,
  observedRequiredXntdLock
)

This validates:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Compatibility behavior

xcEpochMinimumSource is optional in this milestone.

Reason:

- app service / proof submission / e2e call sites are not updated yet
- the registrar boundary can support authoritative validation without forcing the whole app stack to change in the same branch
- a later layer can decide how to pass the source through app service and proof submission

## Mutation safety

Authoritative XC epoch minimum validation runs before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd() / relockXntd()

Therefore rejected authoritative validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

## Test coverage

Updated:

- tests/registrar-xntd-lock.test.ts

Added coverage:

- LOCK_XNTD accepts when observedRequiredXntdLock matches authoritative epoch minimum
- LOCK_XNTD rejects when observedRequiredXntdLock mismatches authoritative epoch minimum
- LOCK_XNTD rejects when authoritative epoch minimum is missing
- rejection does not mutate registrar processed messages
- rejection does not mark XNTD commitment event keys
- rejection does not mutate Build lock state

## Scope boundary

This milestone does not update:

- app service source injection
- proof submission source injection
- e2e source injection
- snapshot schema
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration

## Next step

The next layer should decide how to pass xcEpochMinimumSource above the registrar instruction boundary.

Likely options:

1. Add explicit source argument to appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock().
2. Add explicit source argument to appSubmitProof().
3. Later decide whether BuildApplicationState should own a source provider.

Do not persist the source in snapshots yet.

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 192 tests passed

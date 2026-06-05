# XNTD Observed Required Lock Low-Level Notes

## Branch

xntd-observed-required-lock-low-level

## Purpose

This milestone implements the first runtime layer for observedRequiredXntdLock.

It updates only the low-level XNTD lock / relock primitives and the direct low-level tests.

## Runtime change

Before this milestone, low-level lockXntd() / relockXntd() used:

lockedXntd = amountXntd
requiredXntdLock = amountXntd

After this milestone, they use:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

## New input field

Added to:

- LockXntdInput
- RelockXntdInput

Field:

observedRequiredXntdLock

Meaning:

- amountXntd is the actual user locked / relocked amount
- observedRequiredXntdLock is the observed requirement for the selected lockEpoch
- requiredXntdLock is the Build state value recorded after validation

## New low-level validation

The low-level primitives now validate:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

The low-level primitives do not validate:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

That remains a future registrar / integration validation layer.

## Registrar compatibility

Registrar XNTD lock / relock handlers were updated to preserve current MVP behavior by passing:

observedRequiredXntdLock = amountXntd

This keeps the existing watcher / proof / registrar payload chain unchanged for now.

## Tests updated

Updated:

- tests/xntd-lock-relock.test.ts
- tests/registrar-x1-fee-checkpoint.test.ts
- tests/x1-fee-contribution.test.ts

Added low-level test coverage for:

- lock with amountXntd > observedRequiredXntdLock
- relock with amountXntd > observedRequiredXntdLock
- observedRequiredXntdLock = 0 rejection
- amountXntd < observedRequiredXntdLock lock rejection
- amountXntd < observedRequiredXntdLock relock rejection
- rejected invalid lock/relock does not mutate Build state

## Scope boundary

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- registrar payload builders
- proof submission payload shape
- snapshot schema
- CLI output
- authoritative XC state validation

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 184 tests passed

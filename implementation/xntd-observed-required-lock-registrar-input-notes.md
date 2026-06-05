# XNTD Observed Required Lock Registrar Input Notes

## Branch

xntd-observed-required-lock-registrar-input

## Purpose

This milestone lifts observedRequiredXntdLock from the low-level XNTD lock / relock primitives into the registrar input layer.

It is the second runtime layer of the observedRequiredXntdLock rollout.

## Runtime change

Added observedRequiredXntdLock to:

- ApplyRegistrarXntdLockInput
- ApplyRegistrarXntdRelockInput

Registrar handlers now pass:

observedRequiredXntdLock = input.observedRequiredXntdLock

into:

- lockXntd()
- relockXntd()

## Registrar validation

Registrar handlers now validate:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

This is still not authoritative XC state validation.

The following production validation remains future work:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Mutation safety

The registrar validation is performed before:

- acceptRegistrarMessage()
- acceptXntdCommitmentEvent()
- lockXntd()
- relockXntd()

Rejected under-lock cases do not mark:

- registrar message
- XNTD commitment event key

and do not mutate Build state.

## Compatibility layer

The proof / watcher payload chain is not updated in this milestone.

To preserve current MVP behavior:

- appApplyRegistrarXntdLock() accepts optional observedRequiredXntdLock
- appApplyRegistrarXntdRelock() accepts optional observedRequiredXntdLock
- when omitted, both default observedRequiredXntdLock to amountXntd
- proof-submission passes observedRequiredXntdLock = amountXntd for now

This keeps existing proof payloads working until a later milestone updates proof / watcher payload shapes.

## Tests updated

Updated:

- tests/registrar-xntd-lock.test.ts
- tests/app-build-service.test.ts
- tests/e2e-scenario.test.ts

Added registrar-level coverage for:

- LOCK_XNTD amount below observedRequiredXntdLock rejected
- RELOCK_XNTD amount below observedRequiredXntdLock rejected
- rejected under-lock does not mark registrar message
- rejected under-lock does not mark XNTD commitment event key
- rejected under-lock does not mutate Build state

## Scope boundary

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- registrar payload builder types
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
- 186 tests passed

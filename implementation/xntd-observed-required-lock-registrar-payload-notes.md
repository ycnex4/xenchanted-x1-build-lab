# XNTD Observed Required Lock Registrar Payload Notes

## Branch

xntd-observed-required-lock-registrar-payload

## Purpose

This milestone lifts observedRequiredXntdLock into the registrar payload builder layer.

It is the third runtime layer of the observedRequiredXntdLock rollout.

## Runtime change

Added observedRequiredXntdLock to:

- XntdLockRegistrarPayload
- XntdRelockRegistrarPayload

The registrar payload now carries:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

## Builder behavior

Updated:

- buildXntdLockRegistrarPayload()
- buildXntdRelockRegistrarPayload()

The builders now set:

observedRequiredXntdLock = proof.payload.observedRequiredXntdLock if present and bigint

Otherwise they fallback to:

observedRequiredXntdLock = proof.payload.amountXntd

## Compatibility reason

Proof types and watcher candidate types are not updated in this milestone.

Therefore the builder keeps compatibility with current proof payloads while preparing the registrar payload shape for future proof / watcher updates.

This means:

- old proof payloads still work
- registrar payloads now always include observedRequiredXntdLock
- proof-submission no longer invents observedRequiredXntdLock from amountXntd

## Proof submission change

Updated proof-submission so that:

LOCK_XNTD uses:

observedRequiredXntdLock = lockPayload.observedRequiredXntdLock

RELOCK_XNTD uses:

observedRequiredXntdLock = relockPayload.observedRequiredXntdLock

Before this milestone, proof-submission used:

observedRequiredXntdLock = amountXntd

directly.

Now that compatibility fallback lives in the registrar payload builder, proof-submission consumes the payload as-is.

## Tests updated

Updated:

- tests/proof-registrar-builders.test.ts

Assertions now confirm that XNTD lock / relock registrar payloads contain:

- observedRequiredXntdLock

Current expected compatibility value:

- observedRequiredXntdLock = amountXntd

## Scope boundary

This milestone does not change:

- proof types
- watcher candidate types
- watcher-to-proof conversion
- app proof submission tests
- e2e tests
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

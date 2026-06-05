# XNTD Observed Required Lock Proof Payload Notes

## Branch

xntd-observed-required-lock-proof-payload

## Purpose

This milestone lifts observedRequiredXntdLock into the XNTD proof payload layer.

It is the fourth runtime layer of the observedRequiredXntdLock rollout.

## Runtime change

Added observedRequiredXntdLock to:

- XntdLockProof.payload
- XntdRelockProof.payload

Proof payloads now carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

## Proof conversion behavior

Updated:

- convertXntdLockCandidateToProof()
- convertXntdRelockCandidateToProof()

The conversion now sets:

observedRequiredXntdLock = candidate.payload.observedRequiredXntdLock if present and bigint

Otherwise it falls back to:

observedRequiredXntdLock = candidate.payload.amountXntd

## Compatibility reason

Watcher candidate types are not updated in this milestone.

Therefore proof conversion keeps compatibility with current watcher candidates while preparing the proof payload shape for future watcher candidate updates.

## Registrar builder change

Because XntdLockProof and XntdRelockProof now include observedRequiredXntdLock, registrar builders no longer need the temporary unknown-field fallback helper.

Updated:

- buildXntdLockRegistrarPayload()
- buildXntdRelockRegistrarPayload()

They now read:

observedRequiredXntdLock = proof.payload.observedRequiredXntdLock

directly.

## Tests updated

Updated:

- tests/watcher-proof-conversion.test.ts
- tests/proof-registrar-builders.test.ts

Test coverage confirms:

- XNTD lock proof payload contains observedRequiredXntdLock
- XNTD relock proof payload contains observedRequiredXntdLock
- registrar payload can preserve separated values:
  - amountXntd > observedRequiredXntdLock

## Scope boundary

This milestone does not change:

- watcher candidate types
- watcher candidate constructors
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

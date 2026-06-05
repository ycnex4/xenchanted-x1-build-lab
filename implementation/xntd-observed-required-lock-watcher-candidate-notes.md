# XNTD Observed Required Lock Watcher Candidate Notes

## Branch

xntd-observed-required-lock-watcher-candidate

## Purpose

This milestone lifts observedRequiredXntdLock into the watcher candidate layer.

It completes the runtime propagation chain for observedRequiredXntdLock.

## Runtime change

Added observedRequiredXntdLock to:

- XntdLockCandidate.payload
- XntdRelockCandidate.payload

Watcher candidates now carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

## Constructor change

Updated:

- createXntdLockCandidate()
- createXntdRelockCandidate()

The constructors now include:

observedRequiredXntdLock = input.observedRequiredXntdLock

in candidate payloads.

## Proof conversion change

Because watcher candidates now include observedRequiredXntdLock directly, the temporary fallback helper in proof-conversion was removed.

Updated:

- convertXntdLockCandidateToProof()
- convertXntdRelockCandidateToProof()

They now read:

observedRequiredXntdLock = candidate.payload.observedRequiredXntdLock

directly.

## Full runtime propagation chain

After this milestone, observedRequiredXntdLock flows through:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> registrar input
-> low-level lock / relock
-> Build state requiredXntdLock

## Test coverage

Updated:

- tests/watcher-candidates.test.ts
- tests/watcher-proof-conversion.test.ts
- tests/app-proof-submission.test.ts
- tests/e2e-watcher-proof-registrar-scenario.test.ts

The app proof submission test now verifies separated values through the full chain:

LOCK_XNTD:

- amountXntd = 750
- observedRequiredXntdLock = 500
- lockedXntd = 750
- requiredXntdLock = 500

RELOCK_XNTD:

- amountXntd = 400
- observedRequiredXntdLock = 250
- lockedXntd = 400
- requiredXntdLock = 250

## Scope boundary

This milestone does not change:

- snapshot schema
- CLI output
- authoritative XC state validation
- XC state source integration
- proof source metadata
- event identity model

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

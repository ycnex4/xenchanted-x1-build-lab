# Observed Required XNTD Lock Chain Review Notes

## Branch

review-observed-required-xntd-lock-chain

## Purpose

This milestone reviews the completed observedRequiredXntdLock runtime propagation chain and removes the remaining app-layer compatibility fallback.

## Completed propagation chain

Before this review, observedRequiredXntdLock had already been propagated through:

- watcher candidate
- proof payload
- registrar payload
- proof submission
- registrar input
- low-level lock / relock
- Build state requiredXntdLock

## Cleanup performed

Removed the final app-service fallback in:

- src/app/build-service.ts

Before cleanup, appApplyRegistrarXntdLock() and appApplyRegistrarXntdRelock() used:

observedRequiredXntdLock = input.observedRequiredXntdLock ?? input.amountXntd

After cleanup, they use:

observedRequiredXntdLock = input.observedRequiredXntdLock

directly.

## Reason

Now that observedRequiredXntdLock is carried through the full runtime chain, app-service should not silently derive it from amountXntd.

The field must be explicit by the time execution reaches app-level registrar wrappers.

## Current runtime chain

Current explicit flow:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state requiredXntdLock

## What remains intentionally separate

This review does not implement authoritative XC validation.

Future production validation still needs:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

That belongs to the registrar / integration boundary using the authoritative XC state source.

## Historical docs note

Some implementation notes and checkpoint sections still mention earlier temporary states such as:

- requiredXntdLock = amountXntd
- observedRequiredXntdLock = amountXntd
- fallback behavior

Those historical notes are intentionally not rewritten because they describe past milestones and compatibility phases.

Current runtime behavior is represented by the latest checkpoints and current source code.

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

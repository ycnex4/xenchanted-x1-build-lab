# XNTD Epoch Minimum Docs After Runtime Chain Notes

## Branch

update-xntd-epoch-minimum-docs-after-runtime-chain

## Purpose

This documentation milestone updates active design docs after the observedRequiredXntdLock runtime propagation chain was completed.

## Context

The runtime now separates:

- amountXntd
- observedRequiredXntdLock

and records:

- lockedXntd = amountXntd
- requiredXntdLock = observedRequiredXntdLock

The runtime validates:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

## Documents updated

Updated:

- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md

## Main documentation correction

The previous docs still described the older MVP equality model:

requiredXntdLock = amountXntd

That is no longer the current runtime behavior.

The active docs now describe the current runtime behavior:

requiredXntdLock = observedRequiredXntdLock

## Still not implemented

This milestone does not implement authoritative XC validation.

The remaining production-readiness rule is still:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

That validation belongs to the registrar / integration boundary using the authoritative XC state source.

## Historical docs note

Older implementation notes and older checkpoint sections are not rewritten.

They intentionally preserve the history of earlier rollout phases.

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 186 tests passed

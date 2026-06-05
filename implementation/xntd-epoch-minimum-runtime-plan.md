# XNTD Epoch Minimum Runtime Implementation Plan

## Branch

xntd-epoch-minimum-runtime-plan

## Purpose

This document defines the planned runtime implementation sequence for XNTD lock / relock epoch minimum validation.

It does not change runtime code.

The goal is to introduce observedRequiredXntdLock safely across the full runtime chain before enforcing authoritative XC epoch minimum validation.

## Current runtime state

Current XNTD lock / relock runtime flow carries:

- amountXntd
- lockEpoch
- lockedAt / relockedAt

through:

- watcher candidates
- proof objects
- registrar payload builders
- proof submission
- registrar handlers
- low-level lockXntd() / relockXntd()

Current low-level primitives set:

lockedXntd = amountXntd
requiredXntdLock = amountXntd

This is the current MVP behavior.

## Target runtime state

Future runtime should distinguish:

- amountXntd
- observedRequiredXntdLock
- requiredXntdLock

Meaning:

amountXntd = actual amount locked / relocked by the user
observedRequiredXntdLock = requirement observed by watcher for lockEpoch
requiredXntdLock = Build state value recorded after registrar validation

After successful validation:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

## Runtime chain to update

The future implementation must update all of these layers consistently:

1. proof types
2. watcher candidate types
3. watcher candidate constructors
4. watcher-to-proof conversion
5. registrar payload types
6. registrar payload builders
7. proof submission
8. app build service wrappers
9. registrar XNTD lock / relock handlers
10. low-level lockXntd() / relockXntd()
11. tests

## Phase 1: Add observedRequiredXntdLock to payload shapes

Update:

- XntdLockCandidate
- XntdRelockCandidate
- XntdLockProof
- XntdRelockProof
- XntdLockRegistrarPayload
- XntdRelockRegistrarPayload
- ApplyRegistrarXntdLockInput
- ApplyRegistrarXntdRelockInput
- LockXntdInput
- RelockXntdInput

The field should be named:

observedRequiredXntdLock

Do not call the payload field requiredXntdLock.

Reason:

- observedRequiredXntdLock is observed by watcher / proof
- requiredXntdLock is the Build state field after validation
- the payload value is not authoritative until verified

## Phase 2: Preserve MVP semantics initially

The first runtime implementation can preserve MVP behavior by passing:

observedRequiredXntdLock = amountXntd

in existing tests and helper calls.

This keeps behavior unchanged while making the shape explicit.

Expected result:

- all current tests continue to pass after adding the field
- Build state still records requiredXntdLock equal to amountXntd in existing scenarios
- no authoritative XC validation is required yet

## Phase 3: Split low-level lock state assignment

Change lockXntd() / relockXntd() so that:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

The low-level primitive should validate:

amountXntd > 0
observedRequiredXntdLock > 0
amountXntd >= observedRequiredXntdLock

The primitive should not validate authoritative XC state.

That belongs to registrar / integration validation.

## Phase 4: Add registrar-layer validation

Registrar handlers should validate:

amountXntd > 0
observedRequiredXntdLock > 0
amountXntd >= observedRequiredXntdLock

The existing precondition order should remain:

1. message kind precondition
2. authority precondition
3. duplicate registrar message precondition
4. duplicate XNTD commitment event precondition
5. lockEpoch ordering guard
6. amount / observed required amount validation
7. relock-specific preconditions
8. acceptRegistrarMessage
9. acceptXntdCommitmentEvent
10. lockXntd / relockXntd

This preserves mutation safety.

## Phase 5: Add authoritative XC state validation later

A later production-readiness milestone should verify:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

This requires the authoritative XC state source layer.

That validation should happen in the registrar / integration boundary, not in the low-level lock primitive.

## Phase 6: Update tests

Tests should be updated in layers.

### Shape propagation tests

Update:

- watcher-candidates.test.ts
- watcher-proof-conversion.test.ts
- proof-registrar-builders.test.ts
- app-proof-submission.test.ts
- e2e-watcher-proof-registrar-scenario.test.ts

They should prove that observedRequiredXntdLock travels through:

watcher candidate -> proof -> registrar payload -> proof submission -> Build state

### Low-level lock tests

Update:

- xntd-lock-relock.test.ts

Add tests for:

- lock records requiredXntdLock from observedRequiredXntdLock
- relock records requiredXntdLock from observedRequiredXntdLock
- amountXntd < observedRequiredXntdLock is rejected
- observedRequiredXntdLock = 0 is rejected
- amountXntd = 0 remains rejected

### Registrar tests

Update:

- registrar-xntd-lock.test.ts

Add tests for:

- LOCK_XNTD under-lock rejected
- RELOCK_XNTD under-lock rejected
- rejected under-lock does not mark registrar message
- rejected under-lock does not mark XNTD commitment event key
- rejected under-lock does not mutate Build state

### E2E tests

Update:

- e2e-scenario.test.ts
- e2e-watcher-proof-registrar-scenario.test.ts
- app-build-service.test.ts

Existing tests can initially set:

observedRequiredXntdLock = amountXntd

Then add at least one positive case where:

amountXntd > observedRequiredXntdLock

to prove the fields are truly separated.

## Error code plan

A new error code may be needed:

InvalidXntdRequiredLockAmount

or:

InsufficientXntdLockAmount

Recommended split:

InvalidXntdLockAmount:
- amountXntd <= 0

InvalidXntdRequiredLockAmount:
- observedRequiredXntdLock <= 0

InsufficientXntdLockAmount:
- amountXntd < observedRequiredXntdLock

Final names can be decided during implementation.

## Snapshot impact

Build state already stores:

- lockedXntd
- requiredXntdLock
- lockEpoch

No snapshot schema change is required just because observedRequiredXntdLock is added to transient payloads.

Snapshot tests should still confirm that requiredXntdLock persists correctly.

## CLI impact

No CLI output change is required initially.

If future CLI commands display last lock proof or last observed requirement, that can be a separate milestone.

## Backward compatibility note

Because this is an MVP lab and not a production API, runtime types can be updated directly.

However, implementation should still be staged carefully to avoid losing test clarity.

## Recommended implementation order

1. Add observedRequiredXntdLock to low-level lock inputs and tests.
2. Add observedRequiredXntdLock to registrar inputs and registrar tests.
3. Add observedRequiredXntdLock to registrar payload builders and tests.
4. Add observedRequiredXntdLock to proof types and watcher conversion tests.
5. Add observedRequiredXntdLock to watcher candidates and constructor tests.
6. Update app proof submission and e2e tests.
7. Add under-lock rejection tests.
8. Run full validation.
9. Update docs / assumptions if needed.
10. Update checkpoint.

## Current decision

Do not implement runtime changes in this milestone.

This document exists so the observedRequiredXntdLock runtime rollout can be done in a controlled sequence.

# Authoritative XC Epoch Minimum Validation Runtime Plan

## Branch

authoritative-xc-epoch-minimum-validation-runtime-plan

## Purpose

This document defines the next runtime implementation path for validating XNTD lock / relock required amounts against authoritative XC epoch state.

This is a plan-only milestone.

It does not change runtime code.

## Current completed state

The observedRequiredXntdLock runtime propagation chain is complete.

Current explicit runtime flow:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state

Current runtime records:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

Current runtime validation checks:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock
- monotonic lockEpoch ordering
- registrar message replay protection
- XNTD commitment event replay protection

## Remaining production-readiness gap

The runtime still does not verify:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

This means the registrar / integration layer can currently submit a self-consistent but economically incorrect requirement.

Example:

- amountXntd = 5
- observedRequiredXntdLock = 5
- real authoritative epoch minimum = 100

The current runtime accepts the relationship between amount and observed requirement because:

amountXntd >= observedRequiredXntdLock

But it does not yet know that the authoritative requirement is 100.

## Design principle

Authoritative epoch minimum validation should live at the registrar / integration boundary.

It should not live in the low-level lockXntd() / relockXntd() primitives.

Reason:

- low-level primitives should remain deterministic state transition helpers
- external XC state validation is a boundary concern
- future production integration may use real Ethereum RPC, finalized checkpoints, proofs, or XC Lens/Core reads

## Recommended first runtime layer

Do not start with live Ethereum RPC.

First implement a local validation interface that can be tested deterministically.

Conceptual interface:

interface XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

or a pure function:

getAuthoritativeEpochMinimum(lockEpoch: number): bigint | null

Expected behavior:

- returns bigint when the epoch minimum is known
- returns null when the epoch is unknown or not accepted

## Registrar validation rule

For LOCK_XNTD and RELOCK_XNTD, after existing prechecks and before mutating registrar / event / build state:

1. Validate amountXntd > 0.
2. Validate observedRequiredXntdLock > 0.
3. Validate amountXntd >= observedRequiredXntdLock.
4. Load authoritative minimum for lockEpoch.
5. Reject if no authoritative minimum is available.
6. Reject if observedRequiredXntdLock != authoritativeEpochMinimum(lockEpoch).
7. Continue to acceptRegistrarMessage().
8. Continue to acceptXntdCommitmentEvent().
9. Continue to lockXntd() / relockXntd().

## Mutation safety requirement

Rejected authoritative epoch minimum validation must not mutate:

- registrar.processedMessages
- xntdCommitmentEvents.usedXntdCommitmentEvents
- Build state

This matches the existing under-lock rejection safety model.

## Proposed runtime files

Likely new file:

- src/model/xc-epoch-minimum-source.ts

or:

- src/instructions/xc-epoch-minimum-validation.ts

Likely updated file:

- src/instructions/registrar-xntd-lock.ts

Likely updated tests:

- tests/registrar-xntd-lock.test.ts
- tests/app-proof-submission.test.ts, only if app-level source injection is added
- tests/e2e-watcher-proof-registrar-scenario.test.ts, only if e2e app state stores the source

## Minimal test cases

Registrar-level tests should cover:

1. LOCK_XNTD accepts when:
   - amountXntd >= observedRequiredXntdLock
   - observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

2. LOCK_XNTD rejects when:
   - observedRequiredXntdLock != authoritativeEpochMinimum(lockEpoch)

3. LOCK_XNTD rejects when:
   - authoritative minimum for lockEpoch is missing

4. RELOCK_XNTD accepts when:
   - amountXntd >= observedRequiredXntdLock
   - observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)
   - availableBld >= historyBld
   - commitment is active
   - lockEpoch increases

5. RELOCK_XNTD rejects when:
   - observedRequiredXntdLock != authoritativeEpochMinimum(lockEpoch)

6. Rejected validation does not mark:
   - registrar message
   - XNTD commitment event key

7. Rejected validation does not mutate:
   - lockedXntd
   - requiredXntdLock
   - lockEpoch
   - xcCommitmentActive
   - updatedAt

## App-level integration option

There are two possible implementation styles.

### Option A: explicit validator argument

Pass a validator/source into appApplyRegistrarXntdLock() and appApplyRegistrarXntdRelock().

Pros:

- explicit
- easy to test
- no app state migration needed

Cons:

- every call site must supply it

### Option B: store source in BuildApplicationState

Add an XC epoch minimum source to app state.

Pros:

- appSubmitProof can validate automatically
- closer to production integration

Cons:

- requires app state shape update
- may affect snapshot/storage tests if persisted
- likely more intrusive

## Recommended next implementation

Use Option A first at registrar instruction level.

Start with direct registrar tests.

Do not update snapshot schema yet.

Do not persist any XC epoch source in BuildApplicationState yet.

Once registrar-level validation is correct and tested, decide whether app-level proof submission should receive a validator argument or whether app state should own the source.

## Non-goals for the first runtime layer

Do not implement:

- real Ethereum RPC reads
- XC Core ABI integration
- XC Lens ABI integration
- finalized block verification
- Merkle proofs
- X1 on-chain verification
- persisted epoch checkpoint storage
- snapshot schema migration

## Future production path

After the deterministic validator is in place, production integration can replace the local test source with one of:

- trusted integration service reading XC Core/Lens
- finalized Ethereum block source
- XC epoch state checkpoint proof
- bridge-provided epoch state proof
- X1-native verified checkpoint

## Relationship to existing docs

This runtime plan follows the active design docs:

- docs/registrar/xntd-lock-epoch-minimum-validation.md
- docs/registrar/authoritative-xc-state-source.md
- docs/assumptions.md

Those docs already define the production requirement:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

This plan defines the smallest safe runtime step toward that requirement.

## Current decision

The next runtime layer should introduce deterministic authoritative epoch minimum validation at the registrar boundary using a simple local source / validator.

This keeps the implementation small, testable, and compatible with the current MVP architecture while closing the main economic validation gap.

## Validation for this plan milestone

This plan-only milestone should pass:

- npm run typecheck
- npm test
- npm run build
- npm audit --audit-level=moderate

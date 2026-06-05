# XNTD Lock Epoch Minimum Validation Design

## Purpose

This document defines the intended production validation model for XNTD lock / relock required lock amounts.

The goal is to separate:

- the actual locked amount
- the required XNTD lock amount
- the authoritative XC epoch minimum used to validate the requirement

This document is design-only.

It does not change runtime code.

## Current MVP state

In the current MVP runtime, XNTD lock / relock receives:

- amountXntd
- lockEpoch

The low-level lock primitives currently set:

lockedXntd = amountXntd
requiredXntdLock = amountXntd
lockEpoch = input.lockEpoch

This is acceptable for the current trusted registrar MVP because the registrar / integration layer is responsible for submitting correct values.

However, this is not production-complete.

## Current production gap

The intended production rule is:

requiredXntdLock = current epoch Core L1 nominal from xEnchanted Crypto

The runtime currently does not independently verify that:

requiredXntdLock == authoritative XC epoch minimum

It also does not distinguish the user's actual locked amount from the required amount.

In production, these values should be conceptually separate:

lockedXntd = amount actually locked by the user
requiredXntdLock = minimum required amount for the selected/current XC epoch

## Why amountXntd == requiredXntdLock is not enough for production

If the registrar sends:

amountXntd = 5
lockEpoch = current epoch

the current MVP model will set:

lockedXntd = 5
requiredXntdLock = 5

Even if the real XC epoch minimum is 100.

This does not create token inflation by itself, but it can incorrectly mark a Build as satisfying a commitment requirement.

Therefore production integration must validate the required amount against authoritative XC state.

## Authoritative source

The authoritative source should be the deployed xEnchanted Crypto / XC state source.

Possible source values:

- current epoch
- currentBaseNominal
- current Core L1 nominal
- epochAt(timestamp)
- current epoch minimum lock requirement

The exact integration can depend on the final XC contract / lens interface.

The important principle:

The Build registrar must not invent the required lock amount.

It must derive or verify it from the authoritative XC state source.

## Proposed production payload model

Future LOCK_XNTD / RELOCK_XNTD payloads should distinguish:

- amountXntd
- requiredXntdLock
- lockEpoch
- epochSource
- epochSourceBlock or finalized source reference

Conceptual fields:

amountXntd: actual amount locked / relocked
requiredXntdLock: required minimum for this lockEpoch
lockEpoch: XC epoch used for the requirement
xcEpochMinimumSource: canonical XC state source reference
sourceBlockNumber or finalizedHeight: finalized source context

The exact field names can be decided during implementation.

## Proposed validation rule

Registrar validation should require:

amountXntd > 0
requiredXntdLock > 0
amountXntd >= requiredXntdLock
requiredXntdLock == authoritativeEpochMinimum(lockEpoch)
lockEpoch is current or otherwise accepted by the integration policy

The existing monotonic lockEpoch guard should remain separate:

incomingLockEpoch > currentBuild.lockEpoch

This prevents stale commitment state regression.

Epoch minimum validation prevents under-locking.

These are different protections.

## Where validation should live

The validation should live in the registrar / integration boundary, not in the low-level lock primitive.

Preferred layer:

- registrar XNTD lock / relock handler
- or a dedicated validator used by the registrar handler

Reason:

- low-level lockXntd() / relockXntd() should remain simple state transition primitives
- registrar / integration layer is responsible for source-event validity
- authoritative XC state validation belongs to the boundary that reads or verifies external chain state

## Interaction with current replay protection

This design is independent from replay protection.

Already implemented protections:

processedMessages
usedXntdCommitmentEvents
monotonic lockEpoch guard

Epoch minimum validation adds a different guarantee:

the required lock amount matches authoritative XC epoch state

## Interaction with Build activation

Current conceptual activation rule:

xcCommitmentActive =
  historyBld > 0
  AND lockedXntd >= requiredXntdLock

Production validation must make sure that requiredXntdLock is meaningful.

Otherwise, a too-low registrar-provided requiredXntdLock could make xcCommitmentActive true incorrectly.

## MVP boundary

This document does not change the current runtime behavior.

The current MVP still has:

requiredXntdLock = amountXntd

The MVP relies on trusted registrar input.

This is acceptable before live production integration, but it must remain visible as an explicit integration boundary.

## Recommended implementation sequence

1. Keep this document as design-only.
2. Decide the authoritative XC state source.
3. Decide whether payloads carry requiredXntdLock explicitly or registrar derives it internally.
4. Add tests for:
   - amountXntd < requiredXntdLock rejected
   - requiredXntdLock mismatch rejected
   - correct epoch minimum accepted
   - replay protections still work
   - lockEpoch ordering guard still works
5. Add registrar validation logic.
6. Update assumptions.
7. Update review summary / README if needed.
8. Update checkpoint.

## Current decision

For now:

- do not change runtime code
- keep amountXntd / requiredXntdLock equality as an MVP implementation detail
- document production validation requirements clearly
- treat epoch minimum validation as the next production-readiness layer after replay / ordering safety

# XNTD Lock Epoch Minimum Validation Design

## Purpose

This document defines the intended production validation model for XNTD lock / relock required lock amounts.

The goal is to separate:

- the actual locked amount
- the required XNTD lock amount
- the authoritative XC epoch minimum used to validate the requirement

This document is design-only.

It does not change runtime code.

## Current runtime state

The current runtime propagates observedRequiredXntdLock through the full XNTD lock / relock chain:

watcher candidate
-> proof payload
-> registrar payload
-> proof submission
-> app service
-> registrar input
-> low-level lock / relock
-> Build state

XNTD lock / relock now carries:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

The low-level lock primitives set:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock
lockEpoch = input.lockEpoch

The runtime validates:

- amountXntd > 0
- observedRequiredXntdLock > 0
- amountXntd >= observedRequiredXntdLock

This is an improvement over the earlier MVP equality model, but it is still not production-complete.

## Current production gap

The intended production rule is:

requiredXntdLock = current epoch Core L1 nominal from xEnchanted Crypto

The runtime now distinguishes the user's actual locked amount from the observed required amount:

lockedXntd = amount actually locked by the user
requiredXntdLock = observed required amount for the selected/current XC epoch

However, the runtime currently does not independently verify that:

observedRequiredXntdLock == authoritative XC epoch minimum

That authoritative validation remains the production-readiness gap.

## Why observedRequiredXntdLock still needs authoritative validation

If the registrar sends:

amountXntd = 5
observedRequiredXntdLock = 5
lockEpoch = current epoch

the current runtime will accept the relationship between amount and requirement because:

amountXntd >= observedRequiredXntdLock

Even if the real XC epoch minimum is 100.

This does not create token inflation by itself, but it can incorrectly mark a Build as satisfying a commitment requirement.

Therefore production integration must validate observedRequiredXntdLock against authoritative XC state.

## Authoritative source

The authoritative source should be the deployed xEnchanted Crypto / XC state source.

Possible source values:

- current epoch
- currentBaseNominal
- current Core L1 nominal
- epochAt(timestamp)
- current epoch minimum lock requirement

The exact integration can depend on the final XC contract / lens interface.

The authoritative XC state source model is documented separately in:

- docs/registrar/authoritative-xc-state-source.md

The important principle:

The Build registrar must not invent the required lock amount.

It must derive or verify it from the authoritative XC state source.

## Proposed production payload model

Future LOCK_XNTD / RELOCK_XNTD payloads should distinguish:

- amountXntd
- observedRequiredXntdLock
- lockEpoch
- epochSource
- epochSourceBlock or finalized source reference

Recommended payload decision:

Use observedRequiredXntdLock in watcher / proof / registrar payloads.

Conceptual fields:

amountXntd: actual amount locked / relocked
observedRequiredXntdLock: required minimum observed by the watcher for this lockEpoch
lockEpoch: XC epoch used for the requirement
xcEpochMinimumSource: canonical XC state source reference
sourceBlockNumber or finalizedHeight: finalized source context

Reasoning:

- amountXntd is the actual user lock amount
- observedRequiredXntdLock is the requirement observed from XC state
- registrar validation must still verify observedRequiredXntdLock against the authoritative XC epoch minimum
- the Build state field can remain requiredXntdLock after successful validation

The proof payload should be self-describing, but not blindly trusted.

## Proposed validation rule

Registrar validation should require:

amountXntd > 0
observedRequiredXntdLock > 0
amountXntd >= observedRequiredXntdLock
observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)
lockEpoch is current or otherwise accepted by the integration policy

After successful validation, Build state should record:

requiredXntdLock = observedRequiredXntdLock
lockedXntd = amountXntd

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

## Current MVP boundary

The current runtime already separates:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

However, the MVP still relies on trusted registrar / integration input for the correctness of observedRequiredXntdLock.

The runtime does not yet independently verify:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

This is acceptable before live production integration, but it must remain visible as an explicit integration boundary.

## Recommended implementation sequence

Completed runtime propagation:

1. Add observedRequiredXntdLock to watcher / proof / registrar payloads.
2. Propagate observedRequiredXntdLock through proof submission, app service, registrar input, and low-level lock / relock.
3. Add tests for amountXntd < observedRequiredXntdLock rejection.
4. Remove compatibility fallback that silently derived observedRequiredXntdLock from amountXntd.

Remaining production-readiness sequence:

1. Decide the exact authoritative XC state source / interface.
2. Add registrar / integration validation for:
   - observedRequiredXntdLock mismatch rejected
   - correct epoch minimum accepted
   - replay protections still work
   - lockEpoch ordering guard still works
3. Update assumptions.
4. Update review summary / README if needed.
5. Update checkpoint.

## Current decision

For now:

- keep the completed observedRequiredXntdLock runtime chain
- keep amountXntd and observedRequiredXntdLock explicit and separate
- document production validation requirements clearly
- treat authoritative epoch minimum validation as the next production-readiness layer after runtime propagation, replay protection, and ordering safety

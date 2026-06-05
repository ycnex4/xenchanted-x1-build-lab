# Authoritative XC State Source Design

## Purpose

This document defines the intended authoritative source model for XC epoch state used by XNTD lock / relock validation.

It focuses on the source of truth for:

- lockEpoch
- currentBaseNominal
- current Core L1 nominal
- authoritativeEpochMinimum(lockEpoch)

This document is design-only.

It does not change runtime code.

## Problem

XNTD lock / relock epoch minimum validation requires:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The previous design defines that future payloads should carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

However, observedRequiredXntdLock is not authoritative by itself.

The system must define how the registrar / integration layer verifies that value against XC protocol state.

## Current repository state

Current documents state that:

required_xntd_lock = current epoch Core L1 nominal

and that the Ethereum Registrar should determine this value from Ethereum-side xEnchanted Crypto protocol state.

Current runtime does not implement this validation yet.

## Authoritative source

The authoritative source should be Ethereum-side xEnchanted Crypto protocol state.

Preferred source:

- deployed XC Core contract
- deployed XC Lens contract, if it exposes protocol parameters safely
- finalized Ethereum block context

The authoritative source must provide or allow deriving:

- lockEpoch
- currentBaseNominal
- current Core L1 nominal
- epoch timestamp / epochAt(timestamp), if needed

## Core rule

For XNTD lock / relock validation:

observedRequiredXntdLock must equal the authoritative XC epoch minimum for lockEpoch.

Conceptually:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

and:

amountXntd >= observedRequiredXntdLock

## Recommended integration path

For the next implementation layer, use a pragmatic trusted integration model:

1. Watcher observes XC state at a finalized Ethereum block.
2. Watcher creates LOCK_XNTD / RELOCK_XNTD candidate with:
   - amountXntd
   - observedRequiredXntdLock
   - lockEpoch
   - source block metadata
3. Candidate becomes a validated proof.
4. Registrar / integration layer verifies observedRequiredXntdLock against authoritative XC state for the same lockEpoch / source context.
5. After successful validation:
   - lockedXntd = amountXntd
   - requiredXntdLock = observedRequiredXntdLock

## Why watcher observation is not enough

The watcher can include observedRequiredXntdLock for auditability and self-describing proofs.

But the watcher-provided value must not be treated as final authority.

The registrar / integration layer must verify it.

This keeps the proof readable without moving authority into the payload.

## Recommended source context

Future proof / registrar payloads should include or reference:

- sourceChainId
- XC Core or Lens address
- sourceBlockNumber
- finalized status
- lockEpoch
- observedRequiredXntdLock

If the source context is already included in proof metadata, the payload should not duplicate it unless needed for clarity.

## Production-hardening path

A stricter production model can introduce a separate XC epoch state checkpoint proof.

Conceptual checkpoint:

- xcEpochStateCheckpointId
- sourceChainId
- xcCoreAddress
- xcLensAddress, if used
- sourceBlockNumber
- finalized status
- lockEpoch
- currentBaseNominal
- authoritativeEpochMinimum

Then LOCK_XNTD / RELOCK_XNTD proofs can reference:

- xcEpochStateCheckpointId

This separates:

- XC state verification
- user lock / relock event verification

## Recommended path decision

Use the simpler integration path first:

- watcher observes XC state
- proof carries observedRequiredXntdLock
- registrar verifies against authoritative XC state

Keep the checkpoint-proof model as a production-hardening path.

Short form:

B now, C later.

## Relationship to existing protections

This design is separate from existing replay and ordering protections.

Already implemented:

- processedMessages
- usedXntdCommitmentEvents
- monotonic lockEpoch guard

Epoch minimum validation adds economic correctness.

Authoritative XC state source validation proves that the economic requirement came from XC protocol state.

## Non-goals

This document does not define:

- live RPC implementation
- exact contract ABI
- exact XC Lens interface
- Merkle proof format
- bridge execution
- X1 on-chain verification
- runtime code changes

## Current decision

For now:

- do not change runtime code
- use this document to guide future implementation
- keep observedRequiredXntdLock as the future payload field
- verify it against authoritative XC state before writing requiredXntdLock into Build state

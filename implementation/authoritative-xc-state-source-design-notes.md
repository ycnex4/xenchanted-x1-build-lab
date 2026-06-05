# Authoritative XC State Source Design Notes

## Branch

authoritative-xc-state-source-design

## Purpose

This milestone documents the authoritative source model for XC epoch state used by XNTD lock / relock epoch minimum validation.

It does not change runtime code.

## Problem

Future XNTD lock / relock validation requires:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The previous payload-shape review decided that future payloads should carry:

- amountXntd
- observedRequiredXntdLock
- lockEpoch

However, observedRequiredXntdLock is not authoritative by itself.

The system must define how the registrar / integration layer verifies that value against XC protocol state.

## Decision

Use a pragmatic trusted integration path first:

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

Short form:

B now, C later.

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

## Relationship to existing protections

Already implemented:

- processedMessages
- usedXntdCommitmentEvents
- monotonic lockEpoch guard

Already documented:

- observedRequiredXntdLock payload shape decision

This milestone adds the source-of-truth design for:

- authoritativeEpochMinimum(lockEpoch)

## Scope boundary

This milestone does not define:

- live RPC implementation
- exact contract ABI
- exact XC Lens interface
- Merkle proof format
- bridge execution
- X1 on-chain verification
- runtime code changes

## Updated documents

Added:

- docs/registrar/authoritative-xc-state-source.md

Linked from:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-epoch-minimum-validation.md

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

# xEnchanted X1 Build Lab — MVP Assumptions and Known Limitations

## Purpose

This document records explicit MVP assumptions and known limitations for xEnchanted X1 Build Lab.

The goal is to keep the current implementation honest, reviewable, and easy to evolve without hiding trust boundaries or deferred production requirements.

## Scope

These assumptions apply to the current MVP implementation lab.

They do not describe a final production deployment.

They should be revisited before adding live indexer integration, production chain integration, bridge execution, or token issuance logic.

## 1. Trusted indexer / registrar model

The MVP assumes a trusted registrar / indexer path.

The registrar is responsible for submitting valid proof-derived messages to the application layer.

The current MVP focuses on deterministic accounting, proof routing, replay protection, and state transitions.

It does not yet implement a trustless proof verification layer.

Important implication:

- the registrar is expected to map source events to the correct Build
- the registrar is expected to submit finalized events only
- the registrar is expected to submit correct lock and checkpoint values
- production integration must harden this boundary

## 2. Build ownership mapping assumption

The MVP assumes that the proof submission layer resolves the correct Build before calling registrar handlers.

The current focus is on canonical Build identity and accounting correctness after a Build has been selected.

Important implication:

- proof payload owner / source identity checks are part of the broader registrar / application proof submission boundary
- this assumption should be reviewed before production integration

## 3. XNTD lock / relock replay and ordering protection

XNTD lock and relock now have three runtime protection layers:

- registrar-level replay protection through processed message IDs
- source-event replay protection through XntdCommitmentEventKey / usedXntdCommitmentEvents
- monotonic lockEpoch ordering guard for stale-but-unique commitment events

Lock and relock are overwrite operations, not accumulative accounting operations.

Important implication:

- replay with the same messageId is blocked
- replay of the same source event under a different messageId is blocked
- stale-but-unique lock / relock events with non-increasing lockEpoch are blocked
- these protections prevent old commitment events from regressing Build lock state in the MVP model

Current MVP boundary:

- the ordering guard uses monotonic lockEpoch
- production may later choose a stricter ordering source, such as source block number, finalized slot / block height, event timestamp, or monotonic commitment version

Design and implementation path:

- docs/registrar/xntd-lock-event-identity.md
- implementation/xntd-commitment-event-replay-state-notes.md
- implementation/xntd-lock-epoch-ordering-guard-notes.md

## 4. requiredXntdLock source

In the current MVP, requiredXntdLock is accepted from registrar-provided lock / relock input.

Practically, the lock amount becomes the required lock value inside the model.

The intended production rule is that requiredXntdLock should correspond to the current epoch Core L1 nominal from xEnchanted Crypto.

Important implication:

- the MVP does not independently calculate the epoch minimum
- the registrar / integration layer is responsible for submitting the correct value
- production integration should validate the epoch minimum against the authoritative XC state source

## 5. No unlock flow

The current MVP includes lock and relock flows.

It does not include an unlock flow.

Important implication:

- locked XNTD cannot be reduced through a standalone unlock action in the MVP
- relock is the only modeled update path
- relock is intentionally constrained by available_bld >= history_bld

Accepted MVP limitation:

- this is conservative for the implementation lab
- unlock semantics should be designed separately before production use

## 6. canonicalEventKey convention

Watcher candidates use canonicalEventKey as the source event identity.

For Core Redeem and XEN Burn proofs, the event-specific keys are derived from or aligned with canonicalEventKey.

Important implication:

- canonicalEventKey must include enough source identity to distinguish events
- sourceAddress must correctly identify the source contract
- transaction hash and event index / log index must be part of the event identity
- watcher / indexer code must ensure canonicalEventKey is collision-resistant for the intended source domain

Accepted MVP limitation:

- the MVP relies on watcher-side correctness for canonical event identity construction

## 7. Fee checkpoint finality assumption

X1 fee contribution checkpoints use slot-based monotonicity protection.

The model rejects non-increasing checkpoint slots.

Important implication:

- checkpoint replay / regression is constrained by slot ordering
- the MVP assumes the indexer submits finalized or accepted checkpoint data
- testnet slot finality should not be treated as equivalent to production finality without a finality policy

## 8. Snapshot recovery is read-only

CLI snapshot recovery is intentionally read-only.

The command:

- can load canonical snapshot
- can fall back to backup snapshot
- reports which source was used

It does not:

- repair canonical snapshot
- copy backup into canonical
- delete corrupted files
- migrate snapshot files
- create new backup files

Important implication:

- operators must perform any restore action manually
- this prevents accidental destructive recovery behavior in the MVP

## 9. Snapshot content hash is not implemented

The MVP does not store or verify a separate snapshot content hash.

Snapshot safety currently relies on:

- JSON decode
- snapshot deserialization
- verification through the same load path
- backup-enabled save checks
- recovery load checks

Accepted MVP limitation:

- content hash can be added later if needed
- this is not required for the current implementation lab

## 10. No production integration guarantees yet

The MVP does not yet include:

- production chain deployment
- live RPC integration
- real watcher service runtime
- bridge execution
- token issuance logic
- UI
- operator restore tooling
- trustless proof verification

Important implication:

- the repository is ready for architecture and implementation review
- it is not a final production system

## Current accepted review conclusions

External review found the current model coherent, with the main immediate improvements being:

- reorder successful registrar mutations for Core Redeem and XEN Burn
- document assumptions and known limitations explicitly

The registrar mutation ordering has been addressed in the registrar-mutation-order-assumptions branch.

This document addresses the assumptions / known limitations part.

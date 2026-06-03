# Final Pre-Implementation Checkpoint

## 1. Purpose

This document records the final design checkpoint before starting X1 Build MVP implementation.

The goal is to confirm that the core model, accounting rules, trust assumptions, implementation sequence, and risks are documented before code begins.

---

## 2. Current status

Status: ready for implementation planning.

The repository is still documentation-only.

No implementation code has been started yet.

---

## 3. Core Build model

X1 Build is a voluntary X1-side user object that records separate verified contribution and commitment layers.

Main layers:

- BLD from redeemed Core NFT history
- XEN Burn Power from verified XEN.burn(user, amount) calls
- XNTD commitment through lock / relock
- X1 Fee Contribution from X1 network fees paid by an address
- native X1 Build creation through burn 11 BLD

These layers must remain separate.

---

## 4. BLD model

BLD fields:

- history_bld
- available_bld
- origin_bld

## history_bld

Historical BLD from redeemed Core NFT history.

Rules:

- non-decreasing
- non-transferable
- non-spendable
- never tokenized
- created only by redeemed Core NFT history

## available_bld

Usable BLD balance.

May decrease through transfer, burn, use, or future approved mechanics.

## origin_bld

Genesis Origin allocation.

Not history.

Does not increase history_bld.

---

## 5. XBP model

XEN Burn Power comes from verified XEN.burn(user, amount) calls.

Rules:

- arbitrary zero-address transfers do not count
- XBP does not create BLD
- BLD does not create XBP
- earned_xbp is historical
- available_xbp is separate usable XBP if future mechanics use it

---

## 6. XNTD commitment model

XNTD lock is commitment, not contribution.

Rules:

- XNTD lock does not create BLD
- XNTD lock does not create XBP
- required_xntd_lock is based on current XC epoch Core L1 nominal
- xc_commitment_active requires history_bld > 0 and locked_xntd >= required_xntd_lock
- relock requires available_bld >= history_bld

---

## 7. Native X1 Build path

Native X1 Build creation uses burn 11 BLD.

Rules:

- creates a Build
- does not create history_bld
- does not create fake XC history
- does not require XNTD lock unless XC history is later connected

---

## 8. Registrar model

MVP uses a trusted Ethereum Registrar.

The registrar verifies source data and submits messages to the X1 Build Program.

Registrar messages must be source-based.

Registrar must not submit arbitrary BLD or XBP totals.

Replay protection is mandatory.

---

## 9. Required replay protections

Required protection records:

- processed_messages[message_id]
- used_redeem_events[redeem_key]
- used_xen_burn_events[xen_burn_key]
- genesis_origin_claimed[identity]
- canonical_build_by_identity[identity]

No source event should be usable twice.

---

## 10. Authority model

MVP may use infrastructure authorities:

- registrar_authority
- fee_indexer_authority
- config_authority
- emergency_authority, optional
- upgrade_authority, if MVP program is upgradeable

Authorities must be limited by role.

Authorities must not bypass source-event rules.

---

## 11. BLD tokenization decision

BLD is not tokenized in MVP.

MVP uses internal available_bld accounting.

Post-MVP composability is deferred to a separate design path.

---

## 12. MVP implementation sequence

Implementation should follow layered branches:

1. project scaffold
2. BuildState account
3. create_build
4. registrar message replay protection
5. Core redeem -> history_bld
6. Genesis Origin BLD
7. XEN Burn Power
8. XNTD lock / unlock / relock
9. X1 Fee Contribution checkpoints
10. reader interface
11. integration tests
12. implementation checkpoint update

Do not implement the full MVP in one branch.

---

## 13. Main implementation blockers

Implementation should not start if:

- documentation consistency review is not complete
- branch plan is not accepted
- risk checklist is not understood
- BuildState fields are still unstable
- BLD / XBP / XNTD lock separation is unclear
- authority model is not acceptable
- replay protection keys are not defined

---

## 14. Pre-code checklist

Before creating the first implementation branch:

- run documentation consistency searches
- check README index
- check current-design-checkpoint document list
- confirm no deprecated earned_bld terminology remains
- confirm git status is clean
- confirm base branch is main
- create build-mvp-scaffold branch

---

## 15. Recommended first branch

First implementation branch:

build-mvp-scaffold

Purpose:

Create only project scaffold and test/build setup.

Do not implement accounting logic in the first branch.

---

## 16. Success condition

The project is ready to move from design to implementation when:

- all design documents are committed
- current-design-checkpoint reflects the document set
- MVP scope is clear
- implementation sequence is clear
- implementation risks are documented
- working tree is clean

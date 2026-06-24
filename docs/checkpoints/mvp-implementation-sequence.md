<!--
Superseded note:
This document contains pre-cleanup Build balance terminology such as available_bld,
earned_xbp, available_xbp, one-time Genesis Origin claim, or relock-by-available_bld.
For the current authoritative Build State model, use:

- docs/build/build-state-history-identity-model.md
- docs/build/build-v1-spec.md
- docs/checkpoints/build-state-history-identity-cleanup.md

Current model:
Build State stores durable public history, not live spendable balances.
Build Identity stores owner-controlled name/logo metadata.
Future spendable BLD belongs to a separate BLD asset / ledger / escrow layer.
-->

# MVP Implementation Sequence

## 1. Purpose

This document describes a safe implementation sequence for the X1 Build MVP.

The goal is to move from documentation to code without mixing too many concerns at once.

The MVP should prioritize correctness, replay protection, and readable state.

---

## 2. Implementation principle

Implement in layers.

Do not start with the full system at once.

Recommended order:

1. BuildState account / object
2. basic create_build
3. internal BLD accounting
4. registrar message replay protection
5. Core redeem -> history_bld
6. Genesis Origin BLD
7. XEN Burn Power
8. XNTD lock / unlock / relock
9. X1 Fee Contribution checkpoints
10. reader interface
11. tests and consistency review

---

## 3. Phase 1: project scaffold

Create basic program / project structure.

Include:

- program entry points
- account definitions
- error definitions
- instruction modules
- test setup

Do not implement full accounting yet.

Goal:

A minimal buildable project.

---

## 4. Phase 2: BuildState account

Implement BuildState storage.

Fields:

- owner
- build_id
- version
- created_at
- updated_at
- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp
- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

Goal:

Create and read BuildState.

---

## 5. Phase 3: create_build

Implement basic Build creation.

Rules:

- one canonical Build per identity
- create_build alone does not create history_bld
- create_build alone does not create XBP
- create_build alone does not create XNTD commitment
- create_build alone does not create fee contribution

Goal:

A user can create an empty Build.

---

## 6. Phase 4: internal BLD accounting

Implement internal available_bld mechanics.

For MVP:

- do not tokenize BLD
- keep available_bld inside BuildState
- keep history_bld separate
- keep origin_bld separate

Goal:

Correct accounting before composability.

---

## 7. Phase 5: message replay protection

Implement processed message protection.

Required protection:

processed_messages[message_id]

Rules:

- each registrar message can be processed only once
- message_id must be deterministic
- rejected duplicates must not change state

Goal:

No replayed registrar message can apply twice.

---

## 8. Phase 6: Core redeem -> history_bld

Implement ADD_CORE_REDEEM.

Inputs:

- redeem_key
- normalized_bld
- source metadata
- build_id
- ethereum_identity

Updates:

history_bld += normalized_bld
available_bld += normalized_bld

Required protection:

used_redeem_events[redeem_key]

Rules:

- one Core redeem event can create BLD only once
- Core redeem is the only source of history_bld
- history_bld is non-decreasing

---

## 9. Phase 7: Genesis Origin BLD

Implement Genesis Origin allocation.

Input:

- tiered_origin_bld
- identity
- history_bld tier

Updates:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

Required protection:

genesis_origin_claimed[identity]

Rules:

- granted once only
- granted only during Build Genesis Epoch
- tiered_origin_bld must match history_bld tier
- origin_bld must not increase history_bld

---

## 10. Phase 8: XEN Burn Power

Implement ADD_XEN_BURN_POWER.

Inputs:

- xen_burn_key
- normalized_xbp
- source metadata
- build_id
- ethereum_identity

Updates:

earned_xbp += normalized_xbp
available_xbp += normalized_xbp

Required protection:

used_xen_burn_events[xen_burn_key]

Rules:

- XEN.burn(user, amount) is the only source of XBP
- arbitrary zero-address transfers do not count
- XBP does not create BLD

---

## 11. Phase 9: XNTD lock / unlock / relock

Implement:

- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

Fields updated:

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active

Rules:

- XNTD lock does not create BLD
- XNTD lock does not create XBP
- required_xntd_lock is based on XC epoch Core L1 nominal
- relock requires available_bld >= history_bld

---

## 12. Phase 10: X1 Fee Contribution

Implement fee checkpoint update.

Fields updated:

- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

Rule:

counted_until_slot > stored_x1_fee_counted_until_slot

Fee checkpoints do not create:

- BLD
- XBP
- XNTD commitment

---

## 13. Phase 11: reader interface

Implement reader methods / views.

Expose:

- Build identity
- BLD fields
- XBP fields
- XNTD commitment fields
- X1 Fee Contribution fields
- derived status fields
- timestamps
- version

Goal:

Frontends and other X1 projects can read Build state without internal replay-protection details.

---

## 14. Phase 12: tests

Minimum test groups:

- create_build
- duplicate Build prevention
- Core redeem BLD creation
- duplicate redeem rejection
- Genesis Origin tier allocation
- duplicate Genesis Origin rejection
- XEN Burn Power creation
- duplicate XEN burn rejection
- XNTD lock activation
- XNTD unlock deactivation
- relock integrity
- fee checkpoint update
- stale fee checkpoint rejection
- reader output consistency

---

## 15. Phase 13: documentation consistency review

Before implementation is considered complete, review consistency across:

- build-v1-spec.md
- buildstate-fields.md
- state-transitions.md
- program-instruction-layout.md
- pda-account-layout.md
- program-authority-model.md
- build-reader-interface.md
- registrar documents
- indexer documents
- economics documents
- current-design-checkpoint.md

Goal:

No conflicting terminology or rules.

---

## 16. What not to implement in MVP

Do not implement in MVP:

- tokenized BLD
- full bridge proof verification
- Merkle proof based lock updates
- decentralized registrar set
- complex marketplace logic
- advanced XBP spending mechanics

These can be considered after the MVP accounting model is stable.

---

## 17. Main MVP success condition

MVP is successful if:

- BuildState can be created
- verified source updates can be applied once
- replay protection works
- history_bld is separated from available_bld
- XBP is separated from BLD
- XNTD lock is commitment only
- fee contribution is separate
- reader interface is clear
- documentation and implementation agree

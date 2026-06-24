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

# Documentation Consistency Review

## 1. Purpose

This document defines a consistency review checklist for the X1 Build documentation before implementation begins.

The goal is to catch terminology conflicts, accounting conflicts, and MVP scope drift.

---

## 2. Documents to review

### Build

- docs/build/terminology.md
- docs/build/build-v1-spec.md
- docs/build/buildstate-fields.md
- docs/build/state-transitions.md
- docs/build/program-instruction-layout.md
- docs/build/pda-account-layout.md
- docs/build/program-authority-model.md
- docs/build/build-reader-interface.md

### Registrar

- docs/registrar/ethereum-registrar-concept.md
- docs/registrar/message-format.md
- docs/registrar/trust-model-evolution.md

### Indexers

- docs/indexers/x1-fee-contribution.md
- docs/indexers/xen-burn-power-indexing.md
- docs/indexers/mvp-trusted-indexer-schemas.md

### Economics

- docs/economics/bld-origin-and-native-entry.md
- docs/economics/bld-transfer-and-burn-mechanics.md
- docs/economics/xntd-lock-and-relock.md
- docs/economics/bld-tokenization-decision.md
- docs/economics/xntd-lock-proof-model.md

### Checkpoints

- docs/checkpoints/current-design-checkpoint.md
- docs/checkpoints/mvp-implementation-sequence.md

---

## 3. Terminology checks

Required terminology:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp
- XEN Burn Power
- XNTD commitment
- X1 Fee Contribution
- Ethereum Registrar
- Build Program

Deprecated / avoided terminology:

- earned_bld
- fake earned_bld
- BLD as raw XEN
- XNTD lock as contribution
- XBP as BLD
- BLD as XBP

---

## 4. BLD consistency checks

The documentation must consistently state:

- history_bld is historical.
- history_bld is non-decreasing.
- history_bld is created only by redeemed Core NFT history.
- available_bld is usable / spendable.
- origin_bld is Genesis allocation, not history.
- Genesis Origin BLD increases origin_bld and available_bld.
- Genesis Origin BLD must not increase history_bld.
- burn 11 BLD creates a Build but not history_bld.
- transfer / sale / burn changes available_bld only.

---

## 5. Relock consistency checks

The documentation must consistently state:

relock requires:

available_bld >= history_bld

Relock must not create:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp

Relock updates only XNTD commitment fields.

---

## 6. XBP consistency checks

The documentation must consistently state:

- XBP comes from verified XEN.burn(user, amount) calls.
- arbitrary zero-address transfers do not count.
- XBP does not create BLD.
- BLD does not create XBP.
- earned_xbp is historical.
- available_xbp is separate usable XBP, if future mechanics use it.

---

## 7. XNTD lock consistency checks

The documentation must consistently state:

- XNTD lock is commitment, not contribution.
- XNTD lock does not create BLD.
- XNTD lock does not create XBP.
- required_xntd_lock is based on current XC epoch Core L1 nominal.
- xc_commitment_active requires history_bld > 0 and sufficient locked_xntd.
- native X1 Build creation through burn 11 BLD does not require XNTD lock unless XC history is later connected.

---

## 8. Registrar consistency checks

The documentation must consistently state:

- registrar messages are source-based.
- registrar must not send arbitrary totals.
- one source event creates one accounting action.
- processed_messages[message_id] prevents message replay.
- used_redeem_events[redeem_key] prevents duplicate Core redeem accounting.
- used_xen_burn_events[xen_burn_key] prevents duplicate XEN burn accounting.
- genesis_origin_claimed[identity] prevents duplicate Genesis Origin allocation.
- registrar trust is infrastructure trust, not XC core protocol control.

---

## 9. Authority consistency checks

The documentation must consistently state:

- registrar_authority submits Ethereum-side source messages.
- fee_indexer_authority submits X1 fee checkpoints.
- config_authority is sensitive and should be minimized.
- emergency_authority, if used, should pause external updates only.
- upgrade_authority is an MVP / production-risk decision.
- authorities must not bypass source-event rules.

---

## 10. MVP scope checks

MVP should include:

- BuildState account / object
- create_build
- internal available_bld accounting
- registrar message replay protection
- Core redeem -> history_bld
- Genesis Origin BLD
- XEN Burn Power
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints
- reader interface
- tests

MVP should not include:

- tokenized BLD
- full bridge proof verification
- Merkle proof based lock updates
- decentralized registrar set
- complex marketplace logic
- advanced XBP spending mechanics

---

## 11. Reader interface consistency checks

The reader interface must preserve layer separation.

Readers should not infer that:

- available_bld equals history
- origin_bld is history
- locked_xntd creates BLD
- XBP creates BLD
- X1 fees create BLD
- native Build creation creates XC history

---

## 12. Current review status

Status: completed.

Review result: no blocking terminology, index, replay-protection, or MVP-scope conflicts found in the current documentation set.

Before implementation starts, run:

- terminology search
- field consistency search
- next steps review
- README / checkpoint index review
- duplicate document entry check

---

## 13. Final pass commands

Suggested searches:

Search deprecated term:

earned_bld

Search core BLD fields:

history_bld
available_bld
origin_bld

Search separation rules:

XBP does not create BLD
BLD does not create XBP
XNTD lock does not create BLD
XNTD lock does not create XBP

Search MVP scope:

tokenized BLD
trusted registrar
Merkle
bridge proof

---

## 14. Success condition

Documentation is ready for implementation planning when:

- no deprecated BLD terminology remains
- all documents agree on Build fields
- all source-event protections are named consistently
- MVP scope is clear
- authority trust is clearly disclosed
- reader interface matches BuildState
- current-design-checkpoint matches the document set


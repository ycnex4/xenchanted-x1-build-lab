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

# Build Reader Interface

## 1. Purpose

This document describes the conceptual reader interface for X1 Build.

The goal is to define which Build fields should be easy to read by users, frontends, explorers, and other X1 projects.

This is not implementation code.

---

## 2. Reader design goal

The Build reader interface should make Build state easy to interpret without requiring every external project to understand all internal protection accounts.

External readers should primarily read:

- Build identity
- BLD fields
- XBP fields
- XNTD commitment fields
- X1 Fee Contribution fields
- status fields
- timestamps / version

They should not need to inspect:

- processed_messages
- used_redeem_events
- used_xen_burn_events
- genesis_origin_claimed
- internal registrar metadata

---

## 3. Main Build view

Suggested reader output:

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

---

## 4. Identity fields

## owner

X1 address that owns the Build.

## build_id

Canonical Build identifier.

## version

Build state version.

Useful for future migrations and reader compatibility.

## created_at

Build creation timestamp / slot.

## updated_at

Last Build state update timestamp / slot.

---

## 5. BLD fields

## history_bld

Historical BLD from redeemed Core NFT history.

Non-decreasing.

Must not decrease when available BLD is sold, transferred, burned, or used.

## available_bld

Usable BLD balance.

May increase or decrease through allowed mechanics.

## origin_bld

Genesis Origin allocation.

Not historical contribution.

---

## 6. XBP fields

## earned_xbp

Historical XEN Burn Power from verified XEN.burn(user, amount) calls.

## available_xbp

Usable XEN Burn Power balance, if future mechanics allow XBP use or burn.

## Important

XBP is separate from BLD.

XBP must not create BLD.

BLD must not create XBP.

---

## 7. XNTD commitment fields

## locked_xntd

Current verified locked XNTD amount.

## required_xntd_lock

Required XNTD lock for current XC commitment state.

For Builds with history_bld, this should be based on the current XC epoch Core L1 nominal.

## lock_epoch

XC epoch used for the current lock requirement.

## xc_commitment_active

Whether the XC-derived commitment layer is active.

Rule:

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

---

## 8. X1 Fee Contribution fields

## x1_fee_contribution

Cumulative X1 network fees paid by the address as fee payer.

Stored in raw smallest X1 units.

## x1_tx_count

Number of fee-paying transactions counted.

## x1_fee_counted_until_slot

Latest X1 slot included in the accepted fee checkpoint.

## last_fee_update_at

Timestamp / slot when fee data was last updated.

---

## 9. Derived status fields

The reader may expose derived statuses for easier UI interpretation.

Suggested statuses:

- has_xc_history
- has_xbp
- has_xntd_commitment
- has_x1_fee_contribution
- is_native_build
- is_xc_connected
- is_commitment_active

## has_xc_history

history_bld > 0

## has_xbp

earned_xbp > 0

## has_xntd_commitment

locked_xntd > 0

## has_x1_fee_contribution

x1_fee_contribution > 0

## is_native_build

Build was created through burn 11 BLD and has no connected XC history yet.

## is_xc_connected

Build has a canonical Ethereum/XC identity binding.

## is_commitment_active

Same as xc_commitment_active.

---

## 10. Reader compatibility

Reader interfaces should be stable.

When fields are added later:

- preserve existing field meanings
- increment version
- avoid renaming reader fields without migration
- keep old readers from misinterpreting new values

---

## 11. What readers should not infer

Readers should not infer that:

- available_bld equals historical contribution
- origin_bld is earned contribution
- locked_xntd creates BLD
- XBP creates BLD
- X1 fees create BLD
- native Build creation creates XC history

These layers are separate.

---

## 12. External project interpretation

Other X1 projects may interpret Build fields independently.

Examples:

A project may value:

- history_bld as XC history
- earned_xbp as global XEN burn participation
- xc_commitment_active as current commitment
- x1_fee_contribution as network usage
- available_bld as spendable Build power

The Build Program should expose readable state, not force one universal score.

---

## 13. Minimal reader interface

If a minimal reader is needed, it should expose:

- owner
- build_id
- history_bld
- available_bld
- origin_bld
- earned_xbp
- locked_xntd
- required_xntd_lock
- xc_commitment_active
- x1_fee_contribution
- updated_at
- version

---

## 14. Full reader interface

A full reader may expose:

- all BuildState fields
- derived status fields
- identity binding summary
- last registrar message timestamp
- last fee checkpoint timestamp
- genesis_origin_claimed status

Care must be taken not to expose unnecessary internal complexity.

---

## 15. Main invariants

- Reader fields must preserve accounting separation.
- history_bld is not spendable balance.
- available_bld is not proof of history.
- origin_bld is not history.
- XBP is separate from BLD.
- XNTD lock is commitment, not contribution.
- X1 Fee Contribution is network fee activity, not BLD.
- Other projects should be able to read Build state without internal replay-protection details.

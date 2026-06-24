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

# Implementation Risk Checklist

## 1. Purpose

This document lists the main implementation risks for the X1 Build MVP.

The goal is to catch high-risk mistakes before and during implementation.

This is not implementation code.

---

## 2. Highest priority risks

The highest priority risks are:

- mixing history_bld with available_bld
- allowing duplicate source event accounting
- allowing registrar messages to set arbitrary totals
- allowing XBP to create BLD
- allowing XNTD lock to create BLD or XBP
- allowing relock when available_bld < history_bld
- allowing one identity to create multiple canonical Builds
- unclear authority boundaries
- hidden upgrade / admin risk
- reader interface misrepresenting state

---

## 3. BLD accounting risks

## Risk: history_bld becomes spendable

history_bld must be historical and non-decreasing.

It must not decrease when available BLD is:

- transferred
- sold
- burned
- used to create native Build
- used in future mechanics

## Required protection

Only available_bld changes through spendable mechanics.

---

## 4. Core redeem replay risk

## Risk

The same Core redeem event could be submitted more than once.

## Required protection

used_redeem_events[redeem_key]

A redeem_key must be accepted only once.

Duplicate redeem must fail before state changes.

---

## 5. Registrar message replay risk

## Risk

The same registrar message could be submitted more than once.

## Required protection

processed_messages[message_id]

A message_id must be accepted only once.

Duplicate message must fail before state changes.

---

## 6. XEN burn replay risk

## Risk

The same XEN.burn(user, amount) source event could be counted more than once.

## Required protection

used_xen_burn_events[xen_burn_key]

Duplicate XEN burn source must fail before state changes.

---

## 7. Genesis Origin double claim risk

## Risk

A user could claim Genesis Origin BLD multiple times.

## Required protection

genesis_origin_claimed[identity]

Genesis Origin must be one-time per eligible identity.

---

## 8. Identity duplication risk

## Risk

One identity could create or control multiple canonical Builds for the same contribution source.

## Required protection

canonical_build_by_identity[identity]

One canonical Build per identity unless future rules explicitly allow otherwise.

---

## 9. Arbitrary totals risk

## Risk

Registrar or indexer messages may set totals directly.

## Required rule

Registrar must submit source-based messages.

Allowed cumulative checkpoint exception:

- X1 Fee Contribution checkpoints

Even fee checkpoints must be monotonic and replay-protected.

---

## 10. XBP / BLD mixing risk

## Risk

XEN Burn Power may be treated as BLD.

## Required rule

XBP and BLD are separate layers.

- XBP does not create BLD.
- BLD does not create XBP.

Reader output must preserve this separation.

---

## 11. XNTD lock meaning risk

## Risk

XNTD lock may be interpreted as contribution.

## Required rule

XNTD lock is commitment only.

It must not create:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp

---

## 12. Relock integrity risk

## Risk

A user may sell or burn available BLD and still relock XNTD.

## Required rule

Relock requires:

available_bld >= history_bld

If false, relock must fail before state changes.

---

## 13. Required lock calculation risk

## Risk

required_xntd_lock may be calculated incorrectly or arbitrarily.

## Required rule

required_xntd_lock must be based on current XC epoch Core L1 nominal.

For MVP, Ethereum Registrar calculates it from verified XC protocol state.

---

## 14. Authority overreach risk

## Risk

Authorities may be too broad.

## Required rule

Each authority must have limited scope:

- registrar_authority handles source messages
- fee_indexer_authority handles fee checkpoints
- config_authority handles configuration only
- emergency_authority, if used, should pause external updates only
- upgrade_authority must be treated as production-risk decision

---

## 15. Upgrade risk

## Risk

Upgradeable program can change rules later.

## Required decision

Before production, decide whether to:

- remove upgrade authority
- keep upgrade authority with disclosure
- use timelock / multisig
- deploy immutable final version

MVP upgradeability must be clearly documented if used.

---

## 16. Reader misinterpretation risk

## Risk

Frontends or external projects may misread Build fields.

## Required rule

Reader interface should make these distinctions obvious:

- history_bld is not spendable
- available_bld is not history
- origin_bld is not history
- XBP is not BLD
- XNTD lock is not contribution
- X1 fees do not create BLD

---

## 17. Decimal / normalization risk

## Risk

BLD and XBP normalization may lose precision.

## Required rule

Internal representation should support fractional units or a fixed precision scale.

1 BLD = 100,000,000 XEN burned through redeemed Core history

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

Do not use floating point.

---

## 18. State update ordering risk

## Risk

State may partially update before validation finishes.

## Required rule

Validate first.

Then update state.

Duplicate / unauthorized / invalid messages must fail before state changes.

---

## 19. Test coverage checklist

Required tests:

- duplicate message rejected
- duplicate Core redeem rejected
- duplicate XEN burn rejected
- duplicate Genesis Origin rejected
- duplicate Build rejected
- unauthorized registrar rejected
- unauthorized fee indexer rejected
- relock fails when available_bld < history_bld
- lock does not create BLD
- XBP does not create BLD
- fee checkpoint does not create BLD
- reader output preserves fields

---

## 20. MVP release blocker list

Do not consider MVP ready if:

- history_bld can decrease
- available_bld can create history_bld
- duplicate source events can be counted
- registrar can set arbitrary BLD totals
- XBP can create BLD
- XNTD lock can create BLD
- relock can bypass available_bld >= history_bld
- reader interface hides important separation
- authority model is not documented
- tests do not cover replay protection

---

## 21. Main success condition

Implementation risk is acceptable when:

- every source update is replay-protected
- every accounting layer remains separate
- authority powers are limited and documented
- reader output is clear
- tests cover all critical invariants


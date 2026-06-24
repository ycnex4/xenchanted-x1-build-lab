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

# BLD Origin and Native Build Entry

## 1. Purpose

This document describes the economic purpose of BLD, Genesis Origin BLD, and Build creation in X1 through BLD burn.

The goal is to create a fair entry model for new X1 users while preserving the meaning of earned BLD from xEnchanted Crypto participation.

---

## 2. BLD meaning

BLD is the normalized Build unit derived from redeemed Core NFT history.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

BLD is not raw XEN.

BLD is a normalized contribution unit.

---

## 3. history_bld

history_bld is created only from redeemed Core NFT history.

Core redeem:
history_bld += normalized(Core.xenBurned)
available_bld += normalized(Core.xenBurned)

history_bld is historical.

It must not decrease when a user sells, spends, burns, or transfers available BLD.

---

## 4. available_bld

available_bld is the currently usable / transferable / spendable BLD amount.

It may come from:

- history_bld
- origin_bld
- purchased BLD
- received BLD

It may decrease through:

- sale
- transfer
- burn
- use in approved mechanics

available_bld must not be confused with history_bld.

---

## 5. origin_bld

origin_bld is Genesis Origin BLD.

It is not earned BLD.

It exists to seed Build creation in X1 through BLD burn.

origin_bld is granted only once when a Build first connects valid xEnchanted Crypto history during the Build Genesis Epoch.

---

## 6. Why fixed 121 BLD is not used

A fixed 121 BLD allocation for every eligible Build would be too uneven.

Example:

User A:
history_bld = 1

User B:
history_bld = 500

If both receive 121 origin_bld, the allocation dominates the real contribution of User A, while being relatively small for User B.

This would make Genesis Origin BLD too strong for minimal participation.

Therefore, 121 BLD should be the maximum Genesis Origin cap, not the default allocation.

---

## 7. Tiered Genesis Origin BLD allocation

Genesis Origin BLD is tiered by history_bld.

Allocation tiers:

- history_bld >= 1     -> origin_bld = 11
- history_bld >= 11    -> origin_bld = 22
- history_bld >= 121   -> origin_bld = 55
- history_bld >= 1111  -> origin_bld = 121

This keeps the system symbolic but more fair.

---

## 8. Symbolism

Build creation in X1 requires burning 11 BLD.

The Genesis Origin tiers represent the theoretical ability to support new Build creation:

- 11 BLD  -> 1 new Build
- 22 BLD  -> 2 new Builds
- 55 BLD  -> 5 new Builds
- 121 BLD -> 11 new Builds

121 remains the maximum symbolic cap:

11 x 11 = 121

---

## 9. Build creation in X1 through BLD burn

A user without XEN/XC history may create an active Build in X1 by burning BLD.

Requirement:

burn 11 BLD

This creates an active Build but does not create fake history_bld or fake XEN burn history.

The burned BLD must come from available BLD that exists in the system.

---

## 10. Why burn 11 BLD

The burn requirement creates a bridge between earlier XC participants and new X1 users.

Earlier participants may receive origin_bld and may choose to sell or transfer available BLD.

New X1 users may acquire 11 BLD and burn it to create a Build.

This creates demand for BLD without giving new users fake historical contribution.

---

## 11. Why native Build creation does not require XNTD lock

XNTD lock exists to reduce the Core redeem -> XNTD dump path.

Build creation through BLD burn does not create XNTD through Core redeem.

Therefore, it does not require XNTD lock or relock.

---

## 12. Relock integrity

Relock is allowed only if:

available_bld >= history_bld

This means the user must preserve or restore earned BLD before reducing or updating XNTD commitment.

origin_bld can be sold or used without breaking relock integrity, as long as available_bld remains greater than or equal to history_bld.

---

## 13. Main principles

- history_bld represents real XC Core redeem contribution.
- origin_bld is a Genesis allocation, not earned contribution.
- available_bld is the usable balance.
- 121 BLD is the maximum Genesis Origin cap, not the default.
- Build creation in X1 requires burning 11 BLD.
- Burning 11 BLD does not create history_bld.
- Native Build creation does not require XNTD lock or relock.


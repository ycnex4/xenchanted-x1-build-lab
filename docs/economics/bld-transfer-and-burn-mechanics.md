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

# BLD Transfer and Burn Mechanics

## 1. Purpose

This document describes the conceptual mechanics for BLD transfer, burn, and accounting.

It uses the current BLD terminology:

- history_bld
- available_bld
- origin_bld

The goal is to keep historical contribution separate from usable BLD balance.

---

## 2. Core distinction

BLD has two different meanings that must not be mixed:

1. Historical contribution.
2. Usable balance.

These are represented by different fields.

## history_bld

history_bld represents historical BLD created from redeemed Core NFT history.

It is non-decreasing.

It does not decrease when available BLD is sold, transferred, burned, or used.

## available_bld

available_bld represents the currently usable BLD balance.

It may increase or decrease through allowed mechanics.

## origin_bld

origin_bld represents Genesis Origin allocation.

It is not historical contribution.

It may increase available_bld, but it must not increase history_bld.

---

## 3. Sources of history_bld

Only redeemed Core NFT history creates history_bld.

Core redeem:

history_bld += normalized(Core.xenBurned)
available_bld += normalized(Core.xenBurned)

No other action creates history_bld.

Actions that must not create history_bld:

- Build creation through BLD burn
- Genesis Origin allocation
- BLD purchase
- BLD transfer
- BLD burn
- XBP update
- XNTD lock
- X1 Fee Contribution update

---

## 4. Sources of available_bld

available_bld may come from:

- Core redeem history
- Genesis Origin BLD
- received BLD transfer
- purchased BLD
- future approved mechanics

available_bld may decrease through:

- transfer
- sale
- burn
- use in approved mechanics
- Build creation through burn 11 BLD

---

## 5. Genesis Origin allocation

Genesis Origin BLD is added to origin_bld and available_bld.

When granted:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

It must not increase:

history_bld

---

## 6. BLD transfer

A BLD transfer changes available balances only.

## Sender accounting

sender.available_bld -= amount

## Recipient accounting

recipient.available_bld += amount

## Validation

- sender.available_bld must be greater than or equal to amount
- sender.history_bld must not decrease
- recipient.history_bld must not increase
- total available BLD supply must not increase through transfer

## Meaning

A transfer gives the recipient usable BLD.

It does not give the recipient the sender's historical contribution.

---

## 7. BLD sale

A sale is economically similar to transfer.

Seller:

available_bld decreases

Buyer:

available_bld increases

Neither side changes history_bld.

This is important because users may sell usable BLD without rewriting contribution history.

---

## 8. BLD burn

A BLD burn destroys available BLD.

## Accounting

available_bld -= amount

## Validation

- available_bld must be greater than or equal to amount
- history_bld must not decrease
- origin_bld must not decrease unless a separate origin accounting model is explicitly added
- burn reason must be supported

## Meaning

Burning BLD uses available power.

It does not erase historical participation.

---

## 9. Build creation through BLD burn

A user may create an active Build in X1 by burning BLD.

Requirement:

burn 11 BLD

## Accounting

available_bld -= 11

## Result

A Build is created and becomes active.

## Important

This does not create history_bld.

This does not create fake XEN burn history.

This does not require XNTD lock or relock.

---

## 10. Relock integrity

Relock is allowed only if:

available_bld >= history_bld

## Meaning

If a user has sold, transferred, burned, or used too much available BLD, the user must restore available_bld before relock.

This protects the integrity of the historical Build record.

The user can still sell or use origin_bld as long as available_bld remains greater than or equal to history_bld.

---

## 11. Example

Initial state:

history_bld = 100
origin_bld = 22
available_bld = 122

User sells 20 BLD:

history_bld = 100
origin_bld = 22
available_bld = 102

Relock allowed:

available_bld >= history_bld

102 >= 100

User sells 10 more BLD:

history_bld = 100
origin_bld = 22
available_bld = 92

Relock not allowed:

available_bld < history_bld

92 < 100

User must restore at least 8 BLD before relock.

---

## 12. Internal balance vs tokenized BLD

There are two possible implementation models.

## Model A: internal Build balance

BLD exists only as fields inside Build Program state.

Pros:

- simpler accounting
- easier to enforce relock integrity
- no separate token complexity

Cons:

- harder to trade freely
- requires custom transfer logic

## Model B: tokenized BLD

BLD exists as a token or token-like asset.

Pros:

- easier transfer / sale / market integration
- more composable in X1

Cons:

- harder to keep history_bld separate from available balance
- requires careful supply and burn accounting
- may need extra accounts or mint authority model

## Current direction

Do not finalize tokenization yet.

First define accounting rules.

Then decide whether available_bld should be represented as internal state, Token-2022 asset, or another X1-native asset model.

---

## 13. Main invariants

- history_bld is historical and non-decreasing.
- available_bld is usable and may increase or decrease.
- origin_bld is Genesis allocation, not history.
- Transfer changes available_bld only.
- Sale changes available_bld only.
- Burn decreases available_bld only.
- Build creation through burn 11 BLD does not create history_bld.
- Relock requires available_bld >= history_bld.
- No action should rewrite historical contribution.

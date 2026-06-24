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

# BLD Tokenization Decision

## 1. Purpose

This document describes the open design decision around BLD tokenization.

The goal is to decide whether available_bld should remain an internal Build Program balance or become a tokenized X1-native asset.

This is not implementation code.

---

## 2. Current BLD model

Current fields:

- history_bld
- available_bld
- origin_bld

## history_bld

Historical BLD from redeemed Core NFT history.

Non-decreasing.

Not spendable.

## available_bld

Usable BLD balance.

May be transferred, sold, burned, or used through approved mechanics.

## origin_bld

Genesis Origin allocation.

Not history.

Can add to available_bld.

---

## 3. Core problem

BLD has two meanings:

1. Historical contribution.
2. Usable economic unit.

These must remain separate.

Tokenization may help available_bld become composable, but it must not confuse available_bld with history_bld.

---

## 4. Option A: internal Build balance

In this model, available_bld exists only inside Build Program state.

## Pros

- simplest accounting
- easiest to enforce relock integrity
- no separate token mint authority
- no token account complexity
- easier to keep history_bld and available_bld separate

## Cons

- harder to trade
- harder to integrate with markets
- custom transfer logic needed
- less composable for other X1 projects

## Best use

This is best for MVP if the priority is correctness and simplicity.

---

## 5. Option B: tokenized BLD

In this model, available_bld exists as a token or token-like X1 asset.

## Pros

- easier transfers
- easier sale / market integration
- more composable for other X1 projects
- clearer external balance model

## Cons

- harder to preserve historical meaning
- token balance may be confused with history_bld
- requires mint / burn authority design
- requires supply reconciliation
- relock integrity becomes more complex
- token account state may diverge from Build view if not carefully designed

---

## 6. Option C: hybrid model

In this model:

- history_bld remains only inside BuildState
- available_bld may be represented by a token
- BuildState tracks or mirrors available_bld
- relock checks token balance or a locked BLD balance

## Pros

- preserves history inside Build
- gives available_bld composability
- supports markets and transfers

## Cons

- most complex
- requires synchronization rules
- requires clear source of truth
- requires lock / escrow or balance proof for relock
- more implementation risk

---

## 7. Relock requirement

Relock requires:

available_bld >= history_bld

If available_bld is internal, this is simple.

If available_bld is tokenized, relock must define which balance counts:

- wallet BLD balance
- Build-bound BLD balance
- locked BLD balance
- token account balance snapshot
- program-controlled escrow balance

This must be decided before implementation.

---

## 8. Build creation through BLD burn

Build creation requires:

burn 11 BLD

If BLD is internal:

- Build Program decreases available_bld
- creates Build

If BLD is tokenized:

- token burn must happen through an approved path
- Build Program must verify burn
- burn must not create history_bld

---

## 9. Supply integrity

The total available BLD supply should only change through allowed sources:

Supply increases:

- Core redeem creates available_bld
- Genesis Origin creates available_bld
- future approved mechanics, if any

Supply decreases:

- BLD burn
- Build creation burn
- future approved mechanics, if any

Transfers must not change total supply.

---

## 10. Recommended MVP direction

Recommended MVP direction:

Start with internal Build Program balance.

Reason:

- fewer moving parts
- easier replay protection
- easier relock integrity
- easier accounting audit
- less risk of confusing history_bld and available_bld

Tokenization can be revisited after the accounting model is stable.

---

## 11. Future tokenization path

If tokenization is added later, requirements should include:

- history_bld remains non-tokenized and non-transferable
- tokenized BLD represents available_bld only
- token minting is allowed only through valid source paths
- token burning is program-controlled or verifiable
- relock integrity has a clear balance source
- migration path from internal balances is documented
- external projects are warned not to treat token balance as history

---

## 12. Naming risk

If available_bld becomes a token, naming must be clear.

Possible display names:

- BLD
- available BLD
- Build BLD

Avoid implying that token balance equals historical contribution.

UI should show:

- History BLD
- Available BLD
- Origin BLD

---

## 13. Decision status

Current decision:

Do not tokenize BLD in MVP.

Use internal available_bld accounting first.

Revisit tokenization after:

- BuildState is implemented
- registrar updates are stable
- relock logic is tested
- reader interface is validated
- user flow is understood

---

## 14. Main invariants

- history_bld is never tokenized.
- history_bld is never transferable.
- available_bld may become transferable.
- origin_bld is not history.
- token balance must not be treated as contribution history.
- burn 11 BLD creates Build but not history.
- MVP should prioritize correctness over composability.

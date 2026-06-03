# Post-MVP BLD Composability Path

## 1. Purpose

This document describes the post-MVP path for BLD composability.

The goal is to keep MVP accounting simple while preserving a clear future path for BLD transferability, markets, integrations, and tokenization.

This is not implementation code.

---

## 2. Current MVP decision

BLD is not tokenized in MVP.

MVP uses internal Build Program accounting:

- history_bld
- available_bld
- origin_bld

This keeps the first implementation focused on correctness, replay protection, and state readability.

---

## 3. Why composability is deferred

BLD composability is deferred because tokenization or external transferability increases implementation risk.

Main risks:

- confusing available_bld with history_bld
- external token balance being treated as historical contribution
- relock integrity becoming harder
- supply reconciliation becoming harder
- external markets depending on unfinished accounting
- harder migration if MVP fields change

MVP should prove the accounting model first.

---

## 4. What composability may mean later

Post-MVP BLD composability may include:

- transfer of available_bld
- burn of available_bld
- market sale of available_bld
- tokenized BLD
- escrowed BLD
- BLD use by other X1 projects
- BLD-gated access
- BLD-based project rewards
- BLD liquidity mechanisms

None of these should change history_bld.

---

## 5. Non-negotiable invariant

history_bld is never composable.

history_bld is:

- historical
- non-decreasing
- non-transferable
- non-spendable
- not tokenized

Only available_bld can become composable.

---

## 6. Option A: internal transfer only

available_bld remains internal but can be transferred between Builds.

## Pros

- easier to preserve accounting rules
- no token mint authority
- no external token account mismatch
- relock checks stay simpler

## Cons

- less composable with external X1 projects
- custom transfer interface required
- less market-friendly

## Good for

First post-MVP transfer experiments.

---

## 7. Option B: tokenized available BLD

available_bld becomes a tokenized X1-native asset.

## Pros

- easy transfers
- easier market integration
- easier use by other X1 projects
- clearer external asset model

## Cons

- token balance may be mistaken for history
- requires mint / burn authority model
- requires supply reconciliation
- requires clear relock balance source
- higher implementation risk

## Requirement

Tokenized BLD must represent available_bld only.

---

## 8. Option C: escrowed / locked BLD

available_bld can be moved into a program-controlled escrow or lock account.

## Pros

- useful for relock integrity
- supports commitments
- supports project-specific utility
- can keep tokenized BLD controlled

## Cons

- more account complexity
- more user flow complexity
- requires clear unlock rules

## Good for

Future integrations that require provable available BLD commitment.

---

## 9. Relock and composability

Relock rule remains:

available_bld >= history_bld

If BLD becomes composable, the system must define what counts as available_bld for this check.

Possible sources:

- internal Build balance
- wallet token balance
- locked token account
- escrowed BLD
- sum of multiple program-controlled balances

This must be decided before tokenization.

---

## 10. Market interpretation risk

If BLD becomes tradable, users may think buying BLD buys historical contribution.

This must be avoided.

UI and docs should say:

- History BLD = earned historical contribution
- Available BLD = usable / transferable balance
- Origin BLD = Genesis allocation

Buying available BLD does not create history_bld.

---

## 11. External project integration

External X1 projects may use BLD in different ways.

They may read:

- history_bld for historical XC contribution
- available_bld for usable balance
- origin_bld for Genesis allocation
- xc_commitment_active for current XC commitment

Projects should not treat available_bld as proof of history.

---

## 12. Recommended post-MVP path

Recommended path:

1. Keep MVP internal only.
2. Implement and test all accounting invariants.
3. Add internal available_bld transfer only if needed.
4. Study relock implications.
5. Decide whether tokenized BLD is worth the extra complexity.
6. Only then design token mint / burn / escrow mechanics.

---

## 13. Decision status

Current decision:

Do not tokenize BLD in MVP.

Post-MVP direction:

Start with internal composability research before tokenization.

Tokenization remains possible but should require a separate design review.

---

## 14. Main invariants

- history_bld is never transferable.
- history_bld is never tokenized.
- available_bld may become composable later.
- buying available_bld does not create history_bld.
- origin_bld is not history.
- relock integrity must remain enforceable.
- MVP correctness comes before composability.

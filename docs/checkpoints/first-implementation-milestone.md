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

# First Implementation Milestone

## 1. Purpose

This document defines the first implementation milestone for the X1 Build MVP.

The goal is to start coding with the smallest useful scope and avoid mixing accounting logic too early.

---

## 2. Milestone name

First milestone:

build-mvp-scaffold

---

## 3. Milestone goal

Create the initial implementation scaffold for the X1 Build Program.

This milestone should prove that the project structure, build setup, test setup, and basic module organization are ready.

It should not implement real accounting logic yet.

---

## 4. Scope

Included:

- project scaffold
- base program / module structure
- instruction folder structure
- account / state folder structure
- error definitions
- placeholder tests
- README or checkpoint note if needed

Excluded:

- real BuildState accounting
- Core redeem accounting
- Genesis Origin BLD
- XEN Burn Power
- XNTD lock / relock
- X1 Fee Contribution checkpoints
- tokenized BLD
- bridge proof logic

---

## 5. Suggested branch

Branch name:

build-mvp-scaffold

Suggested commands:

git checkout main
git status --short
git checkout -b build-mvp-scaffold

---

## 6. Success condition

The milestone is complete when:

- project builds
- tests run
- folder structure is clear
- placeholder program entry points exist
- no accounting rules are partially implemented
- working tree is clean after commit

---

## 7. Why this milestone is intentionally small

The X1 Build model has several sensitive invariants:

- history_bld must remain separate from available_bld
- XBP must remain separate from BLD
- XNTD lock must remain commitment only
- replay protection must be explicit
- authorities must be limited by role

Starting with scaffold only reduces the risk of mixing these rules before the structure is ready.

---

## 8. Expected files / areas

Exact implementation paths depend on the chosen X1 tooling.

Conceptual areas:

- program entrypoint
- instructions
- state / accounts
- errors
- constants
- tests
- local config

No final file names are fixed by this document.

---

## 9. Initial tests

The first test set may include only smoke tests:

- project test runner works
- program compiles / builds
- placeholder instruction module loads
- placeholder account module loads

These tests are intentionally minimal.

---

## 10. Next milestone after scaffold

After scaffold is complete, the next implementation milestone should be:

BuildState account / object

That milestone should implement:

- owner
- build_id
- version
- created_at
- updated_at
- initial zeroed accounting fields

---

## 11. Main rule

Do not sneak accounting logic into the scaffold branch.

The scaffold branch should make future implementation easier, not hide unfinished protocol behavior.

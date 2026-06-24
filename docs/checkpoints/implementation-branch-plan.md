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

# Implementation Branch Plan

## 1. Purpose

This document describes the suggested branch plan for moving from X1 Build documentation to MVP implementation.

The goal is to keep implementation work small, reviewable, and aligned with the documented model.

---

## 2. Main rule

Do not implement the full MVP in one branch.

Each branch should have one clear purpose.

Each branch should end with:

- focused code changes
- focused tests
- updated checkpoint notes
- clean working tree

---

## 3. Base branch

Base branch:

main

Before starting implementation:

- confirm documentation is committed
- confirm git status is clean
- create implementation branch from main

Suggested command:

git checkout main
git status --short
git checkout -b build-mvp-scaffold

---

## 4. Branch 1: build-mvp-scaffold

Purpose:

Create project scaffold only.

Scope:

- program/project structure
- basic folders
- basic config
- placeholder modules
- build/test setup
- no real accounting logic yet

Expected result:

Project builds and tests run with placeholder tests.

Do not implement BuildState accounting in this branch.

---

## 5. Branch 2: buildstate-account

Purpose:

Implement BuildState account / object.

Scope:

- BuildState fields
- basic serialization / account sizing
- version field
- owner field
- created_at / updated_at
- initial empty state

Tests:

- create empty BuildState structure
- verify default values
- verify account size / field layout
- verify version

---

## 6. Branch 3: create-build

Purpose:

Implement create_build.

Scope:

- create canonical Build
- owner binding
- identity binding if needed
- duplicate Build prevention

Tests:

- user can create Build
- duplicate Build rejected
- create_build does not create history_bld
- create_build does not create XBP
- create_build does not create XNTD commitment
- create_build does not create fee contribution

---

## 7. Branch 4: registrar-message-replay-protection

Purpose:

Implement basic registrar message processing and replay protection.

Scope:

- processed_messages[message_id]
- registrar authority check
- message domain fields
- duplicate message rejection

Tests:

- valid registrar message accepted
- same message rejected second time
- unauthorized registrar rejected
- rejected message does not change state

---

## 8. Branch 5: core-redeem-bld

Purpose:

Implement Core redeem -> history_bld.

Scope:

- ADD_CORE_REDEEM instruction
- redeem_key protection
- history_bld update
- available_bld update
- source metadata storage if needed

Tests:

- valid redeem creates history_bld
- valid redeem creates available_bld
- duplicate redeem rejected
- history_bld is non-decreasing
- redeem cannot apply to multiple Builds

---

## 9. Branch 6: genesis-origin-bld

Purpose:

Implement Genesis Origin BLD.

Scope:

- tiered_origin_bld
- Build Genesis Epoch check
- genesis_origin_claimed[identity]
- origin_bld update
- available_bld update

Tests:

- tier 11 allocation
- tier 22 allocation
- tier 55 allocation
- tier 121 allocation
- duplicate claim rejected
- outside epoch rejected
- origin_bld does not increase history_bld

---

## 10. Branch 7: xen-burn-power

Purpose:

Implement XEN Burn Power.

Scope:

- ADD_XEN_BURN_POWER instruction
- xen_burn_key protection
- earned_xbp update
- available_xbp update

Tests:

- valid XEN burn creates XBP
- duplicate XEN burn rejected
- XBP does not create BLD
- BLD does not create XBP

---

## 11. Branch 8: xntd-lock-relock

Purpose:

Implement XNTD lock / unlock / relock.

Scope:

- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD
- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- relock integrity check

Tests:

- lock activates commitment when sufficient
- insufficient lock does not activate commitment
- unlock can deactivate commitment
- relock requires available_bld >= history_bld
- XNTD lock does not create BLD
- XNTD lock does not create XBP

---

## 12. Branch 9: x1-fee-contribution

Purpose:

Implement X1 Fee Contribution checkpoints.

Scope:

- update_fee_checkpoint
- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

Tests:

- newer checkpoint accepted
- stale checkpoint rejected
- fee checkpoint does not create BLD
- fee checkpoint does not create XBP
- fee checkpoint does not change XNTD lock

---

## 13. Branch 10: build-reader-interface

Purpose:

Implement reader interface.

Scope:

- minimal Build view
- full Build view if needed
- derived status fields

Tests:

- reader returns expected fields
- derived status fields are correct
- reader does not expose unnecessary internal protection maps

---

## 14. Branch 11: mvp-integration-tests

Purpose:

Add end-to-end MVP flow tests.

Test flow:

1. create Build
2. connect Core redeem history
3. grant Genesis Origin if eligible
4. add XEN Burn Power
5. lock XNTD
6. update fee contribution
7. read final Build state

Expected checks:

- all layers remain separate
- replay protection works
- reader output is clear
- no action creates unrelated accounting fields

---

## 15. Branch 12: implementation-checkpoint-update

Purpose:

Update documentation after implementation.

Scope:

- current-design-checkpoint.md
- implementation notes
- known limitations
- test status
- next steps

No new code unless required by review.

---

## 16. Merge policy

Recommended merge policy:

- one branch per layer
- review diff before merge
- run tests before merge
- keep main clean
- update checkpoint after important milestones

Do not merge branches with mixed unrelated changes.

---

## 17. Naming convention

Suggested branch names:

- build-mvp-scaffold
- buildstate-account
- create-build
- registrar-message-replay-protection
- core-redeem-bld
- genesis-origin-bld
- xen-burn-power
- xntd-lock-relock
- x1-fee-contribution
- build-reader-interface
- mvp-integration-tests
- implementation-checkpoint-update

---

## 18. Main success condition

Implementation is ready for MVP review when:

- all core instructions are implemented
- all replay protections are tested
- all contribution layers remain separate
- reader interface is stable
- documentation matches implementation
- working tree is clean

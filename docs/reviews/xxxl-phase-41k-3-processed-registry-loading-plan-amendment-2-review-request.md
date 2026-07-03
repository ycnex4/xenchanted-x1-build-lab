# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 2 Review Request

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Review target:

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`

## Scope

Пожалуйста, проверь Amendment 2 как fix к двум verdicts:

- Theo: REQUIRES FIXES
- Demon: REQUIRES FIXES

Amendment 2 должен закрыть два blocking issues:

1. `consumed == false` lifecycle;
2. 41J list-based registry view vs per-event PDA point-lookup reconciliation.

## Decisions Made In Amendment 2

1. Missing `AccountInfo` is rejected.
2. Supplied expected PDA in accepted uninitialized runtime representation means unprocessed / eligible.
3. Supplied wrong PDA is rejected.
4. Initialized `consumed == true` means already processed.
5. Initialized `consumed == false` is rejected as invalid lifecycle state.
6. Writable account is allowed, but 41K.3 still does not mutate.
7. 41J reconciliation chooses Option A: internal type-enforced adapter to existing 41J list-based interface.
8. Future adapter must be internal and type-enforced.
9. Existing `mark_processed_event_consumed(...)` is not accepted as live semantics without later 41K.4 review.
10. 41K.4 must bind marked amount / mint amount to the quorum-authorized payload.
11. Rent / close / recreate lifecycle risks are documented and carried forward to 41K.4 / 41K.5.
12. Active deployment blockers remain unchanged.

## Review Focus

Please verify:

- Does Amendment 2 fully resolve the `consumed == false` lifecycle blocker?
- Does Amendment 2 fully resolve the 41J reconciliation blocker?
- Is rejecting missing `AccountInfo` correct?
- Is treating supplied expected uninitialized PDA as unprocessed correct?
- Is rejecting initialized `consumed == false` correct?
- Is Option A adapter to existing 41J preferable for 41K.3 acceptance?
- Does this still preserve all accepted 41J invariants?
- Does 41K.3 still stay inside read/loading/classification boundary?
- Are rent / close / recreate lifecycle risks sufficiently documented for plan acceptance?
- Are deployment blockers still preserved?

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is Amendment 2 sufficient before 41K.3 plan acceptance:

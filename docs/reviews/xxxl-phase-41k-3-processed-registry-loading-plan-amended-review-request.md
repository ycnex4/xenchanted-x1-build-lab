# XXXL Phase 41K.3 — Amended Processed-Registry Loading Plan Review Request

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Base plan:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`

Amendment:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`

## Scope

Пожалуйста, проводите review по base plan together with Amendment 1.

Amendment 1 supersedes earlier wording where it conflicts with the base plan.

This is still plan review, not code review.

## Why Amendment 1 Exists

Pre-review guidance found three important issues in the first 41K.3 plan wording:

1. Missing / uninitialized processed-event PDA should not automatically reject.
2. Writable processed-event account should not be rejected solely for writability.
3. Current 41J list-based registry view must be reconciled with per-event PDA lookup.

## Updated Review Focus

Please verify especially:

1. Missing / uninitialized expected processed-event PDA should mean unprocessed, not replay rejection.
2. Expected processed-event PDA key must still be derived and checked.
3. 41K.3 must remain read-only in behavior.
4. Writable processed-event account should not be rejected solely for writability.
5. Initialized `consumed == true` should mean already processed.
6. Initialized `consumed == false` lifecycle must be explicitly accepted or rejected.
7. Current 41J list-based registry interface must be reconciled with per-event PDA lookup.
8. Successful 41K.3 loading must later feed an authoritative processed-registry view through a type-enforced adapter or an explicit 41J interface refinement.
9. 41K.3 must still not enable replay write, processed marking, mutation, CPI, mint, handler or live route.

## Specific Questions

1. Is PDA seed format `["xxxl", "processed-event", canonical_event_key]` correct?
2. What exact runtime representation should code use for missing / uninitialized expected processed-event PDA?
3. Should initialized `consumed == false` be supported, or should initialized processed-event PDA imply processed?
4. Should 41K.3 adapt to current 41J list interface, or should 41J get a point-lookup runtime interface?
5. Are `canonical_event_key`, `route_id`, and `recipient` sufficient identity fields for initialized processed-event PDA validation?
6. Are there close/recreate/rent/uninitialized-account lifecycle risks that must be pinned down before code?

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is amended plan sufficient before 41K.3 implementation:

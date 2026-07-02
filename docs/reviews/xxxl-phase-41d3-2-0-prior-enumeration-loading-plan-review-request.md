# XXXL Phase 41D3.2.0 Review Request — Prior Enumeration / Checked Loading Plan

Date: 2026-07-02

## Review Target

`docs/xxxl/xxxl-phase-41d3-2-0-prior-enumeration-loading-plan.md`

## Current Main Baseline

`e52d8ac Merge XXXL phase 41D3 current index external acceptance record`

## Scope

Docs-only plan for splitting the accepted Phase 41D3 next sub-step.

No code is introduced.

## Requested Review

Confirm whether this split is safe:

1. Phase 41D3.2.1 — prior index range enumeration only;
2. Phase 41D3.2.2 — checked prior instruction loading via `load_instruction_at_checked`;
3. Phase 41D3.2.3 — prefilter + Phase 41C3 descriptors + explicit same/later reject.

## Key Boundary Questions

1. Is it correct to split prior enumeration and instruction loading?
2. Is `current_index == 0 => empty prior range` the correct fail-closed behavior?
3. Is strict prior ordering `< current_index` sufficient for the enumeration layer?
4. Should same-index and later-index reject be explicit at runtime boundary?
5. Is `load_instruction_at_checked` the correct checked helper for the loading layer?
6. Should `load_instruction` and unchecked loading remain forbidden?
7. Should raw Instructions sysvar byte parsing remain forbidden?
8. Is it safe to construct Phase 41C3 descriptors without accepting evidence?
9. Are the expected safety flag flips limited to:
   - `locates_prior_ed25519_instruction: true`;
   - `load_instruction_called: true`;
10. Do all proof/evidence/auth/replay/CPI/mint/live-route boundaries remain closed?
11. Is this plan narrow enough to start 41D3.2.1 code after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope acceptable: yes/no
- Split acceptable: yes/no
- Next code sub-step allowed: yes/no

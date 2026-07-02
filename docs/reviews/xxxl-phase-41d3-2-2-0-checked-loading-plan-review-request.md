# XXXL Phase 41D3.2.2.0 Review Request — Checked Prior Instruction Loading Plan

Date: 2026-07-02

## Review Target

`docs/xxxl/xxxl-phase-41d3-2-2-0-checked-prior-instruction-loading-plan.md`

## Current Main Baseline

`9880d63 Merge XXXL phase 41D3 prior index range acceptance record`

## Scope

Docs-only plan for checked prior instruction loading.

No code is introduced.

## Requested Review

Confirm whether this boundary is safe for the next code sub-step:

- accept bounded prior indexes from Phase 41D3.2.1;
- iterate prior indexes lazily;
- call `load_instruction_at_checked` only for prior indexes;
- avoid unchecked loading;
- avoid raw sysvar parsing;
- avoid descriptor construction;
- keep loaded instructions non-authorizing.

## Key Boundary Questions

1. Is a docs-only plan appropriate before introducing `load_instruction_at_checked`?
2. Is lazy iteration over the prior range the right loading pattern?
3. Should the implementation avoid materializing a second large index/loading buffer?
4. Is `load_instruction_at_checked` the only acceptable loading helper?
5. Should `load_instruction` and `load_instruction_at` remain forbidden?
6. Should raw Instructions sysvar byte parsing remain forbidden?
7. Should empty prior range cause no loading attempt?
8. Should checked loading failure be deterministic and non-panicking?
9. Should Phase 41D3.2.2 avoid prefiltering and descriptors?
10. Should `locates_prior_ed25519_instruction` remain false in 41D3.2.2?
11. Are the only allowed flag flips loading-related:
    - `prior_instruction_loading_enabled`;
    - `load_instruction_called`;
    - `load_instruction_enabled`;
12. Do proof/evidence/quorum/auth/replay/CPI/mint/live-route boundaries remain closed?
13. Is Phase 41D3.2.2 code allowed after this plan is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope acceptable: yes/no
- Loading boundary acceptable: yes/no
- Next code sub-step allowed: yes/no

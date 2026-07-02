# XXXL Phase 41D3.2.1 Review Request — Prior Index Range Boundary

Date: 2026-07-02

## Review Target

- `programs/xxxl-svm/src/verifier/prior_instruction_index_range_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d3-2-1-prior-index-range-runtime-boundary.md`

## Current Main Baseline

`b1c17cd Merge XXXL phase 41D3 prior enumeration plan acceptance record`

## Scope

Code sub-step for prior index range construction only.

## Requested Review

Confirm whether Phase 41D3.2.1 safely implements only:

- accepting checked current index from Phase 41D3.1;
- constructing bounded prior range `0..current_index`;
- mapping `current_index == 0` to empty range;
- preserving strict `< current_index`;
- failing closed if current index is unavailable or inconsistent;
- avoiding all loading/parsing/descriptor/proof/auth behavior.

## Key Boundary Questions

1. Is the code limited to pure prior index range construction?
2. Does `current_index == 0` correctly map to an empty prior range?
3. Is every constructed prior index strictly `< current_index`?
4. Are same-index and later-index values excluded by construction?
5. Is the unavailable current-index path fail-closed?
6. Is the forged oversized current-index path fail-closed without allocation?
7. Is there any instruction loading?
8. Is there any raw sysvar parsing?
9. Is there any Phase 41C3 descriptor construction?
10. Are any safety flags flipped?
11. Are all proof/evidence/auth/replay/CPI/mint/live-route boundaries still closed?
12. Can Phase 41D3.2.2 checked loading start after this phase is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Forbidden operations detected: yes/no
- Trust-sensitive boundary drift: yes/no
- Next code sub-step allowed: yes/no

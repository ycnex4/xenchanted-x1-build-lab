# XXXL Phase 41D3.2.2 Review Request — Checked Prior Instruction Loading Boundary

Date: 2026-07-02

## Review Target

- `programs/xxxl-svm/src/verifier/checked_prior_instruction_loading_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d3-2-2-checked-prior-instruction-loading-runtime-boundary.md`

## Current Main Baseline

`5b8850e Merge XXXL phase 41D3 checked loading plan acceptance record`

## Scope

Code sub-step for checked prior instruction loading only.

## Requested Review

Confirm whether Phase 41D3.2.2 safely implements only:

- accepting bounded prior range from Phase 41D3.2.1;
- verifying Instructions sysvar account key before loading;
- empty prior range => no loading attempt;
- iterating prior indexes lazily with `.iter().copied()`;
- calling `load_instruction_at_checked` only for prior indexes;
- deterministic success/failure mapping;
- loaded instruction remains runtime data only;
- no prefilter/descriptors/evidence/auth behavior.

## Key Boundary Questions

1. Is the code limited to checked prior instruction loading?
2. Does empty prior range cause no loading attempt?
3. Does missing/wrong Instructions sysvar fail closed before loading?
4. Is `load_instruction_at_checked` the only loading helper used?
5. Are `load_instruction`, `load_instruction_at`, and unchecked loading absent?
6. Is raw sysvar parsing absent?
7. Is direct sysvar byte slicing absent?
8. Is iteration over prior indexes lazy enough for this boundary?
9. Does checked loading failure map deterministically without panic?
10. Does loaded instruction remain runtime data only?
11. Are prefiltering and Phase 41C3 descriptors still absent?
12. Does `locates_prior_ed25519_instruction` remain false?
13. Are only loading-related flags flipped?
14. Are all proof/evidence/auth/replay/CPI/mint/live-route boundaries still closed?
15. Can Phase 41D3.2.3 prefilter/descriptor construction start after this phase is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Forbidden operations detected: yes/no
- Trust-sensitive boundary drift: yes/no
- Next code sub-step allowed: yes/no

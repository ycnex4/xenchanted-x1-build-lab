# XXXL Phase 41D3.2.3 Review Request — Prefilter + Phase 41C3 Candidate Descriptor Boundary

Date: 2026-07-02

## Review Target

- `programs/xxxl-svm/src/verifier/prefilter_phase_41c3_candidate_descriptor_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d3-2-3-prefilter-descriptor-runtime-boundary.md`

## Current Main Baseline

`c6bbf72 Merge XXXL phase 41D3 prefilter descriptor plan acceptance record`

## Scope

Code sub-step for structural prefiltering and Phase 41C3 candidate descriptor construction only.

## Requested Review

Confirm whether Phase 41D3.2.3 safely implements only:

- consuming loaded prior instructions from Phase 41D3.2.2;
- processing runtime-data-only entries;
- prefiltering unrelated loaded instructions by Ed25519 program id;
- constructing non-authorizing Phase 41C3 candidate descriptors;
- delegating ordering/duplicate/ambiguous handling to existing Phase 41C3 model;
- explicitly covering same/later candidate reject through Phase 41C3;
- flipping `locates_prior_ed25519_instruction: true` only as structural candidate location.

## Key Boundary Questions

1. Is the code limited to structural prefiltering and descriptor construction?
2. Are loaded prior entries processed by reference?
3. Are unrelated non-candidates discarded immediately?
4. Does descriptor construction avoid cloning full Instruction data?
5. Are candidate descriptors non-authorizing?
6. Does the code delegate duplicate/ambiguous/ordering handling to Phase 41C3?
7. Are same-index candidates rejected?
8. Are later-index candidates rejected?
9. Is `locates_prior_ed25519_instruction: true` limited to structural candidate location?
10. Are cryptographic verification and proof acceptance absent?
11. Is verification evidence acceptance absent?
12. Are quorum/auth/replay/mutation/CPI/mint/live-route boundaries still closed?
13. Are all trust-sensitive flags except structural location still false?
14. Can the next phase start only after this code is externally accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Forbidden operations detected: yes/no
- Trust-sensitive boundary drift: yes/no
- Descriptor boundary acceptable: yes/no
- Next phase allowed: yes/no

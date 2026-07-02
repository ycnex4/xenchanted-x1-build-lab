# XXXL Phase 41D3.2.3.0 Review Request — Prefilter + Phase 41C3 Candidate Descriptor Plan

Date: 2026-07-02

## Review Target

`docs/xxxl/xxxl-phase-41d3-2-3-0-prefilter-descriptor-plan.md`

## Current Main Baseline

`0cb2478 Merge XXXL phase 41D3 checked prior loading acceptance record`

## Scope

Docs-only plan for prefiltering and Phase 41C3 candidate descriptor construction.

No code is introduced.

## Requested Review

Confirm whether this boundary is safe for the next code sub-step:

- consume loaded prior instructions from Phase 41D3.2.2;
- prefilter unrelated instructions;
- identify Ed25519 program-id candidates structurally;
- construct non-authorizing Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- allow `locates_prior_ed25519_instruction: true` only as structural candidate location.

## Key Boundary Questions

1. Is a docs-only plan appropriate before first `locates_prior_ed25519_instruction: true` flip?
2. Is the distinction clear between structural candidate location and evidence acceptance?
3. Is prefiltering by program id safe at this phase?
4. Are Phase 41C3 candidate descriptors correctly non-authorizing?
5. Should same-index candidates be explicitly rejected even though previous layers exclude them?
6. Should later-index candidates be explicitly rejected even though previous layers exclude them?
7. Is the streaming / heap guidance sufficient for code phase?
8. Should descriptor storage avoid cloning full instruction data where possible?
9. Should malformed structural candidates remain deterministic and non-authorizing?
10. Should duplicate/ambiguous structural candidates remain deterministic and non-authorizing?
11. Is `locates_prior_ed25519_instruction: true` the only new trust-sensitive flag allowed?
12. Do crypto/proof/evidence/quorum/auth/replay/CPI/mint/live-route boundaries remain closed?
13. Can Phase 41D3.2.3 code start after this plan is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope acceptable: yes/no
- Descriptor boundary acceptable: yes/no
- Trust-sensitive wording acceptable: yes/no
- Next code sub-step allowed: yes/no

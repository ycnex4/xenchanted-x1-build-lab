# XXXL Phase 41E.0 Review Request — Ed25519 Instruction Byte Parsing Plan

Date: 2026-07-02

## Review Target

`docs/xxxl/xxxl-phase-41e-0-ed25519-byte-parsing-plan.md`

## Current Main Baseline

`99ba836 Merge XXXL phase 41D3 structural prior lookup closure`

## Scope

Docs-only plan.

No runtime code is introduced.

## Requested Review

Confirm whether Phase 41E.0 is an acceptable next plan after Phase 41D3 closure.

Planned future boundary:

- real Ed25519 instruction byte parsing;
- deterministic malformed/out-of-bounds classification;
- non-authorizing parsed metadata;
- no verification;
- no proof acceptance;
- no evidence acceptance;
- no quorum/auth/replay/mutation/CPI/mint/live route.

## Key Review Questions

1. Is byte parsing an acceptable next micro-phase after Phase 41D3 closure?
2. Is the required entry gate correct:
   - `status == PriorEd25519InstructionStructurallyLocated`;
   - `matched_instruction_index.is_some()`?
3. Is it clear that `locates_prior_ed25519_instruction` must not be used as an evidence gate?
4. Is it clear that Phase 41D3.2.3 descriptor booleans are not validated evidence?
5. Is parsing real Ed25519 instruction bytes allowed without verification?
6. Are parsed fields correctly non-authorizing?
7. Are malformed/out-of-bounds/ambiguous cases fail-closed?
8. Are panic-safety requirements sufficient?
9. Are heap/allocation requirements sufficient?
10. Is a parsing-specific flag acceptable if it means parsing only?
11. Are crypto/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route still closed?
12. Can Phase 41E byte parsing code begin after this plan is accepted?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Entry gate acceptable: yes/no
- Descriptor boolean guardrail acceptable: yes/no
- Byte parsing boundary acceptable: yes/no
- Trust-sensitive boundary drift: yes/no
- Next code sub-step allowed: yes/no

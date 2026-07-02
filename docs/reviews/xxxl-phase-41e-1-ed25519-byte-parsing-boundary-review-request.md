# XXXL Phase 41E.1 Review Request — Ed25519 Instruction Byte Parsing Boundary

Date: 2026-07-02

## Review Target

- `programs/xxxl-svm/src/verifier/ed25519_instruction_byte_parsing_boundary.rs`
- `docs/xxxl/xxxl-phase-41e-1-ed25519-byte-parsing-boundary.md`

## Current Main Baseline

`e550a51 Merge XXXL phase 41E Ed25519 byte parsing plan acceptance record`

## Scope

Code boundary for non-authorizing Ed25519 instruction byte parsing only.

## Requested Review

Confirm whether Phase 41E.1 safely implements only:

- entry gate on located status plus matched instruction index;
- no gate on `locates_prior_ed25519_instruction`;
- no trust in Phase 41D3.2.3 descriptor booleans;
- parsing already loaded matched Ed25519 instruction bytes;
- no new loading surface;
- rejecting cross-instruction references;
- storing variable-length message as bounded indices;
- deterministic overlap rejection;
- checked offset arithmetic;
- non-authorizing parsed metadata;
- no signature verification/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route.

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Forbidden operations detected: yes/no
- Entry gate acceptable: yes/no
- Cross-instruction reference policy acceptable: yes/no
- Message range allocation policy acceptable: yes/no
- Overlap policy acceptable: yes/no
- Trust-sensitive boundary drift: yes/no
- Next phase allowed: yes/no

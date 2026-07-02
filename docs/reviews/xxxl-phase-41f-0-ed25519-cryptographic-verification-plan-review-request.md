# XXXL Phase 41F.0 Review Request — Ed25519 Cryptographic Verification Plan

Date: 2026-07-02

## Review Target

`docs/xxxl/xxxl-phase-41f-0-ed25519-cryptographic-verification-plan.md`

## Current Main Baseline

`2f759b7 Merge XXXL phase 41E offset table hardening acceptance record`

## Scope

Docs-only plan.

No runtime code is introduced.

## Requested Review

Confirm whether Phase 41F.0 is an acceptable next docs-only plan after Phase 41E completion.

Key issue:

Phase 41F must introduce or plan signature validity without accidentally accepting proof, evidence, guardian validity, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.

## Review Questions

1. Is Phase 41F.0 the correct next docs-only plan after Phase 41E completion?
2. Is the separation between signature validity and proof/evidence/auth clear enough?
3. Should Phase 41F prefer the native Ed25519 instruction verification model?
4. Should local cryptographic verification be deferred unless explicitly reviewed?
5. Should checked byte extraction be a separate Phase 41F.1 gate before signature verification?
6. Is message range verification clearly not message correctness/proof acceptance?
7. Is parsed public key use clearly not guardian validity?
8. Are fail-closed requirements sufficient?
9. Are active blockers preserved?
10. Can Phase 41F.1 checked byte extraction planning/code begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Native verification model preference acceptable: yes/no
- Checked extraction sub-phase acceptable: yes/no
- Signature/proof separation acceptable: yes/no
- Trust-sensitive boundary drift: yes/no
- Next sub-phase allowed: yes/no

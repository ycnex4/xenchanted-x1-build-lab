# XXXL Phase 41F.1 Review Request — Checked Ed25519 Byte Extraction Boundary

Date: 2026-07-02

## Review Target

Code:

`programs/xxxl-svm/src/verifier/checked_ed25519_byte_extraction_boundary.rs`

Docs:

`docs/xxxl/xxxl-phase-41f-1-checked-byte-extraction-boundary.md`

## Current Baseline

Parent accepted checkpoint:

`e45869c Merge XXXL phase 41F Ed25519 verification plan acceptance record`

## Scope

Phase 41F.1 is intended to do only checked byte extraction.

It must not perform cryptographic verification.

It must not establish native Ed25519 verification.

It must not accept proof/evidence/guardian/quorum/auth.

## Requested Review

Please check:

1. Does the boundary consume Phase 41E parsed offsets safely?
2. Does it extract signature as checked `&[u8; 64]`?
3. Does it extract public key as checked `&[u8; 32]`?
4. Does it expose message as checked borrowed `&[u8]` without attacker-sized `Vec` copy?
5. Are all range accesses checked?
6. Are unchecked indexing/slicing, unwrap/expect/panic/unsafe absent?
7. Does it avoid local crypto verification?
8. Does it avoid native verification establishment?
9. Does it keep `ed25519_signature_verification_performed` false?
10. Does it avoid proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live drift?
11. Are tests sufficient?
12. Can Phase 41F.2 planning begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Scope violations: yes/no
- Checked extraction acceptable: yes/no
- Message borrow/no-copy acceptable: yes/no
- Forbidden operations detected: yes/no
- Signature verification drift: yes/no
- Trust-sensitive boundary drift: yes/no
- Next phase allowed: yes/no

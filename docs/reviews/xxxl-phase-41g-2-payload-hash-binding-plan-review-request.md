# XXXL Phase 41G.2 Review Request — Payload Hash Binding Plan

Date: 2026-07-03

## Current Main

`c89fc59 Merge XXXL phase 41G payload evidence shape acceptance`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41g-2-payload-hash-binding-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41g-2-payload-hash-binding-plan.md`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Plan the narrow binding relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

## Required Existing Boundary Reuse

- `programs/xxxl-svm/src/verifier/raw_payload.rs`
- `programs/xxxl-svm/src/verifier/canonical_payload.rs`
- `compute_guardian_payload_hash`
- `validate_guardian_payload_hash`
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1`
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1`

## Exact Domain Separator Requirement

The domain separator is:

`keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1"))`

The payload hash preimage is:

`domain_separator_32_bytes || raw_payload_bytes`

The implementation must not prepend the literal UTF-8 label bytes directly.

## Requested Review

Please check:

1. Is 41G.2 the correct next step after accepted 41G.1?
2. Is the exact domain separator preimage specified correctly?
3. Is the hash relation `signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)` correct?
4. Is reuse of `canonical_payload.rs` mandatory enough?
5. Is raw payload provenance/trust correctly stated?
6. Are structural decode and authenticity correctly separated?
7. Is hash comparison still separated from guardian/quorum/auth?
8. Are guardian set ID, public key, finality, and expiration correctly bound but not validated?
9. Are negative cases sufficient?
10. Are forbidden operations preserved?
11. Can Phase 41G.2 implementation planning begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Correct next phase after 41G.1: yes/no
- Exact domain separator preimage acceptable: yes/no
- Hash relation acceptable: yes/no
- Existing canonicalizer reuse sufficient: yes/no
- Raw payload provenance/trust acceptable: yes/no
- Structural decode/authenticity separation acceptable: yes/no
- Guardian/quorum/auth separation acceptable: yes/no
- Guardian/finality/expiration deferral acceptable: yes/no
- Negative matrix sufficient: yes/no
- Forbidden operations preserved: yes/no
- Phase 41G.2 implementation planning allowed after acceptance: yes/no

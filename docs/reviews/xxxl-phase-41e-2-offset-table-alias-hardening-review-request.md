# XXXL Phase 41E.2 Review Request — Offset-Table Alias Hardening

Date: 2026-07-02

## Review Target

- `programs/xxxl-svm/src/verifier/ed25519_instruction_byte_parsing_boundary.rs`
- `docs/xxxl/xxxl-phase-41e-2-offset-table-alias-hardening.md`

## Current Main Baseline

`ac7fcb3 Merge XXXL phase 41E Ed25519 byte parsing boundary acceptance record`

## Scope

Parser hardening only.

## Requested Review

Confirm whether Phase 41E.2 correctly addresses the Phase 41E.1 Audit Demon non-blocking note:

- parsed ranges must not alias the Ed25519 header/offset-table range `[0, 16)`;
- signature/public-key/message ranges must start at or after `ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- failure is deterministic and non-authorizing;
- no verification/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route is introduced.

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Blocking risks:
- Scope violations: yes/no
- Offset-table alias hardening acceptable: yes/no
- Forbidden operations detected: yes/no
- Trust-sensitive boundary drift: yes/no
- Next phase allowed: yes/no

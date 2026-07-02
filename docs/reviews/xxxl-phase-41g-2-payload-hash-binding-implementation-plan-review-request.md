# XXXL Phase 41G.2 Review Request — Payload Hash Binding Implementation Plan

Date: 2026-07-03

## Current Main

`0825dad Merge XXXL phase 41G payload hash binding plan acceptance`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41g-2-payload-hash-binding-implementation-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41g-2-payload-hash-binding-implementation-plan.md`

## Scope

Docs-only implementation planning checkpoint.

No runtime code.

No `.rs` changes.

No verification logic change.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Plan the future implementation of the narrow payload hash binding boundary.

Preferred future validation path:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

## Proposed Future Flow

1. require Phase 41F verification established;
2. require `signed_message_bytes.len() == 32`;
3. checked-convert to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. success means only payload hash binding established;
6. failure is fail-closed.

## Requested Review

Please check:

1. Is implementation planning the correct next step after accepted 41G.2 plan?
2. Is the proposed future function boundary narrow enough?
3. Is `validate_guardian_payload_hash` reuse strong enough?
4. Is the signed message length and checked 32-byte conversion flow correct?
5. Is the result/status model narrow enough?
6. Is the error model fail-closed enough?
7. Are report/safety flags sufficient and non-authorizing?
8. Are tests sufficient?
9. Are Stage-1 / Phase 33 / Phase 34 parity requirements sufficient?
10. Are guardian/quorum/auth/replay/mutation/CPI/mint/live route still excluded?
11. Can Phase 41G.2 implementation begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Correct next phase after accepted 41G.2 plan: yes/no
- Function boundary narrow enough: yes/no
- validate_guardian_payload_hash reuse sufficient: yes/no
- Signed message length / checked conversion flow acceptable: yes/no
- Result/status model narrow enough: yes/no
- Error model fail-closed enough: yes/no
- Report/safety flags acceptable: yes/no
- Tests sufficient: yes/no
- Vector parity requirements sufficient: yes/no
- Forbidden operations preserved: yes/no
- Phase 41G.2 implementation allowed after acceptance: yes/no

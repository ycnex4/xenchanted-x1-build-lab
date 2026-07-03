# XXXL Phase 41H.2 — Signed Message Binding Hardening Plan Review Request

Date: 2026-07-03

Status: review request

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41h-2-signed-message-binding-hardening`

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Primary plan:

`docs/xxxl/xxxl-phase-41h-2-signed-message-binding-hardening-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-2-signed-message-binding-hardening-plan.md`

## Review Scope

This is a docs-only planning review.

No 41H.2 code has been written.

No `.rs` files are changed.

## Why This Exists

During Phase 41I high-risk audit, Audit Demon found a blocking inherited soundness gap.

Current chain proves two separate things:

1. 41F proves guardian Ed25519 verification over the message bytes extracted from the Ed25519 instruction.
2. 41G proves a separate `signed_message_bytes` input equals the canonical hash of `raw_payload_bytes`.

But the current chain does not prove:

`41F-verified message bytes == signed_message_bytes == canonical hash(raw_payload_bytes)`

This means a valid guardian signature over arbitrary message bytes could be paired with a separate payload hash.

41I must remain blocked until this is closed.

## Proposed Closure

41H must stop accepting free `signed_message_bytes`.

41H must derive the signed message from:

`extraction_result.extracted_slices.message_bytes`

Then 41H must call:

`establish_payload_hash_binding(raw_payload_bytes, extracted_slices.message_bytes, phase_41f_result)`

This makes the signed message operand authoritative from 41F extraction.

## Required 41H API Direction

Before:

`establish_guardian_membership_validation(..., raw_payload_bytes, signed_message_bytes, expected_configured_guardian_set_id, guardian_set)`

After:

`establish_guardian_membership_validation(..., raw_payload_bytes, expected_configured_guardian_set_id, guardian_set)`

41H derives signed message bytes internally.

## Required Invariants

41H.2 must enforce:

- extracted 41F message bytes exist;
- extracted message length is exactly 32;
- 41G receives extracted message bytes, not caller-supplied bytes;
- free signed message input is removed;
- decoded payload remains internally decoded from `raw_payload_bytes`;
- guardian set ID linkage remains unchanged;
- downstream flags remain false.

## Proposed Error / Report Additions

Proposed error kind:

`ExtractedSignedMessageLengthInvalid`

Proposed report fields:

- `derives_signed_message_from_41f_extraction: true`
- `accepts_free_signed_message_input: false`
- `checks_extracted_signed_message_len_32: true`
- `binds_41f_verified_message_to_payload_hash: true`

## Still Forbidden

41H.2 must not add:

- quorum counting;
- authorization marker;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- instruction handler;
- live route.

## Review Questions

Check:

1. Does this plan close the signed-message binding gap?
2. Is removing free `signed_message_bytes` from 41H the correct closure?
3. Is deriving the signed message from `extraction_result.extracted_slices.message_bytes` correct?
4. Is requiring extracted message length exactly 32 correct?
5. Is calling 41G with extracted message bytes correct?
6. Are the proposed error/report additions sufficient?
7. Are test requirements sufficient?
8. Are forbidden runtime surfaces excluded?
9. Is 41I correctly blocked until this is implemented and accepted?
10. Is this plan sufficient before writing 41H.2 code?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- signed-message binding gap closed by plan: yes/no
- free signed message removed: yes/no
- 41F extracted message used as source: yes/no
- extracted message len check sufficient: yes/no
- 41G call direction correct: yes/no
- 41I correctly blocked: yes/no
- forbidden runtime surfaces absent: yes/no
- plan sufficient before code: yes/no

## High-Risk Audit Required Fix Incorporated

Audit Demon identified that the plan must also bind `verified_ranges.message_range` to `extracted_slices.message_range`.

The plan now requires 41H.2 to check:

- public key range binding;
- message range binding;
- signature range binding.

Mandatory new error kind:

- `VerifiedMessageRangeMismatch`

Recommended additional error kind:

- `VerifiedSignatureRangeMismatch`

The review should now also confirm:

- message range binding required: yes/no
- signature range binding included: yes/no
- arbitrary-M range-pairing attack closed: yes/no

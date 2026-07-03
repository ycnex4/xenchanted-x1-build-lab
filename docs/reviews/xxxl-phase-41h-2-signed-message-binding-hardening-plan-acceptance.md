# XXXL Phase 41H.2 — Signed Message Binding Hardening Plan Acceptance

Date: 2026-07-03

Status: accepted plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41h-2-signed-message-binding-hardening`

Accepted commit:

`0e043f5 Document phase 41H signed message binding hardening plan`

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

## Accepted Documents

- `docs/xxxl/xxxl-phase-41h-2-signed-message-binding-hardening-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-2-signed-message-binding-hardening-plan.md`
- `docs/reviews/xxxl-phase-41h-2-signed-message-binding-hardening-plan-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Final Verdict

Phase 41H.2 signed message binding hardening plan is accepted.

Required fixes: none.

Blocking risks: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- signed-message binding gap fully closed by plan: yes
- message range binding required and included: yes
- signature range binding included: yes
- arbitrary-M range-pairing attack closed: yes
- plan sufficient before code: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Required Fix 1 closed now: yes
- message range binding sufficient: yes
- signature range binding sufficient: yes
- arbitrary-M range-pairing attack closed: yes
- free signed_message_bytes removed: yes
- extracted message is sole signed message source: yes
- 41G call direction sound: yes
- tests now sufficient as planned: yes
- 41I correctly blocked: yes
- forbidden runtime surfaces absent: yes
- plan sufficient before code: yes

## Accepted Closure

41H.2 must remove free `signed_message_bytes`.

41H.2 must derive the signed message only from:

`extraction_result.extracted_slices.message_bytes`

41H.2 must prove that the extracted message bytes are the same bytes covered by native Ed25519 verification.

Accepted required range bindings:

- `verified_ranges.public_key_range == extracted_slices.public_key_range`
- `verified_ranges.message_range == extracted_slices.message_range`
- `verified_ranges.signature_range == extracted_slices.signature_range`

Accepted signed message length check:

- `extracted_slices.message_bytes.len() == 32`

Accepted 41G call:

`establish_payload_hash_binding(raw_payload_bytes, extracted_slices.message_bytes, phase_41f_result)`

## Accepted Proof Chain

After implementation, 41H.2 must establish:

1. 41F.2 establishes native Ed25519 verification.
2. 41F.2 verified ranges exist.
3. 41F.1 extracted slices exist.
4. matched instruction index matches.
5. instruction data length matches.
6. verified public key range equals extracted public key range.
7. verified message range equals extracted message range.
8. verified signature range equals extracted signature range.
9. extracted message length is exactly 32.
10. 41G receives `raw_payload_bytes` and `extracted_slices.message_bytes`.
11. 41G proves extracted message bytes equal canonical hash of `raw_payload_bytes`.
12. 41H proves extracted public key is a member of the authoritative guardian set.

Conclusion:

`guardian signed canonical_hash(raw_payload_bytes)`

## Accepted Error Kinds

41H.2 implementation must include:

- `ExtractedSignedMessageLengthInvalid`
- `VerifiedMessageRangeMismatch`
- `VerifiedSignatureRangeMismatch`

Existing public key range mismatch handling remains required.

## Downstream Status

Phase 41I remains blocked until 41H.2 code is implemented, reviewed, and accepted.

After this plan acceptance is merged, the next branch may implement 41H.2 code.

## Still Forbidden

41H.2 plan acceptance does not allow:

- quorum counting;
- authorization marker;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- instruction handler;
- live route;
- production program ID;
- production guardian account loading.

# XXXL Phase 41H.2 — Signed Message Binding Hardening Plan

Date: 2026-07-03

Status: planning only

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Blocked downstream branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-plan`

Blocked downstream commit:

`e43fd42 Document phase 41I quorum authorization plan`

## Purpose

Phase 41H.2 closes the signed-message binding gap found during the Phase 41I high-risk audit.

41H.1 closed the decoded-payload gap by removing free `DecodedGuardianPayloadRaw` from the public 41H boundary.

41H.2 must close the equivalent signed-message gap by removing free `signed_message_bytes` from the public 41H boundary.

## Current Gap

The current chain proves two separate facts:

1. 41F proves that a guardian's Ed25519 signature verified over the message bytes inside the Ed25519 instruction.
2. 41G proves that a caller-supplied `signed_message_bytes` value equals the canonical hash of `raw_payload_bytes`.

But the chain does not yet prove:

`41F-verified message bytes == signed_message_bytes == canonical hash(raw_payload_bytes)`

This means a real guardian signature over arbitrary message `M` could be paired with a separate `signed_message_bytes = canonical hash(raw_payload)` unless the message operands are bound.

41I would turn this into a forgeable quorum authorization marker.

Therefore 41I code must remain blocked until 41H.2 is implemented and accepted.

## Required Closure

41H must use the message bytes extracted by 41F.1 as the only signed message input.

Preferred closure:

- remove `signed_message_bytes: &[u8]` from `establish_guardian_membership_validation`;
- after `extracted_slices` is available, use `extracted_slices.message_bytes` as the signed message;
- require `extracted_slices.message_bytes.len() == 32`;
- call `establish_payload_hash_binding(raw_payload_bytes, extracted_slices.message_bytes, phase_41f_result)`;
- preserve payload hash binding errors;
- add explicit error/report markers for the new binding.

This proves:

- the native SVM Ed25519 verification applies to the extracted message bytes;
- those exact extracted message bytes are the payload hash;
- the guardian signed the payload hash, not an unrelated message.

## Intended 41H API Change

Before:

`establish_guardian_membership_validation(..., raw_payload_bytes, signed_message_bytes, expected_configured_guardian_set_id, guardian_set)`

After:

`establish_guardian_membership_validation(..., raw_payload_bytes, expected_configured_guardian_set_id, guardian_set)`

41H derives the signed message internally from:

`extraction_result.extracted_slices.message_bytes`

## New Invariants

41H.2 must enforce:

1. `extraction_result.extracted_slices` exists.
2. `extracted_slices.message_bytes` is the 41F.1 extracted Ed25519 message.
3. `extracted_slices.message_bytes.len() == 32`.
4. The 41G payload hash binding uses `extracted_slices.message_bytes`, not a caller-supplied value.
5. No free signed message input is accepted by 41H.
6. The decoded payload is still derived internally from `raw_payload_bytes`.
7. The payload guardian set ID check still uses the internally decoded payload.
8. All downstream execution flags remain false.

## Proposed Error Kind

Add an explicit 41H error kind:

`ExtractedSignedMessageLengthInvalid`

This error should fire if:

`extracted_slices.message_bytes.len() != 32`

The existing `PayloadHashBindingNotEstablished` error should still wrap 41G errors when the extracted signed message is 32 bytes but does not equal the canonical payload hash.

## Proposed Report Fields

The 41H report should explicitly state:

- `derives_signed_message_from_41f_extraction: true`
- `accepts_free_signed_message_input: false`
- `checks_extracted_signed_message_len_32: true`
- `binds_41f_verified_message_to_payload_hash: true`

Existing false downstream flags must remain false:

- quorum counting enabled: false;
- authorization enabled: false;
- replay write enabled: false;
- processed event marking enabled: false;
- account mutation enabled: false;
- CPI enabled: false;
- invoke_signed enabled: false;
- SPL token mint_to enabled: false;
- handler added: false;
- live route enabled: false.

## Required Tests

Future code implementation must add or update tests for:

1. valid path uses `extracted_slices.message_bytes` as the signed message;
2. free `signed_message_bytes` input is removed from the public 41H API;
3. extracted message length not equal to 32 is rejected;
4. guardian signature over arbitrary 32-byte message cannot be paired with a different payload hash;
5. payload substitution with mismatched extracted message fails at payload hash binding;
6. decoded payload binding from 41H.1 remains closed;
7. guardian membership success still works for the canonical payload;
8. caller-supplied guardian set is still rejected;
9. unauthenticated guardian set is still rejected;
10. false downstream flags remain false;
11. forbidden runtime surfaces remain absent.

## Impact On 41I

41I planning remains conceptually correct, but it is blocked until 41H.2 is accepted.

After 41H.2, 41I can safely compose 41H internally because each counted guardian will prove:

`41F-verified message == canonical hash(raw_payload)`

The 41I counting invariant must be updated to say:

Every counted guardian must have passed 41H.2, where the 41F-verified extracted message bytes are the same payload hash used by 41G.

## Forbidden In 41H.2

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
- live route;
- production program ID;
- production guardian account loading.

## Active Blockers Remain

No blocker is removed by 41H.2 planning.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Review Questions

Reviewers should check:

1. Does this plan close the signed-message binding gap?
2. Is removing free `signed_message_bytes` from 41H the right closure?
3. Is using `extracted_slices.message_bytes` as the only signed message correct?
4. Is requiring extracted message length exactly 32 correct?
5. Is it correct to call 41G with `raw_payload_bytes` and `extracted_slices.message_bytes`?
6. Are error kinds sufficient?
7. Are report fields sufficient?
8. Are tests sufficient?
9. Are forbidden runtime surfaces excluded?
10. Is this plan sufficient before writing 41H.2 code?

## Required Fix From High-Risk Audit

Audit Demon identified that removing free `signed_message_bytes` is necessary but not sufficient.

41H.2 must also bind the 41F.2 verified message range to the 41F.1 extracted message range.

The required proof is:

- 41F.2 verified a signature over bytes at `verified_ranges.message_range`;
- 41F.1 extracted `extracted_slices.message_bytes` from `extracted_slices.message_range`;
- 41H.2 uses `extracted_slices.message_bytes` as the payload hash operand;
- therefore 41H.2 must prove `verified_ranges.message_range == extracted_slices.message_range`.

Without this check, an inconsistent caller-supplied pair of `phase_41f_result` and `extraction_result` could re-open the arbitrary-message pairing attack.

## Additional Required Range Bindings

41H.2 must check:

- `verified_ranges.public_key_range == extracted_slices.public_key_range`;
- `verified_ranges.message_range == extracted_slices.message_range`;
- `verified_ranges.signature_range == extracted_slices.signature_range`.

The public key range check already exists in 41H.

The message range check is mandatory.

The signature range check is recommended and should be included for full 41F.1 ↔ 41F.2 consistency.

## Additional Error Kinds

Add explicit 41H error kinds:

- `VerifiedMessageRangeMismatch`
- `VerifiedSignatureRangeMismatch`

Existing `VerifiedSignerRangeMismatch` may remain for public key range mismatch.

## Updated Closure Chain

The final closure must prove:

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

Only then does 41H prove:

`guardian signed canonical_hash(raw_payload_bytes)`

## Additional Report Fields

Add report fields:

- `checks_message_range_binding: true`
- `checks_signature_range_binding: true`

Together with the existing public key range binding, these show that 41H.2 binds the full Ed25519 operand set between 41F.1 and 41F.2.

## Additional Tests

Add tests for:

- verified message range mismatch is rejected;
- verified signature range mismatch is rejected;
- arbitrary-M pairing through inconsistent message ranges is rejected;
- valid path still succeeds when public key, message, and signature ranges all match.

The substitution attack test must explicitly prove:

- guardian signs message `M1`;
- extraction tries to use message bytes `hash(P2)`;
- `verified_ranges.message_range != extracted_slices.message_range`;
- 41H.2 rejects before quorum or authorization can exist.

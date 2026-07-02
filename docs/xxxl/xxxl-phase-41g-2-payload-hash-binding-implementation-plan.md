# XXXL Phase 41G.2 — Payload Hash Binding Implementation Plan

Date: 2026-07-03

## Status

Docs-only implementation planning checkpoint.

No runtime code is introduced in this phase.

No `.rs` file is changed in this phase.

No verification logic is changed in this phase.

No guardian validity, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41G.2 payload hash binding plan acceptance:

`0825dad Merge XXXL phase 41G payload hash binding plan acceptance`

## Purpose

This document plans the future implementation of the narrow payload hash binding boundary.

The future implementation should establish only:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

The preferred implementation path is narrower:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

Where:

- `signed_message_bytes` come from the SVM-verified Ed25519 message bytes accepted in Phase 41F;
- `signed_hash_32` is a checked 32-byte view of `signed_message_bytes`;
- `raw_payload_bytes` are caller/instruction-supplied and untrusted;
- `validate_guardian_payload_hash` recomputes the domain-separated canonical payload hash and compares it internally.

## Implementation Must Remain Narrow

The future implementation may establish only payload hash binding.

It must not establish:

- source burn proof acceptance;
- watcher honesty;
- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay safety;
- account mutation;
- CPI;
- SPL Token mint;
- process instruction handler;
- live route.

## Existing Boundary Reuse

The implementation must reuse existing repository boundaries.

Raw payload decoder:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Canonical hash boundary:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

Verifier exports:

`programs/xxxl-svm/src/verifier/mod.rs`

Required existing function:

`validate_guardian_payload_hash`

Allowed supporting references:

- `compute_guardian_payload_hash_domain_separator`;
- `compute_guardian_payload_hash`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1`;
- `CanonicalPayloadHashValidationError`;
- `CanonicalPayloadHashValidationErrorKind`.

Future code should not create a parallel canonicalizer.

Future code should not duplicate the hash comparison manually unless a reviewer explicitly accepts a reason.

## Proposed Future Module

Recommended future module:

`programs/xxxl-svm/src/verifier/payload_hash_binding_boundary.rs`

Recommended future registration:

- add `pub mod payload_hash_binding_boundary;` to `programs/xxxl-svm/src/verifier/mod.rs`;
- re-export only the narrow public API needed for tests and later phases.

This module name is a plan, not yet an implementation.

## Proposed Future Function Boundary

Recommended future function:

`establish_payload_hash_binding`

Conceptual signature:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result) -> Result<PayloadHashBindingEstablished, PayloadHashBindingError>`

Boundary rules:

- borrow input slices;
- do not allocate copied attacker-sized buffers;
- require Phase 41F verification result to be established;
- require `signed_message_bytes.len() == 32`;
- checked-convert signed message bytes to a 32-byte hash reference;
- call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
- map success to a narrow payload hash binding result;
- map every failure to fail-closed rejection.

## Recommended Implementation Flow

Future code should follow this flow:

1. accept borrowed `raw_payload_bytes`;
2. accept borrowed `signed_message_bytes`;
3. confirm Phase 41F result is `NativeEd25519VerificationEstablished`;
4. reject if `signed_message_bytes.len() != 32`;
5. checked-convert `signed_message_bytes` to `&[u8; 32]`;
6. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
7. if validation succeeds, return narrow payload hash binding established;
8. if validation fails, return fail-closed error;
9. do not mutate state;
10. do not mark replay;
11. do not authorize;
12. do not mint.

## Why validate_guardian_payload_hash Is Preferred

`validate_guardian_payload_hash` is preferred because it already:

- decodes the raw payload before hash acceptance;
- recomputes the domain-separated hash internally;
- compares recomputed hash to the expected 32-byte hash;
- rejects malformed raw payloads;
- rejects hash mismatch;
- avoids trusting caller-provided hash;
- preserves canonical payload boundary behavior.

Manual `compute_guardian_payload_hash(raw_payload_bytes) == signed_hash_32` comparison should be avoided in the first implementation unless separately justified and reviewed.

## Exact Domain Separator Requirement

Future implementation must preserve the accepted exact preimage.

The domain separator is:

`keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1"))`

The payload hash preimage is:

`domain_separator_32_bytes || raw_payload_bytes`

The payload hash is:

`keccak256(domain_separator_32_bytes || raw_payload_bytes)`

Future implementation must not prepend literal UTF-8 label bytes directly.

Future implementation must not compute `keccak256(raw_payload_bytes)` without the domain separator.

Future implementation must not accept a caller-provided hash without recomputation.

## Raw Payload Provenance Rule

`raw_payload_bytes` are caller/instruction-supplied and untrusted.

Structural decode proves only well-formedness.

Structural decode does not prove authenticity.

Authenticity for Phase 41G.2 implementation comes only from:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Future implementation must not treat decoded payload fields as authentic until hash binding succeeds.

Even after hash binding succeeds, Phase 41G.2 still does not prove source burn, watcher honesty, guardian membership, quorum, authorization, replay safety, or mint permission.

## Proposed Result Model

Recommended future success type:

`PayloadHashBindingEstablished`

Recommended fields:

- marker/status id;
- borrowed or copied 32-byte bound hash if needed;
- reference to the accepted canonical payload hash validator report if needed;
- no authorization data;
- no guardian membership data;
- no quorum data;
- no replay-write data;
- no mint data.

Recommended future status name:

`GatewayPayloadHashBindingEstablished`

Meaning:

- Phase 41F established SVM verified the Ed25519 message;
- signed message bytes were exactly 32 bytes;
- raw payload bytes structurally decoded through the accepted decoder;
- `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)` succeeded.

Non-meaning:

- signer is guardian;
- guardian set is valid;
- quorum exists;
- burn happened;
- finality is verified;
- expiration is enforced;
- mint is authorized.

## Proposed Error Model

Recommended future error enum:

`PayloadHashBindingErrorKind`

Recommended variants:

- `Phase41FVerificationNotEstablished`;
- `SignedMessageLengthMismatch`;
- `SignedMessageHashConversionFailed`;
- `RawPayloadDecodeFailed`;
- `PayloadHashMismatch`;
- `CanonicalPayloadHashValidationFailed`.

Mapping expectations:

- `signed_message_bytes.len() != 32` maps to `SignedMessageLengthMismatch`;
- checked conversion failure maps to `SignedMessageHashConversionFailed`;
- `CanonicalPayloadHashValidationErrorKind::RawPayloadDecode` maps to `RawPayloadDecodeFailed`;
- `CanonicalPayloadHashValidationErrorKind::HashMismatch` maps to `PayloadHashMismatch`;
- any unexpected validation error maps to fail-closed validation failure.

## Proposed Report / Safety Flags

Recommended future report:

`PayloadHashBindingBoundaryReport`

Recommended true fields:

- payload_hash_binding_boundary_defined;
- requires_phase_41f_verification_established;
- signed_message_length_checked;
- signed_message_converted_to_hash32;
- uses_validate_guardian_payload_hash;
- raw_payload_decode_required_by_validator;
- domain_separator_reused_from_canonical_payload;
- caller_provided_hash_trusted: false;
- parallel_canonicalizer_introduced: false.

Recommended false fields:

- source_burn_proof_accepted;
- watcher_honesty_accepted;
- guardian_validity_accepted;
- guardian_set_membership_accepted;
- quorum_counting_enabled;
- authorization_enabled;
- replay_write_enabled;
- processed_event_marking_enabled;
- account_mutation_enabled;
- cpi_enabled;
- invoke_signed_enabled;
- spl_token_mint_to_enabled;
- process_instruction_handler_added;
- live_route_enabled.

## Authoritative 21-Field Payload Shape

The future implementation must preserve compatibility with this exact canonical field order:

1. `message_type`
2. `schema_version`
3. `instruction_layout_version`
4. `route_id`
5. `source_chain_id`
6. `source_token`
7. `source_sender`
8. `source_burn_tx_hash`
9. `source_burn_event_index`
10. `source_block_number`
11. `source_block_hash`
12. `source_finality_block`
13. `canonical_event_key`
14. `x1_recipient`
15. `burned_amount`
16. `source_chain_weight_bps`
17. `xxxl_mint_amount`
18. `target_mint`
19. `guardian_set_id`
20. `message_nonce`
21. `expiration_slot_or_unix_ts`

The future implementation should not manually parse this order inside the new boundary.

It should rely on the existing raw payload decoder through `validate_guardian_payload_hash`.

## Proposed Tests

Future implementation tests should include at least:

### Success Path

- accepts valid raw payload bytes and `XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1` as the signed hash;
- returns the narrow binding-established result;
- confirms no guardian/quorum/auth/replay/mutation/CPI/mint/live flags are enabled.

### Signed Message Length

- rejects signed message length 0;
- rejects signed message length 31;
- rejects signed message length 33;
- accepts only length 32 before checked conversion.

### Hash Mismatch

- rejects valid raw payload bytes with wrong signed hash;
- maps mismatch to fail-closed payload hash mismatch error.

### Raw Payload Structural Rejection

- rejects truncated raw payload;
- rejects raw payload with trailing bytes;
- rejects raw payload with empty variable bytes;
- rejects malformed raw payload encoding;
- maps raw decode failure without treating payload as authentic.

### Domain Separator / Canonicalizer Drift

- rejects or prevents raw `keccak256(raw_payload_bytes)` model;
- rejects or prevents literal UTF-8 label bytes as direct prefix model;
- confirms `compute_guardian_payload_hash_domain_separator` is reused;
- confirms `validate_guardian_payload_hash` is used or directly covered.

### Stage-1 / Phase 33 / Phase 34 Parity

- re-runs accepted valid payload vector;
- confirms expected domain separator;
- confirms expected payload hash;
- confirms validation success;
- confirms mismatch failure.

### Boundary Preservation

- success does not validate guardian membership;
- success does not count quorum;
- success does not authorize;
- success does not write replay state;
- success does not mutate accounts;
- success does not CPI;
- success does not mint;
- success does not add handler;
- success does not unlock live route.

## Negative Matrix Carry-Forward

Future implementation should preserve the accepted negative matrix from the 41G.2 plan:

- signed message length less than 32;
- signed message length greater than 32;
- signed message bytes mismatch expected hash;
- caller-provided expected hash trusted without recomputation;
- raw payload truncated;
- raw payload with trailing bytes;
- raw payload with empty variable bytes;
- malformed raw payload encoding;
- wrong domain separator;
- literal UTF-8 label bytes used directly instead of 32-byte domain separator hash;
- hash computed as `keccak256(raw_payload_bytes)` without domain separator;
- hash computed with reordered fields;
- hash computed with missing field;
- hash computed with extra field;
- hash computed through a new parallel canonicalizer;
- wrong-value case for every canonical field;
- Stage-1 vector mismatch.

Each failure must be fail-closed.

No failure path may enable guardian/quorum/auth/replay/mutation/CPI/mint/live behavior.

## Public Key Boundary

The future implementation may carry public key bytes from Phase 41F.

It must not decide:

- whether the public key belongs to the guardian set;
- whether `guardian_set_id` is current;
- whether that public key counts toward quorum.

Guardian validation remains a later phase.

## Guardian Set ID Boundary

The payload includes:

- `guardian_set_id`.

The future implementation may prove only that `guardian_set_id` is part of the signed payload hash.

It must not validate guardian set membership.

It must not validate guardian set rotation.

It must not validate guardian set currentness.

## Finality / Expiration Boundary

The payload includes:

- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

The future implementation may prove only that both fields are part of the signed payload hash.

It must not validate live source-chain finality.

It must not enforce expiration.

Those checks remain later reviewed phases.

## Still Forbidden In This Implementation Plan

The following remain forbidden:

- runtime instruction handler;
- local cryptographic verification unless separately reviewed;
- new parallel canonicalizer;
- trusting caller-provided hash;
- source burn proof acceptance;
- watcher honesty acceptance;
- guardian validity acceptance;
- guardian set membership acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by this plan.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Review Questions

External review should answer:

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

## Next Gate

After external acceptance, Phase 41G.2 implementation may begin under a separate reviewed boundary.

Implementation must be limited to:

- payload hash binding boundary;
- checked 32-byte signed message conversion;
- reuse of `validate_guardian_payload_hash`;
- narrow result/status model;
- fail-closed errors;
- tests.

Implementation must not include:

- guardian validation;
- quorum;
- authorization;
- replay write;
- mutation;
- CPI;
- mint;
- handler;
- live route.

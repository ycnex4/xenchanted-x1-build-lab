# XXXL Phase 41G.2 — Payload Hash Binding Implementation Plan Acceptance

Date: 2026-07-03

## Accepted Main

`56c782e Merge XXXL phase 41G payload hash binding implementation plan`

## Parent Gate

`0825dad Merge XXXL phase 41G payload hash binding plan acceptance`

## Plan Commit

`7d7110c Document phase 41G payload hash binding implementation plan`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.2 implementation may begin under a separate reviewed boundary.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Phase 41G.2 implementation allowed after acceptance: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Forbidden operations preserved: yes
- Phase 41G.2 implementation allowed after acceptance: yes

## Accepted Purpose

The accepted implementation plan allows a future narrow `.rs` implementation of the payload hash binding boundary.

Future implementation may establish only:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Preferred implementation path:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

## Accepted Future Function Boundary

Recommended future function:

`establish_payload_hash_binding`

Conceptual boundary:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result) -> Result<PayloadHashBindingEstablished, PayloadHashBindingError>`

Accepted requirements:

- borrow input slices;
- do not allocate attacker-sized copied buffers;
- require Phase 41F verification established;
- require `signed_message_bytes.len() == 32`;
- checked-convert signed message bytes to `&[u8; 32]`;
- call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
- map success to a narrow payload hash binding result;
- map failure to fail-closed rejection.

## Accepted Future Implementation Flow

1. accept borrowed `raw_payload_bytes`;
2. accept borrowed `signed_message_bytes`;
3. confirm Phase 41F result is established;
4. reject if `signed_message_bytes.len() != 32`;
5. checked-convert `signed_message_bytes` to `&[u8; 32]`;
6. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
7. if validation succeeds, return narrow payload hash binding established;
8. if validation fails, return fail-closed error;
9. do not mutate state;
10. do not mark replay;
11. do not authorize;
12. do not mint.

## Accepted validate_guardian_payload_hash Reuse

Future implementation should prefer:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

Reason:

- it decodes raw payload bytes;
- it recomputes the domain-separated canonical payload hash internally;
- it compares internally;
- it rejects malformed raw payloads;
- it rejects hash mismatch;
- it avoids trusting caller-provided hash;
- it avoids a parallel canonicalizer;
- it avoids duplicated manual equality logic.

Manual `compute_guardian_payload_hash(raw_payload_bytes) == signed_hash_32` comparison should not be used unless separately justified and reviewed.

## Accepted Signed Message Length / Conversion Flow

Accepted flow:

- require `signed_message_bytes.len() == 32`;
- checked-convert to `&[u8; 32]`;
- call `validate_guardian_payload_hash`.

Demon note:

`SignedMessageHashConversionFailed` is practically unreachable after a successful `len == 32` guard, because `TryInto<&[u8; 32]>` on a slice of length 32 should succeed.

Implementation must handle this explicitly by either:

- marking `SignedMessageHashConversionFailed` as defensive-only; or
- combining length check and conversion into one fallible step.

Tests should not imply that this branch is normally reachable after a correct length guard.

## Accepted Result / Status Boundary

Recommended future success type:

`PayloadHashBindingEstablished`

Recommended future status name:

`GatewayPayloadHashBindingEstablished`

This status may mean only:

- Phase 41F established SVM verified the Ed25519 message;
- signed message bytes were exactly 32 bytes;
- raw payload bytes structurally decoded through the accepted decoder;
- `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)` succeeded.

This status must not mean:

- signer is guardian;
- guardian set is valid;
- quorum exists;
- burn happened;
- finality is verified;
- expiration is enforced;
- authorization is granted;
- replay is safe;
- minting is allowed.

## Accepted Error Model

Recommended future error enum:

`PayloadHashBindingErrorKind`

Accepted variants:

- `Phase41FVerificationNotEstablished`;
- `SignedMessageLengthMismatch`;
- `SignedMessageHashConversionFailed`;
- `RawPayloadDecodeFailed`;
- `PayloadHashMismatch`;
- `CanonicalPayloadHashValidationFailed`.

Accepted mapping:

- `signed_message_bytes.len() != 32` maps to `SignedMessageLengthMismatch`;
- checked conversion failure maps to `SignedMessageHashConversionFailed` or is merged into the fallible length/conversion step;
- `CanonicalPayloadHashValidationErrorKind::RawPayloadDecode` maps to `RawPayloadDecodeFailed`;
- `CanonicalPayloadHashValidationErrorKind::HashMismatch` maps to `PayloadHashMismatch`;
- any unexpected validation error maps to fail-closed validation failure.

## Accepted Report / Safety Flag Boundary

A module-local report may be introduced, such as:

`PayloadHashBindingBoundaryReport`

Accepted true fields:

- payload_hash_binding_boundary_defined;
- requires_phase_41f_verification_established;
- signed_message_length_checked;
- signed_message_converted_to_hash32;
- uses_validate_guardian_payload_hash;
- raw_payload_decode_required_by_validator;
- domain_separator_reused_from_canonical_payload.

Accepted explicit false fields:

- caller_provided_hash_trusted: false;
- parallel_canonicalizer_introduced: false;
- source_burn_proof_accepted: false;
- watcher_honesty_accepted: false;
- guardian_validity_accepted: false;
- guardian_set_membership_accepted: false;
- quorum_counting_enabled: false;
- authorization_enabled: false;
- replay_write_enabled: false;
- processed_event_marking_enabled: false;
- account_mutation_enabled: false;
- cpi_enabled: false;
- invoke_signed_enabled: false;
- spl_token_mint_to_enabled: false;
- process_instruction_handler_added: false;
- live_route_enabled: false.

## Carry-Forward Requirement — Cumulative Phase41BSafetyFlags Taxonomy

Demon note:

The implementation must explicitly define how this boundary relates to cumulative `Phase41BSafetyFlags`.

Accepted requirement:

Implementation must choose one of these approaches under review:

1. introduce a new separate cumulative flag such as `payload_hash_binding_established`; or
2. leave cumulative `Phase41BSafetyFlags` unchanged.

In both cases, payload hash binding must not be over-claimed as proof or evidence acceptance.

The following cumulative trust flags must remain false unless a later reviewed phase explicitly changes them:

- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`.

Reason:

Payload hash binding means only that the SVM-verified signed message bytes equal the domain-separated hash of the raw payload bytes.

It does not mean:

- signer is guardian-authenticated;
- guardian set membership is valid;
- quorum exists;
- proof is accepted;
- verification evidence is accepted;
- authorization exists.

## Accepted Raw Payload Provenance Rule

`raw_payload_bytes` are caller/instruction-supplied and untrusted.

Structural decode proves only well-formedness.

Structural decode does not prove authenticity.

Authenticity for this phase comes only from:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Even after hash binding succeeds, this phase still does not prove source burn, watcher honesty, guardian membership, quorum, authorization, replay safety, or mint permission.

## Accepted Tests

Future implementation tests should include:

- success path using valid raw payload bytes and expected valid hash;
- signed message length 0 rejection;
- signed message length 31 rejection;
- signed message length 33 rejection;
- signed message length 32 accepted before checked conversion;
- wrong signed hash rejection;
- raw payload truncation rejection;
- raw payload trailing bytes rejection;
- empty variable bytes rejection;
- malformed raw payload rejection;
- domain separator drift prevention;
- literal UTF-8 label prefix model rejected or prevented;
- raw `keccak256(raw_payload_bytes)` model rejected or prevented;
- Stage-1 / Phase 33 / Phase 34 vector parity;
- boundary preservation tests proving no guardian/quorum/auth/replay/mutation/CPI/mint/live behavior is enabled.

## Accepted Stage-1 / Phase 33 / Phase 34 Parity

Future implementation tests must re-run accepted vectors against the canonical hash boundary.

Required checks:

- accepted valid payload vector;
- expected domain separator;
- expected payload hash;
- validation success;
- mismatch failure.

Any vector mismatch must fail closed.

## Still Forbidden

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

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41G.2 payload hash binding implementation plan is accepted.

Phase 41G.2 `.rs` implementation may begin under a separate reviewed boundary.

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

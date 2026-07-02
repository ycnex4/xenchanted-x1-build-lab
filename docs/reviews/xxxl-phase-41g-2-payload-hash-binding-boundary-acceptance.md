# XXXL Phase 41G.2 — Payload Hash Binding Boundary Code Acceptance

Date: 2026-07-03

## Accepted Main

`730b5d9 Merge XXXL phase 41G payload hash binding boundary`

## Parent Gate

`36d5828 Merge XXXL phase 41G payload hash binding implementation plan acceptance`

## Implementation Commit

`2714972 Add phase 41G payload hash binding boundary`

## Changed Files

- `programs/xxxl-svm/src/verifier/payload_hash_binding_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.2 payload hash binding boundary is accepted.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Code boundary accepted: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Code boundary accepted: yes

## Accepted Scope

The implementation adds only the narrow Phase 41G.2 payload hash binding boundary.

Implemented function:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result)`

Accepted flow:

1. require Phase 41F native Ed25519 verification established;
2. require `signed_message_bytes.len() == 32`;
3. checked-convert to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. success returns only the narrow payload hash binding marker/status;
6. failure is fail-closed.

## Accepted Function Boundary

The function boundary is accepted as narrow.

Accepted properties:

- borrowed input slices;
- no attacker-sized copied allocation;
- no state mutation;
- no replay write;
- no CPI;
- no mint;
- no handler;
- no live route.

## Accepted Phase 41F Gate

The implementation requires both:

- `status == NativeEd25519VerificationEstablished`;
- `establishes_native_ed25519_verification == true`.

This defense-in-depth check prevents treating a spoofed or partial Phase 41F result as sufficient.

## Accepted Canonical Validator Reuse

The implementation delegates payload hash validation to:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

Accepted properties:

- no parallel canonicalizer;
- no manual hash reimplementation;
- raw payload decode is delegated to the accepted canonical/raw payload boundary;
- domain-separated hash recomputation is delegated to the accepted canonical payload boundary;
- caller-provided hash is not trusted.

## Accepted Signed Message Length / Conversion

The implementation requires exact 32-byte signed message bytes before conversion.

Accepted behavior:

- length 0 rejected;
- length 31 rejected;
- length 33 rejected;
- length 32 accepted for checked conversion;
- conversion failure, if ever reached, fails closed.

Demon non-blocking note:

`SignedMessageHashConversionFailed` is practically unreachable after the `len == 32` guard because `TryInto<&[u8; 32]>` on a 32-byte slice should succeed.

Accepted interpretation:

- this branch is defensive-only;
- tests should not imply it is normally reachable after a correct length guard;
- keeping it is acceptable because it is panic-safe and fail-closed.

## Accepted Error Model

The error model is accepted as fail-closed.

Accepted failure paths:

- Phase 41F not established;
- signed message length mismatch;
- defensive conversion failure;
- raw payload decode failure;
- payload hash mismatch;
- canonical payload validation failure.

No failure path enables downstream trust or execution.

## Accepted Result / Status Taxonomy

The success result is accepted as a narrow marker/status.

It may mean only:

- Phase 41F gate passed;
- signed message bytes were exactly 32 bytes;
- raw payload bytes passed canonical validation;
- signed message hash matched the canonical domain-separated payload hash.

It must not mean:

- proof accepted;
- verification evidence accepted;
- signer is a guardian;
- guardian set membership is valid;
- quorum exists;
- authorization exists;
- replay is safe;
- mutation is allowed;
- CPI is allowed;
- mint is allowed;
- handler/live route is enabled.

## Accepted Phase41BSafetyFlags Taxonomy

The implementation does not introduce or flip a cumulative `Phase41BSafetyFlags` const.

The implementation uses a module-local non-authorizing report.

The cumulative taxonomy is preserved.

The following remain false:

- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `quorum_counting_enabled`;
- `authorization_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

Payload hash binding is not proof acceptance.

Payload hash binding is not verification evidence acceptance.

## Accepted Per-Field Negative Granularity Delegation

The implementation does not duplicate all 21 canonical field-level negative cases inside the 41G.2 boundary.

This is accepted because:

- 41G.2 delegates raw payload structure to Phase 33 raw payload decoder;
- 41G.2 delegates hash/canonical validation to Phase 34 canonical payload validator;
- any field change changes raw bytes and therefore causes hash mismatch unless signed by the SVM-verified message;
- full crate tests confirm the existing canonical layer remains green.

41G.2 tests cover the boundary-specific negative class through wrong 32-byte signed hash and malformed raw payload rejection.

## Accepted Forbidden Operations Check

The implementation does not introduce:

- handler;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- replay write;
- processed event marking;
- account mutation;
- guardian validation;
- guardian set membership validation;
- quorum;
- authorization;
- live route unlock.

No deployment, SBF, keypair, `.env`, `.local-keys`, or network operation is introduced.

## Validation Evidence

Validation reported before commit:

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml` passed;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml payload_hash_binding_boundary` passed: 7 tests;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml` passed: 55 passed, 0 failed, 10 ignored;
- `git diff --check` passed.

Ignored tests are existing SBF/deploy-gated tests and are not failures for this boundary.

## Accepted Tests

Accepted test coverage:

- success path with valid canonical payload hash;
- Phase 41F gate rejection, including both status and capability flag;
- signed message length rejection for 0, 31, and 33;
- wrong 32-byte signed hash rejection;
- malformed raw payload rejection before authenticity;
- report/trust/execution flags remain false;
- cumulative flags remain false;
- domain separator and vector parity with canonical payload validator.

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

## Forward Risk Reminder

The live-wiring Model A precondition from Phase 41F.2 remains a future high-risk audit item when handler integration is eventually planned.

Payload hash binding is accepted only as an isolated verifier boundary.

## Next Gate

Phase 41G.2 code boundary is accepted.

Recommended next gate:

Phase 41G.3 — payload binding negative matrix and focused audit.

Alternative later gate:

Phase 41H — guardian validation planning.

Recommendation:

Complete Phase 41G.3 before moving to guardian validation, so payload binding is closed with its focused negative matrix and audit record.

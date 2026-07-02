# XXXL Phase 41G.3 — Payload Binding Negative Matrix + Focused Audit Record

Date: 2026-07-03

## Status

Focused audit record.

Docs-only.

No runtime code.

No `.rs` changes.

## Accepted Main

`8ebac43 Merge XXXL phase 41G payload binding focused audit plan`

## Parent Accepted Gate

`1ce0fb4 Merge XXXL phase 41G payload hash binding boundary acceptance`

## Plan Under Audit

`docs/xxxl/xxxl-phase-41g-3-payload-binding-negative-matrix-focused-audit-plan.md`

## Reviewer Verdicts On Plan

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- 41G.3 is the correct next gate after 41G.2: yes
- Phase 41H may begin after 41G.3 acceptance: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Scope drift: no
- Correct next gate: yes
- Phase 41H may begin after 41G.3 acceptance: yes

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.3 payload binding negative matrix and focused audit shell is accepted.

## Accepted Scope

41G.3 is accepted as a docs-only focused audit shell around the already accepted 41G.2 payload hash binding boundary.

No code is introduced.

No `.rs` file is changed.

No runtime behavior is changed.

No guardian validation, guardian set membership, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Accepted Boundary Under Audit

Accepted 41G.2 function:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result)`

Accepted flow:

1. require Phase 41F native Ed25519 verification established;
2. require `signed_message_bytes.len() == 32`;
3. checked-convert to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. success returns only narrow payload hash binding marker/status;
6. failure is fail-closed.

## Accepted Direct Coverage

41G.2 direct tests cover:

- success path with valid canonical payload hash;
- Phase 41F gate rejection;
- Phase 41F status spoofing protection;
- Phase 41F capability flag false protection;
- signed message length 0 rejection;
- signed message length 31 rejection;
- signed message length 33 rejection;
- wrong 32-byte signed hash rejection;
- malformed raw payload rejection before authenticity;
- non-authorizing report flags;
- cumulative flags remain false;
- domain separator/vector parity with canonical payload validator.

## Accepted Delegated Coverage

41G.2 intentionally delegates lower-level raw/canonical coverage to accepted earlier boundaries.

Delegated to Phase 33 raw payload decoder:

- raw payload field decoding;
- canonical 21-field shape decoding;
- malformed payload structure;
- truncated payload;
- trailing bytes;
- empty variable bytes;
- raw structural validity.

Delegated to Phase 34 canonical payload validator:

- domain separator stability;
- canonical hash recomputation;
- caller-provided hash not trusted;
- hash mismatch rejection;
- canonical validator authority.

## Accepted Per-Field Negative Granularity

41G.2 does not need one local test per canonical field.

Accepted rationale:

- any canonical field change changes `raw_payload_bytes`;
- changed raw payload changes computed domain-separated hash;
- unless signed message bytes also match the changed payload hash, 41G.2 rejects;
- field-level structural validity belongs to Phase 33;
- hash/domain validity belongs to Phase 34;
- local duplication would add noise and drift risk.

## Accepted 21-Field Authority

The 21-field canonical payload shape remains authoritative.

41G.2 must not manually re-parse the fields.

41G.2 relies on:

- `validate_guardian_payload_hash`;
- `canonical_payload.rs`;
- `raw_payload.rs`.

## Accepted Trust Taxonomy

Payload hash binding means only:

- SVM-verified signed message bytes equal the domain-separated hash of supplied raw payload bytes.

Payload hash binding does not mean:

- source burn happened;
- watcher evidence is valid;
- proof is accepted;
- verification evidence is accepted;
- signer is a guardian;
- guardian set membership is valid;
- quorum exists;
- authorization exists;
- replay is safe;
- minting is allowed.

## Accepted False Flags

The following remain false:

- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `guardian_validity_accepted`;
- `guardian_set_membership_accepted`;
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

## Accepted Phase41BSafetyFlags Taxonomy

The cumulative `Phase41BSafetyFlags` taxonomy is preserved.

41G.2 does not introduce or flip a cumulative `Phase41BSafetyFlags` const.

Payload hash binding remains a module-local capability/report.

Payload hash binding is not proof acceptance.

Payload hash binding is not verification evidence acceptance.

## Accepted Forbidden Operation Audit

The accepted 41G.2 boundary does not introduce:

- handler;
- `process_instruction`;
- `AccountInfo`;
- `next_account_info`;
- sysvar loading;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- replay write;
- processed event marking;
- account mutation;
- guardian validation;
- guardian set membership validation;
- quorum counting;
- authorization;
- live route unlock.

## Accepted Panic / Allocation Audit

The accepted 41G.2 production path uses:

- no `unwrap`;
- no `panic`;
- no attacker-sized copied allocation;
- borrowed input slices;
- checked conversion to `&[u8; 32]`;
- fail-closed malformed-data handling.

Any panic in static test helpers is outside the production path.

## SignedMessageHashConversionFailed Note

`SignedMessageHashConversionFailed` is defensive-only after the `signed_message_bytes.len() == 32` guard.

Accepted interpretation:

- practically unreachable in normal control flow;
- panic-safe;
- fail-closed;
- no reachable test should be expected after a correct length guard.

## Ignored Test Evidence

Demon requested explicit evidence that ignored tests do not hide payload hash binding regressions.

Confirmed evidence:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml -- --ignored --list`

Reported ignored test list:

- 1 ignored unit dry-run:
  - `pda::x1_testnet_program_id_candidate_dry_run_tests::x1_testnet_program_id_candidate_pda_dry_run`
- 10 ignored mollusk tests:
  - `invalid_consume_gateway_mint_account_count_rejects_before_live_route`
  - `invalid_consume_gateway_mint_discriminator_rejects_before_scaffold_path`
  - `invalid_consume_gateway_mint_length_rejects_before_scaffold_path`
  - `invalid_consume_gateway_mint_readonly_account_passed_writable_rejects_before_validation`
  - `invalid_consume_gateway_mint_required_writable_account_passed_readonly_rejects_before_validation`
  - `invalid_consume_gateway_mint_unexpected_signer_rejects_before_validation`
  - `invalid_consume_gateway_mint_version_rejects_before_scaffold_path`
  - `invalid_consume_gateway_mint_wrong_program_account_owner_rejects_before_live_route`
  - `invalid_consume_gateway_mint_wrong_recipient_token_owner_rejects_before_live_route`
  - `invalid_consume_gateway_mint_zero_amount_rejects_before_live_route`

Ignore attribute count evidence:

- all ignore attrs: 11;
- sbf/deploy-gated attrs: 10;
- off-chain env-gated attrs: 1;
- payload_hash_binding ignore attrs: 0.

Conclusion:

- the 10 ignored tests in the regular full test result are SBF/deploy-gated mollusk tests;
- there is also 1 pre-existing off-chain env-gated dry-run test;
- no payload hash binding boundary test is ignored;
- all 7 payload hash binding boundary tests are active.

## Validation Evidence

Accepted validation evidence from 41G.2 and 41G.3:

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml` passed;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml payload_hash_binding_boundary` passed: 7 active tests;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml` passed: 55 passed, 0 failed, 10 ignored;
- `git diff --check` passed;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml -- --ignored --list` confirmed ignored test identities;
- ignore attribute grep confirmed `payload_hash_binding_boundary.rs` has 0 ignore attributes.

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

The live-wiring Model A precondition from Phase 41F.2 remains a future high-risk audit item.

41G.3 does not close handler integration risk.

41G.3 only closes the payload binding negative matrix and focused audit shell.

## Next Gate

Phase 41G.3 payload binding negative matrix and focused audit shell is accepted.

Phase 41H guardian validation planning may begin next, under a separate reviewed boundary.

41H will start the guardian-set membership → quorum → authorization trust chain and must be treated as high-risk.

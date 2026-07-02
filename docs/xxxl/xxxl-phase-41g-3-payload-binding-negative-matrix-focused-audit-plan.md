# XXXL Phase 41G.3 — Payload Binding Negative Matrix + Focused Audit Plan

Date: 2026-07-03

## Status

Docs-only focused audit planning checkpoint.

No runtime code is introduced.

No `.rs` file is changed.

No verification logic is changed.

No guardian validation, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41G.2 payload hash binding boundary code acceptance:

`1ce0fb4 Merge XXXL phase 41G payload hash binding boundary acceptance`

## Purpose

Phase 41G.3 closes the audit shell around the accepted Phase 41G.2 payload hash binding boundary.

41G.2 implemented the narrow boundary:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result)`

41G.3 should verify that the boundary is sufficiently covered by:

- direct 41G.2 tests;
- delegated Phase 33 raw payload decoder tests;
- delegated Phase 34 canonical payload hash validator tests;
- focused trust-taxonomy review;
- focused forbidden-operation review.

## Why 41G.3 Exists

41G.2 is accepted as a code boundary.

Before moving to guardian validation, we need a focused negative matrix and audit record showing:

- what 41G.2 tests directly cover;
- what 41G.2 intentionally delegates to earlier accepted boundaries;
- what 41G.2 intentionally does not cover;
- why payload hash binding is not proof acceptance;
- why payload hash binding is not guardian validation;
- why no downstream execution/trust gate is opened.

## Accepted 41G.2 Boundary Under Audit

Accepted function:

`establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result)`

Accepted flow:

1. require Phase 41F native Ed25519 verification established;
2. require `signed_message_bytes.len() == 32`;
3. checked-convert to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. success returns only narrow payload hash binding marker/status;
6. failure is fail-closed.

## Direct 41G.2 Negative Coverage

The focused audit should confirm direct tests cover:

- Phase 41F not established;
- Phase 41F status spoofing protection;
- Phase 41F capability flag false protection;
- signed message length 0 rejected;
- signed message length 31 rejected;
- signed message length 33 rejected;
- wrong 32-byte signed hash rejected;
- malformed raw payload rejected before authenticity;
- trust/execution flags remain false;
- cumulative `Phase41BSafetyFlags` trust/execution flags remain false;
- domain separator/vector parity with canonical payload validator.

## Delegated Negative Coverage

41G.2 must not duplicate all lower-level raw/canonical cases.

The following are delegated to accepted Phase 33 / Phase 34 boundaries:

- raw payload field decoding;
- 21-field canonical order;
- truncated payload rejection;
- trailing bytes rejection;
- empty variable bytes rejection;
- malformed raw payload encoding rejection;
- domain separator stability;
- canonical hash recomputation;
- caller-provided hash not trusted;
- hash mismatch rejection.

This delegation is intentional.

41G.2 tests should confirm the integration boundary:

- malformed payload maps to raw payload decode failure;
- wrong 32-byte signed hash maps to hash mismatch;
- valid vector succeeds.

## Canonical 21-Field Shape

41G.3 should confirm that the accepted 21-field payload shape remains authoritative:

- `message_type`;
- `schema_version`;
- `instruction_layout_version`;
- `route_id`;
- `source_chain_id`;
- `source_token`;
- `source_sender`;
- `source_burn_tx_hash`;
- `source_burn_event_index`;
- `source_block_number`;
- `source_block_hash`;
- `source_finality_block`;
- `canonical_event_key`;
- `x1_recipient`;
- `burned_amount`;
- `source_chain_weight_bps`;
- `xxxl_mint_amount`;
- `target_mint`;
- `guardian_set_id`;
- `message_nonce`;
- `expiration_slot_or_unix_ts`.

41G.2 should not manually re-parse these fields.

41G.2 should rely on:

- `validate_guardian_payload_hash`;
- `canonical_payload.rs`;
- `raw_payload.rs`.

## Per-Field Negative Granularity Rule

41G.3 should record the accepted rule:

41G.2 does not need one local test per canonical field.

Reason:

- any canonical field change changes `raw_payload_bytes`;
- a changed `raw_payload_bytes` changes the computed domain-separated hash;
- unless the signed message bytes also match that changed payload hash, 41G.2 rejects;
- field-level structural validity remains owned by Phase 33 raw decoder;
- hash/domain validity remains owned by Phase 34 canonical validator.

Therefore, 41G.2 local field-level duplication would add noise and create drift risk.

## Trust Taxonomy Audit

41G.3 should confirm payload hash binding means only:

- the SVM-verified signed message bytes equal the domain-separated hash of the supplied raw payload bytes.

41G.3 should confirm payload hash binding does not mean:

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

## Required False Flags

41G.3 should confirm these remain false:

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

## Cumulative Phase41BSafetyFlags Taxonomy

41G.3 should confirm the 41G.2 implementation does not flip cumulative `Phase41BSafetyFlags`.

Accepted 41G.2 behavior:

- no new cumulative `Phase41BSafetyFlags` const;
- module-local report only;
- payload hash binding capability is local;
- proof/evidence/auth/execution flags stay false.

## Forbidden Operation Audit

41G.3 should confirm the accepted 41G.2 code does not introduce:

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

## Panic / Allocation Audit

41G.3 should confirm:

- production path uses no `unwrap`;
- production path uses no `panic`;
- production path uses no attacker-sized copied allocation;
- input slices are borrowed;
- conversion to `&[u8; 32]` is checked;
- malformed data fails closed.

Test helpers may panic on invalid static test hex; that does not affect production path.

## SignedMessageHashConversionFailed Note

41G.3 should preserve the accepted note:

`SignedMessageHashConversionFailed` is defensive-only after `signed_message_bytes.len() == 32`.

It is acceptable because:

- it is panic-safe;
- it fails closed;
- reviewers should not expect a normal reachable test case after a correct length guard.

## Validation Evidence To Re-Run

Focused audit should re-run or confirm:

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml`;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml payload_hash_binding_boundary`;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml`;
- `git diff --check`.

Expected current evidence from 41G.2 acceptance:

- targeted payload hash binding tests: 7 passed;
- full `xxxl-svm` tests: 55 passed, 0 failed, 10 ignored;
- ignored tests are existing SBF/deploy-gated tests.

## Out Of Scope

41G.3 must not introduce or plan implementation for:

- guardian validation;
- guardian set membership;
- quorum;
- authorization;
- replay write;
- processed event marking;
- account mutation;
- CPI;
- mint;
- handler;
- live route.

Those remain later phases.

## Forward Risk Reminder

The live-wiring Model A precondition from Phase 41F.2 remains a future high-risk audit item when handler integration is eventually planned.

41G.3 does not close that future handler integration risk.

It only closes the payload binding negative matrix and focused audit shell.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by 41G.3.

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

1. Is 41G.3 the correct next gate after accepted 41G.2 code boundary?
2. Does the matrix correctly separate direct 41G.2 tests from Phase 33/34 delegated coverage?
3. Is per-field negative granularity delegation acceptable?
4. Is the trust taxonomy correct?
5. Are proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live flags still correctly false?
6. Is cumulative `Phase41BSafetyFlags` taxonomy preserved?
7. Are forbidden-operation checks sufficient?
8. Are panic/allocation checks sufficient?
9. Is the `SignedMessageHashConversionFailed` defensive-only note preserved?
10. Can 41G.3 focused audit proceed without code changes?
11. After 41G.3 acceptance, may Phase 41H guardian validation planning begin?

## Next Gate

After external acceptance of this plan, create the 41G.3 focused audit record.

After 41G.3 focused audit acceptance, Phase 41H guardian validation planning may begin under a separate reviewed boundary.

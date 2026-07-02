# XXXL Phase 41G.0 — Proof / Evidence / Payload Binding Plan Acceptance

Date: 2026-07-03

## Accepted Main

`2eb7416 Merge XXXL phase 41G canonical payload field order fix`

## Parent Gate

`72951e8 Merge XXXL phase 41F focused crypto boundary audit acceptance`

## Original Plan Commit

`25a0362 Document phase 41G payload binding plan`

## Canonical Field-Order Fix

`ed4b7ac Fix phase 41G canonical payload field order`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.1 may begin under a separate reviewed boundary.

## Review History

Initial Phase 41G.0 plan was accepted on architecture separation, but external review detected a blocking canonical-field mismatch:

- initial plan listed 19 fields;
- authoritative Rust decoder declares 21 fields.

Authoritative source:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

The blocking issue was security-relevant and required a docs-only fix before Phase 41G.1.

After the canonical field-order fix, external re-review accepted the plan.

## Blocking Fixes Addressed

The canonical field-order fix addressed:

1. canonical field list updated from 19 to authoritative 21 fields;
2. `instruction_layout_version` added;
3. `guardian_set_id` added;
4. `source_finality_block` separated from `expiration_slot_or_unix_ts`;
5. collapsed finality/expiration model removed from the plan;
6. binding requirements updated to bind all 21 canonical fields;
7. negative matrix updated with:
   - wrong `instruction_layout_version`;
   - wrong `guardian_set_id`;
   - wrong `source_finality_block`;
   - wrong `expiration_slot_or_unix_ts`;
   - Stage-1 vector mismatch;
8. fix document added with replay-risk rationale.

## Accepted Authoritative Canonical Field Order

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

This order must match `RAW_PAYLOAD_PHASE_23_FIELD_ORDER` exactly.

## Accepted Security Rationale

`guardian_set_id` must be signed.

Reason:

- prevents replay across guardian-set rotation;
- binds the payload to the intended guardian set;
- does not itself establish guardian membership.

`instruction_layout_version` must be signed.

Reason:

- prevents layout/canonicalization replay;
- binds the signature to a specific payload decoding model.

`source_finality_block` and `expiration_slot_or_unix_ts` must remain separate.

Reason:

- `source_finality_block` binds source-chain finality;
- `expiration_slot_or_unix_ts` binds payload expiration;
- collapsing them may create semantic ambiguity.

## Accepted Phase 41G.0 Model

Phase 41G.0 accepts the following plan:

- guardians sign expected gateway payload hash bytes;
- Phase 41G checks `signed_message_bytes == expected_gateway_payload_hash_bytes`;
- expected hash is planned as `keccak256(canonical_gateway_payload_bytes)` at the planning shorthand level;
- canonical gateway payload bytes must be derived from the authoritative 21-field order;
- Stage-1 vector parity must be preserved during implementation.

## Carry-Forward Requirement For Phase 41G.2 — Domain Separator

The phrase `keccak256(canonical_gateway_payload_bytes)` is accepted only as a planning shorthand.

Phase 41G.2 implementation must use the already accepted payload-hash model from the existing canonical payload boundary.

Expected implementation-level model:

`keccak256(XXXL_GUARDIAN_PAYLOAD_HASH_V1 || encoded_canonical_payload_bytes)`

The signed guardian hash and the expected program hash must use the identical preimage, including:

- domain separator;
- encoded canonical payload bytes;
- exact field order;
- exact encoding;
- exact byte parity with accepted Stage-1 vectors.

This requirement prevents cross-context hash reuse and hash-preimage drift.

## Carry-Forward Requirement For Phase 41G.2 — Reuse Existing Canonicalizer

Phase 41G.2 must not rewrite canonicalization from scratch.

Expected sources to reuse:

- `programs/xxxl-svm/src/verifier/raw_payload.rs`;
- `programs/xxxl-svm/src/verifier/canonical_payload.rs`.

Reason:

- preserves byte-for-byte parity with the already validated canonical payload boundary;
- prevents renewed field-order drift;
- prevents encoding drift;
- preserves compatibility with Stage-1 / Phase 33 / Phase 34 vectors.

## Accepted Boundary

Phase 41G may establish only:

- the SVM-verified Ed25519 message bytes are bound to the expected gateway payload hash.

Phase 41G must not establish:

- source burn proof acceptance;
- watcher honesty;
- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay protection;
- account mutation;
- CPI;
- SPL Token mint;
- process instruction handler;
- live route.

## Accepted Negative Matrix Requirements

Future tests should include:

- wrong-value cases for all 21 canonical fields;
- signed message length mismatch;
- signed message hash mismatch;
- field order mismatch;
- missing field;
- extra field;
- invalid encoding;
- malformed canonical bytes;
- Stage-1 vector mismatch.

Each failure must be fail-closed and must not enable guardian/quorum/auth/mint.

## Public Key Deferral

Phase 41G may carry forward public key bytes from Phase 41F.

Phase 41G must not decide whether that public key belongs to the guardian set.

Guardian set membership remains a later phase.

## Still Forbidden

The following remain forbidden:

- local cryptographic verification unless separately reviewed;
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
- process instruction handler;
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

Phase 41G.0 is accepted after canonical field-order fix.

Phase 41G.1 may begin under a separate reviewed boundary.

Recommended Phase 41G.1 scope:

- payload evidence shape;
- structured representation of the 21-field canonical payload;
- no hashing yet unless separately scoped;
- no guardian validation;
- no quorum;
- no authorization;
- no replay write;
- no mutation;
- no CPI;
- no mint;
- no live route.

Phase 41G.2 carry-forward notes:

- use domain-separated payload hash preimage;
- reuse existing `raw_payload.rs` and `canonical_payload.rs`;
- preserve Stage-1 vector parity.

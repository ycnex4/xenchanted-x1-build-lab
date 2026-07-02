# XXXL Phase 41G.2 — Payload Hash Binding Plan Acceptance

Date: 2026-07-03

## Accepted Main

`7eee7d7 Merge XXXL phase 41G payload hash binding plan`

## Parent Gate

`c89fc59 Merge XXXL phase 41G payload evidence shape acceptance`

## Plan Commit

`46f833b Document phase 41G payload hash binding plan`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.2 implementation planning may begin under a separate reviewed boundary.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Phase 41G.2 implementation planning allowed: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Scope violations: no
- Forbidden operations preserved: yes
- Phase 41G.2 implementation planning allowed after acceptance: yes

## Accepted Purpose

Phase 41G.2 plans the narrow payload hash binding relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Where:

- `signed_message_bytes` are the SVM-verified Ed25519 message bytes from Phase 41F;
- `raw_payload_bytes` are caller/instruction-supplied candidate payload bytes shaped in Phase 41G.1;
- `compute_guardian_payload_hash` is the existing domain-separated canonical payload hash function from `canonical_payload.rs`.

## Accepted Boundary

Phase 41G.2 may establish only:

- the SVM-verified signed message bytes equal the domain-separated hash of the decoded canonical payload bytes.

Phase 41G.2 must not establish:

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

## Accepted Authoritative Sources

Raw payload decoder:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative field-order constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

Decoded shape:

`DecodedGuardianPayloadRaw<'a>`

Canonical hash boundary:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

Accepted existing functions/constants:

- `compute_guardian_payload_hash_domain_separator`;
- `compute_guardian_payload_hash`;
- `validate_guardian_payload_hash`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1`.

No parallel canonicalizer may be introduced.

No caller-provided hash may be trusted.

## Accepted Canonical 21-Field Shape

The candidate raw payload bytes must decode to this exact canonical field order:

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

This order must match `RAW_PAYLOAD_PHASE_23_FIELD_ORDER`.

No field may be omitted.

No field may be reordered.

No field may be renamed without explicit review.

## Accepted Exact Domain Separator Preimage

The domain separator is not the literal UTF-8 bytes of the label.

The domain separator is:

`keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1"))`

The payload hash preimage is:

`domain_separator_32_bytes || raw_payload_bytes`

The payload hash is:

`keccak256(domain_separator_32_bytes || raw_payload_bytes)`

Correct:

`keccak256(keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1")) || raw_payload_bytes)`

Wrong:

`keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1") || raw_payload_bytes)`

Future implementation must reuse the existing canonical path.

## Accepted Hash Binding Relation

The accepted future binding relation is:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Required implementation checks:

- `signed_message_bytes` length must be exactly 32 bytes;
- `raw_payload_bytes` must structurally decode through the accepted raw payload decoder;
- hash must be computed through the accepted domain-separated canonical hash boundary;
- equality must be byte-for-byte exact;
- mismatch must fail closed.

## Carry-Forward Requirement For Implementation — Prefer validate_guardian_payload_hash

During Phase 41G.2 implementation planning, prefer the already tested validation path:

`validate_guardian_payload_hash(raw_payload_bytes, &signed_hash_32)`

Recommended implementation flow:

1. receive borrowed `signed_message_bytes`;
2. require `signed_message_bytes.len() == 32`;
3. perform checked conversion to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. map success to the narrow payload-hash-binding status;
6. map failure to fail-closed binding rejection.

Rationale:

- `validate_guardian_payload_hash` recomputes the hash internally;
- caller-provided hash is not trusted;
- existing tested behavior is reused;
- manual duplicate equality logic is avoided.

## Accepted Raw Payload Provenance

`raw_payload_bytes` are caller/instruction-supplied and untrusted.

Successful structural decode proves only well-formedness.

Successful structural decode does not prove authenticity.

Authenticity for Phase 41G.2 comes only from:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Future implementation must not treat decoded payload fields as authentic until hash binding succeeds.

## Accepted Structural Decode / Authenticity Separation

Structural decode means:

- payload bytes are well-formed according to the accepted raw payload decoder.

Structural decode does not mean:

- the payload is authentic;
- the burn happened;
- finality was verified;
- watcher evidence is valid;
- guardian membership is valid;
- quorum exists;
- authorization exists.

Authenticity at this phase means only:

- SVM-verified signed message bytes match the recomputed domain-separated hash of the same raw payload bytes.

## Accepted Existing Canonicalizer Reuse

Phase 41G.2 must reuse:

- `raw_payload.rs`;
- `canonical_payload.rs`.

Reason:

- preserves Stage-1 vector parity;
- preserves Phase 33 / Phase 34 boundary;
- prevents renewed field-order drift;
- prevents encoding drift;
- prevents domain-separator drift.

No parallel encoder/canonicalizer should be introduced.

No caller-provided hash should be trusted.

## Accepted Stage-1 / Phase 33 / Phase 34 Parity Requirement

Phase 41G.2 must preserve parity with accepted vectors.

Future implementation tests should re-run existing vectors against the canonical hash boundary.

Required vector checks:

- known valid Stage-1 / Phase 33 / Phase 34 payload vector;
- expected domain separator;
- expected payload hash;
- raw payload decode success;
- hash validation success;
- mismatch failure.

If any vector mismatch occurs, 41G.2 must fail closed.

## Accepted Negative Matrix Requirements

Future implementation tests should include:

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

## Guardian / Finality / Expiration Deferral

Phase 41G.2 may prove only that the following are part of the signed payload hash:

- public key bytes are associated with the SVM-verified signature boundary;
- `guardian_set_id`;
- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

Phase 41G.2 must not validate:

- public key membership in guardian set;
- guardian set currentness;
- guardian set rotation;
- quorum;
- live source-chain finality;
- expiration enforcement.

Those checks remain later reviewed phases.

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

Phase 41G.2 payload hash binding plan is accepted.

Phase 41G.2 implementation planning may begin under a separate reviewed boundary.

Recommended implementation planning scope:

- exact function boundaries;
- result/status model;
- checked 32-byte signed message conversion;
- use of `validate_guardian_payload_hash`;
- tests;
- Stage-1 / Phase 33 / Phase 34 vector parity;
- no guardian validation;
- no quorum;
- no authorization;
- no replay write;
- no mutation;
- no CPI;
- no mint;
- no live route.

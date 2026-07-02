# XXXL Phase 41G.2 — Payload Hash Binding Plan

Date: 2026-07-03

## Status

Docs-only planning checkpoint.

No runtime code is introduced.

No verification logic is changed.

No guardian validity, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41G.1 payload evidence shape acceptance:

`c89fc59 Merge XXXL phase 41G payload evidence shape acceptance`

## Purpose

Phase 41G.2 plans the payload hash binding step.

It should establish only this narrow relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Where:

- `signed_message_bytes` are the SVM-verified Ed25519 message bytes from Phase 41F;
- `raw_payload_bytes` are caller/instruction-supplied candidate payload bytes shaped in Phase 41G.1;
- `compute_guardian_payload_hash` is the existing domain-separated canonical payload hash function from `canonical_payload.rs`.

## Critical Boundary

Phase 41G.2 payload hash binding must not establish:

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

It establishes only:

- the SVM-verified signed message bytes equal the domain-separated hash of the decoded canonical payload bytes.

## Authoritative Sources

Phase 41G.2 must reuse existing repository boundaries.

Raw payload decoder:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative field-order constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

Decoded shape:

`DecodedGuardianPayloadRaw<'a>`

Canonical hash boundary:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

Required existing functions/constants:

- `compute_guardian_payload_hash_domain_separator`;
- `compute_guardian_payload_hash`;
- `validate_guardian_payload_hash`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1`;
- `XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1`.

41G.2 must not rewrite canonicalization from scratch.

## Authoritative 21-Field Payload Shape

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

This order is authoritative and order-sensitive.

No field may be omitted.

No field may be reordered.

No field may be renamed without explicit review.

## Exact Hash Preimage

The domain separator is not the literal UTF-8 bytes of the label.

The domain separator is:

`keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1"))`

The payload hash preimage is:

`domain_separator_32_bytes || raw_payload_bytes`

The payload hash is:

`keccak256(domain_separator_32_bytes || raw_payload_bytes)`

Implementation must reuse the existing canonical path, equivalent to:

- compute domain separator through `compute_guardian_payload_hash_domain_separator`;
- compute payload hash through `compute_guardian_payload_hash`;
- or validate through `validate_guardian_payload_hash`.

The signed guardian hash and expected program hash must use identical preimage bytes.

## Hash Binding Relation

The accepted future binding relation is:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Required checks for future implementation:

- `signed_message_bytes` length must be exactly 32 bytes;
- `raw_payload_bytes` must structurally decode through the accepted raw payload decoder;
- hash must be computed through the accepted domain-separated canonical hash boundary;
- equality must be byte-for-byte exact;
- mismatch must fail closed.

## Raw Payload Provenance

`raw_payload_bytes` are caller/instruction-supplied and untrusted.

Successful structural decode proves only well-formedness.

Successful structural decode does not prove authenticity.

Authenticity for Phase 41G.2 comes only from:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

A future implementation must not treat decoded payload fields as authentic until hash binding succeeds.

## Phase 41F Link

Phase 41G.2 consumes Phase 41F only as an established SVM verification boundary.

Inputs from Phase 41F may include:

- `NativeEd25519VerificationEstablished`;
- borrowed signed Ed25519 message bytes;
- borrowed public key bytes;
- matched native Ed25519 instruction index;
- extracted message range.

Phase 41G.2 must not reinterpret Phase 41F as local cryptographic verification.

The SVM remains the verifier.

XXXL only consumes the established fact that SVM already verified the native Ed25519 instruction.

## Phase 41G.1 Link

Phase 41G.2 consumes Phase 41G.1 payload evidence shape.

Inputs from 41G.1 may include:

- borrowed raw canonical payload bytes;
- decoded raw payload;
- Phase 41F verified signed message bytes;
- Phase 41F public key bytes.

41G.2 may bind bytes to hash.

41G.2 must not accept source-chain proof, watcher evidence, guardian membership, quorum, or authorization.

## Expected Result Model

Future implementation may introduce a narrow status such as:

`GatewayPayloadHashBindingEstablished`

This status should mean only:

- raw payload bytes decoded structurally through the accepted decoder;
- expected domain-separated payload hash was computed through the accepted canonical hash boundary;
- SVM-verified signed message bytes exactly matched the computed hash.

This status must not mean:

- signer is a valid guardian;
- guardian set membership is accepted;
- quorum is reached;
- source burn is proven;
- finality is independently verified;
- expiration is enforced;
- authorization is granted;
- replay is safe;
- minting is allowed.

## Existing Canonicalizer Reuse Requirement

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

## Stage-1 / Phase 33 / Phase 34 Parity

Phase 41G.2 must preserve parity with accepted vectors.

Future tests should include:

- known valid Stage-1 / Phase 33 / Phase 34 payload vector;
- expected domain separator;
- expected payload hash;
- raw payload decode success;
- hash validation success;
- mismatch failure.

If any vector mismatch occurs, 41G.2 must fail closed.

## Required Negative Cases

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
- wrong `message_type`;
- wrong `schema_version`;
- wrong `instruction_layout_version`;
- wrong `route_id`;
- wrong `source_chain_id`;
- wrong `source_token`;
- wrong `source_sender`;
- wrong `source_burn_tx_hash`;
- wrong `source_burn_event_index`;
- wrong `source_block_number`;
- wrong `source_block_hash`;
- wrong `source_finality_block`;
- wrong `canonical_event_key`;
- wrong `x1_recipient`;
- wrong `burned_amount`;
- wrong `source_chain_weight_bps`;
- wrong `xxxl_mint_amount`;
- wrong `target_mint`;
- wrong `guardian_set_id`;
- wrong `message_nonce`;
- wrong `expiration_slot_or_unix_ts`;
- Stage-1 vector mismatch.

Each failure must be fail-closed.

No failure path may enable guardian/quorum/auth/replay/mutation/CPI/mint/live behavior.

## Public Key Boundary

Phase 41G.2 may carry public key bytes from Phase 41F.

Phase 41G.2 must not decide:

- whether the public key belongs to the guardian set;
- whether `guardian_set_id` is current;
- whether that public key counts toward quorum.

Guardian validation remains a later phase.

## Guardian Set ID Boundary

The canonical payload includes:

- `guardian_set_id`.

Phase 41G.2 may prove only that `guardian_set_id` is part of the signed payload hash.

It must not validate guardian set membership.

It must not validate guardian set rotation.

It must not validate guardian set currentness.

## Finality / Expiration Boundary

The canonical payload includes:

- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

Phase 41G.2 may prove only that both fields are part of the signed payload hash.

It must not validate live source-chain finality.

It must not enforce expiration.

Those checks remain later reviewed phases.

## SAFETY_FLAGS Expectations

Phase 41G.2 planning must not flip any runtime flag.

Future implementation may introduce a narrow payload-hash-binding status only if separately reviewed.

Even after payload hash binding succeeds, the following must remain false:

- source burn proof accepted;
- watcher honesty accepted;
- guardian validity accepted;
- guardian set membership accepted;
- quorum counting enabled;
- authorization enabled;
- replay write enabled;
- processed event marking enabled;
- account mutation enabled;
- CPI enabled;
- `invoke_signed` enabled;
- SPL Token `mint_to` enabled;
- process instruction handler added;
- live route enabled.

## Still Forbidden In Phase 41G.2 Plan

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

No blocker is removed, weakened, or reinterpreted by Phase 41G.2.

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

1. Is 41G.2 the correct next step after accepted 41G.1?
2. Is the exact domain separator preimage specified correctly?
3. Is the hash relation `signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)` correct?
4. Is reuse of `canonical_payload.rs` mandatory enough?
5. Is raw payload provenance/trust correctly stated?
6. Are structural decode and authenticity correctly separated?
7. Is hash comparison still separated from guardian/quorum/auth?
8. Are guardian set ID, public key, finality, and expiration correctly bound but not validated?
9. Are negative cases sufficient?
10. Are forbidden operations preserved?
11. Can Phase 41G.2 implementation planning begin after acceptance?

## Next Gate

After external acceptance, Phase 41G.2 implementation planning may begin under a separate reviewed boundary.

Implementation planning should cover:

- exact function boundaries;
- result/status model;
- tests;
- Stage-1 / Phase 33 / Phase 34 vector parity;
- no guardian/quorum/auth/replay/mutation/CPI/mint/live route.

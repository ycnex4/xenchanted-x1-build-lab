# XXXL Phase 41G.1 — Payload Evidence Shape Plan

Date: 2026-07-03

## Status

Docs-only planning checkpoint.

No runtime code is introduced.

No verification logic is changed.

No hash comparison is implemented.

No guardian validity, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41G.0 accepted after canonical field-order fix:

`b4ff536 Merge XXXL phase 41G payload binding plan acceptance`

## Purpose

Phase 41G.1 defines the shape of payload evidence that a later 41G.2 binding step may consume.

41G.1 should answer:

- what raw canonical payload bytes are carried;
- what decoded fields exist;
- how those fields are named;
- which existing decoder/canonicalizer owns the shape;
- what the shape must not claim.

41G.1 must not decide:

- whether the source burn happened;
- whether watcher evidence is valid;
- whether a public key is a guardian;
- whether quorum exists;
- whether minting is authorized;
- whether replay state can be written.

## Grounding Sources

Payload evidence shape must be grounded in existing repository boundaries.

Authoritative raw payload decoder:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative field-order constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

Authoritative decoded shape:

`DecodedGuardianPayloadRaw<'a>`

Authoritative canonical payload hash boundary for later 41G.2:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

Phase 41G.1 must not invent a parallel payload schema.

## Authoritative 21-Field Shape

The payload evidence shape must preserve this exact field order:

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

This field order is order-sensitive.

No field may be omitted.

No field may be renamed without explicit review.

No camelCase documentation alias may become implementation authority.

## Evidence Shape Concept

Phase 41G.1 should define a narrow evidence shape conceptually equivalent to this structure:

PayloadEvidenceShape:
- raw_payload_bytes: borrowed canonical payload bytes
- decoded_raw_payload: DecodedGuardianPayloadRaw<'a>
- phase_41f_verified_message_bytes: borrowed Ed25519 message bytes
- phase_41f_public_key_bytes: borrowed Ed25519 public key bytes

This is a conceptual shape only.

It is not an implementation requirement in this phase.

Future implementation may use Rust structs or reports, but must preserve the same boundary.

## Raw Payload Bytes

The shape must carry or reference the raw canonical payload bytes.

Rules:

- bytes are not trusted by caller assertion;
- bytes must be structurally decoded through the existing raw payload decoder;
- trailing bytes must be rejected by the decoder;
- truncated payloads must be rejected by the decoder;
- empty variable bytes must be rejected by the decoder;
- malformed structural encoding must be rejected by the decoder.

41G.1 does not compute the payload hash.

41G.2 must reuse the existing canonical hash boundary.

## Decoded Raw Payload

The decoded raw payload must correspond to the existing `DecodedGuardianPayloadRaw<'a>` fields:

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

The shape must preserve borrowed views where applicable.

The shape must not allocate attacker-sized copied buffers merely to represent decoded evidence.

## Phase 41F Link

Payload evidence shape may reference accepted Phase 41F outputs:

- `NativeEd25519VerificationEstablished`;
- borrowed signed Ed25519 message bytes;
- borrowed Ed25519 public key bytes;
- matched native Ed25519 instruction index;
- extracted message range;
- extracted public key range.

But Phase 41G.1 must not reinterpret Phase 41F as local cryptographic verification.

The SVM remains the verifier.

XXXL only consumes the established fact that SVM already verified the native Ed25519 instruction.

## Signed Message Bytes Role

In the accepted 41G model, guardians sign the expected gateway payload hash bytes.

Therefore the Phase 41F signed message bytes are expected to be the candidate guardian payload hash bytes.

41G.1 may shape this relationship.

41G.1 must not verify it.

41G.2 must perform the actual binding check.

Expected later relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Implementation-level carry-forward:

`compute_guardian_payload_hash(raw_payload_bytes)` must include the accepted domain separator.

## Domain Separator Carry-Forward

41G.1 must record that Phase 41G.2 must use the existing domain-separated hash boundary.

The accepted implementation-level model is:

`keccak256(XXXL_GUARDIAN_PAYLOAD_HASH_V1 || encoded_canonical_payload_bytes)`

41G.2 must not implement shorthand `keccak256(payload_bytes)`.

41G.2 must not bypass `canonical_payload.rs`.

## Existing Canonicalizer Reuse

41G.2 must reuse existing canonical payload code:

- `raw_payload.rs`;
- `canonical_payload.rs`.

41G.1 should make this dependency explicit so future implementation does not rewrite canonicalization from scratch.

Reason:

- preserves Stage-1 vector parity;
- preserves Phase 33 / Phase 34 boundary;
- prevents renewed field-order drift;
- prevents encoding drift.

## Evidence Shape Does Not Mean Evidence Acceptance

A shaped payload is not an accepted proof.

A shaped payload means only:

- bytes can be represented as candidate payload evidence;
- fields have known canonical names and order;
- a later phase can attempt binding.

A shaped payload does not mean:

- the burn happened;
- finality was independently checked;
- watcher proof is valid;
- public key is a guardian;
- guardian set is valid;
- quorum was reached;
- authorization exists;
- replay is safe;
- mint is allowed.

## Guardian Set ID Boundary

The shape includes:

- `guardian_set_id`.

41G.1 may carry and expose this field.

41G.1 must not validate guardian set membership.

Guardian set validation remains a later phase.

## Instruction Layout Version Boundary

The shape includes:

- `instruction_layout_version`.

41G.1 may carry and expose this field.

41G.1 must not introduce alternate layout acceptance.

Only the accepted decoder/canonicalizer boundary may interpret layout compatibility.

## Finality / Expiration Boundary

The shape includes two distinct temporal fields:

- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

41G.1 must keep them separate.

41G.1 must not validate live source-chain finality.

41G.1 must not enforce expiration.

Those checks remain later reviewed phases.

## Public Key Boundary

The shape may carry public key bytes from Phase 41F.

The public key bytes are useful for later guardian validation.

But Phase 41G.1 must not decide:

- whether the public key is in the guardian set;
- whether the guardian set is current;
- whether the key participates in quorum.

## Suggested Future Phase Split

### Phase 41G.1

Payload evidence shape only.

No hash computation.

No binding acceptance.

No guardian validation.

### Phase 41G.2

Payload hash binding.

Reuse existing `compute_guardian_payload_hash`.

Compare SVM-verified signed message bytes to the domain-separated canonical payload hash.

### Phase 41G.3

Negative matrix and audit for payload binding.

Wrong fields, malformed bytes, wrong hash, wrong domain, Stage-1 vector mismatch.

## Required 41G.1 Documentation Checks

41G.1 acceptance should require:

- exact 21-field list is present;
- field order matches `RAW_PAYLOAD_PHASE_23_FIELD_ORDER`;
- evidence shape uses existing `DecodedGuardianPayloadRaw<'a>` concept;
- raw bytes are not trusted by caller assertion;
- decoder ownership is explicit;
- domain-separated hash is deferred to 41G.2;
- existing canonicalizer reuse is required for 41G.2;
- guardian validation remains deferred;
- quorum/auth/replay/mutation/CPI/mint/live route remain forbidden.

## Still Forbidden In Phase 41G.1

The following remain forbidden:

- runtime instruction handler;
- local cryptographic verification;
- new hash implementation;
- payload hash comparison;
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

No blocker is removed, weakened, or reinterpreted by Phase 41G.1.

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

1. Is 41G.1 the correct next step after accepted 41G.0?
2. Is the evidence shape correctly grounded in `raw_payload.rs` and `DecodedGuardianPayloadRaw<'a>`?
3. Does the shape preserve all 21 canonical fields in exact order?
4. Is raw payload decode ownership explicit?
5. Is hash comparison correctly deferred to 41G.2?
6. Is domain-separated hash reuse correctly carried forward?
7. Is canonicalizer reuse correctly required for 41G.2?
8. Is guardian validation correctly deferred?
9. Are finality and expiration kept separate without validating either?
10. Are forbidden operations preserved?
11. Can 41G.2 payload hash binding plan begin after acceptance?

## Next Gate

After external acceptance, Phase 41G.2 may be planned under a separate reviewed boundary.

41G.2 should cover:

- domain-separated canonical payload hash computation;
- reuse of existing `compute_guardian_payload_hash`;
- comparison to SVM-verified signed message bytes;
- Stage-1 vector parity;
- no guardian/quorum/auth/replay/mutation/CPI/mint/live route.

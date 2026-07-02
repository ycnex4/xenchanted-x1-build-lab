# XXXL Phase 41G.1 — Payload Evidence Shape Plan Acceptance

Date: 2026-07-03

## Accepted Main

`b62b704 Merge XXXL phase 41G payload evidence shape plan`

## Parent Gate

`b4ff536 Merge XXXL phase 41G payload binding plan acceptance`

## Plan Commit

`1de3cf0 Document phase 41G payload evidence shape plan`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41G.2 may begin under a separate reviewed boundary.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Phase 41G.2 allowed after acceptance: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Scope violations: no
- Forbidden operations preserved: yes
- Phase 41G.2 allowed after acceptance: yes

## Accepted Purpose

Phase 41G.1 defines only the shape of candidate payload evidence for future Phase 41G.2 payload hash binding.

It defines:

- raw canonical payload bytes;
- decoded raw payload fields;
- Phase 41F verified signed message bytes;
- Phase 41F public key bytes;
- ownership of decoding by existing repository boundaries.

It does not accept the evidence as authentic.

## Key Accepted Principle

Evidence shape is not evidence acceptance.

A shaped payload means only:

- bytes can be represented as candidate payload evidence;
- fields have known canonical names and order;
- a later phase can attempt binding.

A shaped payload does not mean:

- the source burn happened;
- source finality was checked;
- watcher evidence is valid;
- signer is a guardian;
- guardian set membership is valid;
- quorum exists;
- authorization exists;
- replay is safe;
- mint is allowed.

## Authoritative Sources

Authoritative raw payload source:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative field-order constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

Authoritative decoded shape:

`DecodedGuardianPayloadRaw<'a>`

Authoritative future hash boundary:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

Phase 41G.1 must not invent a parallel payload schema.

## Accepted Canonical 21-Field Shape

The accepted payload evidence shape preserves this exact field order:

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

No field may be renamed without explicit review.

No camelCase documentation alias may become implementation authority.

## Accepted Decode Ownership

Raw payload bytes are not trusted by caller assertion.

Raw payload bytes must be structurally decoded through the existing raw payload decoder.

The existing decoder owns structural interpretation.

The decoder must reject:

- trailing bytes;
- truncated payloads;
- empty variable bytes;
- malformed structural encoding.

Structural decode proves only well-formedness.

Structural decode does not prove authenticity.

## Accepted Borrowed Evidence Shape

The accepted conceptual shape is:

PayloadEvidenceShape:
- raw_payload_bytes: borrowed canonical payload bytes
- decoded_raw_payload: DecodedGuardianPayloadRaw<'a>
- phase_41f_verified_message_bytes: borrowed Ed25519 message bytes
- phase_41f_public_key_bytes: borrowed Ed25519 public key bytes

This is a conceptual evidence-shape boundary.

It is not a mandate to introduce this exact Rust struct.

Future implementation may choose Rust structs or reports, but must preserve the same boundary.

## Accepted Phase 41F Link

41G.1 may reference accepted Phase 41F outputs:

- `NativeEd25519VerificationEstablished`;
- borrowed signed Ed25519 message bytes;
- borrowed Ed25519 public key bytes;
- matched native Ed25519 instruction index;
- extracted message range;
- extracted public key range.

41G.1 must not reinterpret Phase 41F as local cryptographic verification.

The SVM remains the verifier.

XXXL only consumes the established fact that SVM already verified the native Ed25519 instruction.

## Accepted Hash Deferral

41G.1 does not compute the payload hash.

41G.1 does not compare the signed message bytes to the payload hash.

41G.2 must perform the actual binding check.

Future expected relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

## Carry-Forward Requirement For Phase 41G.2 — Exact Domain Separator Preimage

Phase 41G.2 must use the existing domain-separated hash boundary.

The domain separator is not the raw UTF-8 label bytes prepended directly to the payload.

The implementation-level preimage is:

`keccak256(keccak256(utf8("XXXL_GUARDIAN_PAYLOAD_HASH_V1")) || encoded_canonical_payload_bytes)`

Equivalently, Phase 41G.2 must reuse the existing `compute_guardian_payload_hash` path from `canonical_payload.rs`.

The signed guardian hash and the expected program hash must use identical preimage bytes, including:

- the 32-byte domain separator hash;
- encoded canonical payload bytes;
- exact field order;
- exact byte encoding;
- exact Stage-1 / Phase 33 / Phase 34 parity.

## Carry-Forward Requirement For Phase 41G.2 — Raw Payload Provenance

Raw payload bytes are caller/instruction-supplied and untrusted.

Structural decode proves only that bytes are well-formed according to the accepted decoder.

Authenticity comes only from the Phase 41G.2 binding relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

Future implementation must not treat successful decode as proof that the payload is authentic.

## Carry-Forward Requirement For Phase 41G.2 — Existing Canonicalizer Reuse

Phase 41G.2 must not rewrite canonicalization from scratch.

Phase 41G.2 must reuse existing canonical payload code:

- `programs/xxxl-svm/src/verifier/raw_payload.rs`;
- `programs/xxxl-svm/src/verifier/canonical_payload.rs`.

Reason:

- preserve Stage-1 vector parity;
- preserve Phase 33 / Phase 34 boundary;
- prevent renewed field-order drift;
- prevent encoding drift.

## Guardian Validation Deferral

41G.1 may carry public key bytes from Phase 41F.

41G.1 may carry `guardian_set_id`.

41G.1 must not decide:

- whether the public key is in the guardian set;
- whether the guardian set is current;
- whether the key participates in quorum.

Guardian set membership remains a later phase.

## Finality / Expiration Deferral

41G.1 keeps these fields separate:

- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

41G.1 must not validate live source-chain finality.

41G.1 must not enforce expiration.

Those checks remain later reviewed phases.

## Still Forbidden

The following remain forbidden:

- runtime instruction handler;
- local cryptographic verification unless separately reviewed;
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

Phase 41G.1 is accepted.

Phase 41G.2 may begin under a separate reviewed boundary.

Recommended Phase 41G.2 scope:

- domain-separated canonical payload hash computation;
- reuse of existing `compute_guardian_payload_hash`;
- exact 32-byte domain separator hash preimage;
- comparison to SVM-verified signed message bytes;
- raw payload bytes remain untrusted until hash binding succeeds;
- Stage-1 vector parity;
- no guardian validation;
- no quorum;
- no authorization;
- no replay write;
- no mutation;
- no CPI;
- no mint;
- no live route.

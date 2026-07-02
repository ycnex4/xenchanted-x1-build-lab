# XXXL X1 Testnet Local Runtime Skeleton — Phase 41G.0 Proof / Evidence / Payload Binding Plan Checkpoint

Date: 2026-07-02

## Phase

Phase 41G.0 — Proof / Evidence / Payload Binding Plan.

## Parent Gate

`72951e8 Merge XXXL phase 41F focused crypto boundary audit acceptance`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

## Purpose

Plan the next layer after Phase 41F:

- bind the SVM-verified Ed25519 message bytes to the expected gateway payload hash.

## Critical Separation

Phase 41G payload binding remains separate from:

- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- handler;
- live route.

## Preferred Model

Preferred model:

- guardians sign `expected_gateway_payload_hash_bytes`;
- Phase 41G checks `signed_message_bytes == expected_gateway_payload_hash_bytes`;
- expected hash is planned as `keccak256(canonical_gateway_payload_bytes)`.

## Next Gate

After external acceptance, Phase 41G.1 payload evidence shape may begin under separate review.

## Canonical Field Order Fix

The authoritative canonical field order must exactly match:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

The canonical field count is 21.

Required fields include:

- `instruction_layout_version`;
- `guardian_set_id`;
- `source_finality_block`;
- `expiration_slot_or_unix_ts`.

The previous 19-field planning list is superseded.

`source_finality_block` and `expiration_slot_or_unix_ts` are distinct and must not be collapsed.

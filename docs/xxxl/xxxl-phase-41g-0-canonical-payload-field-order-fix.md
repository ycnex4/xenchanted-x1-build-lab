# XXXL Phase 41G.0 — Canonical Payload Field Order Fix

Date: 2026-07-03

## Status

Docs-only blocking fix after external review.

## Parent

`c05732d Merge XXXL phase 41G payload binding plan`

## Reason

Audit Demon detected a security-relevant mismatch:

- Phase 41G.0 plan listed 19 canonical payload fields;
- authoritative Rust decoder declares 21 canonical fields.

Authoritative source:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Constant:

`RAW_PAYLOAD_PHASE_23_FIELD_ORDER`

## Authoritative Canonical Field Order

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

## Blocking Fixes Addressed

This patch addresses:

- missing `instruction_layout_version`;
- missing `guardian_set_id`;
- conflated finality/expiration field;
- incorrect field order;
- missing negative cases for `instruction_layout_version`, `guardian_set_id`, `source_finality_block`, and `expiration_slot_or_unix_ts`.

## Security Rationale

`guardian_set_id` is required in the signed payload hash.

Without it, a signature may not be bound to a specific guardian set, creating replay risk across guardian-set rotation.

`instruction_layout_version` is required to bind signatures to a specific decoding/canonicalization layout.

`source_finality_block` and `expiration_slot_or_unix_ts` are separate canonical fields and must not be collapsed.

## Scope

Docs-only.

No runtime code.

No verification logic change.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

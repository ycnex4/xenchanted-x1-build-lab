# XXXL Phase 41G.1 Review Request — Payload Evidence Shape Plan

Date: 2026-07-03

## Current Main

`b4ff536 Merge XXXL phase 41G payload binding plan acceptance`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41g-1-payload-evidence-shape-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41g-1-payload-evidence-shape-plan.md`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

No hash comparison.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Define the shape of candidate payload evidence for future Phase 41G.2 payload hash binding.

## Grounding

Authoritative raw payload source:

`programs/xxxl-svm/src/verifier/raw_payload.rs`

Authoritative decoded shape:

`DecodedGuardianPayloadRaw<'a>`

Authoritative future hash boundary:

`programs/xxxl-svm/src/verifier/canonical_payload.rs`

## Canonical Field Order

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

## Requested Review

Please check:

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

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Correct next phase after 41G.0: yes/no
- Grounded in raw_payload.rs / DecodedGuardianPayloadRaw: yes/no
- Canonical 21-field shape preserved: yes/no
- Decode ownership explicit: yes/no
- Hash comparison deferred: yes/no
- Domain-separated hash carry-forward acceptable: yes/no
- Existing canonicalizer reuse required: yes/no
- Guardian validation deferral acceptable: yes/no
- Finality/expiration separation acceptable: yes/no
- Forbidden operations preserved: yes/no
- Phase 41G.2 allowed after acceptance: yes/no

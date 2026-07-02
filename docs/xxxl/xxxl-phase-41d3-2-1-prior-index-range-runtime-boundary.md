# XXXL Phase 41D3.2.1 — Prior Index Range Runtime Boundary

Date: 2026-07-02

## Status

Code boundary implemented.

## Parent Gate

Parent accepted checkpoint:

`b1c17cd Merge XXXL phase 41D3 prior enumeration plan acceptance record`

Phase 41D3.2.0 allowed 41D3.2.1 to start with a narrow boundary:

- accept checked current index from Phase 41D3.1;
- construct bounded prior range `0..current_index`;
- `current_index == 0` yields an empty range;
- no instruction loading;
- no sysvar parsing;
- no descriptors;
- no flag flips.

## Scope

Phase 41D3.2.1 introduces only prior index range construction.

Allowed:

- consume the Phase 41D3.1 checked current-index result;
- require `CurrentInstructionIndexAcquired`;
- require `Some(current_instruction_index)`;
- construct `0..current_index`;
- ensure all constructed indexes are strictly `< current_index`;
- map `current_index == 0` to empty prior range;
- fail closed if current index is unavailable or structurally inconsistent;
- fail closed if a forged oversized current index is supplied.

Not allowed:

- `load_instruction`;
- `load_instruction_at`;
- `load_instruction_at_checked`;
- raw Instructions sysvar byte parsing;
- instruction data access;
- Ed25519 candidate construction;
- Phase 41C3 descriptor construction;
- proof acceptance;
- evidence acceptance;
- guardian quorum counting;
- authorization;
- replay writes;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- runtime handler;
- live route unlock.

## Boundary Behavior

### Current index unavailable

If Phase 41D3.1 did not acquire a checked current index, Phase 41D3.2.1 returns:

- status: `CurrentInstructionIndexUnavailable`;
- no prior indexes;
- no instruction loading;
- no evidence acceptance;
- no authorization.

### Current index zero

If current index is `0`, Phase 41D3.2.1 returns:

- status: `EmptyPriorIndexRange`;
- prior range: empty;
- reason: no instruction can exist before index `0`.

### Current index greater than zero

If current index is `n > 0`, Phase 41D3.2.1 returns:

- status: `PriorIndexRangeConstructed`;
- prior range: `0..n`;
- same index `n` is excluded by construction;
- later indexes are excluded by construction.

## Safety Flags

Phase 41D3.2.1 does not flip loading, locating, evidence, authorization, replay, CPI, mint, handler, or live-route flags.

Still false:

- `raw_instructions_sysvar_parser_implemented`;
- `load_instruction_called`;
- `load_instruction_enabled`;
- `locates_prior_ed25519_instruction`;
- `ed25519_signature_verification_performed`;
- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
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

## Validation Targets

Expected targeted test:

- `cargo test prior_instruction_index_range_runtime_boundary --lib`

Expected broad tests:

- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

## Next Step

Do not start Phase 41D3.2.2 checked instruction loading until Phase 41D3.2.1 is externally reviewed and accepted.

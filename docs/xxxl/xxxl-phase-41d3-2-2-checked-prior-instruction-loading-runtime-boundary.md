# XXXL Phase 41D3.2.2 — Checked Prior Instruction Loading Runtime Boundary

Date: 2026-07-02

## Status

Code boundary implemented.

## Parent Gate

Parent accepted checkpoint:

`5b8850e Merge XXXL phase 41D3 checked loading plan acceptance record`

Phase 41D3.2.2.0 accepted this code boundary:

- consume bounded prior range from Phase 41D3.2.1;
- borrow prior indexes and iterate with `.iter().copied()`;
- accept Instructions sysvar AccountInfo as checked loading source;
- verify Instructions sysvar account key before loading;
- call `load_instruction_at_checked(index, instructions_sysvar_account)` only for prior indexes;
- deterministic mapping of checked loading success/failure;
- empty prior range causes no loading attempt;
- checked loading failure is non-panic and non-authorizing;
- loaded instruction remains runtime data only.

## Scope

Phase 41D3.2.2 introduces checked prior instruction loading only.

Allowed:

- consume the Phase 41D3.2.1 prior index range result;
- require a valid prior range;
- verify all prior indexes remain strictly before current index;
- accept Instructions sysvar AccountInfo;
- verify the Instructions sysvar account key;
- iterate over prior indexes with `.iter().copied()`;
- call `load_instruction_at_checked` only after key check and only for prior indexes;
- map success to runtime-data-only loaded entries;
- map failure to deterministic non-authorizing failure;
- treat empty prior range as no loading attempt.

Not allowed:

- `load_instruction`;
- `load_instruction_at`;
- unchecked loading;
- raw Instructions sysvar byte parsing;
- direct sysvar byte slicing;
- prefiltering;
- Phase 41C3 descriptor construction;
- Ed25519 candidate location;
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

### Empty prior range

If Phase 41D3.2.1 returns an empty prior range:

- no sysvar account is required;
- no loading is attempted;
- `load_instruction_at_checked` is not called;
- no evidence or authorization is produced.

### Missing or wrong Instructions sysvar

If the prior range is non-empty and the Instructions sysvar account is missing or has the wrong key:

- the boundary fails closed;
- no loading is attempted;
- no evidence or authorization is produced.

### Checked loading success

A successfully loaded instruction is recorded as runtime data only.

It does not mean:

- Ed25519 candidate exists;
- evidence is valid;
- signature is valid;
- guardian is authorized;
- quorum is reached;
- mint is authorized;
- replay registry may be updated;
- runtime state may mutate.

### Checked loading failure

Checked loading failure maps to deterministic failure:

- no panic;
- no partial authorization;
- no evidence acceptance;
- no state mutation.

## Safety Flags

Allowed loading-related flips:

- `prior_instruction_loading_enabled: true`;
- `load_instruction_called: true`;
- `load_instruction_enabled: true`.

Still false:

- `raw_instructions_sysvar_parser_implemented`;
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

- `cargo test checked_prior_instruction_loading_runtime_boundary --lib`

Expected broad tests:

- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

## Next Step

Do not start Phase 41D3.2.3 prefilter/descriptor construction until Phase 41D3.2.2 is externally reviewed and accepted.

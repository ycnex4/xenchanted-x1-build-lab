# XXXL Mollusk Coverage Review Package

Status: Completed
Branch: `stage-xxxl-mollusk-coverage-review-package`
Base: `848cde9 Merge XXXL Mollusk rent lifecycle coverage`

## Purpose

This document consolidates the accumulated Mollusk/SBF entrypoint evidence for the XXXL SVM scaffold after the completed Mollusk coverage stages.

The purpose is review and classification only.

This package does not remove `MOLLUSK_COVERAGE_INCOMPLETE`.

This package does not transition any deployment blocker.

This package does not claim deployment readiness or release readiness.

## Current Mollusk Baseline

The current branch contains 44 Mollusk tests according to:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml mollusk -- --list`

The accumulated direct Mollusk/SBF evidence now covers:

1. harness boundary
2. account meta and account ordering rejection
3. program-owned account validation rejection
4. SPL Token mint and recipient account validation rejection
5. PDA validation rejection
6. entrypoint no-mutation behavior
7. replay and validation rejection
8. instruction strictness rejection
9. rent-exemption rejection

## Direct Mollusk/SBF Evidence

The following areas are covered by tests that execute through the Mollusk/SBF entrypoint.

### 1. Harness Boundary

Evidence:

- the Mollusk harness can execute the compiled SBF program
- malformed instruction input is rejected through the SBF entrypoint
- the rejected path does not require live route execution
- the rejected path does not require SPL CPI execution
- the rejected path does not require `invoke_signed`
- the rejected path does not require SPL Token `mint_to`

Representative test:

- `mollusk_harness_rejects_malformed_instruction_without_live_route`

Boundary:

This proves that the harness is real and useful for entrypoint-level negative coverage. It does not prove live route execution or SPL CPI behavior.

### 2. Account Meta / Order Coverage

Evidence:

- wrong account count is rejected
- wrong account order is rejected
- writable / readonly mismatch is rejected
- unexpected signer is rejected

Representative tests:

- `mollusk_rejects_wrong_account_count_without_live_route`
- `mollusk_rejects_wrong_account_order_without_live_route`
- `mollusk_rejects_writable_readonly_mismatch_without_live_route`
- `mollusk_rejects_unexpected_signer_without_live_route`

Boundary:

This proves that malformed account lists and account metadata are rejected before live route execution.

### 3. Program-Owned Account Validation Coverage

Evidence:

- wrong owner for program-owned accounts is rejected
- wrong discriminator is rejected
- truncated program-owned account data is rejected

Representative tests:

- `mollusk_rejects_wrong_mint_state_owner_without_live_route`
- `mollusk_rejects_wrong_gateway_config_owner_without_live_route`
- `mollusk_rejects_wrong_guardian_set_owner_without_live_route`
- `mollusk_rejects_wrong_mint_state_discriminator_without_live_route`
- `mollusk_rejects_truncated_gateway_config_without_live_route`

Boundary:

This proves direct entrypoint rejection for invalid program-owned account state. It does not prove broader closed/reinitialized lifecycle assumptions beyond the tested rejection cases.

### 4. SPL Token Mint and Recipient Account Validation Coverage

Evidence:

- wrong SPL mint owner is rejected
- wrong SPL mint authority is rejected
- uninitialized SPL mint is rejected
- wrong recipient token mint is rejected
- wrong recipient token owner is rejected
- uninitialized recipient token account is rejected

Representative tests:

- `mollusk_rejects_wrong_spl_mint_owner_without_live_route`
- `mollusk_rejects_wrong_spl_mint_authority_without_live_route`
- `mollusk_rejects_uninitialized_spl_mint_without_live_route`
- `mollusk_rejects_wrong_recipient_token_mint_without_live_route`
- `mollusk_rejects_wrong_recipient_token_owner_without_live_route`
- `mollusk_rejects_uninitialized_recipient_token_account_without_live_route`

Boundary:

This proves direct entrypoint rejection for invalid SPL Token mint and recipient token account inputs. It does not execute SPL Token CPI.

### 5. PDA Coverage

Evidence:

- wrong mint authority PDA is rejected
- wrong mint authority bump is rejected
- mint authority PDA derived for the wrong program id is rejected
- semantic PDA mismatch is rejected

Representative tests:

- `mollusk_rejects_wrong_mint_authority_pda_without_live_route`
- `mollusk_rejects_wrong_mint_authority_bump_without_live_route`
- `mollusk_rejects_mint_authority_pda_for_wrong_program_id_without_live_route`
- `mollusk_rejects_mint_authority_pda_semantic_mismatch_without_live_route`

Boundary:

This proves PDA validation rejection before any `invoke_signed` path is enabled or reachable.

### 6. Entrypoint No-Mutation Coverage

Evidence:

- valid scaffold entrypoint path leaves mutable accounts unchanged while live route is disabled
- selected rejection paths leave mutable accounts unchanged
- zero amount rejection leaves mutable accounts unchanged
- replay / processed event rejection leaves mutable accounts unchanged
- wrong processed event recipient rejection leaves mutable accounts unchanged
- wrong recipient token account rejection leaves mutable accounts unchanged

Representative tests:

- `mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged`
- `mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_wrong_recipient_token_account_rejection_leaves_mutable_accounts_unchanged`

Boundary:

This proves no-mutation behavior for the current locked scaffold paths. It does not prove future live route atomicity after live execution is enabled.

### 7. Replay and Validation Rejection Coverage

Evidence:

- already-consumed processed event replay is rejected
- wrong processed event canonical event key is rejected
- wrong processed event route id is rejected
- wrong processed event recipient is rejected
- wrong recipient balance owner is rejected
- wrong recipient balance mint is rejected

Representative tests:

- `mollusk_rejects_consumed_processed_event_replay_without_live_route`
- `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`
- `mollusk_rejects_wrong_processed_event_route_id_without_live_route`
- `mollusk_rejects_wrong_processed_event_recipient_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`

Boundary:

This proves replay and state-mismatch rejection before live mint execution. It does not prove future positive mint settlement.

### 8. Instruction Strictness Coverage

Evidence:

- wrong instruction discriminator is rejected
- wrong instruction version is rejected
- extra instruction bytes are rejected
- wrong encoded account meta count is rejected
- wrong encoded processed event account index is rejected
- wrong encoded recipient balance account index is rejected

Representative tests:

- `mollusk_rejects_wrong_instruction_discriminator_without_live_route`
- `mollusk_rejects_wrong_instruction_version_without_live_route`
- `mollusk_rejects_extra_instruction_bytes_without_live_route`
- `mollusk_rejects_wrong_encoded_account_meta_count_without_live_route`
- `mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route`
- `mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route`

Boundary:

This proves strict instruction decoding and encoded-field rejection through the current SBF entrypoint.

### 9. Rent / Lifecycle Coverage

Evidence:

- low-rent mint state account is rejected
- low-rent gateway config account is rejected
- low-rent guardian set account is rejected
- low-rent processed event account is rejected
- low-rent recipient balance account is rejected
- low-rent SPL Token mint account is rejected
- low-rent recipient token account is rejected

Representative tests:

- `mollusk_rejects_low_rent_mint_state_without_live_route`
- `mollusk_rejects_low_rent_gateway_config_without_live_route`
- `mollusk_rejects_low_rent_guardian_set_without_live_route`
- `mollusk_rejects_low_rent_processed_event_without_live_route`
- `mollusk_rejects_low_rent_recipient_balance_without_live_route`
- `mollusk_rejects_low_rent_spl_token_mint_without_live_route`
- `mollusk_rejects_low_rent_recipient_token_account_without_live_route`

Boundary:

This proves rent-exemption rejection for program-owned accounts and SPL Token accounts through the SBF entrypoint.

It does not claim full lifecycle closure. Broader closed/reinitialized account assumptions remain future review topics.

## Rust-Boundary Evidence

The following evidence exists, but it is not the same as direct Mollusk/SBF entrypoint coverage.

### Deployment Status Blockers

The runtime deployment status still reports active blockers, including:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This confirms that the runtime remains blocked and not deployable.

### Disabled SPL CPI Gate

Current evidence around disabled SPL CPI execution is still Rust-boundary evidence.

The current scaffold prevents SPL CPI execution from becoming reachable, but there is no direct Mollusk test that executes a real SPL Token `mint_to` CPI path, because that path remains intentionally disabled.

### `invoke_signed` and `mint_to`

The current tests prove that invalid inputs fail before any `invoke_signed` or SPL Token `mint_to` path is enabled or reachable.

They do not prove successful `invoke_signed` execution.

They do not prove successful SPL Token `mint_to` execution.

They do not prove SPL CPI failure handling after CPI execution is enabled.

## Remaining Coverage Gaps

The following areas remain outside the completed direct Mollusk/SBF coverage:

1. live route execution success path
2. SPL Token `mint_to` CPI success path
3. SPL Token `mint_to` CPI failure path
4. `invoke_signed` execution with production PDA authority
5. real Program ID and regenerated PDA fixtures
6. production guardian set and threshold policy
7. production proof-log publication and retention policy
8. external review completion
9. closed/reinitialized account lifecycle assumptions beyond current negative validations
10. future live-route atomicity after live execution is enabled

## Review Assessment

The accumulated Mollusk/SBF evidence is now broad and covers the current locked scaffold rejection surface well.

However, this review package does not conclude that `MOLLUSK_COVERAGE_INCOMPLETE` should be removed.

A separate coverage assessment stage should decide whether the current direct Mollusk evidence is sufficient to transition only `MOLLUSK_COVERAGE_INCOMPLETE`, while keeping all other deployment blockers active.

## Safety Boundary

This package does not enable or modify:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian configuration
- production proof logs
- deployment readiness
- release readiness

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

## Recommended Next Stage

The next stage should be:

`stage-xxxl-mollusk-coverage-assessment`

That stage should make an explicit decision on whether the accumulated direct Mollusk/SBF evidence is sufficient for a future `MOLLUSK_COVERAGE_INCOMPLETE` blocker transition.

This review package itself does not recommend an immediate blocker transition.

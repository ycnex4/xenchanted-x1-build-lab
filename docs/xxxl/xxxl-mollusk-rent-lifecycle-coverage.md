# XXXL Mollusk Rent / Lifecycle Coverage

Status: Completed
Branch: `stage-xxxl-mollusk-rent-lifecycle-coverage`

## Purpose

This stage adds direct Mollusk/SBF entrypoint coverage for rent-exemption rejection paths in the XXXL SVM scaffold.

The covered path is:

`process_instruction`
→ `process_consume_gateway_mint`
→ `Rent::get()`
→ `build_runtime_consume_gateway_mint_execution_plan_boundary`
→ `prepare_consume_gateway_mint_cpi_boundary`
→ `assert_rent_exempt`

The goal is to prove that low-rent / non-rent-exempt accounts are rejected before any live route execution, SPL CPI execution, `invoke_signed`, or SPL Token `mint_to`.

## Tests Added

The following non-ignored Mollusk tests were added to:

`programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

1. `mollusk_rejects_low_rent_mint_state_without_live_route`
2. `mollusk_rejects_low_rent_gateway_config_without_live_route`
3. `mollusk_rejects_low_rent_guardian_set_without_live_route`
4. `mollusk_rejects_low_rent_processed_event_without_live_route`
5. `mollusk_rejects_low_rent_recipient_balance_without_live_route`
6. `mollusk_rejects_low_rent_spl_token_mint_without_live_route`
7. `mollusk_rejects_low_rent_recipient_token_account_without_live_route`

Each test uses an otherwise valid scaffold instruction and account set, lowers the lamports of exactly one account below rent-exempt requirements, and expects:

`ProgramError::Custom(XxxlError::InvalidRentExemption as u32)`

## Coverage Boundary

This stage covers rent-exemption rejection for:

- program-owned XXXL accounts
- SPL Token mint account
- SPL Token recipient token account

This is direct Mollusk/SBF entrypoint evidence.

This stage does not add new runtime semantics. It only proves that existing reachable rent checks are enforced through the current entrypoint path.

## Lifecycle Boundary

This stage intentionally does not claim full lifecycle closure.

Lifecycle-like validation cases have already been partially covered by earlier Mollusk stages, including:

- wrong owner rejection
- wrong account order / account meta rejection
- wrong discriminator rejection
- truncated data rejection
- uninitialized SPL mint rejection
- uninitialized recipient token account rejection
- replay / already-processed rejection
- instruction strictness rejection

Broader account lifecycle assumptions, including closed/reinitialized account scenarios and future runtime composition assumptions, remain review-package topics.

## Safety Boundary

This stage does not enable or modify:

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

No blocker is removed or transitioned by this stage.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

Remaining active blockers include:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Validation

Reported local validation:

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml --check`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml mollusk`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml`
- `git diff --check`

Reported result:

- fmt passed
- Mollusk tests passed: 44 passed
- full package tests passed: 199 lib passed, 44 Mollusk passed, 10 ignored
- `git diff --check` passed

## Result

This stage adds direct evidence that low-rent accounts are rejected through the Mollusk/SBF entrypoint before any live mint path can execute.

The next likely stage is a Mollusk coverage review package, not a blocker transition.

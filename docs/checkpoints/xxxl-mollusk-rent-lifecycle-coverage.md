# XXXL Mollusk Rent / Lifecycle Coverage Checkpoint

Status: Completed
Branch: `stage-xxxl-mollusk-rent-lifecycle-coverage`

## Summary

This checkpoint records completion of the XXXL Mollusk rent/lifecycle coverage stage.

The stage added direct Mollusk/SBF entrypoint tests proving that low-rent / non-rent-exempt accounts are rejected with:

`ProgramError::Custom(XxxlError::InvalidRentExemption as u32)`

The stage does not modify runtime source, Cargo files, deployment blocker logic, or safety invariant logic.

## Files Changed

Expected changed files:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-rent-lifecycle-coverage.md`
- `docs/checkpoints/xxxl-mollusk-rent-lifecycle-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Tests Added

Added to:

`programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

- `mollusk_rejects_low_rent_mint_state_without_live_route`
- `mollusk_rejects_low_rent_gateway_config_without_live_route`
- `mollusk_rejects_low_rent_guardian_set_without_live_route`
- `mollusk_rejects_low_rent_processed_event_without_live_route`
- `mollusk_rejects_low_rent_recipient_balance_without_live_route`
- `mollusk_rejects_low_rent_spl_token_mint_without_live_route`
- `mollusk_rejects_low_rent_recipient_token_account_without_live_route`

## Validation Reported

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

## Explicit Non-Changes

This stage did not change:

- `programs/xxxl-svm/src/*`
- `Cargo.toml`
- `Cargo.lock`
- deployment blocker logic
- safety invariant logic
- production PDA fixtures

This stage did not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Boundary

This stage adds direct rent-exemption rejection evidence through Mollusk/SBF entrypoint tests.

It does not claim full lifecycle closure.

Broader lifecycle assumptions, including closed/reinitialized account cases and future runtime composition assumptions, remain review-package topics.

## Next Step

The next likely stage should be:

`stage-xxxl-mollusk-coverage-review-package`

It should summarize accumulated direct Mollusk/SBF evidence and clearly separate it from Rust-boundary evidence.

This checkpoint does not recommend a blocker transition.

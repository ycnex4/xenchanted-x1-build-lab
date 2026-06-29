# Checkpoint: XXXL Mollusk Instruction Strictness Coverage

## Status

Completed as a narrow Mollusk/SVM instruction strictness coverage stage for the
current locked `consume_gateway_mint` scaffold.

This stage adds non-ignored Mollusk tests for malformed instruction bytes and
wrong encoded account/index/count field rejection paths.

## Tests Added

- `mollusk_rejects_wrong_instruction_discriminator_without_live_route`
- `mollusk_rejects_wrong_instruction_version_without_live_route`
- `mollusk_rejects_extra_instruction_bytes_without_live_route`
- `mollusk_rejects_wrong_encoded_account_meta_count_without_live_route`
- `mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route`
- `mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route`

## Files Changed

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-instruction-strictness-coverage.md`
- `docs/checkpoints/xxxl-mollusk-instruction-strictness-coverage.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Cargo Changes

No.

`Cargo.toml` and `Cargo.lock` are unchanged.

No dependencies were added.

## Blocker State

No blocker was removed.

No blocker was transitioned.

`MOLLUSK_COVERAGE_INCOMPLETE` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Safety Non-Changes

Runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

Production PDA fixtures were not regenerated.

This stage does not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`

This stage does not claim deployment readiness, future live route atomicity, or
future SPL CPI success/failure behavior.

This stage does not claim direct Mollusk entrypoint coverage for the disabled
SPL CPI gate. The disabled SPL CPI gate remains Rust-boundary evidence until a
future reviewed runtime-composition stage.

## Next Stage

The next Mollusk stage should be rent and lifecycle coverage or a coverage
review package, not blocker transition.

## Final Statement

The Mollusk instruction strictness coverage stage is complete, but
`MOLLUSK_COVERAGE_INCOMPLETE` remains active and runtime deployment remains
blocked.

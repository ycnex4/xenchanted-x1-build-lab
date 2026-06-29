# Checkpoint: XXXL Mollusk Replay and Validation Rejection Coverage

## Status

Completed as a narrow Mollusk/SVM replay and validation rejection coverage
stage for the current locked `consume_gateway_mint` scaffold.

This stage adds non-ignored Mollusk tests for selected reachable
processed-event replay, processed-event binding, and recipient-balance binding
rejection paths.

## Tests Added

- `mollusk_rejects_consumed_processed_event_replay_without_live_route`
- `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`
- `mollusk_rejects_wrong_processed_event_route_id_without_live_route`
- `mollusk_rejects_wrong_processed_event_recipient_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`
- `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`

## Files Changed

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-replay-validation-rejection-coverage.md`
- `docs/checkpoints/xxxl-mollusk-replay-validation-rejection-coverage.md`
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

This stage does not claim future live route atomicity or future SPL CPI
success/failure behavior.

This stage does not claim direct Mollusk entrypoint coverage for the disabled
SPL CPI gate. The disabled SPL CPI gate remains Rust-boundary evidence until a
future reviewed runtime-composition stage.

## Next Stage

The next Mollusk stage should be instruction strictness coverage or rent and
lifecycle coverage, not blocker transition.

## Final Statement

The Mollusk replay and validation rejection coverage stage is complete, but
`MOLLUSK_COVERAGE_INCOMPLETE` remains active and runtime deployment remains
blocked.

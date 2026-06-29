# Checkpoint: XXXL Mollusk Harness Boundary

## Status

Completed as the first narrow Mollusk/SVM harness boundary.

This stage adds one safe non-ignored Mollusk sanity test for the locked
scaffold.

## Harness Scope

Added test:

- `mollusk_harness_rejects_malformed_instruction_without_live_route`

The test loads the XXXL SBF artifact through Mollusk and verifies that a
malformed instruction length is rejected with `XxxlError::InvalidInstruction`.

## Files Changed

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `docs/xxxl/xxxl-mollusk-harness-boundary.md`
- `docs/checkpoints/xxxl-mollusk-harness-boundary.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Cargo Changes

No.

`Cargo.toml` and `Cargo.lock` are unchanged.

The existing Mollusk dev-dependency is reused.

## Validation

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml --check`: passed
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml deployment_status --lib`: passed, 18 tests
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml safety_invariant --lib`: passed, 22 tests
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib`: passed, 199 tests
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml mollusk`: passed, including `mollusk_harness_rejects_malformed_instruction_without_live_route`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml`: passed, 199 lib tests plus 1 non-ignored Mollusk integration test, 12 ignored integration tests
- `git diff --check`: passed

## Blocker State

No blocker is removed.

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

This stage does not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`

## Next Stage

The next Mollusk stage should be account meta/order coverage, not blocker
transition.

## Final Statement

The Mollusk harness now has one safe rejected-path sanity test, but
`MOLLUSK_COVERAGE_INCOMPLETE` remains active and runtime deployment remains
blocked.

# XXXL X1 Testnet Local Runtime Skeleton Phase 41C2 Current Instruction Identity Boundary

Status: Narrow Rust boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41c2-current-instruction-identity-boundary`

## Purpose

Phase 41C2 introduces a current-instruction identity boundary over an explicit
descriptor.

It intentionally does not read real Solana `AccountInfo`.

It intentionally does not call `load_instruction`, `load_instruction_at`, or
`load_instruction_at_checked`.

It intentionally does not locate prior Ed25519 instructions.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/current_instruction_identity_boundary.rs`
- `docs/xxxl/xxxl-phase-41c2-current-instruction-identity-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c2-current-instruction-identity-boundary.md`
- `docs/reviews/xxxl-phase-41c2-current-instruction-identity-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Expected Validation

- `git diff --check`
- `cargo fmt --check`
- `cargo test current_instruction_identity_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41C2 must be reviewed before Phase 41C3.

Phase 41C3 must not combine prior lookup with proof, quorum, authorization,
replay, CPI, or mint execution.

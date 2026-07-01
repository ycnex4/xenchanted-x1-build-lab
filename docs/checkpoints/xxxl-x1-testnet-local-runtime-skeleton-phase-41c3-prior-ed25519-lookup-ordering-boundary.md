# XXXL X1 Testnet Local Runtime Skeleton Phase 41C3 Prior Ed25519 Lookup Ordering Boundary

Status: Narrow Rust boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41c3-prior-ed25519-lookup-ordering-boundary`

## Purpose

Phase 41C3 introduces prior Ed25519 lookup and strict ordering over descriptors.

It intentionally does not read real Solana `AccountInfo`.

It intentionally does not call `load_instruction`, `load_instruction_at`, or
`load_instruction_at_checked`.

It intentionally does not accept verification evidence, proof, quorum,
authorization, replay, CPI, or mint execution.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/prior_ed25519_lookup_ordering_boundary.rs`
- `docs/xxxl/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/reviews/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary-review-request.md`

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
- `cargo test prior_ed25519_lookup_ordering_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41C3 must be reviewed before any next phase.

Real runtime wiring must remain separate.

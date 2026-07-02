# XXXL X1 Testnet Local Runtime Skeleton Phase 41D3.1 Current Index Runtime Boundary

Status: Real checked current-index runtime boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-1-current-index-runtime-boundary`

## Purpose

Phase 41D3.1 implements only checked current-instruction index acquisition.

This is a small sub-step inside the accepted Phase 41D3 boundary.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/current_instruction_index_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d3-1-current-index-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-1-current-index-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d3-1-current-index-runtime-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No deploy artifact is touched.

## Scope

Allowed in Phase 41D3.1:

- Instructions sysvar AccountInfo key check
- checked current-index acquisition through `load_current_index_checked`
- deterministic rejection on checked read failure
- current index exposed only as ordering data

Forbidden in Phase 41D3.1:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- prior-instruction enumeration
- raw Instructions sysvar data parsing
- Phase 41C3 candidate descriptor construction
- prior Ed25519 lookup
- Ed25519 cryptographic verification
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- account mutation
- CPI
- SPL Token mint execution
- live route execution

## Expected Validation

- forbidden runtime call check
- panic token check
- unchecked index/slice check
- `git diff --check`
- `cargo fmt`
- `cargo fmt --check`
- `cargo test current_instruction_index_runtime_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41D3.1 must be reviewed before opening the next prior-enumeration/loading sub-step.

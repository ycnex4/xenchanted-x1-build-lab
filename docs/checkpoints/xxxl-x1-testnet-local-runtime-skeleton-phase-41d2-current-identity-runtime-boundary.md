# XXXL X1 Testnet Local Runtime Skeleton Phase 41D2 Current Identity Runtime Boundary

Status: Real current-instruction identity runtime boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-current-identity-runtime-boundary`

## Purpose

Phase 41D2 introduces only real current-instruction identity population from entrypoint context.

It maps entrypoint `program_id` and `instruction_data` into existing Phase 41C2 descriptor states.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/current_instruction_identity_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d2-current-identity-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d2-current-identity-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d2-current-identity-runtime-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Scope

Allowed in Phase 41D2:

- direct entrypoint `program_id` comparison
- direct entrypoint `instruction_data` discriminator-prefix comparison
- payload/context binding result consumption
- Phase 41C2 descriptor construction
- Phase 41C2 state mapping
- `current_instruction_identity_derived_from_runtime: true`

Forbidden in Phase 41D2:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- raw Instructions sysvar parsing
- prior instruction enumeration
- Phase 41C3 candidate descriptor construction
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

- forbidden runtime call check for `load_instruction*`, CPI, mint, handler
- panic token check
- unchecked slicing/indexing check
- `git diff --check`
- `cargo fmt --check`
- `cargo test current_instruction_identity_runtime_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41D2 must be reviewed before Phase 41D3.

Phase 41D3 must not start before this runtime identity boundary is accepted.

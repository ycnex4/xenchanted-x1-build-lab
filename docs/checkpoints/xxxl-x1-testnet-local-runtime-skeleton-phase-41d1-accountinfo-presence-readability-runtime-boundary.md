# XXXL X1 Testnet Local Runtime Skeleton Phase 41D1 AccountInfo Presence Readability Runtime Boundary

Status: First real runtime-read boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41d1-accountinfo-presence-readability-runtime-boundary`

## Purpose

Phase 41D1 introduces only real Instructions sysvar `AccountInfo`
presence/readability.

It maps that runtime access into existing Phase 41C1 descriptor states.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_accountinfo_presence_readability_runtime_boundary.rs`
- `docs/xxxl/xxxl-phase-41d1-accountinfo-presence-readability-runtime-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d1-accountinfo-presence-readability-runtime-boundary.md`
- `docs/reviews/xxxl-phase-41d1-accountinfo-presence-readability-runtime-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No runtime handler is added.

No deploy artifact is touched.

## Scope

Allowed in Phase 41D1:

- real `AccountInfo` presence check
- real Instructions sysvar key check
- real borrow/readability check
- deterministic mapping into Phase 41C1 states

Forbidden in Phase 41D1:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- concrete instruction content parsing
- current instruction identity derivation
- prior instruction enumeration
- Phase 41C3 descriptor construction
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
- `git diff --check`
- `cargo fmt --check`
- `cargo test instructions_sysvar_accountinfo_presence_readability_runtime_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41D1 must be reviewed before Phase 41D2.

Phase 41D2 must not start before this runtime-read boundary is accepted.

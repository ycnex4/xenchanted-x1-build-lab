# XXXL X1 Testnet Local Runtime Skeleton Phase 41C1 Instructions Sysvar Presence Readability Boundary

Status: Narrow Rust boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41c1-instructions-sysvar-presence-readability-boundary`

## Purpose

Phase 41C1 introduces the narrow Instructions sysvar presence/readability
boundary approved in Phase 41C0A.

It adds:

- one Rust boundary module
- deterministic structural results
- mapping to the Phase 41B rejection taxonomy
- safety flags proving only concrete runtime API selection is enabled

It does not call `load_instruction`.

It does not read concrete instruction contents.

It does not derive current instruction identity.

It does not locate prior Ed25519 instructions.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_presence_readability_boundary.rs`
- `docs/xxxl/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/reviews/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary-review-request.md`

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
- `cargo test instructions_sysvar_presence_readability_boundary --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Review Gate

Phase 41C1 must be reviewed before Phase 41C2.

Phase 41C2 must not combine current instruction identity with prior Ed25519
lookup, proof, quorum, authorization, replay, CPI, or mint execution.

# XXXL X1 Testnet Local Runtime Skeleton Phase 40E Ed25519 Prior Instruction Ordering Model

Status: Tiny read-only Rust/SVM ordering model.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40e-ed25519-prior-instruction-ordering-model`

## Purpose

Phase 40E models the future requirement that an Ed25519 verification instruction
must appear before the XXXL instruction that consumes it.

It uses prepared Phase 39 scanned evidence and a modeled current instruction
index.

It does not parse raw Instructions sysvar data.

It does not call `load_instruction`.

It does not parse `AccountInfo`.

It does not verify Ed25519 signatures.

It does not accept cryptographic proof.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/ed25519_prior_instruction_ordering.rs`
- `docs/xxxl/xxxl-phase-40e-ed25519-prior-instruction-ordering-model.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40e-ed25519-prior-instruction-ordering-model.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo manifest is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Boundary Preserved

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Active Blockers Preserved

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Validation

Suggested validation:

- `git diff --check`
- `cargo fmt --check`
- `cargo test ed25519_prior_instruction_ordering --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Recommended Next Stage

Phase 40F can add a small requirement-to-error coverage matrix for future
verification evidence integration. Real raw Instructions sysvar integration
should remain a dedicated reviewed phase.

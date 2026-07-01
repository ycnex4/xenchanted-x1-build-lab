# XXXL X1 Testnet Local Runtime Skeleton Phase 40D Ed25519 Verification Evidence Integration Design Surface

Status: Tiny read-only Rust/SVM design surface.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40d-ed25519-verification-evidence-integration-design-surface`

## Purpose

Phase 40D adds a typed Rust design surface for future SVM Ed25519 verification
evidence integration.

It records future requirements and future rejection cases in Rust while keeping
all implementation and runtime surfaces disabled.

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

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_integration_design.rs`
- `docs/xxxl/xxxl-phase-40d-ed25519-verification-evidence-integration-design-surface.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40d-ed25519-verification-evidence-integration-design-surface.md`

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
- `cargo test ed25519_verification_evidence_integration_design --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Recommended Next Stage

Phase 40E should remain narrow. Real raw Instructions sysvar integration should
wait for a dedicated reviewed phase.

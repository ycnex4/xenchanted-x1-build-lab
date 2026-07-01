# XXXL X1 Testnet Local Runtime Skeleton Phase 40F Ed25519 Verification Evidence Coverage Matrix

Status: Tiny read-only Rust/SVM coverage matrix.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40f-ed25519-verification-evidence-coverage-matrix`

## Purpose

Phase 40F maps future Ed25519 verification evidence requirements from Phase 40D
to primary future rejection cases.

It records coverage in Rust while keeping every runtime/proof/quorum/auth and
execution surface disabled.

It does not parse raw Instructions sysvar data.

It does not parse `AccountInfo`.

It does not verify Ed25519 signatures.

It does not accept cryptographic proof.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_coverage_matrix.rs`
- `docs/xxxl/xxxl-phase-40f-ed25519-verification-evidence-coverage-matrix.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40f-ed25519-verification-evidence-coverage-matrix.md`

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
  != requirement coverage
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
- `cargo test ed25519_verification_evidence_coverage_matrix --lib`
- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Recommended Next Stage

Phase 40G may add a docs-only closure checkpoint for the Phase 40 series before
attempting real raw Instructions sysvar integration in a reviewed phase.

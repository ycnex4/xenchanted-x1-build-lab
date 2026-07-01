# XXXL X1 Testnet Local Runtime Skeleton Phase 40B Ed25519 Verification Evidence Model

Status: Tiny read-only Rust/SVM boundary model.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-40b-ed25519-verification-evidence-model`

Base context:

- Phase 37 added the Ed25519 instruction evidence layout model.
- Phase 38 added the Ed25519 instruction data parser.
- Phase 39 added the prepared-entry Instructions sysvar evidence scanner.
- Phase 40A documented the Ed25519 verification evidence boundary.

## Purpose

Phase 40B adds a small Rust model for the boundary between located/parsed
candidate evidence and future Ed25519 verification evidence.

It accepts Phase 39 scanned evidence and returns only a non-authorizing boundary
result.

It does not verify Ed25519 signatures.

It does not accept cryptographic signature proof.

It does not count quorum.

It does not authorize minting.

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence.rs`
- `docs/xxxl/xxxl-phase-40b-ed25519-verification-evidence-model.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-40b-ed25519-verification-evidence-model.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Model Boundary

The core rule remains:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

The model rejects candidate evidence if it claims signature verification,
cryptographic proof, quorum, authorization, or execution.

The model returns a non-authorizing boundary result only.

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
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo fmt --check"`
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test ed25519_verification_evidence --lib"`
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test --lib --locked"`
- `npm run typecheck`
- `npm run build`

No SBF build should be run.

## Recommended Next Stage

Phase 40C should define a read-only Ed25519 verification evidence integration
design for actual SVM Ed25519 verification evidence, still without quorum
authorization, handler or account parsing, CPI, mint execution, replay writes,
or runtime unlock.

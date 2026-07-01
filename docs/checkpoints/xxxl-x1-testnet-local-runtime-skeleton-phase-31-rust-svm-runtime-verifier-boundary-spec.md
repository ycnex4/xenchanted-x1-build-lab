# XXXL X1 Testnet Local Runtime Skeleton Phase 31 Rust/SVM Runtime Verifier Boundary Spec

Status: Docs-only runtime verifier boundary specification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-31-rust-svm-runtime-verifier-boundary-spec`

Base context:

- Phase 30 closed as execution-backed TypeScript parity validation

## Purpose

Phase 31 defines the reviewed boundary for a future Rust/SVM runtime verifier.

Phase 31 does not implement Rust/SVM code.

Phase 31 does not unlock runtime execution.

Phase 31 records which verifier components must exist before any future runtime
execution path can be considered.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-31-rust-svm-runtime-verifier-boundary-spec.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-31-rust-svm-runtime-verifier-boundary-spec.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## Boundary Components Defined

Phase 31 defines the future runtime verifier boundary for:

- raw payload decoder
- canonical payload validation
- source proof identity verification
- guardian approval and quorum verification
- route binding verification
- target mint legitimacy verification
- replay verification
- expiration verification
- amount control verification
- deterministic error surface

## Future Runtime Cases Covered By Spec

Phase 31 specifically carries forward the 7 Phase 30 future-runtime cases:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

These cases remain future runtime verifier obligations.

Phase 31 does not satisfy them by implementation.

Phase 31 only defines the implementation boundary.

## Explicit Non-Goals

Phase 31 does not implement Rust/SVM verifier code.

Phase 31 does not modify `programs/xxxl-svm`.

Phase 31 does not modify Cargo files.

Phase 31 does not build SBF artifacts.

Phase 31 does not touch `target/deploy`.

Phase 31 does not read or modify keypair files.

Phase 31 does not read or modify `.env`.

Phase 31 does not inspect `.local-keys`.

Phase 31 does not run deploy commands.

Phase 31 does not run network commands.

Phase 31 does not spend SOL.

Phase 31 does not enable live route execution.

Phase 31 does not enable SPL CPI.

Phase 31 does not enable `invoke_signed`.

Phase 31 does not enable SPL Token `mint_to`.

Phase 31 does not mutate runtime/account state.

Phase 31 does not enable processed-event marking.

Phase 31 does not select a production Program ID.

Phase 31 does not regenerate production PDA fixtures.

Phase 31 does not remove deployment blockers.

Phase 31 does not claim production readiness.

Phase 31 does not claim final immutability while upgrade authority exists.

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

Required commands:

- `git diff --check`
- `npm run typecheck`
- `npm run build`
- `git status --short --untracked-files=all`

No Cargo command should be run.

No SBF build should be run.

No deploy or network command should be run.

## Recommended Next Stage

Choose a separately reviewed Phase 32 boundary before any Rust/SVM verifier code.

The recommended first implementation boundary is read-only Rust/SVM verifier
scaffolding without:

- SPL CPI
- mint execution
- replay writes
- processed-event marking
- live route unlock
- production deployment

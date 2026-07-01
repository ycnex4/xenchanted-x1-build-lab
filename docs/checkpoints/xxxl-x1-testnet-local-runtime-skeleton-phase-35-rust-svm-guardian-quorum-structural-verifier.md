# XXXL X1 Testnet Local Runtime Skeleton Phase 35 Rust/SVM Guardian Quorum Structural Verifier

Status: Narrow Rust/SVM guardian approval and quorum structural verifier.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-35-rust-svm-guardian-quorum-structural-verifier`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the Rust/SVM raw payload decoder.
- Phase 34 added Rust/SVM canonical payload hash/domain validation.

## Purpose

Phase 35 implements only guardian approval membership and quorum structural verification.

The verifier checks structural quorum conditions over a provided guardian set and approval claims.

Phase 35 is not Ed25519 cryptographic signature verification.

Phase 35 is not a full runtime verifier.

Phase 35 does not unlock runtime execution.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/guardian_quorum.rs`
- `docs/xxxl/xxxl-phase-35-rust-svm-guardian-quorum-structural-verifier.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-35-rust-svm-guardian-quorum-structural-verifier.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Structural Quorum Boundary

Phase 35 verifies only structural guardian approval conditions.

Expected structural checks:

- guardian set is non-empty
- threshold is non-zero
- threshold does not exceed guardian set size
- approval list is non-empty
- approval guardian set id matches the guardian set id
- approval guardian public key is known in the guardian set
- duplicate guardian approvals are rejected
- unique known approvals meet threshold

The accepted condition is:

~~~text
unique known approvals >= threshold
~~~

This is only structural quorum.

It is not cryptographic signature verification.

## Ed25519 Boundary

Phase 35 does not implement Ed25519 verification.

Phase 35 does not validate Ed25519 instruction evidence.

Phase 35 does not parse instruction sysvar data.

Phase 35 does not validate ed25519 program instructions.

Phase 35 does not prove that a guardian actually signed the Phase 34 canonical payload hash.

A future separately reviewed phase must prove cryptographic signatures over the Phase 34 canonical payload hash.

A structurally valid quorum is not enough to authorize minting.

Phase 35 alone cannot make `authorized=true`.

## Honest Remaining Obligations

The following obligations remain unsatisfied:

- Ed25519 cryptographic signature verification
- guardian signature evidence binding to Phase 34 canonical payload hash
- source proof verification
- route config verification
- target mint account legitimacy verification
- amount cap enforcement
- replay storage
- replay checks
- replay writes
- account parsing
- instruction processing
- mint execution
- runtime unlock

The following Phase 30 future-runtime cases remain unsatisfied:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`
- same-shape `wrong-field-order` variants not structurally rejected by Phase 33

## Explicit Non-Goals

Phase 35 does not implement Ed25519 verification.

Phase 35 does not implement instruction sysvar parsing.

Phase 35 does not implement ed25519 program instruction validation.

Phase 35 does not implement source proof verification.

Phase 35 does not implement route config verification.

Phase 35 does not implement target mint account legitimacy verification.

Phase 35 does not implement amount cap enforcement.

Phase 35 does not implement replay storage.

Phase 35 does not implement replay checks.

Phase 35 does not implement replay writes.

Phase 35 does not parse runtime accounts.

Phase 35 does not add an instruction handler.

Phase 35 does not enable live route execution.

Phase 35 does not enable SPL CPI.

Phase 35 does not enable `invoke_signed`.

Phase 35 does not enable SPL Token `mint_to`.

Phase 35 does not add mint execution.

Phase 35 does not mutate runtime/account state.

Phase 35 does not enable processed-event marking.

Phase 35 does not select a production Program ID.

Phase 35 does not regenerate production PDA fixtures.

Phase 35 does not remove deployment blockers.

Phase 35 does not claim production readiness.

Phase 35 does not claim final immutability while upgrade authority exists.

Phase 35 does not build SBF artifacts.

Phase 35 does not touch `target/deploy`.

Phase 35 does not read or modify keypair files.

Phase 35 does not read or modify `.env`.

Phase 35 does not inspect `.local-keys`.

Phase 35 does not run deploy commands.

Phase 35 does not run network commands.

Phase 35 does not spend SOL.

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

Commands to run:

- `git diff --check`
- `cargo fmt --check`
- `cargo test guardian_quorum --lib`
- `cargo test canonical_payload --lib`
- `cargo test raw_payload --lib`
- `cargo test verifier --lib`
- `npm test -- --run tests/xxxl/ts-svm-parity-execution-backed-validation.test.ts tests/xxxl/ts-svm-parity-verifier-validation.test.ts tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`
- `npm run typecheck`
- `npm run build`
- `git status --short --untracked-files=all`

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

No deploy or network command was run.

Next possible phase requires separate review before Ed25519 cryptographic verification, instruction sysvar parsing, ed25519 program instruction validation, source proof verification, route config verification, target mint account checks, amount cap enforcement, replay checks/writes, account parsing, instruction processing, mint execution, or runtime unlock.

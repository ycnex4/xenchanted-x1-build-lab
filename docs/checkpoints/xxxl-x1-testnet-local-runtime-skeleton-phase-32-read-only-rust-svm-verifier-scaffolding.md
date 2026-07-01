# XXXL X1 Testnet Local Runtime Skeleton Phase 32 Read-Only Rust/SVM Verifier Scaffolding

Status: Read-only Rust/SVM verifier scaffold.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-32-read-only-rust-svm-verifier-scaffolding`

Base context:

- Phase 31 closed and merged as `Merge XXXL phase 31 Rust SVM runtime verifier boundary spec`

## Purpose

Phase 32 adds a read-only Rust/SVM verifier scaffold that mirrors the Phase 31
boundary components and carries forward Phase 30 future-runtime cases.

The scaffold is testable Rust code.

The scaffold is not a runtime verifier implementation.

The scaffold does not unlock runtime execution.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `programs/xxxl-svm/src/verifier/boundary.rs`
- `programs/xxxl-svm/src/verifier/errors.rs`
- `programs/xxxl-svm/src/verifier/types.rs`
- `docs/xxxl/xxxl-phase-32-read-only-rust-svm-verifier-scaffolding.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-32-read-only-rust-svm-verifier-scaffolding.md`

Changed:

- `programs/xxxl-svm/src/lib.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

## Rust/SVM Scaffold Boundary

Scaffold marker:

- `READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32`

Scaffold version:

- `1`

Boundary status:

- `READ_ONLY_SCAFFOLD_ONLY`

The scaffold lists exactly 10 Phase 31 verifier components:

- raw payload decoder
- canonical payload validation
- source proof identity verifier
- guardian approval and quorum verifier
- route binding verifier
- target mint legitimacy verifier
- replay verifier
- expiration verifier
- amount control verifier
- deterministic error surface

The scaffold lists exactly 7 Phase 30 future-runtime cases:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

Every future-runtime case is marked not implemented.

## Explicit Safety Flags

The scaffold records these flags as disabled:

- `live_route_enabled = false`
- `spl_cpi_enabled = false`
- `invoke_signed_enabled = false`
- `mint_execution_enabled = false`
- `runtime_state_mutation_enabled = false`
- `replay_write_enabled = false`
- `processed_event_marking_enabled = false`
- `production_program_id_selected = false`
- `deployment_blockers_removed = false`

Unit tests verify these flags remain false.

## Explicit Non-Goals

Phase 32 does not implement raw payload decoding.

Phase 32 does not implement Ed25519 verification.

Phase 32 does not implement source proof verification.

Phase 32 does not implement amount cap enforcement.

Phase 32 does not implement target mint account legitimacy checks.

Phase 32 does not parse runtime accounts.

Phase 32 does not add an instruction handler.

Phase 32 does not enable live route execution.

Phase 32 does not enable SPL CPI.

Phase 32 does not enable `invoke_signed`.

Phase 32 does not enable SPL Token `mint_to`.

Phase 32 does not add mint execution.

Phase 32 does not mutate runtime/account state.

Phase 32 does not add replay writes.

Phase 32 does not enable processed-event marking.

Phase 32 does not select a production Program ID.

Phase 32 does not regenerate production PDA fixtures.

Phase 32 does not remove deployment blockers.

Phase 32 does not claim production readiness.

Phase 32 does not claim final immutability while upgrade authority exists.

Phase 32 does not build SBF artifacts.

Phase 32 does not touch `target/deploy`.

Phase 32 does not read or modify keypair files.

Phase 32 does not read or modify `.env`.

Phase 32 does not inspect `.local-keys`.

Phase 32 does not run deploy commands.

Phase 32 does not run network commands.

Phase 32 does not spend SOL.

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

Commands run:

- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo fmt --check"`: passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`: passed, 6 tests passed
- `git diff --check`: passed
- `npm run typecheck`: passed
- `npm run build`: passed
- `npm test -- --run tests/xxxl/ts-svm-parity-execution-backed-validation.test.ts tests/xxxl/ts-svm-parity-verifier-validation.test.ts tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed, 4 test files passed, 40 tests passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

No deploy or network command was run.

Next possible phase requires separate review before any runtime verifier
implementation, account parsing, source proof verification, amount cap
enforcement, target mint account checks, replay writes, mint execution, or
runtime unlock.

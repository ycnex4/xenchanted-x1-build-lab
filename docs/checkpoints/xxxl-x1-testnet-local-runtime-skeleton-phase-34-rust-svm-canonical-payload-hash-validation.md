# XXXL X1 Testnet Local Runtime Skeleton Phase 34 Rust/SVM Canonical Payload Hash Validation

Status: Narrow Rust/SVM canonical payload hash/domain validation.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-34-rust-svm-canonical-payload-hash-validation`

Base context:

- Phase 31 defined the docs-only runtime verifier boundary.
- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the Rust/SVM raw payload decoder.

## Purpose

Phase 34 implements only canonical payload hash/domain validation on top of the
Phase 33 raw payload decoder.

It follows the Phase 23 TypeScript canonical payload hash model.

It is not a full runtime verifier.

It does not unlock runtime execution.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/canonical_payload.rs`
- `docs/xxxl/xxxl-phase-34-rust-svm-canonical-payload-hash-validation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-34-rust-svm-canonical-payload-hash-validation.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change was required.

## Hash Boundary

Validator marker:

- `CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34`

Validator version:

- `1`

Canonical hash domain label:

- `XXXL_GUARDIAN_PAYLOAD_HASH_V1`

Phase 23 domain separator:

- `0xf1958bbf04d45ddbc5a9f93f200f5005ee47b05cf61a90faf4d93cd6e3eccd66`

Phase 23 valid payload hash:

- `0xab0ee59a1268f3eebf4a9d42725640ce68226e642a61dabd5f904e7680f08015`

The validator computes:

~~~text
keccak256(keccak256("XXXL_GUARDIAN_PAYLOAD_HASH_V1") || payload_bytes)
~~~

The implementation uses `solana_program::keccak`, already available through
the existing `solana-program` dependency.

## Rejections Implemented

Phase 34 distinguishes:

- raw payload decode failure from the Phase 33 decoder
- canonical payload hash mismatch after recomputation

The validator calls `decode_guardian_payload_raw(input)` before computing the
hash.

The validator recomputes the hash from payload bytes.

The validator does not trust caller-provided payload hashes.

## Honest Remaining Obligations

Phase 34 does not mark route, source proof, replay, amount, target mint, or
guardian quorum cases implemented.

The following obligations remain:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`
- same-shape `wrong-field-order` variants not structurally rejected by Phase 33

A payload can hash correctly while still lying about source proof fields.

Canonical payload hash validation is not source proof verification.

## Explicit Non-Goals

Phase 34 does not implement Ed25519 verification.

Phase 34 does not implement guardian quorum.

Phase 34 does not implement source proof verification.

Phase 34 does not implement route config verification.

Phase 34 does not implement target mint account legitimacy verification.

Phase 34 does not implement amount cap enforcement.

Phase 34 does not implement replay storage.

Phase 34 does not implement replay checks.

Phase 34 does not implement replay writes.

Phase 34 does not parse runtime accounts.

Phase 34 does not add an instruction handler.

Phase 34 does not enable live route execution.

Phase 34 does not enable SPL CPI.

Phase 34 does not enable `invoke_signed`.

Phase 34 does not enable SPL Token `mint_to`.

Phase 34 does not add mint execution.

Phase 34 does not mutate runtime/account state.

Phase 34 does not enable processed-event marking.

Phase 34 does not select a production Program ID.

Phase 34 does not regenerate production PDA fixtures.

Phase 34 does not remove deployment blockers.

Phase 34 does not claim production readiness.

Phase 34 does not claim final immutability while upgrade authority exists.

Phase 34 does not build SBF artifacts.

Phase 34 does not touch `target/deploy`.

Phase 34 does not read or modify keypair files.

Phase 34 does not read or modify `.env`.

Phase 34 does not inspect `.local-keys`.

Phase 34 does not run deploy commands.

Phase 34 does not run network commands.

Phase 34 does not spend SOL.

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

- `git diff --check`: passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo fmt --check"`: passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test canonical_payload --lib"`: passed, 11 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test raw_payload --lib"`: passed, 11 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`: passed, 26 tests passed
- `npm test -- --run tests/xxxl/ts-svm-parity-execution-backed-validation.test.ts tests/xxxl/ts-svm-parity-verifier-validation.test.ts tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed, 4 test files passed, 40 tests passed
- `npm run typecheck`: passed
- `npm run build`: passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

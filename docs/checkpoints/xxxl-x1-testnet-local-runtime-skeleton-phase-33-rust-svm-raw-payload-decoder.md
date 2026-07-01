# XXXL X1 Testnet Local Runtime Skeleton Phase 33 Rust/SVM Raw Payload Decoder

Status: Narrow Rust/SVM raw payload decoder implementation.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-33-rust-svm-raw-payload-decoder`

Base context:

- Phase 32 added read-only Rust/SVM verifier scaffolding.

## Purpose

Phase 33 implements only the raw payload decoder boundary component from the
Phase 32 scaffold.

The decoder follows the Phase 23 TypeScript canonical binary payload layout.

The decoder is not a full runtime verifier.

The decoder does not unlock runtime execution.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `programs/xxxl-svm/src/verifier/raw_payload.rs`
- `docs/xxxl/xxxl-phase-33-rust-svm-raw-payload-decoder.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-33-rust-svm-raw-payload-decoder.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change was required.

## Decoder Boundary

Decoder marker:

- `RAW_PAYLOAD_DECODER_PHASE_33`

Decoder version:

- `1`

The decoder parses:

- u16 little-endian length-prefixed byte fields
- u16 little-endian schema and layout values
- u64 little-endian integer fields
- u128 little-endian amount fields
- 32-byte fixed-width fields

Decoded fields follow the Phase 23 canonical order.

The decoded structure borrows slices from the input payload.

## Rejections Implemented

The decoder rejects:

- truncated payloads
- trailing bytes
- empty variable-length fields
- malformed length-prefixed encoding that overruns the payload
- unsupported message type
- unsupported schema version
- unsupported instruction layout version

The Phase 28 `wrong-byte-encoding` fixture is rejected at decoder level by the
corrupted message-type length byte.

Phase 33 also tests a structurally detectable `wrong-field-order` case where
`source_token` is placed before `source_chain_id`.

That case is rejected by the decoder because the reordered bytes violate a
later variable-length field boundary.

## Honest Remaining Obligations

Phase 33 does not claim all possible `wrong-field-order` cases are satisfied.

Field-order swaps that remain structurally valid raw bytes are still future
canonical validation, hash validation, or semantic verifier obligations.

The following Phase 30 future-runtime cases remain unsatisfied:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

No future-runtime case outside raw decoder scope is marked implemented.

## Explicit Non-Goals

Phase 33 does not implement Ed25519 verification.

Phase 33 does not implement guardian quorum.

Phase 33 does not implement source proof verification.

Phase 33 does not implement route config verification.

Phase 33 does not implement target mint account legitimacy verification.

Phase 33 does not implement amount cap enforcement.

Phase 33 does not implement replay storage.

Phase 33 does not implement replay checks.

Phase 33 does not implement replay writes.

Phase 33 does not parse runtime accounts.

Phase 33 does not add instruction processing.

Phase 33 does not enable live route execution.

Phase 33 does not enable SPL CPI.

Phase 33 does not enable `invoke_signed`.

Phase 33 does not enable SPL Token `mint_to`.

Phase 33 does not add mint execution.

Phase 33 does not mutate runtime/account state.

Phase 33 does not enable processed-event marking.

Phase 33 does not select a production Program ID.

Phase 33 does not regenerate production PDA fixtures.

Phase 33 does not remove deployment blockers.

Phase 33 does not claim production readiness.

Phase 33 does not claim final immutability while upgrade authority exists.

Phase 33 does not build SBF artifacts.

Phase 33 does not touch `target/deploy`.

Phase 33 does not read or modify keypair files.

Phase 33 does not read or modify `.env`.

Phase 33 does not inspect `.local-keys`.

Phase 33 does not run deploy commands.

Phase 33 does not run network commands.

Phase 33 does not spend SOL.

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
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test raw_payload --lib"`: passed, 10 tests passed
- `wsl bash -lc "cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm && cargo test verifier --lib"`: passed, 16 tests passed
- `npm test -- --run tests/xxxl/ts-svm-parity-execution-backed-validation.test.ts tests/xxxl/ts-svm-parity-verifier-validation.test.ts tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed, 4 test files passed, 40 tests passed
- `npm run typecheck`: passed
- `npm run build`: passed
- `git diff --check`: passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

No deploy or network command was run.

Next possible phase requires separate review before canonical payload hash
validation, Ed25519 verification, source proof verification, route config
verification, target mint account checks, amount cap enforcement, replay
checks/writes, account parsing, instruction processing, mint execution, or
runtime unlock.

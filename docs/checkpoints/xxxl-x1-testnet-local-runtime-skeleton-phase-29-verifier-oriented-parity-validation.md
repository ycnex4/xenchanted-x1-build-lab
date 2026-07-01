# XXXL X1 Testnet Local Runtime Skeleton Phase 29 Verifier-Oriented Parity Validation

Status: TypeScript-only verifier-oriented parity validation boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-29-verifier-oriented-parity-validation`

Base context:

- Phase 28 closed as concrete invalid parity fixture materialization

## Purpose

Phase 29 classifies every Phase 28 invalid fixture into an explicit verifier
validation expectation.

The phase separates:

- invalid fixtures already explainable by existing TypeScript model boundaries
- invalid fixtures that remain future Rust/SVM runtime verifier obligations

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `src/xxxl/ts-svm-parity-verifier-validation.ts`
- `tests/xxxl/ts-svm-parity-verifier-validation.test.ts`
- `docs/xxxl/xxxl-phase-29-verifier-oriented-parity-validation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-29-verifier-oriented-parity-validation.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## Validation Suite Boundary

Suite id:

- `XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_PHASE_29`

Suite version:

- `1`

Boundary marker:

- `VERIFIER_ORIENTED_VALIDATION_ONLY_NO_RUNTIME_EXECUTION`

The suite depends on:

- Phase 23 canonical guardian payload vector
- Phase 27 parity vector suite
- Phase 28 concrete invalid fixtures
- existing TypeScript model boundaries where safe

The suite does not redefine canonical payload encoding.

The suite does not introduce a new hash algorithm.

The suite does not implement Rust/SVM verification.

## Case Classification

`TS_MODEL_VALIDATED_REJECTION`:

- `wrong-hash-domain`
- `malformed-bytes32-field`
- `malformed-var-bytes-field`
- `invalid-source-chain-id`
- `wrong-target-mint`
- `wrong-guardian-set-id`
- `wrong-source-chain-weight`
- `invalid-signature`
- `duplicate-guardian-approval`
- `insufficient-quorum`
- `expired-payload`
- `duplicate-canonical-event-key`

`FUTURE_RUNTIME_VALIDATION_REQUIRED`:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

All invalid entries have expected decision:

~~~text
REJECT_BEFORE_EXECUTION
~~~

The valid canonical payload is not included in the invalid validation matrix.

## Explicit Non-Goals

Phase 29 does not enable live route execution.

Phase 29 does not enable SPL CPI.

Phase 29 does not enable `invoke_signed`.

Phase 29 does not enable SPL Token `mint_to`.

Phase 29 does not mutate runtime/account state.

Phase 29 does not enable processed-event marking.

Phase 29 does not select a production Program ID.

Phase 29 does not regenerate production PDA fixtures.

Phase 29 does not remove deployment blockers.

Phase 29 does not claim production readiness.

Phase 29 does not claim final immutability while upgrade authority exists.

Phase 29 does not modify `programs/xxxl-svm`.

Phase 29 does not modify Cargo files.

Phase 29 does not build SBF artifacts.

Phase 29 does not touch `target/deploy`.

Phase 29 does not read or modify keypair files.

Phase 29 does not read or modify `.env`.

Phase 29 does not inspect `.local-keys`.

Phase 29 does not run deploy commands.

Phase 29 does not run network commands.

Phase 29 does not spend SOL.

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

- `npm test -- --run tests/xxxl/ts-svm-parity-verifier-validation.test.ts tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed, 3 test files passed, 29 tests passed
- `npm run typecheck`: passed
- `npm run build`: passed
- `git diff --check`: passed
- `git status --short --untracked-files=all`: run for final workspace state

No Cargo command was run.

No SBF build was run.

No deploy or network command was run.

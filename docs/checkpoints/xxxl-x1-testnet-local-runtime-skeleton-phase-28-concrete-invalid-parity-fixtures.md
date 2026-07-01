# XXXL X1 Testnet Local Runtime Skeleton Phase 28 Concrete Invalid Parity Fixtures

Status: TypeScript-only concrete invalid parity fixture materialization.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-28-concrete-invalid-parity-fixtures`

Base context:

- Phase 27 closed and merged as `Merge XXXL phase 27 TS SVM parity vector suite`

## Purpose

Phase 28 materializes the 19 invalid Phase 27 parity cases as deterministic
concrete tampered fixtures.

Phase 27 created the parity contract/checklist.

Phase 28 adds concrete corrupted inputs so a future Rust/SVM verifier can
compare against exact invalid material.

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `src/xxxl/ts-svm-parity-invalid-fixtures.ts`
- `tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts`
- `docs/xxxl/xxxl-phase-28-concrete-invalid-parity-fixtures.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-28-concrete-invalid-parity-fixtures.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## Fixture Suite Boundary

Suite id:

- `XXXL_TS_SVM_PARITY_INVALID_FIXTURES_PHASE_28`

Suite version:

- `1`

Boundary marker:

- `CONCRETE_INVALID_FIXTURES_ONLY_NO_RUNTIME_EXECUTION`

The valid control fixture reuses:

- `XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR`

The suite depends on Phase 27 required case ids.

The suite does not redefine canonical payload encoding.

The suite does not introduce a new hash algorithm.

## Materialized Invalid Cases

Phase 28 materializes these 19 invalid cases:

- `wrong-field-order`
- `wrong-byte-encoding`
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
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

All invalid fixtures have expected decision:

~~~text
REJECT_BEFORE_EXECUTION
~~~

Every fixture includes concrete `tamperedInput` material.

## Explicit Non-Goals

Phase 28 does not enable live route execution.

Phase 28 does not enable SPL CPI.

Phase 28 does not enable `invoke_signed`.

Phase 28 does not enable SPL Token `mint_to`.

Phase 28 does not mutate runtime/account state.

Phase 28 does not enable processed-event marking.

Phase 28 does not select a production Program ID.

Phase 28 does not regenerate production PDA fixtures.

Phase 28 does not remove deployment blockers.

Phase 28 does not claim production readiness.

Phase 28 does not claim final immutability while upgrade authority exists.

Phase 28 does not modify `programs/xxxl-svm`.

Phase 28 does not modify Cargo files.

Phase 28 does not build SBF artifacts.

Phase 28 does not touch `target/deploy`.

Phase 28 does not read or modify keypair files.

Phase 28 does not read or modify `.env`.

Phase 28 does not inspect `.local-keys`.

Phase 28 does not run deploy commands.

Phase 28 does not run network commands.

Phase 28 does not spend SOL.

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

- `npm test -- --run tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed, 2 test files passed, 19 tests passed
- `npm run typecheck`: passed
- `npm run build`: passed

Required final workspace checks:

- `git diff --check`
- `git status --short --untracked-files=all`

No Cargo command was run.

No SBF build was run.

No deploy or network command was run.

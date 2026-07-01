# XXXL X1 Testnet Local Runtime Skeleton Phase 27 TS/SVM Parity Vector Suite

Status: TypeScript-only parity vector suite.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-27-ts-svm-parity-vector-suite`

Base context:

- Phase 26 reviewed closure: `Authorization-Runtime Handoff Spec Reviewed`

## Purpose

Phase 27 creates a bounded TS/SVM parity vector suite for future independent
Rust/SVM runtime verifier work.

The suite prevents drift from:

- Phase 22 guardian payload field order
- Phase 23 canonical payload byte encoding and hash vector
- Phase 24 guardian signature/quorum verifier model
- Phase 25 verifier-to-runtime authorization boundary model
- Phase 26 authorization-runtime handoff review closure

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Files Added Or Changed

Added:

- `src/xxxl/ts-svm-parity-vector-suite.ts`
- `tests/xxxl/ts-svm-parity-vector-suite.test.ts`
- `docs/xxxl/xxxl-phase-27-ts-svm-parity-vector-suite.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-27-ts-svm-parity-vector-suite.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## Suite Boundary

Suite id:

- `XXXL_TS_SVM_PARITY_VECTOR_SUITE_PHASE_27`

Suite version:

- `1`

Boundary marker:

- `PARITY_ONLY_PRE_RUNTIME_VECTOR_SUITE`

The valid canonical parity vector reuses:

- `XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR`

The suite does not duplicate canonical payload encoding logic.

The suite does not define a new payload hash algorithm.

## Required Case Coverage

Required case ids:

- `valid-canonical-payload`
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

Only `valid-canonical-payload` is accepted by the parity matrix.

All invalid cases have expected decision:

~~~text
REJECT_BEFORE_EXECUTION
~~~

Every case has explicit future runtime check text.

## Explicit Non-Goals

Phase 27 does not enable live route execution.

Phase 27 does not enable SPL CPI.

Phase 27 does not enable `invoke_signed`.

Phase 27 does not enable SPL Token `mint_to`.

Phase 27 does not mutate runtime/account state.

Phase 27 does not mark processed events.

Phase 27 does not select a production Program ID.

Phase 27 does not regenerate production PDA fixtures.

Phase 27 does not remove deployment blockers.

Phase 27 does not claim production readiness.

Phase 27 does not claim final immutability while upgrade authority exists.

Phase 27 does not modify `programs/xxxl-svm`.

Phase 27 does not modify Cargo files.

Phase 27 does not build SBF artifacts.

Phase 27 does not touch `target/deploy`.

Phase 27 does not read or modify keypair files.

Phase 27 does not read or modify `.env`.

Phase 27 does not inspect `.local-keys`.

Phase 27 does not run deploy commands.

Phase 27 does not run network commands.

Phase 27 does not spend SOL.

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

- `npm test -- --run tests/xxxl/ts-svm-parity-vector-suite.test.ts`: passed,
  1 test file passed, 9 tests passed
- `npm run typecheck`: passed
- `npm run build`: passed

Required final workspace checks:

- `git diff --check`
- `git status --short --untracked-files=all`

No Cargo command was run.

No SBF build was run.

No deploy or network command was run.

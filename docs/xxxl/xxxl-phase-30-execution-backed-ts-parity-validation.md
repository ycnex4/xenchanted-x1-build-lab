# XXXL Phase 30 Execution-Backed TS Parity Validation

Status: Phase 30 TypeScript-only execution-backed parity validation boundary.

## Purpose

Phase 29 classified the Phase 28 invalid parity fixtures, but it was
reference/classification-based.

Phase 30 follows Phase 29 by executing existing TypeScript verifier/model
functions for every Phase 29 case classified as `TS_MODEL_VALIDATED_REJECTION`.

The goal is to prove that the current TypeScript model can actually reject the
specific Phase 28 tampered inputs where Phase 29 said it could.

This phase does not implement a Rust/SVM verifier.

This phase does not implement runtime execution.

## Reviewed Inputs

Phase 30 depends on:

- Phase 23 canonical guardian payload vector and payload encoding helpers
- Phase 24 guardian approval and quorum verifier
- Phase 25 gateway authorization boundary model
- Phase 27 TS/SVM parity vector suite
- Phase 28 concrete invalid parity fixtures
- Phase 29 verifier-oriented validation matrix

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Phase 30 does not redefine canonical payload encoding.

Phase 30 does not introduce new verifier semantics.

Phase 30 does not replace future Rust/SVM runtime verification.

## Source And Test Boundary

New source:

- `src/xxxl/ts-svm-parity-execution-backed-validation.ts`

New tests:

- `tests/xxxl/ts-svm-parity-execution-backed-validation.test.ts`

Updated export:

- `src/index.ts`

The suite boundary marker is:

~~~text
EXECUTION_BACKED_TS_PARITY_ONLY_NO_RUNTIME_EXECUTION
~~~

The source module builds the suite by executing existing TypeScript model
functions and returning captured expected/actual errors.

## Execution-Backed Cases

All 12 Phase 29 `TS_MODEL_VALIDATED_REJECTION` cases are execution-backed in
Phase 30:

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

The executed TypeScript boundaries are:

- Phase 23 payload vector validation
- Phase 23 payload encoder validation
- Phase 24 guardian quorum verification
- Phase 25 gateway authorization boundary checks

Each execution-backed entry records:

- `caseId`
- `fixtureId`
- `phase27VectorId`
- Phase 29 validation status
- expected rejection/error
- actual returned error
- `matchesExpected: true`
- forbidden-boundary flags set to `false`

No Phase 29 TS-model case is marked `TS_EXECUTION_PATH_UNAVAILABLE`.

## Future Runtime Required Cases

The following 7 Phase 29 cases remain `FUTURE_RUNTIME_VALIDATION_REQUIRED`:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

These cases require future reviewed Rust/SVM runtime verifier work because the
current TypeScript model does not execute raw byte decoding, source proof
preimage verification, amount cap enforcement, or target mint account
legitimacy checks.

Phase 30 does not satisfy those future runtime obligations.

## Explicit Non-Goals

Phase 30 does not enable live route execution.

Phase 30 does not enable SPL CPI.

Phase 30 does not enable `invoke_signed`.

Phase 30 does not enable SPL Token `mint_to`.

Phase 30 does not mutate runtime/account state.

Phase 30 does not mark processed events.

Phase 30 does not select a production Program ID.

Phase 30 does not regenerate production PDA fixtures.

Phase 30 does not remove deployment blockers.

Phase 30 does not claim production readiness.

Phase 30 does not claim final immutability while upgrade authority exists.

Phase 30 does not modify `programs/xxxl-svm`.

Phase 30 does not modify Cargo files.

Phase 30 does not build SBF artifacts.

Phase 30 does not touch `target/deploy`.

Phase 30 does not read or modify keypair files.

Phase 30 does not read or modify `.env`.

Phase 30 does not inspect `.local-keys`.

Phase 30 does not run deploy commands.

Phase 30 does not run network commands.

Phase 30 does not spend SOL.

## No Live Execution Enabled

Phase 30 executes only local TypeScript model functions in tests and suite
builders.

Phase 30 adds no SVM instruction handler.

Phase 30 adds no account mutation.

Phase 30 adds no token minting path.

Phase 30 adds no replay write.

The Phase 26 security decision remains preserved:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 30.

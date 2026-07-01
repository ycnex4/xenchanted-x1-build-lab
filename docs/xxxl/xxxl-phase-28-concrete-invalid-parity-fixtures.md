# XXXL Phase 28 Concrete Invalid Parity Fixtures

Status: Phase 28 TypeScript-only concrete invalid fixture materialization.

## Purpose

Phase 27 created a TS/SVM parity checklist and contract.

Phase 27 fixed:

- one valid canonical payload vector
- 20 required parity case ids
- `valid-canonical-payload` as the only accepted case
- all other cases as `REJECT_BEFORE_EXECUTION`

The Phase 27 audit boundary noted that Phase 27 did not yet materialize the 19
invalid cases as concrete tampered byte/input fixtures.

Phase 28 fills that gap.

Phase 28 creates deterministic concrete invalid fixtures so a future Rust/SVM
verifier can compare against specific corrupted inputs, not only a checklist.

## Reviewed Inputs

Phase 28 uses:

- Phase 23 canonical guardian payload valid vector
- Phase 27 required case ids
- Phase 27 valid/invalid parity matrix
- Phase 26 reviewed authorization-runtime handoff decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Phase 28 does not redefine canonical payload encoding.

Phase 28 does not introduce a new hash algorithm.

Phase 28 uses the Phase 23 valid vector as the baseline.

Phase 28 uses Phase 27 case ids as the fixture contract.

## Source And Test Boundary

New source:

- `src/xxxl/ts-svm-parity-invalid-fixtures.ts`

New tests:

- `tests/xxxl/ts-svm-parity-invalid-fixtures.test.ts`

New export:

- `src/index.ts`

The suite boundary marker is:

~~~text
CONCRETE_INVALID_FIXTURES_ONLY_NO_RUNTIME_EXECUTION
~~~

## Covered Invalid Cases

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

Each fixture includes:

- `caseId`
- `fixtureId`
- `phase27VectorId`
- `expectedDecision`
- `expectedFailureLayer`
- `futureRuntimeCheck`
- `sourceBoundary`
- `baselinePayloadHash`
- `baselineEncodedPayloadHex`
- concrete `tamperedInput`

The valid control fixture reuses:

- `XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR`

## Not Runtime Implementation

Phase 28 is not a runtime implementation.

Phase 28 does not execute verifier logic in Rust/SVM.

Phase 28 does not produce instructions.

Phase 28 does not submit transactions.

Phase 28 does not mutate runtime/account state.

Phase 28 does not mark processed events.

Phase 28 only creates deterministic materialized invalid inputs for future
independent verifier parity work.

## No Live Execution Enabled

Phase 28 does not enable live route execution.

Phase 28 does not enable SPL CPI.

Phase 28 does not enable `invoke_signed`.

Phase 28 does not enable SPL Token `mint_to`.

Phase 28 does not select a production Program ID.

Phase 28 does not regenerate production PDA fixtures.

Phase 28 does not remove deployment blockers.

## Explicit Non-Goals

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

Phase 28 does not claim production readiness.

Phase 28 does not claim final immutability while upgrade authority exists.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 28.

# XXXL Phase 29 Verifier-Oriented Parity Validation Boundary

Status: Phase 29 TypeScript-only verifier-oriented parity validation boundary.

## Purpose

Phase 29 classifies the Phase 28 concrete invalid fixtures from the point of
view of a future independent Rust/SVM verifier.

The goal is to make verifier expectations explicit before any runtime verifier
implementation exists.

Phase 29 answers two bounded questions for each Phase 28 invalid fixture:

- can the existing TypeScript model already validate the rejection?
- or must the future Rust/SVM runtime implement the rejection independently?

This is a validation boundary and matrix only.

## Reviewed Inputs

Phase 29 depends on:

- Phase 22 payload and source burn identity model
- Phase 23 canonical guardian payload vector and hash domain
- Phase 24 guardian approval and quorum verifier model
- Phase 25 authorization boundary model
- Phase 26 authorization-runtime handoff review
- Phase 27 TS/SVM parity vector suite
- Phase 28 concrete invalid parity fixtures

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Phase 29 does not redefine canonical payload encoding.

Phase 29 does not duplicate the Phase 23 payload encoding rules.

Phase 29 does not replace future Rust/SVM runtime verification.

## Source And Test Boundary

New source:

- `src/xxxl/ts-svm-parity-verifier-validation.ts`

New tests:

- `tests/xxxl/ts-svm-parity-verifier-validation.test.ts`

Updated export:

- `src/index.ts`

The suite boundary marker is:

~~~text
VERIFIER_ORIENTED_VALIDATION_ONLY_NO_RUNTIME_EXECUTION
~~~

The source module:

- reuses Phase 28 invalid fixtures
- links every entry back to the Phase 27 vector id
- keeps the Phase 23 canonical vector as a control dependency
- marks every invalid entry as `REJECT_BEFORE_EXECUTION`
- partitions each invalid case into current TS-model validation or future
  runtime validation

## Required Case Coverage

Phase 29 covers all 19 Phase 28 invalid fixture ids:

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

The valid canonical case is not part of the invalid validation matrix.

The valid canonical payload remains a Phase 23/27 control vector only.

## TS-Model Validated Rejections

These cases are classified as `TS_MODEL_VALIDATED_REJECTION` because existing
TypeScript model boundaries can already explain the rejection:

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

This classification is not runtime authorization.

It is only a TypeScript-side parity expectation for a future independent
verifier.

## Future Runtime Validation Required

These cases are classified as `FUTURE_RUNTIME_VALIDATION_REQUIRED` because the
current TypeScript model does not yet model raw bytes, source proof preimages,
amount caps, or target mint account legitimacy as executable verifier checks:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

These cases remain required runtime verifier obligations.

Phase 29 does not satisfy those obligations.

## Explicit Non-Goals

Phase 29 does not enable live route execution.

Phase 29 does not enable SPL CPI.

Phase 29 does not enable `invoke_signed`.

Phase 29 does not enable SPL Token `mint_to`.

Phase 29 does not mutate runtime/account state.

Phase 29 does not mark processed events.

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

## No Live Execution Enabled

Phase 29 adds no route execution path.

Phase 29 adds no SVM instruction handler.

Phase 29 adds no account mutation.

Phase 29 adds no token minting path.

Phase 29 adds no replay write.

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

No blocker is removed, weakened, renamed, or satisfied by Phase 29.

# XXXL Phase 40E Ed25519 Prior Instruction Ordering Model

Status: Tiny read-only Rust/SVM ordering model.

## Purpose

Phase 40E adds a Rust model for one critical future SVM Ed25519 verification
evidence rule:

~~~text
The Ed25519 verification instruction must appear before the XXXL instruction
that consumes it.
~~~

This is still not raw Instructions sysvar integration.

This is still not Ed25519 verification evidence acceptance.

This is still not quorum, authorization, or execution.

## What This Phase Adds

New Rust module:

- `programs/xxxl-svm/src/verifier/ed25519_prior_instruction_ordering.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

The model accepts Phase 39 scanned candidate evidence and a modeled current
instruction index.

It rejects:

- zero instruction count
- current instruction index out of bounds
- matched Ed25519 instruction index out of bounds
- Ed25519 instruction equal to the current instruction
- Ed25519 instruction after the current instruction
- candidate evidence that claims it already read the Instructions sysvar

It accepts only:

- matched Ed25519 instruction index strictly before current instruction index

## Boundary Preserved

The preserved rule remains:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Explicit Non-Goals

Phase 40E does not parse raw Instructions sysvar account data.

Phase 40E does not parse `AccountInfo`.

Phase 40E does not call `load_instruction`.

Phase 40E does not verify Ed25519 signatures.

Phase 40E does not accept cryptographic signature proof.

Phase 40E does not accept verification evidence.

Phase 40E does not count quorum.

Phase 40E does not authorize minting.

Phase 40E does not add a runtime instruction handler.

Phase 40E does not add CPI.

Phase 40E does not enable `invoke_signed`.

Phase 40E does not enable SPL Token `mint_to`.

Phase 40E does not add replay writes.

Phase 40E does not mark processed events.

Phase 40E does not mutate runtime/account state.

Phase 40E does not unlock live route execution.

Phase 40E does not remove deployment blockers.

Phase 40E does not select a production Program ID.

Phase 40E does not claim production readiness.

Phase 40E does not claim final immutability while upgrade authority exists.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40E.

## Recommended Next Stage

Phase 40F can add a small requirement-to-error coverage matrix for future
verification evidence integration. Real raw Instructions sysvar integration
should remain a dedicated reviewed phase.

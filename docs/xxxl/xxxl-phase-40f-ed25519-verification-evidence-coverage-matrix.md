# XXXL Phase 40F Ed25519 Verification Evidence Coverage Matrix

Status: Tiny read-only Rust/SVM coverage matrix.

## Purpose

Phase 40F adds a Rust coverage matrix for future Ed25519 verification evidence
requirements and rejection cases.

It maps Phase 40D requirements to their primary future rejection cases.

It does not implement raw Instructions sysvar integration.

It does not accept verification evidence.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## New Rust Surface

New module:

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_coverage_matrix.rs`

Updated export:

- `programs/xxxl-svm/src/verifier/mod.rs`

The matrix covers:

- all Phase 40D future verification evidence requirements
- selected primary rejection cases for each requirement
- disabled runtime/proof/quorum/auth/execution flags

## Boundary Preserved

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Explicit Non-Goals

Phase 40F does not parse raw Instructions sysvar account data.

Phase 40F does not parse `AccountInfo`.

Phase 40F does not verify Ed25519 signatures.

Phase 40F does not accept cryptographic signature proof.

Phase 40F does not accept verification evidence.

Phase 40F does not count quorum.

Phase 40F does not authorize minting.

Phase 40F does not add a runtime instruction handler.

Phase 40F does not add CPI.

Phase 40F does not enable `invoke_signed`.

Phase 40F does not enable SPL Token `mint_to`.

Phase 40F does not add replay writes.

Phase 40F does not mark processed events.

Phase 40F does not mutate runtime/account state.

Phase 40F does not unlock live route execution.

Phase 40F does not remove deployment blockers.

Phase 40F does not select a production Program ID.

Phase 40F does not claim production readiness.

Phase 40F does not claim final immutability while upgrade authority exists.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40F.

## Recommended Next Stage

Phase 40G may add a docs-only closure checkpoint for the Phase 40 series before
attempting real raw Instructions sysvar integration in a reviewed phase.

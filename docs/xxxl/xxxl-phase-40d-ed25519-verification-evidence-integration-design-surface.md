# XXXL Phase 40D Ed25519 Verification Evidence Integration Design Surface

Status: Tiny read-only Rust/SVM design surface.

## Purpose

Phase 40D adds a Rust design-surface module for future SVM Ed25519 verification
evidence integration.

It does not implement the integration.

It does not parse the raw Instructions sysvar.

It does not call `load_instruction`.

It does not parse `AccountInfo`.

It does not verify Ed25519 signatures.

It does not accept cryptographic proof.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## New Rust Surface

New module:

- `programs/xxxl-svm/src/verifier/ed25519_verification_evidence_integration_design.rs`

Updated export:

- `programs/xxxl-svm/src/verifier/mod.rs`

The module exposes:

- future verification evidence requirements
- future rejection cases
- a design report showing required future bindings
- disabled flags proving no runtime integration is implemented

## Boundary Preserved

The preserved rule remains:

~~~text
located candidate evidence
  != parsed evidence
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

Phase 40D is not the verification evidence implementation.

It is a typed Rust design surface for the future implementation.

## Explicit Non-Goals

Phase 40D does not add a runtime instruction handler.

Phase 40D does not parse raw Instructions sysvar account data.

Phase 40D does not parse `AccountInfo`.

Phase 40D does not call `load_instruction`.

Phase 40D does not verify Ed25519 signatures.

Phase 40D does not accept cryptographic signature proof.

Phase 40D does not accept verification evidence.

Phase 40D does not count quorum.

Phase 40D does not authorize minting.

Phase 40D does not add CPI.

Phase 40D does not enable `invoke_signed`.

Phase 40D does not enable SPL Token `mint_to`.

Phase 40D does not add replay writes.

Phase 40D does not mark processed events.

Phase 40D does not mutate runtime/account state.

Phase 40D does not unlock live route execution.

Phase 40D does not remove deployment blockers.

Phase 40D does not select a production Program ID.

Phase 40D does not claim production readiness.

Phase 40D does not claim final immutability while upgrade authority exists.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40D.

## Recommended Next Stage

Phase 40E should remain narrow. It may either extend tests around the design
surface or stay docs-only. Real raw Instructions sysvar integration should wait
for a dedicated reviewed phase.

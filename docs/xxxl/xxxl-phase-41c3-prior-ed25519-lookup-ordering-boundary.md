# XXXL Phase 41C3 Prior Ed25519 Lookup Ordering Boundary

Status: Narrow Rust boundary.

## Purpose

Phase 41C3 follows the accepted Phase 41C2 review.

It introduces prior Ed25519 lookup and strict ordering over already-separated
descriptors.

This phase intentionally does not read Solana `AccountInfo`.

This phase intentionally does not populate descriptors from real Instructions
sysvar account data.

This phase intentionally does not call `load_instruction`,
`load_instruction_at`, or `load_instruction_at_checked`.

Real runtime population remains deferred to a separate future reviewed phase.

## Files

Added:

- `programs/xxxl-svm/src/verifier/prior_ed25519_lookup_ordering_boundary.rs`
- `docs/xxxl/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c3-prior-ed25519-lookup-ordering-boundary.md`
- `docs/reviews/xxxl-phase-41c3-prior-ed25519-lookup-ordering-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

## Boundary

~~~text
prior Ed25519 lookup + strict ordering over descriptors
  != real AccountInfo population
  != concrete instruction content reading
  != Ed25519 signature verification
  != verification evidence acceptance
  != cryptographic proof
  != quorum
  != authorization
  != replay consumption
  != execution
~~~

## Allowed Results

Phase 41C3 allows exactly seven structural results:

- `PriorEd25519InstructionNotFound`
- `WrongEd25519ProgramId`
- `MalformedStructuralCandidate`
- `DuplicateGuardianEvidence`
- `Ed25519InstructionNotBeforeCurrentInstruction`
- `AmbiguousCandidateEvidence`
- `PriorEd25519InstructionLocatedAndOrdered`

These map to the Phase 41B rejection taxonomy where applicable.

`PriorEd25519InstructionLocatedAndOrdered` is non-authorizing.

It is not proof, verification evidence, quorum, authorization, replay, CPI, or
mint execution.

## Strict Ordering

The only successful located case requires:

~~~text
candidate.instruction_index < current_instruction_index
~~~

Same-index and later-index candidates are rejected as:

- `Ed25519InstructionAfterCurrentInstruction`

## Safety Flags

Phase 41C3 keeps real runtime sysvar population deferred.

It keeps these flags false:

- `raw_instructions_sysvar_parser_implemented`
- `account_info_parser_implemented`
- `load_instruction_called`
- `load_instruction_enabled`
- `current_instruction_identity_derived_from_runtime`
- `verification_evidence_accepted`
- `authorization_enabled`
- `live_route_enabled`

Phase 41C3 keeps this inherited flag true:

- `concrete_runtime_api_selected`

All privilege-increasing flags remain false.

## Explicit Non-Goals

Phase 41C3 does not parse raw Instructions sysvar account data.

Phase 41C3 does not parse `AccountInfo`.

Phase 41C3 does not call `load_instruction`.

Phase 41C3 does not call `load_instruction_at`.

Phase 41C3 does not call `load_instruction_at_checked`.

Phase 41C3 does not read concrete transaction instruction contents.

Phase 41C3 does not verify Ed25519 signatures.

Phase 41C3 does not accept cryptographic signature proof.

Phase 41C3 does not accept verification evidence.

Phase 41C3 does not count quorum.

Phase 41C3 does not authorize minting.

Phase 41C3 does not add a runtime instruction handler.

Phase 41C3 does not add CPI.

Phase 41C3 does not enable `invoke_signed`.

Phase 41C3 does not enable SPL Token `mint_to`.

Phase 41C3 does not add replay writes.

Phase 41C3 does not mark processed events.

Phase 41C3 does not mutate runtime/account state.

Phase 41C3 does not unlock live route execution.

Phase 41C3 does not remove deployment blockers.

Phase 41C3 does not select a production Program ID.

Phase 41C3 does not claim production readiness.

Phase 41C3 does not claim final immutability while upgrade authority exists.

Phase 41C3 does not build SBF artifacts.

Phase 41C3 does not touch `target/deploy`.

Phase 41C3 does not read or modify keypair files.

Phase 41C3 does not read or modify `.env`.

Phase 41C3 does not inspect `.local-keys`.

Phase 41C3 does not run deploy commands.

Phase 41C3 does not run network commands.

Phase 41C3 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C3.

## Recommended Next Stage

Phase 41C3 must be reviewed before any next phase.

The real runtime-wiring step must be introduced as a separate future reviewed
phase.

That future phase is panic-safety-critical because it will be the first place
where real `AccountInfo`, real Instructions sysvar bytes, or real
`load_instruction`-family helpers may be considered.

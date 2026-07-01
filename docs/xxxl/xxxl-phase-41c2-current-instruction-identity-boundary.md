# XXXL Phase 41C2 Current Instruction Identity Boundary

Status: Narrow Rust boundary.

## Purpose

Phase 41C2 follows the accepted Phase 41C1 review.

It introduces a current-instruction identity boundary over an explicit descriptor.

This phase intentionally does not read Solana `AccountInfo`.

This phase intentionally does not populate the descriptor from real Instructions
sysvar account data.

This phase intentionally does not call `load_instruction`,
`load_instruction_at`, or `load_instruction_at_checked`.

Real runtime population of the descriptor remains deferred to a separate future
reviewed phase.

## Files

Added:

- `programs/xxxl-svm/src/verifier/current_instruction_identity_boundary.rs`
- `docs/xxxl/xxxl-phase-41c2-current-instruction-identity-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c2-current-instruction-identity-boundary.md`
- `docs/reviews/xxxl-phase-41c2-current-instruction-identity-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

## Boundary

~~~text
current instruction identity descriptor binding
  != real AccountInfo population
  != concrete instruction content reading
  != prior Ed25519 lookup
  != structural candidate evidence
  != verification evidence
  != cryptographic proof
  != quorum
  != authorization
  != replay consumption
  != execution
~~~

## Allowed Results

Phase 41C2 allows exactly three structural results:

- `MissingCurrentInstructionIdentity`
- `InconsistentCurrentInstructionIdentity`
- `CurrentInstructionIdentityBound`

The first two map to the Phase 41B rejection case:

- `MissingCurrentInstructionIdentity`

`CurrentInstructionIdentityBound` is non-authorizing.

It is not proof, verification evidence, quorum, authorization, replay, CPI, or
mint execution.

## Safety Flags

Phase 41C2 keeps real runtime sysvar population deferred.

It therefore keeps this flag false:

- `current_instruction_identity_derived_from_runtime`

Phase 41C2 keeps this inherited Phase 41C1 flag true:

- `concrete_runtime_api_selected`

All privilege-increasing flags remain false.

## Explicit Non-Goals

Phase 41C2 does not parse raw Instructions sysvar account data.

Phase 41C2 does not parse `AccountInfo`.

Phase 41C2 does not call `load_instruction`.

Phase 41C2 does not call `load_instruction_at`.

Phase 41C2 does not call `load_instruction_at_checked`.

Phase 41C2 does not read concrete transaction instruction contents.

Phase 41C2 does not locate prior Ed25519 instructions.

Phase 41C2 does not verify Ed25519 signatures.

Phase 41C2 does not accept cryptographic signature proof.

Phase 41C2 does not accept verification evidence.

Phase 41C2 does not count quorum.

Phase 41C2 does not authorize minting.

Phase 41C2 does not add a runtime instruction handler.

Phase 41C2 does not add CPI.

Phase 41C2 does not enable `invoke_signed`.

Phase 41C2 does not enable SPL Token `mint_to`.

Phase 41C2 does not add replay writes.

Phase 41C2 does not mark processed events.

Phase 41C2 does not mutate runtime/account state.

Phase 41C2 does not unlock live route execution.

Phase 41C2 does not remove deployment blockers.

Phase 41C2 does not select a production Program ID.

Phase 41C2 does not claim production readiness.

Phase 41C2 does not claim final immutability while upgrade authority exists.

Phase 41C2 does not build SBF artifacts.

Phase 41C2 does not touch `target/deploy`.

Phase 41C2 does not read or modify keypair files.

Phase 41C2 does not read or modify `.env`.

Phase 41C2 does not inspect `.local-keys`.

Phase 41C2 does not run deploy commands.

Phase 41C2 does not run network commands.

Phase 41C2 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C2.

## Recommended Next Stage

Phase 41C2 must be reviewed before Phase 41C3.

Phase 41C3 may focus only on prior Ed25519 lookup and strict ordering over
already-separated descriptors.

Phase 41C3 must not include proof, quorum, authorization, replay, CPI, or mint
execution.

A separate future phase must be used before real `AccountInfo` population or
real Instructions sysvar parsing is introduced.

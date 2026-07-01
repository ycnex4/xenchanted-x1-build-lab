# XXXL Phase 41C1 Instructions Sysvar Presence Readability Boundary

Status: Narrow Rust boundary.

## Purpose

Phase 41C1 follows the accepted Phase 41C0A clarification.

It introduces the smallest possible runtime-read boundary surface:

- concrete runtime API/helper selection is modeled as selected
- Instructions sysvar presence can be represented
- Instructions sysvar readability can be represented
- deterministic structural results are mapped to the Phase 41B taxonomy

Phase 41C1 does not call `load_instruction`.

Phase 41C1 does not call `load_instruction_at`.

Phase 41C1 does not call `load_instruction_at_checked`.

Phase 41C1 does not read concrete transaction instruction contents.

Phase 41C1 does not derive current instruction identity.

Phase 41C1 does not locate prior Ed25519 instructions.

## Files

Added:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_presence_readability_boundary.rs`
- `docs/xxxl/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41c1-instructions-sysvar-presence-readability-boundary.md`
- `docs/reviews/xxxl-phase-41c1-instructions-sysvar-presence-readability-boundary-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

## Boundary

~~~text
runtime sysvar container presence/readability
  != concrete instruction content reading
  != current instruction identity
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

Phase 41C1 allows exactly three structural results:

- `MissingInstructionsSysvar`
- `UnreadableInstructionsSysvar`
- `PresentAndReadable`

The first two map to Phase 41B rejection cases.

`PresentAndReadable` has no rejection case, but it is still not proof,
verification evidence, quorum, authorization, replay, or execution.

## Safety Flags

Phase 41C1 sets only this safety flag to true:

- `concrete_runtime_api_selected`

All other safety flags remain false:

- `raw_instructions_sysvar_parser_implemented`
- `account_info_parser_implemented`
- `load_instruction_called`
- `load_instruction_enabled`
- `current_instruction_identity_derived_from_runtime`
- `ed25519_signature_verification_performed`
- `cryptographic_signature_proof_accepted`
- `verification_evidence_accepted`
- `quorum_counting_enabled`
- `authorization_enabled`
- `replay_write_enabled`
- `processed_event_marking_enabled`
- `account_mutation_enabled`
- `cpi_enabled`
- `invoke_signed_enabled`
- `spl_token_mint_to_enabled`
- `process_instruction_handler_added`
- `live_route_enabled`

## Explicit Non-Goals

Phase 41C1 does not parse raw Instructions sysvar account data.

Phase 41C1 does not parse `AccountInfo`.

Phase 41C1 does not call `load_instruction`.

Phase 41C1 does not call `load_instruction_at`.

Phase 41C1 does not call `load_instruction_at_checked`.

Phase 41C1 does not read concrete transaction instruction contents.

Phase 41C1 does not derive current instruction identity from runtime context.

Phase 41C1 does not locate prior Ed25519 instructions.

Phase 41C1 does not verify Ed25519 signatures.

Phase 41C1 does not accept cryptographic signature proof.

Phase 41C1 does not accept verification evidence.

Phase 41C1 does not count quorum.

Phase 41C1 does not authorize minting.

Phase 41C1 does not add a runtime instruction handler.

Phase 41C1 does not add CPI.

Phase 41C1 does not enable `invoke_signed`.

Phase 41C1 does not enable SPL Token `mint_to`.

Phase 41C1 does not add replay writes.

Phase 41C1 does not mark processed events.

Phase 41C1 does not mutate runtime/account state.

Phase 41C1 does not unlock live route execution.

Phase 41C1 does not remove deployment blockers.

Phase 41C1 does not select a production Program ID.

Phase 41C1 does not claim production readiness.

Phase 41C1 does not claim final immutability while upgrade authority exists.

Phase 41C1 does not build SBF artifacts.

Phase 41C1 does not touch `target/deploy`.

Phase 41C1 does not read or modify keypair files.

Phase 41C1 does not read or modify `.env`.

Phase 41C1 does not inspect `.local-keys`.

Phase 41C1 does not run deploy commands.

Phase 41C1 does not run network commands.

Phase 41C1 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C1.

## Recommended Next Stage

Phase 41C1 must be reviewed before Phase 41C2.

Phase 41C2 may focus on current instruction identity derivation only.

Phase 41C2 must not include prior Ed25519 lookup, proof, quorum,
authorization, replay, CPI, or mint execution.

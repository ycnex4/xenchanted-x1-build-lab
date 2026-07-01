# XXXL Phase 41C0A Clarify 41C1 Sysvar Access Boundary

Status: Docs-only clarification.

## Purpose

Phase 41C0A clarifies the Phase 41C1 boundary after review feedback from the
audit demon and Theo.

Both reviewers accepted Phase 41C0 and agreed that Phase 41C may continue.

Both reviewers also recommended that Phase 41C1 must not call
`load_instruction`, `load_instruction_at`, or any equivalent helper that reads a
specific transaction instruction.

Phase 41C0A records that decision before any Phase 41C1 runtime code is written.

## Review Consensus

The review consensus is:

- Phase 41C0 is accepted.
- Required fixes: none.
- Blocking risks: none.
- Phase 41C1 may start after clarification.
- Phase 41C1 must remain narrower than originally allowed across the full 41C
  series.
- `load_instruction` / `load_instruction_at` must be deferred out of Phase 41C1.

## Clarified Phase 41C1 Boundary

Phase 41C1 is limited to:

- selecting the concrete SVM runtime API or helper path
- adding a minimal read-only Instructions sysvar access boundary
- checking Instructions sysvar presence
- checking Instructions sysvar readability
- returning deterministic structural results:

~~~text
MissingInstructionsSysvar
UnreadableInstructionsSysvar
PresentAndReadable
~~~

Phase 41C1 must not read the contents of a specific transaction instruction.

Phase 41C1 must not call:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- any equivalent helper that returns a concrete instruction by index

## Phase 41C1 Allowed Flag Transitions

Phase 41C1 may set only this flag to true:

- `concrete_runtime_api_selected: true`

Phase 41C1 may propose a narrow read-boundary flag only if the implementation
defines it as sysvar-container presence/readability, not instruction-content
parsing.

Phase 41C1 must not set these flags to true:

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

## Deferred To Later Subphases

Phase 41C2 is responsible for:

- current instruction identity derivation
- current instruction program id binding
- current instruction discriminator or payload binding
- deterministic rejection for missing or inconsistent current identity

Phase 41C3 is responsible for:

- prior Ed25519 instruction lookup
- strict ordering:

~~~text
matched_ed25519_instruction_index < current_instruction_index
~~~

- ambiguity rejection
- duplicate guardian evidence rejection
- malformed structural candidate rejection

## Preserved Boundary

The preserved boundary remains:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != runtime sysvar read
  != structural candidate evidence
  != verification evidence
  != cryptographic proof
  != quorum
  != authorization
  != replay consumption
  != execution
~~~

Phase 41C1 may only open the smallest sysvar-container access boundary.

It must not cross into structural candidate evidence.

## Authoritative Taxonomy

Phase 41C1 must use the Phase 41B taxonomy as authoritative:

- 20 requirements
- 18 rejection cases
- every rejection case has an owning requirement
- four Phase 40 orphan rejection cases closed in Rust

The older Phase 40D/40F 16-requirement planning surface must not be used as the
implementation authority.

## Explicit Non-Goals

Phase 41C0A does not add Rust code.

Phase 41C0A does not modify Rust source files.

Phase 41C0A does not modify TypeScript source files.

Phase 41C0A does not modify test files.

Phase 41C0A does not modify Cargo files.

Phase 41C0A does not modify package files.

Phase 41C0A does not select a concrete runtime API.

Phase 41C0A does not parse raw Instructions sysvar account data.

Phase 41C0A does not parse `AccountInfo`.

Phase 41C0A does not call `load_instruction`.

Phase 41C0A does not call `load_instruction_at`.

Phase 41C0A does not call `load_instruction_at_checked`.

Phase 41C0A does not read any concrete transaction instruction.

Phase 41C0A does not derive current instruction identity from runtime context.

Phase 41C0A does not locate prior Ed25519 instructions.

Phase 41C0A does not verify Ed25519 signatures.

Phase 41C0A does not accept cryptographic signature proof.

Phase 41C0A does not accept verification evidence.

Phase 41C0A does not count quorum.

Phase 41C0A does not authorize minting.

Phase 41C0A does not add a runtime instruction handler.

Phase 41C0A does not add CPI.

Phase 41C0A does not enable `invoke_signed`.

Phase 41C0A does not enable SPL Token `mint_to`.

Phase 41C0A does not add replay writes.

Phase 41C0A does not mark processed events.

Phase 41C0A does not mutate runtime/account state.

Phase 41C0A does not unlock live route execution.

Phase 41C0A does not remove deployment blockers.

Phase 41C0A does not select a production Program ID.

Phase 41C0A does not claim production readiness.

Phase 41C0A does not claim final immutability while upgrade authority exists.

Phase 41C0A does not build SBF artifacts.

Phase 41C0A does not touch `target/deploy`.

Phase 41C0A does not read or modify keypair files.

Phase 41C0A does not read or modify `.env`.

Phase 41C0A does not inspect `.local-keys`.

Phase 41C0A does not run deploy commands.

Phase 41C0A does not run network commands.

Phase 41C0A does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C0A.

## Required Review Before Phase 41C1

Phase 41C0A must be reviewed before Phase 41C1 starts.

Reviewers should confirm:

- `load_instruction` is explicitly deferred out of Phase 41C1
- Phase 41C1 is limited to sysvar presence/readability
- Phase 41C2 owns current instruction identity
- Phase 41C3 owns prior Ed25519 lookup and strict ordering
- proof, quorum, authorization, replay, CPI, and mint execution remain forbidden

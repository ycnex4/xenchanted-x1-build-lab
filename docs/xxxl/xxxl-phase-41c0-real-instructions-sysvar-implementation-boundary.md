# XXXL Phase 41C0 Real Instructions Sysvar Implementation Boundary

Status: Docs-only implementation boundary.

## Purpose

Phase 41C0 opens the Phase 41C series after Phase 41B review.

Phase 41B review consensus:

- verdict: accepted
- required fixes: none
- blocking risks: none
- Phase 41C may start
- Phase 41C must be the first real runtime layer
- Phase 41C must not combine runtime sysvar read with proof, quorum,
  authorization, replay, CPI, or mint execution

Phase 41C0 does not implement real runtime logic.

It defines how the real Instructions sysvar work must be split into narrow
reviewed subphases.

## Why Phase 41C Is Split

Phase 41C is the first phase that may touch real SVM runtime instruction access.

That is a qualitative risk boundary.

The work must be split so that one phase never combines:

- real sysvar read
- current instruction identity derivation
- prior Ed25519 instruction lookup
- verification evidence acceptance
- quorum
- authorization
- replay protection
- mint execution

Phase 41C0 freezes the split before any implementation starts.

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

Phase 41C may introduce `runtime sysvar read`.

It must still not introduce proof, quorum, authorization, replay consumption, or
execution.

## Authoritative Taxonomy

Phase 41C must build from the Phase 41B taxonomy:

- 20 requirements
- 18 rejection cases
- every rejection case has an owning requirement
- four Phase 40 orphan rejection cases closed in Rust

The Phase 41B taxonomy is authoritative for Phase 41C.

The older Phase 40D/40F 16-requirement planning surface must not be treated as
the implementation authority for Phase 41C.

## Phase 41C Subphase Plan

Phase 41C is split into narrow subphases.

### Phase 41C1: Runtime API Selection And Read-Only Sysvar Access Boundary

Allowed purpose:

- select the concrete SVM runtime API or helper path for reading Instructions
  sysvar data
- add only a minimal read-only access layer
- return structural read results or deterministic errors
- keep every privilege-increasing layer disabled

Still forbidden:

- verification evidence acceptance
- Ed25519 signature verification by our verifier
- cryptographic proof acceptance
- quorum
- authorization
- replay writes
- processed-event marking
- account mutation
- CPI
- `invoke_signed`
- SPL Token `mint_to`
- live route unlock

Phase 41C1 must be reviewed before merge.

### Phase 41C2: Current Instruction Identity Derivation

Allowed purpose:

- derive current instruction identity from runtime context
- reject missing or inconsistent current instruction identity
- bind the derived identity to the expected XXXL instruction context

Still forbidden:

- prior Ed25519 lookup beyond the reviewed C2 scope
- verification evidence acceptance
- proof, quorum, authorization, replay, CPI, mint execution

Phase 41C2 must be reviewed before merge.

### Phase 41C3: Prior Ed25519 Instruction Lookup And Strict Ordering

Allowed purpose:

- inspect only instructions strictly before the current XXXL instruction
- locate structurally matching SVM Ed25519 program instruction candidates
- enforce strict ordering:

~~~text
matched_ed25519_instruction_index < current_instruction_index
~~~

- reject ambiguity
- reject duplicate guardian evidence
- reject malformed structural candidates
- map results to the Phase 41B 20-requirement / 18-rejection taxonomy

Still forbidden:

- Ed25519 signature verification by our verifier
- cryptographic proof acceptance
- verification evidence acceptance
- quorum
- authorization
- replay writes
- account mutation
- CPI
- mint execution

Phase 41C3 must be reviewed before merge.

## Possible Safety Flag Transitions Across The 41C Series

Phase 41C0 does not change safety flags.

Future 41C subphases may propose limited flag transitions, but only after review.

Potentially allowed across the whole 41C runtime-read series:

- `concrete_runtime_api_selected: true`
- `raw_instructions_sysvar_parser_implemented: true`
- `account_info_parser_implemented: true`
- `load_instruction_called: true`
- `load_instruction_enabled: true`
- `current_instruction_identity_derived_from_runtime: true`

These are not automatically allowed in every subphase.

Each subphase must explicitly state which flags it changes and why.

All privilege-increasing flags must remain false:

- `ed25519_signature_verification_performed: false`
- `cryptographic_signature_proof_accepted: false`
- `verification_evidence_accepted: false`
- `quorum_counting_enabled: false`
- `authorization_enabled: false`
- `replay_write_enabled: false`
- `processed_event_marking_enabled: false`
- `account_mutation_enabled: false`
- `cpi_enabled: false`
- `invoke_signed_enabled: false`
- `spl_token_mint_to_enabled: false`
- `process_instruction_handler_added: false`
- `live_route_enabled: false`

## Phase 41C0 Explicit Non-Goals

Phase 41C0 does not add Rust code.

Phase 41C0 does not modify Rust source files.

Phase 41C0 does not modify TypeScript source files.

Phase 41C0 does not modify test files.

Phase 41C0 does not modify Cargo files.

Phase 41C0 does not modify package files.

Phase 41C0 does not select a concrete runtime API.

Phase 41C0 does not parse raw Instructions sysvar account data.

Phase 41C0 does not parse `AccountInfo`.

Phase 41C0 does not call `load_instruction`.

Phase 41C0 does not derive current instruction identity from runtime context.

Phase 41C0 does not locate prior Ed25519 instructions.

Phase 41C0 does not verify Ed25519 signatures.

Phase 41C0 does not accept cryptographic signature proof.

Phase 41C0 does not accept verification evidence.

Phase 41C0 does not count quorum.

Phase 41C0 does not authorize minting.

Phase 41C0 does not add a runtime instruction handler.

Phase 41C0 does not add CPI.

Phase 41C0 does not enable `invoke_signed`.

Phase 41C0 does not enable SPL Token `mint_to`.

Phase 41C0 does not add replay writes.

Phase 41C0 does not mark processed events.

Phase 41C0 does not mutate runtime/account state.

Phase 41C0 does not unlock live route execution.

Phase 41C0 does not remove deployment blockers.

Phase 41C0 does not select a production Program ID.

Phase 41C0 does not claim production readiness.

Phase 41C0 does not claim final immutability while upgrade authority exists.

Phase 41C0 does not build SBF artifacts.

Phase 41C0 does not touch `target/deploy`.

Phase 41C0 does not read or modify keypair files.

Phase 41C0 does not read or modify `.env`.

Phase 41C0 does not inspect `.local-keys`.

Phase 41C0 does not run deploy commands.

Phase 41C0 does not run network commands.

Phase 41C0 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C0.

## Required Review Before Phase 41C1

Phase 41C0 must be reviewed before Phase 41C1 starts.

Reviewers should confirm:

- the split is safe
- Phase 41C1 is narrow enough
- no proof/quorum/auth/replay/mint layer can enter Phase 41C1
- the Phase 41B taxonomy remains authoritative
- the allowed flag transitions are scoped correctly

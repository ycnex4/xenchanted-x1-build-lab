# XXXL Phase 41D2.0 Current Identity Runtime Mechanism Clarification

Status: Docs-only clarification before Phase 41D2 code.

## Purpose

Phase 41D2.0 clarifies the runtime mechanism for future Phase 41D2 current-instruction identity population.

Phase 41D2.0 is docs-only.

Phase 41D2.0 does not add Rust code.

Phase 41D2.0 does not modify tests.

Phase 41D2.0 does not parse real `AccountInfo`.

Phase 41D2.0 does not call `load_instruction`.

Phase 41D2.0 does not call `load_instruction_at`.

Phase 41D2.0 does not call `load_instruction_at_checked`.

Phase 41D2.0 does not derive current-instruction identity in code.

## Review Context

Phase 41D1 introduced the first real runtime-read boundary:

- real Instructions sysvar `AccountInfo` presence check
- real Instructions sysvar key check
- real borrow/readability check
- deterministic mapping into Phase 41C1 states

Phase 41D1 was accepted by Theo and the audit demon.

The audit demon raised a forward-looking note for 41D2:

- the per-flag plan keeps `load_instruction_called: false` until 41D3
- therefore 41D2 must derive current-instruction identity without `load_instruction_at`
- if current-index reading is needed, it must use a checked current-index helper and must not load the full current instruction

Phase 41D2.0 records this requirement before any Phase 41D2 code.

## Future 41D2 Identity Source

Future Phase 41D2 must derive current-instruction identity from reviewed runtime context only.

The intended runtime context is:

- direct entrypoint `program_id`
- direct entrypoint `instruction_data`
- expected program id
- expected instruction discriminator
- expected payload/context binding checks

This is current-instruction identity from the executing program context.

It is not prior-instruction enumeration.

It is not Instructions sysvar instruction loading.

It is not Ed25519 evidence lookup.

## Current Instruction Index

If future Phase 41D2 needs the current instruction index, it may only use a checked current-index helper.

Allowed:

- checked current-index access only
- deterministic rejection on missing, unreadable, malformed, or out-of-range current-index data
- no panic on failure

Forbidden:

- `load_instruction`
- `load_instruction_at`
- `load_instruction_at_checked`
- loading the full current instruction from the Instructions sysvar
- reading prior instructions
- enumerating transaction instructions
- constructing Phase 41C3 candidate descriptors

The current-index helper, if used, must not flip `load_instruction_called`.

The current-index helper, if used, must be documented separately in Phase 41D2.

## Flag Consistency Requirement

Future Phase 41D2 may flip:

- `current_instruction_identity_derived_from_runtime: true`

Future Phase 41D2 must keep false:

- `raw_instructions_sysvar_parser_implemented`
- `load_instruction_called`
- `load_instruction_enabled`
- `locates_prior_ed25519_instruction_from_runtime`
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

`account_info_parser_implemented` may remain true from Phase 41D1.

No trust-sensitive flag may become true in Phase 41D2.

## Mapping Requirement

Future Phase 41D2 must map runtime identity into Phase 41C2 states:

- `MissingCurrentInstructionIdentity`
- `InconsistentCurrentInstructionIdentity`
- `CurrentInstructionIdentityBound`

A missing runtime identity input must reject deterministically.

An inconsistent runtime identity input must reject deterministically.

A malformed runtime identity input must reject deterministically.

No identity failure may panic.

No identity failure may fall through to proof, evidence, quorum, authorization, replay, or execution.

## Panic-Safety Requirements For Future 41D2

Future Phase 41D2 must preserve:

- no `unwrap`
- no `expect`
- no `panic!`
- no `unsafe`
- no unchecked indexing
- no unchecked instruction-data slicing
- no unchecked sysvar read
- no out-of-bounds read
- no borrow-failure panic
- no overflow-dependent logic
- deterministic rejection on failure
- no state mutation on failure

Instruction discriminator checking must be length-safe.

Payload/context binding checks must be length-safe.

Current-index checking, if used, must be checked and fail-closed.

## Explicitly Deferred To 41D3

The following remain deferred to 41D3:

- real prior-instruction enumeration
- checked instruction loading for prior instructions
- prefiltering unrelated instructions
- construction of Phase 41C3 candidate descriptors
- same/later fully-matching Ed25519 anomaly decision

## Deferred Beyond 41D

The following remain deferred beyond 41D:

- Ed25519 cryptographic verification
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- processed event marking
- account mutation
- CPI
- `invoke_signed`
- SPL Token `mint_to`
- runtime execution handler
- live route execution

## Active Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41D2.0.

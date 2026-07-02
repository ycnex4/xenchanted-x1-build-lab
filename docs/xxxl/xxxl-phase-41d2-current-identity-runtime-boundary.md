# XXXL Phase 41D2 Current Identity Runtime Boundary

Status: Real current-instruction identity runtime boundary.

## Purpose

Phase 41D2 introduces real current-instruction identity population from direct entrypoint context.

The only runtime identity sources are:

- entrypoint `program_id`
- entrypoint `instruction_data`
- expected program id
- expected instruction discriminator
- expected payload/context binding result

Phase 41D2 maps those runtime inputs into the existing Phase 41C2 descriptor boundary.

## Allowed Scope

Phase 41D2 may:

- compare entrypoint `program_id` with the expected program id
- compare the instruction-data discriminator prefix with the expected discriminator
- use a precomputed payload/context binding result
- construct a Phase 41C2 current-instruction identity descriptor
- map into Phase 41C2 states:
  - `MissingCurrentInstructionIdentity`
  - `InconsistentCurrentInstructionIdentity`
  - `CurrentInstructionIdentityBound`
- flip `current_instruction_identity_derived_from_runtime: true`

## Explicitly Forbidden

Phase 41D2 must not:

- call `load_instruction`
- call `load_instruction_at`
- call `load_instruction_at_checked`
- parse raw Instructions sysvar account data
- enumerate prior instructions
- construct Phase 41C3 candidate descriptors
- locate prior Ed25519 instructions
- verify Ed25519 signatures
- accept cryptographic signature proof
- accept verification evidence
- count guardian quorum
- authorize minting
- write replay state
- mark processed events
- mutate runtime/account state
- perform CPI
- call `invoke_signed`
- call SPL Token `mint_to`
- add a runtime execution handler
- unlock live route execution

## Runtime Mapping

Mapping:

| Runtime condition | Phase 41C2 state | Rejection |
| --- | --- | --- |
| missing entrypoint program id | `MissingCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| missing entrypoint instruction data | `MissingCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| empty expected discriminator | `MissingCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| program id mismatch | `InconsistentCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| instruction data shorter than discriminator | `InconsistentCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| discriminator mismatch | `InconsistentCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| payload/context binding mismatch | `InconsistentCurrentInstructionIdentity` | `MissingCurrentInstructionIdentity` |
| all identity checks match | `CurrentInstructionIdentityBound` | none |

A valid current identity is not proof.

A valid current identity is not verification evidence.

A valid current identity is not authorization.

A valid current identity is not execution.

## Panic-Safety

Phase 41D2 must remain panic-safe:

- no `unwrap`
- no `expect`
- no `panic!`
- no `unsafe`
- no unchecked indexing
- no unchecked slicing
- no unchecked sysvar read
- no out-of-bounds read
- no borrow-failure panic
- no overflow-dependent logic
- no state mutation on failure

Instruction discriminator comparison is length-safe.

The implementation uses bounded slice access through `get`.

Short instruction data maps to deterministic inconsistency.

No failure path may fall through to proof, evidence, quorum, authorization, replay, or execution.

## Flag Transition

Phase 41D2 flips only the current-identity runtime flag:

- `current_instruction_identity_derived_from_runtime: true`

The following remain false:

- `raw_instructions_sysvar_parser_implemented`
- `load_instruction_called`
- `load_instruction_enabled`
- `locates_prior_ed25519_instruction`
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

`account_info_parser_implemented` remains true from Phase 41D1.

## Deferred To 41D3

The following remain deferred to Phase 41D3:

- prior-instruction enumeration
- checked instruction loading for prior instructions
- prefiltering unrelated instructions
- construction of Phase 41C3 candidate descriptors
- same/later fully-matching Ed25519 anomaly decision

## Deferred Beyond 41D

The following remain deferred beyond Phase 41D:

- Ed25519 cryptographic verification
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- account mutation
- CPI
- SPL Token mint execution
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

No blocker is removed, weakened, renamed, or satisfied by Phase 41D2.

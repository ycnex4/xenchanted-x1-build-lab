# XXXL Phase 41D1 AccountInfo Presence Readability Runtime Boundary

Status: First real runtime-read boundary.

## Purpose

Phase 41D1 introduces the first real runtime-facing read boundary.

The scope is intentionally narrow:

- identify the Instructions sysvar `AccountInfo`
- check whether it is supplied
- check whether it can be borrowed/read
- map the result into the existing Phase 41C1 descriptor states

Phase 41D1 does not parse concrete transaction instruction contents.

Phase 41D1 does not call `load_instruction`.

Phase 41D1 does not call `load_instruction_at`.

Phase 41D1 does not call `load_instruction_at_checked`.

Phase 41D1 does not derive current instruction identity.

Phase 41D1 does not enumerate prior instructions.

Phase 41D1 does not construct Phase 41C3 candidate descriptors.

Phase 41D1 does not accept verification evidence.

## Runtime Mapping

Phase 41D1 maps real runtime `AccountInfo` access into Phase 41C1.

Mapping:

| Runtime condition | Phase 41C1 status | Phase 41B rejection |
| --- | --- | --- |
| no account supplied | `MissingInstructionsSysvar` | `MissingInstructionsSysvar` |
| supplied account key is not Instructions sysvar | `MissingInstructionsSysvar` | `MissingInstructionsSysvar` |
| Instructions sysvar account cannot be borrowed | `UnreadableInstructionsSysvar` | `UnreadableInstructionsSysvar` |
| Instructions sysvar account can be borrowed | `PresentAndReadable` | none |

The readable case only proves that the account data can be borrowed.

It does not prove that the data is valid Instructions sysvar contents.

It does not read any instruction contents.

It does not load any instruction.

## Panic-Safety

Phase 41D1 must remain panic-safe:

- no `unwrap`
- no `expect`
- no `panic!`
- no `unsafe`
- no unchecked indexing
- no unchecked sysvar parsing
- no out-of-bounds read
- no borrow-failure panic
- no state mutation on failure

Borrow failure maps to `UnreadableInstructionsSysvar`.

Missing account maps to `MissingInstructionsSysvar`.

Wrong account key maps to `MissingInstructionsSysvar`.

No failure path may fall through to authorization.

## Flag Transition

Phase 41D1 may flip only the first real read-capability flag:

- `account_info_parser_implemented: true`

The following remain false:

- `raw_instructions_sysvar_parser_implemented`
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

## Preserved Boundary

Phase 41D1 is not a proof boundary.

Phase 41D1 is not a verification-evidence boundary.

Phase 41D1 is not a quorum boundary.

Phase 41D1 is not an authorization boundary.

Phase 41D1 is not an execution boundary.

A `PresentAndReadable` result is not proof.

A `PresentAndReadable` result is not verification evidence.

A `PresentAndReadable` result is not authorization.

## Deferred To Later Phases

Deferred to 41D2:

- current-instruction identity population

Deferred to 41D3:

- prior-instruction enumeration
- checked instruction loading
- prefiltering unrelated instructions
- construction of Phase 41C3 candidate descriptors
- same/later fully-matching Ed25519 anomaly decision

Deferred beyond 41D:

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

No blocker is removed, weakened, renamed, or satisfied by Phase 41D1.

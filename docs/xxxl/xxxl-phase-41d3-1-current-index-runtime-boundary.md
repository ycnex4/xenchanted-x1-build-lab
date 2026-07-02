# XXXL Phase 41D3.1 Current Instruction Index Runtime Boundary

Status: Real checked current-index runtime boundary.

## Purpose

Phase 41D3.1 is a narrow code sub-step inside the accepted Phase 41D3 boundary.

It introduces only checked current-instruction index acquisition.

It does not enumerate prior instructions.

It does not load prior instructions.

It does not construct Phase 41C3 candidate descriptors.

It does not locate prior Ed25519 instructions.

## Why Split 41D3

Phase 41D3 is the widest runtime step so far.

It eventually combines:

- current-index acquisition
- prior-instruction enumeration
- checked instruction loading
- prefiltering
- Phase 41C3 descriptor construction
- same/later anomaly decision

Phase 41D3.1 splits out only the first part.

This keeps the first code step small and easier to audit.

## Allowed Scope

Phase 41D3.1 may:

- receive the Instructions sysvar AccountInfo as optional input
- check that the account key equals the Instructions sysvar id
- call `load_current_index_checked`
- map current-index acquisition failure to deterministic rejection
- expose the acquired current index as ordering-only data
- keep all proof, evidence, quorum, authorization, replay, CPI, mint, and live route flags false

## Explicitly Forbidden

Phase 41D3.1 must not:

- call `load_instruction`
- call `load_instruction_at`
- call `load_instruction_at_checked`
- enumerate prior instructions
- parse raw Instructions sysvar account data
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

| Runtime condition | Phase 41D3.1 status | Rejection |
| --- | --- | --- |
| missing Instructions sysvar account | `MissingInstructionsSysvar` | `MissingInstructionsSysvar` |
| wrong account key | `MissingInstructionsSysvar` | `MissingInstructionsSysvar` |
| checked current-index read fails | `CurrentInstructionIndexUnavailable` | `UnreadableInstructionsSysvar` |
| checked current-index read succeeds | `CurrentInstructionIndexAcquired` | none |

## Current Index Boundary

The current index is used only for ordering.

The current index is not proof.

The current index is not verification evidence.

The current index is not authorization.

The current index cannot trigger replay writes, CPI, mint execution, or state mutation.

## Panic-Safety

Phase 41D3.1 must remain panic-safe:

- no `unwrap`
- no `expect`
- no `panic!`
- no `unsafe`
- no unchecked indexing
- no unchecked slicing
- no raw sysvar data parsing
- no out-of-bounds instruction access
- no state mutation on failure

Checked current-index helper errors map to deterministic rejection. Phase 41D3.1 does not classify raw sysvar bytes itself.

## Flag Transition

Phase 41D3.1 does not yet flip:

- `load_instruction_called`
- `load_instruction_enabled`
- `locates_prior_ed25519_instruction`

Those remain deferred to the later prior-enumeration/loading sub-step.

The following inherited flags remain true:

- `account_info_parser_implemented`
- `concrete_runtime_api_selected`
- `current_instruction_identity_derived_from_runtime`

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

## Deferred To Later 41D3 Sub-Step

The following remain deferred:

- prior-instruction enumeration
- checked prior-instruction loading
- prefiltering unrelated instructions
- Phase 41C3 candidate descriptor construction
- same-index reject
- later-index reject
- prior-index candidate-only result

## Active Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41D3.1.

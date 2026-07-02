# XXXL Phase 41D3.0 Prior Instruction Enumeration Runtime Boundary Plan

Status: Docs-only safety plan before Phase 41D3 code.

## Purpose

Phase 41D3 will introduce the first real prior-instruction enumeration boundary.

Because this phase touches the Instructions sysvar surface, it must be planned before code.

Phase 41D3.0 is docs-only.

It does not add code.

It does not select final production runtime behavior.

It does not authorize execution.

## Review Gate Source

Phase 41D3 is allowed by Phase 41D2 external acceptance.

Demon and Theo both accepted Phase 41D2 with no required fixes.

Theo defined the minimum safe Phase 41D3 boundary:

Allowed:

- real prior-instruction enumeration via Instructions sysvar
- checked instruction loading through `load_instruction_at_checked` or reviewed equivalent
- prefiltering unrelated instructions
- non-Ed25519 instruction discard
- Phase 41C3 candidate descriptor construction
- explicit same/later fully-matching Ed25519 anomaly decision
- `locates_prior_ed25519_instruction: true` flag flip
- `load_instruction_called: true` flag flip if a checked helper is used

Forbidden:

- Ed25519 cryptographic verification
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- account mutation
- CPI
- `invoke_signed`
- SPL Token `mint_to`
- live route unlock
- handler or execution flag enablement

## Phase 41D3 Intended Scope

Phase 41D3 may add a narrow runtime boundary that:

1. Receives the Instructions sysvar account as runtime input.
2. Obtains the current instruction index through a checked, panic-safe path.
3. Enumerates only prior instructions.
4. Loads prior instructions using a checked helper.
5. Prefilters unrelated instructions.
6. Identifies candidate prior Ed25519 instructions.
7. Constructs Phase 41C3 candidate descriptors.
8. Preserves the Phase 41C3A pre-filter contract.
9. Explicitly handles the same/later fully-matching Ed25519 anomaly decision.
10. Flips only the intended prior-location/runtime-loading flags.

## Phase 41D3 Must Not Do

Phase 41D3 must not:

- verify Ed25519 signatures cryptographically
- parse signatures as accepted proof
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
- select a production Program ID
- remove deployment blockers
- claim production readiness
- claim final immutability

## Required Runtime Safety Properties

The Phase 41D3 code must be panic-safe.

It must avoid:

- `unwrap`
- `expect`
- `panic!`
- `unsafe`
- unchecked indexing
- unchecked slicing
- unchecked current-index reads
- unchecked prior-instruction loads
- out-of-bounds instruction access
- fallthrough from malformed instruction to accepted evidence
- fallthrough from found candidate to authorization

All malformed, missing, unrelated, same-current, later, or structurally invalid instructions must fail closed.

## Current Index Boundary

Phase 41D3 may acquire current index only through a checked path.

The current index must be used only for ordering.

The current index must not be treated as proof.

The current index must not authorize execution.

The current index must not mark replay state.

If current index acquisition fails, Phase 41D3 must return a deterministic rejection.

## Prior Enumeration Boundary

Phase 41D3 may enumerate prior instruction indexes only.

Allowed prior range:

- indexes strictly less than current index

Forbidden ranges:

- current instruction index
- later instruction indexes
- unchecked indexes
- synthetic indexes outside the transaction instruction list

If no suitable prior candidate exists, the result must be deterministic rejection.

## Instruction Loading Boundary

Phase 41D3 may load prior instructions only through:

- `load_instruction_at_checked`, or
- a reviewed equivalent checked helper

The helper must:

- fail closed on invalid index
- fail closed on invalid sysvar account
- fail closed on malformed instruction data
- avoid panic
- avoid unchecked slicing
- avoid unchecked indexing

`load_instruction_at` without a checked wrapper is not allowed.

Raw Instructions sysvar byte parsing is not allowed unless separately reviewed.

## Prefilter Boundary

Phase 41D3 must preserve the Phase 41C3A pre-filter contract.

Unrelated instructions must be discarded before descriptor construction.

Prefiltering may inspect only enough structure to reject unrelated instructions.

Prefiltering must not:

- verify signatures
- accept evidence
- count quorum
- authorize execution
- mutate state

## Candidate Descriptor Boundary

Phase 41D3 may construct Phase 41C3 candidate descriptors.

A candidate descriptor is not proof.

A candidate descriptor is not accepted evidence.

A candidate descriptor is not authorization.

Candidate descriptors may only describe whether a prior instruction structurally matches the expected Ed25519 instruction shape and expected message hash binding.

## Same/Later Anomaly Decision

Phase 41D3 must explicitly decide same/later fully-matching Ed25519 anomaly behavior.

Minimum safe decision:

- same-index match: reject
- later-index match: reject
- prior-index match: candidate only, not proof

This decision must be documented in code tests and docs.

## Allowed Flag Transitions

Phase 41D3 may flip:

- `load_instruction_called: true`
- `locates_prior_ed25519_instruction: true`

Only if the implementation really performs checked prior-instruction loading and prior candidate location.

Depending on exact implementation naming, Phase 41D3 may also introduce a report field describing checked instruction loading.

## Flags That Must Remain False

The following must remain false:

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

## Required Test Coverage For 41D3 Code

Phase 41D3 code must include tests for:

- missing Instructions sysvar account
- unreadable or invalid Instructions sysvar account
- failed current-index acquisition
- current index zero
- no prior instructions
- unrelated prior instructions
- malformed prior instruction data
- prior non-Ed25519 instruction discarded
- prior Ed25519-shaped instruction produces candidate descriptor
- current-index matching instruction rejected
- later matching instruction rejected
- short data does not panic
- invalid index does not panic
- no evidence accepted
- no authorization enabled
- no CPI/mint/replay flags enabled
- exact intended flag flips only

## Required Validation For 41D3 Code

The Phase 41D3 code phase must run:

- forbidden operation grep
- panic token grep
- unchecked index/slice grep
- `git diff --check`
- `cargo fmt`
- `cargo fmt --check`
- targeted cargo test for the new 41D3 module
- verifier cargo tests
- full `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

No SBF build should run.

No deploy command should run.

No network command should run.

No keypair or `.env` file should be read or modified.

## Active Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Phase 41D3.0 does not remove, weaken, rename, or satisfy any blocker.

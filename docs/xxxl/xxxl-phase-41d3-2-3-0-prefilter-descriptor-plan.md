# XXXL Phase 41D3.2.3.0 — Prefilter + Phase 41C3 Candidate Descriptor Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41D3.2.3.0.

## Parent Gate

Phase 41D3.2.2 was accepted by external review.

Accepted parent checkpoint:

`0cb2478 Merge XXXL phase 41D3 checked prior loading acceptance record`

Accepted checked loading boundary:

- consumes bounded prior range from Phase 41D3.2.1;
- accepts Instructions sysvar AccountInfo;
- checks Instructions sysvar account key before loading;
- empty prior range causes no loading attempt;
- iterates prior indexes with `.iter().copied()`;
- calls `load_instruction_at_checked` only for prior indexes;
- maps checked loading success to runtime-data-only entries;
- maps checked loading failure to deterministic non-authorizing failure.

## Purpose

Define the minimum safe implementation plan for Phase 41D3.2.3:

- consume loaded prior instructions from Phase 41D3.2.2;
- prefilter unrelated prior instructions;
- identify Ed25519 program-id candidates structurally;
- construct Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- allow `locates_prior_ed25519_instruction: true`;
- keep descriptors non-authorizing;
- keep evidence/proof/auth closed.

## Why This Is a Separate Gate

Phase 41D3.2.3 is the first step where the runtime boundary may say:

`locates_prior_ed25519_instruction: true`

This is trust-sensitive wording.

The phrase must mean only:

- a prior instruction with the Ed25519 program id was structurally located;
- a non-authorizing candidate descriptor was created.

It must not mean:

- Ed25519 signature was cryptographically verified;
- signature proof was accepted;
- verification evidence was accepted;
- guardian quorum was counted;
- mint execution was authorized;
- replay registry may be written;
- runtime state may mutate.

## Minimum Safe Code Boundary

Allowed in Phase 41D3.2.3:

- consume Phase 41D3.2.2 loaded prior instruction entries;
- process only loaded entries marked as runtime data only;
- keep loaded entries non-authorizing;
- prefilter unrelated instructions by program id;
- identify structurally relevant Ed25519 program-id candidates;
- construct Phase 41C3 candidate descriptors from structural metadata only;
- preserve instruction index ordering metadata;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- keep malformed structural candidates non-authorizing;
- keep duplicate or ambiguous structural candidates non-authorizing;
- flip `locates_prior_ed25519_instruction: true` only when a prior Ed25519 structural candidate is located.

Forbidden in Phase 41D3.2.3:

- Ed25519 cryptographic verification;
- cryptographic signature proof acceptance;
- verification evidence acceptance;
- guardian quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- runtime handler;
- live route unlock;
- production route readiness claims.

Still forbidden from earlier loading boundaries:

- `load_instruction`;
- `load_instruction_at`;
- unchecked loading;
- raw Instructions sysvar byte parsing;
- direct sysvar byte slicing.

## Streaming / Heap Requirement

Phase 41D3.2.2 accepted a bounded vector of loaded prior instructions.

Phase 41D3.2.3 should avoid compounding heap pressure.

Preferred implementation pattern:

- iterate over loaded prior entries by reference;
- prefilter each entry immediately;
- discard non-candidates immediately;
- store only candidate descriptor metadata;
- do not clone full `Instruction` values unless required and bounded;
- do not hold both all loaded instructions and additional full-instruction candidate copies.

Acceptable descriptor storage:

- instruction index;
- program id;
- minimal structural status;
- non-authorizing marker fields;
- ordering status;
- rejection status for same/later/malformed/ambiguous cases.

Descriptors should not store or reinterpret proof material as trusted evidence.

## Same / Later Reject Requirement

Even though Phase 41D3.2.1 constructs strict prior ranges and Phase 41D3.2.2 contains a runtime same/later guard, Phase 41D3.2.3 must still explicitly handle same/later candidates as defense in depth.

Required behavior:

- candidate index `< current_instruction_index` may be structurally considered;
- candidate index `== current_instruction_index` must be rejected explicitly;
- candidate index `> current_instruction_index` must be rejected explicitly;
- same/later rejection must happen before any candidate is treated as located;
- same/later rejection must not accept evidence or authorize execution.

## Candidate Descriptor Meaning

A Phase 41C3 candidate descriptor may mean only:

- a prior instruction was loaded;
- its program id matched the Ed25519 program id;
- its position is prior to the current instruction;
- its structural metadata was recorded for a later phase.

A candidate descriptor must not mean:

- signature is valid;
- instruction data is valid proof;
- guardian key is valid;
- quorum is met;
- message hash is accepted;
- mint may execute;
- replay registry may update.

## Expected Safety Flag Changes

Allowed in Phase 41D3.2.3 after code implementation and review:

- `locates_prior_ed25519_instruction: true`.

Already enabled by Phase 41D3.2.2 and may remain true:

- `prior_instruction_loading_enabled`;
- `load_instruction_called`;
- `load_instruction_enabled`.

Must remain false:

- `ed25519_signature_verification_performed`;
- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `quorum_counting_enabled`;
- `authorization_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

## Expected Tests For Code Phase

Phase 41D3.2.3 code should test:

- empty loaded prior instruction list produces no candidate descriptors;
- unrelated loaded instructions are discarded;
- Ed25519 program-id loaded instruction creates a structural candidate descriptor;
- candidate descriptor remains non-authorizing;
- descriptor does not accept verification evidence;
- descriptor does not verify signatures;
- descriptor does not count quorum;
- descriptor does not authorize execution;
- same-index candidate is explicitly rejected;
- later-index candidate is explicitly rejected;
- malformed structural candidate is deterministic and non-authorizing;
- duplicate/ambiguous structural candidates are deterministic and non-authorizing;
- non-candidates are discarded without cloning full instruction data;
- `locates_prior_ed25519_instruction` flips only in the descriptor-location layer;
- all proof/evidence/auth/replay/CPI/mint/live-route flags remain false.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Phase 41D3.2.3.0 Output

This document is only a plan and review target.

It does not implement:

- prefiltering;
- descriptor construction;
- candidate location;
- cryptographic verification;
- evidence acceptance;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- live route.

## Review Gate

Do not start Phase 41D3.2.3 code until this plan is reviewed and accepted.

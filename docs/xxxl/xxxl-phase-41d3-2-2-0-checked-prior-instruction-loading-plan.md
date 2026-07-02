# XXXL Phase 41D3.2.2.0 — Checked Prior Instruction Loading Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41D3.2.2.0.

## Parent Gate

Phase 41D3.2.1 was accepted by external review.

Accepted parent checkpoint:

`9880d63 Merge XXXL phase 41D3 prior index range acceptance record`

Accepted prior-index range boundary:

- accepts checked current index from Phase 41D3.1;
- constructs bounded prior range `0..current_index`;
- `current_index == 0` yields empty prior range;
- same index is excluded by range construction;
- later indexes are excluded by range construction;
- unavailable/inconsistent/forged oversized current index fails closed.

## Purpose

Define the minimum safe implementation plan for Phase 41D3.2.2:

- accept bounded prior indexes from Phase 41D3.2.1;
- load prior instructions through Solana checked helper only;
- use `load_instruction_at_checked`;
- iterate prior indexes lazily for loading;
- avoid materializing a second large loading vector unless explicitly bounded;
- map checked loading success/failure deterministically;
- introduce no descriptor construction;
- introduce no evidence acceptance;
- introduce no authorization;
- introduce no mutation/CPI/mint behavior.

## Why This Is a Separate Gate

Phase 41D3.2.2 is the first step where real runtime instruction loading is allowed.

This creates a new risk surface:

- checked loading API misuse;
- accidental unchecked loading;
- accidental raw sysvar parsing;
- memory pressure from materializing large ranges;
- accidental interpretation of loaded instruction data as evidence;
- accidental coupling to Phase 41C3 descriptors too early.

Therefore this step must remain separate from prefiltering and descriptor construction.

## Minimum Safe Code Boundary

Allowed in Phase 41D3.2.2:

- consume the Phase 41D3.2.1 prior index range result;
- require the prior index range result to be valid;
- accept the Instructions sysvar AccountInfo as loading source;
- verify the account key is the Instructions sysvar id before loading;
- iterate prior indexes lazily;
- call `load_instruction_at_checked(index, instructions_sysvar_account)` only for prior indexes;
- map each checked loading result into a deterministic non-authorizing loaded-entry result;
- fail closed if the Instructions sysvar account is missing or wrong;
- fail closed or deterministically reject if checked loading fails;
- set loading-specific report fields:
  - `prior_instruction_loading_enabled: true`;
  - `load_instruction_called: true`;
  - `load_instruction_enabled: true`.

Forbidden in Phase 41D3.2.2:

- `load_instruction`;
- `load_instruction_at`;
- unchecked loading;
- raw Instructions sysvar byte parsing;
- direct byte slicing of sysvar data;
- Ed25519 cryptographic verification;
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
- deployment readiness claims.

## Lazy Iteration Requirement

Phase 41D3.2.1 materialized the prior range as a `Vec<usize>`, which was accepted for range-only modeling.

Phase 41D3.2.2 should not compound memory pressure during loading.

Preferred loading pattern:

- borrow the existing bounded prior range by reference;
- iterate with `.iter().copied()`;
- do not construct a second index vector;
- do not allocate proportional temporary buffers unless explicitly justified and bounded.

If the implementation chooses to collect loaded results, it must remain bounded by the already checked prior range and must not introduce unbounded allocation.

## Current Index Zero Behavior

If the Phase 41D3.2.1 result contains an empty prior index range:

- no loading is attempted;
- `load_instruction_at_checked` is not called;
- result remains deterministic and non-authorizing;
- no descriptor construction occurs.

This preserves the accepted rule:

- `current_index == 0 => empty prior range`;
- no prior instruction can exist before index `0`.

## Loading Failure Behavior

A checked loading failure must not panic.

Acceptable behavior:

- return a deterministic failure status for the whole loading boundary; or
- return a deterministic per-index failure entry and mark the boundary non-locating/non-authorizing.

Required either way:

- no proof/evidence accepted;
- no authorization;
- no mutation;
- no CPI;
- no mint;
- no live route.

## Loaded Instruction Boundary

A loaded instruction is still only runtime data.

It does not mean:

- Ed25519 candidate exists;
- evidence is valid;
- signature is valid;
- guardian is authorized;
- quorum is reached;
- mint is authorized;
- replay registry may be updated;
- runtime state may mutate.

Phase 41D3.2.2 may load instructions only.

Phase 41D3.2.3 remains responsible for:

- prefiltering unrelated instructions;
- building Phase 41C3 descriptors;
- explicit same/later reject path;
- any `locates_prior_ed25519_instruction: true` flip.

## Expected Safety Flag Changes

Allowed in Phase 41D3.2.2 after code implementation and review:

- `load_instruction_called: true`;
- `load_instruction_enabled: true`;
- `prior_instruction_loading_enabled: true`.

Must remain false:

- `raw_instructions_sysvar_parser_implemented`;
- `locates_prior_ed25519_instruction`;
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

Phase 41D3.2.2 code should test:

- empty prior range causes no loading attempt;
- missing Instructions sysvar account fails closed;
- wrong Instructions sysvar account key fails closed;
- checked loading failure is deterministic and non-panicking;
- loading is attempted only for prior indexes from Phase 41D3.2.1;
- no same/later index is introduced by the loading layer;
- no raw sysvar parsing exists;
- no unchecked loading exists;
- no descriptor construction exists;
- loaded instruction does not imply evidence acceptance;
- loaded instruction does not imply authorization;
- safety flags only flip loading-related fields;
- all trust-sensitive flags remain false.

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

## Phase 41D3.2.0 Output

This document is only a plan and review target.

It does not implement:

- checked instruction loading;
- runtime helper calls;
- prefiltering;
- descriptors;
- evidence acceptance;
- authorization;
- mutation;
- CPI;
- mint;
- live route.

## Review Gate

Do not start Phase 41D3.2.2 code until this plan is reviewed and accepted.

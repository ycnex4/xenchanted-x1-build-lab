# XXXL Phase 41K.1 — Real Instructions Sysvar Loading Plan

Date: 2026-07-03

Status: draft plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-1-instructions-sysvar-plan`

Base checkpoint:

`bd53ace Merge XXXL phase 41K live-wiring plan acceptance`

## Purpose

Phase 41K.1 is the first sub-gate of the accepted 41K live-wiring master plan.

Its purpose is to plan how the verifier can load the real Solana / SVM Instructions sysvar and derive real prior Ed25519 instruction data from runtime state.

41K.1 exists to replace the earlier boundary-model assumption:

`preloaded prior instruction data`

with the runtime-derived model:

`real Instructions sysvar -> checked current instruction index -> real prior Ed25519 instruction -> accepted 41F.1 / 41F.2 pipeline`

## Scope

This is a docs-only plan.

No Rust code is implemented by this document.

No runtime sysvar loading is enabled by this document.

No handler is added by this document.

No CPI, mint, processed registry write, guardian PDA loading, or live route is enabled by this document.

## Why 41K.1 Is High Risk

Until 41J, the pipeline intentionally avoided real runtime loading.

41K.1 introduces the first runtime-derived authority source:

`Instructions sysvar`

This is high risk because the accepted 41F.2 Model A reasoning is only sound if the verifier uses real runtime instruction context, not caller-provided instruction bytes.

## Accepted Prior Chain

41K.1 must preserve the accepted chain up to 41J:

`checked Ed25519 byte extraction -> native Ed25519 verification evidence -> payload hash binding -> guardian membership -> quorum -> replay eligibility`

41K.1 does not change 41G, 41H, 41I, or 41J.

41K.1 only plans the real runtime source for the instruction bytes consumed by 41F.1 and 41F.2.

## Model A Live-Wiring Soundness Precondition

41K.1 must explicitly preserve the Model A soundness precondition behind 41F.2.

Loading the Instructions sysvar is necessary but not sufficient.

41K.1 must guarantee:

- the XXXL verifier is executing as the current instruction;
- the current instruction index is loaded from the real Instructions sysvar;
- the current instruction index is not caller-provided;
- the Ed25519 instruction is a real prior instruction in the same transaction;
- the prior instruction is the real Ed25519 precompile program instruction;
- the prior instruction index is checked relative to the real current instruction index;
- prior index must be strictly less than current index;
- instruction data must come from the real sysvar entry;
- instruction data must not come from caller instruction data;
- instruction data must not come from frontend/watcher/off-chain proof;
- reaching the current instruction implies the prior Ed25519 precompile verification already passed under the SVM transaction execution model.

If any part of this precondition is not met, 41K.1 must fail closed.

## Required Runtime Source

The only valid source for prior instruction data in 41K.1 is:

`real Instructions sysvar account`

Invalid sources:

- caller-provided instruction bytes;
- instruction bytes stored inside XXXL instruction data;
- frontend-provided Ed25519 proof;
- watcher-provided Ed25519 proof;
- test fixture bytes in production path;
- previously cached off-chain proof;
- arbitrary account pretending to be Instructions sysvar.

## Current Instruction Index Rule

The current instruction index must be loaded using a checked runtime path.

Required:

- read current instruction index from real Instructions sysvar;
- reject caller-provided current index;
- reject missing current index;
- reject malformed sysvar data;
- reject current index out of bounds;
- reject prior index greater than or equal to current index.

The verifier must not trust any user-supplied current index.

## Prior Instruction Rule

The selected prior instruction must be:

- loaded from the real Instructions sysvar;
- strictly prior to the current instruction;
- owned by / addressed to the real Ed25519 precompile program;
- structurally parseable by accepted Ed25519 instruction parsing boundaries;
- the source of signature, public key, and message byte ranges consumed by 41F.1.

The verifier must reject:

- current instruction as Ed25519 proof;
- future instruction as Ed25519 proof;
- non-Ed25519 instruction as Ed25519 proof;
- fabricated instruction entry;
- instruction data copied from caller payload.

## Output of 41K.1

41K.1 should produce a runtime-derived instruction context that can feed the already accepted boundaries.

Conceptual output:

- real current instruction index;
- real prior Ed25519 instruction index;
- real prior Ed25519 instruction program id;
- real prior Ed25519 instruction data;
- provenance marker showing the data came from real Instructions sysvar.

The output must not authorize execution by itself.

The output only makes the earlier 41F.1 / 41F.2 pipeline runtime-grounded.

## Still Out of Scope

41K.1 must not include:

- real guardian-set PDA loading;
- real processed-registry PDA loading;
- replay registry write;
- processed event marking;
- mint amount execution;
- SPL token CPI;
- handler live route;
- production deployment address selection;
- removal of deployment blockers.

Those remain later 41K sub-gates.

## Required Safety Flags

Any 41K.1 implementation report must keep these false:

- `guardian_set_runtime_loading_enabled: false`
- `processed_registry_runtime_loading_enabled: false`
- `replay_write_enabled: false`
- `processed_event_marking_enabled: false`
- `account_mutation_enabled: false`
- `cpi_enabled: false`
- `invoke_signed_enabled: false`
- `spl_token_mint_to_enabled: false`
- `handler_enabled: false`
- `live_route_enabled: false`

41K.1 may only plan to enable:

- `instructions_sysvar_loading_enabled: true`

but only inside a separately reviewed 41K.1 implementation.

## Active Deployment Blockers

41K.1 does not remove any deployment blocker.

Relevant blockers remain:

- `EXTERNAL_REVIEW_INCOMPLETE`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `SPL_CPI_EXECUTION_DISABLED`
- `LIVE_ROUTE_DISABLED`

41K.1 only addresses the runtime source of Ed25519 prior instruction data.

## Forbidden Design Patterns

41K.1 must reject any design that:

- accepts caller-provided current instruction index;
- accepts caller-provided prior instruction bytes;
- accepts caller-provided Ed25519 proof;
- accepts watcher-provided Ed25519 proof as authority;
- treats arbitrary account data as Instructions sysvar;
- treats current instruction as prior Ed25519 proof;
- treats future instruction as prior Ed25519 proof;
- skips Ed25519 program id verification;
- bypasses accepted 41F.1 / 41F.2 boundaries;
- directly authorizes mint or mutation.

## Required Review Questions

Before 41K.1 code, reviewers must answer:

1. Is the Model A live-wiring soundness precondition explicit enough?
2. Is the current instruction index required to come from real Instructions sysvar?
3. Is caller-provided current index rejected?
4. Is the prior Ed25519 instruction required to be strictly prior to current instruction?
5. Is Ed25519 program id verification required?
6. Are fabricated instruction entries rejected?
7. Are frontend/watcher supplied proofs rejected as authority?
8. Does 41K.1 avoid guardian-set PDA loading?
9. Does 41K.1 avoid processed-registry PDA loading?
10. Does 41K.1 avoid mutation, CPI, mint, handler, and live route?
11. Is this plan sufficient before 41K.1 code?

## Review Notes Incorporated

The following review notes are incorporated into the 41K.1 plan before implementation.

### Multi-Guardian Prior Ed25519 Enumeration

41K.1 code must support the accepted 41I quorum model.

41I does not rely on a single guardian attempt.

A quorum may require N guardian attempts, and each successful attempt may correspond to a separate prior Ed25519 precompile instruction in the same transaction.

Therefore 41K.1 implementation must specify how to enumerate N prior Ed25519 precompile instructions from the real Instructions sysvar.

For each prior Ed25519 precompile instruction, the implementation must preserve the same Model A requirements:

- instruction is loaded from the real Instructions sysvar;
- instruction is strictly prior to the current instruction;
- instruction program id is the real Ed25519 precompile program id;
- instruction entry is not fabricated;
- instruction data is not caller-provided;
- instruction data is not watcher/frontend-provided;
- signature, public key, and message byte ranges flow into the accepted 41F.1 / 41F.2 boundaries.

Model A applies per prior precompile instruction.

All prior Ed25519 precompile instructions used for quorum must be verified independently before their guardian attempts can feed 41I.

### Instructions Sysvar Identity and Checked API Rule

41K.1 implementation must verify the identity of the Instructions sysvar account before trusting its data.

Required:

- the Instructions sysvar account key must equal the canonical `instructions::id()`;
- arbitrary accounts pretending to be the Instructions sysvar must be rejected;
- current instruction index must be loaded through a checked runtime path equivalent to `load_current_index_checked`;
- prior instructions must be loaded through a checked runtime path equivalent to `load_instruction_at_checked`;
- checked loading must fail closed on malformed sysvar data, missing current index, out-of-bounds indexes, or invalid prior instruction indexes.

The current instruction index must never be caller-provided.

## Current Plan Status

This is a plan document only.

No 41K.1 code is implemented.

No runtime sysvar loading is enabled.

No execution or mutation surface is enabled.

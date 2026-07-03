# XXXL Phase 41K.1 — Instructions Sysvar Loading Plan Acceptance

Date: 2026-07-03

Status: accepted plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-1-instructions-sysvar-plan`

Accepted commits:

- `6b41d47 Document phase 41K.1 instructions sysvar loading plan`
- `50536c0 Address phase 41K.1 instructions sysvar plan review notes`

Base main:

`bd53ace Merge XXXL phase 41K live-wiring plan acceptance`

## Final Verdict

Phase 41K.1 real Instructions sysvar loading plan is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Sufficient before 41K.1 code: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Sufficient before 41K.1 code: yes

## Notes Addressed

Demon notes were incorporated into the plan:

- 41K.1 code must support enumeration of N prior Ed25519 precompile instructions for quorum.
- Model A applies per prior Ed25519 precompile instruction.
- Each prior precompile must be strictly prior, Ed25519-program-id checked, and non-fabricated.
- The Instructions sysvar account identity must be checked against canonical `instructions::id()`.
- Current index must be loaded through a checked runtime path equivalent to `load_current_index_checked`.
- Prior instructions must be loaded through a checked runtime path equivalent to `load_instruction_at_checked`.

## Accepted 41K.1 Scope

41K.1 plans only real Instructions sysvar loading.

It replaces the boundary-model assumption:

`preloaded prior instruction data`

with:

`real Instructions sysvar -> checked current instruction index -> N real prior Ed25519 precompile instructions -> accepted 41F.1 / 41F.2 pipeline`

## Still Out of Scope

41K.1 does not enable:

- real guardian-set PDA loading;
- real processed-registry PDA loading;
- replay registry write;
- processed event marking;
- account mutation;
- CPI;
- SPL token mint;
- handler;
- live route.

## Future Gate

41K.1 code implementation must be separately reviewed before merge.

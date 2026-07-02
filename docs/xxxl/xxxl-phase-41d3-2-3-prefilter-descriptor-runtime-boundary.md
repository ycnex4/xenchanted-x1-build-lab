# XXXL Phase 41D3.2.3 — Prefilter + Phase 41C3 Candidate Descriptor Runtime Boundary

Date: 2026-07-02

## Status

Code boundary implemented.

## Parent Gate

Parent accepted checkpoint:

`c6bbf72 Merge XXXL phase 41D3 prefilter descriptor plan acceptance record`

Phase 41D3.2.3.0 accepted this code boundary:

- consume loaded prior instructions from Phase 41D3.2.2;
- process only loaded entries marked runtime-data-only;
- prefilter unrelated loaded prior instructions;
- identify Ed25519 program-id candidates structurally;
- construct Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- keep descriptors non-authorizing;
- allow `locates_prior_ed25519_instruction: true` only as structural candidate location.

## Scope

Phase 41D3.2.3 introduces structural prefiltering and descriptor construction only.

Allowed:

- consume Phase 41D3.2.2 loaded prior instructions;
- iterate loaded entries by reference;
- discard unrelated non-candidates immediately;
- match only Ed25519 program id structurally;
- construct Phase 41C3 candidate descriptors;
- delegate ordering, duplicate, ambiguous, same/later cases to Phase 41C3;
- keep descriptors non-authorizing;
- flip `locates_prior_ed25519_instruction: true`.

Not allowed:

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
- live route unlock.

## Descriptor Meaning

A Phase 41D3.2.3 descriptor means only:

- a loaded prior instruction had the Ed25519 program id;
- its instruction index was recorded;
- a Phase 41C3 structural candidate descriptor was constructed.

It does not mean:

- signature is valid;
- instruction data is proof;
- guardian key is valid;
- quorum is reached;
- message hash is accepted;
- mint may execute;
- replay registry may update.

## Phase 41C3 Delegation

Phase 41D3.2.3 does not reimplement duplicate, ambiguous, or ordering logic.

It delegates candidate descriptor evaluation to:

`locate_prior_ed25519_lookup_ordering_boundary`

Phase 41C3 remains authoritative for:

- duplicate candidate handling;
- ambiguous candidate handling;
- same/later candidate ordering rejection;
- located-and-ordered structural candidate result.

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

## Validation Targets

Expected targeted test:

- `cargo test prefilter_phase_41c3_candidate_descriptor_runtime_boundary --lib`

Expected broad tests:

- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

## Next Step

Do not start any cryptographic verification, evidence acceptance, quorum counting, authorization, replay, mutation, CPI, mint, handler, or live route phase until Phase 41D3.2.3 is externally reviewed and accepted.

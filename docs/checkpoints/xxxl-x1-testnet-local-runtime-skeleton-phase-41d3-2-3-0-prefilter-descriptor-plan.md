# XXXL X1 Testnet Local Runtime Skeleton — Phase 41D3.2.3.0 Checkpoint

Date: 2026-07-02

## Phase

Phase 41D3.2.3.0 — prefilter + Phase 41C3 candidate descriptor plan.

## Type

Docs-only planning checkpoint.

## Parent Checkpoint

`0cb2478 Merge XXXL phase 41D3 checked prior loading acceptance record`

## Files Added

- `docs/xxxl/xxxl-phase-41d3-2-3-0-prefilter-descriptor-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-2-3-0-prefilter-descriptor-plan.md`
- `docs/reviews/xxxl-phase-41d3-2-3-0-prefilter-descriptor-plan-review-request.md`

## Files Modified

- `docs/checkpoints/current-design-checkpoint.md`

## Runtime Code

No runtime code changed.

## Scope

This phase documents the intended boundary for the next runtime code sub-step:

- consume loaded prior instructions from Phase 41D3.2.2;
- prefilter unrelated instructions;
- identify Ed25519 program-id candidates structurally;
- construct non-authorizing Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- allow `locates_prior_ed25519_instruction: true` only as structural candidate location.

## Still Deferred

- code implementation;
- cryptographic Ed25519 verification;
- signature proof acceptance;
- verification evidence acceptance;
- guardian quorum counting;
- authorization;
- replay writes;
- account mutation;
- CPI;
- mint;
- handler;
- live route.

## Review Requirement

External review must accept this plan before Phase 41D3.2.3 code starts.

# XXXL X1 Testnet Local Runtime Skeleton — Phase 41D3.2.2.0 Checkpoint

Date: 2026-07-02

## Phase

Phase 41D3.2.2.0 — checked prior instruction loading plan.

## Type

Docs-only planning checkpoint.

## Parent Checkpoint

`9880d63 Merge XXXL phase 41D3 prior index range acceptance record`

## Files Added

- `docs/xxxl/xxxl-phase-41d3-2-2-0-checked-prior-instruction-loading-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-2-2-0-checked-loading-plan.md`
- `docs/reviews/xxxl-phase-41d3-2-2-0-checked-loading-plan-review-request.md`

## Files Modified

- `docs/checkpoints/current-design-checkpoint.md`

## Runtime Code

No runtime code changed.

## Scope

This phase documents the intended boundary for the next runtime code sub-step:

- accept bounded prior range from Phase 41D3.2.1;
- iterate prior indexes lazily;
- use `load_instruction_at_checked` only;
- deterministically map checked loading success/failure;
- do not prefilter;
- do not build descriptors;
- do not accept evidence;
- do not authorize;
- do not mutate runtime state.

## Still Deferred

- code implementation;
- checked instruction loading;
- prefiltering;
- Phase 41C3 descriptors;
- explicit same/later reject path;
- cryptographic verification;
- evidence acceptance;
- authorization;
- replay writes;
- CPI;
- mint;
- handler;
- live route.

## Review Requirement

External review must accept this plan before Phase 41D3.2.2 code starts.

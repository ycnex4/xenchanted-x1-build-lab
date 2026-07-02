# XXXL X1 Testnet Local Runtime Skeleton — Phase 41D3.2.0 Checkpoint

Date: 2026-07-02

## Phase

Phase 41D3.2.0 — prior enumeration / checked loading plan.

## Type

Docs-only planning checkpoint.

## Parent Checkpoint

`e52d8ac Merge XXXL phase 41D3 current index external acceptance record`

## Files Added

- `docs/xxxl/xxxl-phase-41d3-2-0-prior-enumeration-loading-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-2-0-prior-enumeration-loading-plan.md`
- `docs/reviews/xxxl-phase-41d3-2-0-prior-enumeration-loading-plan-review-request.md`

## Files Modified

- `docs/checkpoints/current-design-checkpoint.md`

## Runtime Code

No runtime code changed.

## Scope

This phase documents the intended split for the next runtime sub-step:

- Phase 41D3.2.1: prior index range enumeration only;
- Phase 41D3.2.2: checked prior instruction loading;
- Phase 41D3.2.3: prefilter + Phase 41C3 descriptors + explicit same/later reject.

## Still Deferred

- code implementation;
- prior enumeration;
- checked instruction loading;
- descriptor construction;
- cryptographic verification;
- evidence acceptance;
- authorization;
- replay writes;
- CPI;
- mint;
- handler;
- live route.

## Review Requirement

External review must accept this plan before 41D3.2.1 code starts.

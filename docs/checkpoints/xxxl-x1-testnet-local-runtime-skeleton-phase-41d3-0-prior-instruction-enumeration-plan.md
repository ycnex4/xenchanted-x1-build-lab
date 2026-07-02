# XXXL X1 Testnet Local Runtime Skeleton Phase 41D3.0 Prior Instruction Enumeration Plan

Status: Docs-only safety checkpoint.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-0-prior-instruction-enumeration-plan`

## Purpose

This checkpoint opens Phase 41D3 planning after Phase 41D2 external acceptance.

Phase 41D3.0 does not add runtime code.

It defines the minimum safe boundary for the next code phase.

## Background

Phase 41D2 closed the current-instruction identity runtime boundary.

External review gate:

- Demon: `ACCEPT`
- Theo: `ACCEPT`
- required fixes: none
- blocking risks: none

Therefore, Phase 41D3 may start.

## Intended Next Code Phase

The next code phase should be Phase 41D3.

Its intended scope:

- real prior-instruction enumeration via Instructions sysvar
- checked instruction loading
- prefiltering unrelated instructions
- Phase 41C3 candidate descriptor construction
- explicit same/later fully-matching Ed25519 anomaly decision

## Stop Line

Phase 41D3 must stop before:

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
- runtime handler enablement
- deployment readiness

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-41d3-0-prior-instruction-enumeration-runtime-boundary-plan.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41d3-0-prior-instruction-enumeration-plan.md`
- `docs/reviews/xxxl-phase-41d3-0-prior-instruction-enumeration-plan-review-request.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust code is changed.

No Cargo manifest is changed.

No package manifest is changed.

No dependency is added.

No deploy artifact is touched.

## Gate

Before Phase 41D3 code begins, this plan should be used as the code boundary.

If reviewers request changes to this plan, they must be made before code.

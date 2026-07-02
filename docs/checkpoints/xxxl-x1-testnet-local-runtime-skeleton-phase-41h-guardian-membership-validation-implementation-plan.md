# XXXL X1 Testnet Local Runtime Skeleton — Phase 41H Guardian Membership Validation Implementation Plan Checkpoint

Date: 2026-07-03

## Phase

Phase 41H — Guardian Membership Validation Implementation Plan.

## Parent Gate

`0fa2220 Merge XXXL phase 41H guardian membership validation plan acceptance`

## Scope

Docs-only implementation plan.

No runtime code.

No `.rs` changes.

No quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Plan the narrow code boundary for:

`verified_signer_public_key ∈ authoritative_guardian_set`

## Mandatory Provenance Rule

Both membership operands require trusted provenance.

Signer public key:

- must come from Phase 41F.1 extracted Ed25519 public key bytes;
- must be bound to Phase 41F.2 native SVM Ed25519 verification.

Guardian set:

- must come from authoritative program-controlled / on-chain source;
- must not come from caller instruction data.

## Next Gate

External review of the 41H implementation plan.

No `.rs` implementation may begin until this implementation plan is reviewed and accepted.

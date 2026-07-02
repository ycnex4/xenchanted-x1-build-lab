# XXXL X1 Testnet Local Runtime Skeleton — Phase 41G.1 Payload Evidence Shape Plan Checkpoint

Date: 2026-07-03

## Phase

Phase 41G.1 — Payload Evidence Shape Plan.

## Parent Gate

`b4ff536 Merge XXXL phase 41G payload binding plan acceptance`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

No hash comparison.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Define the shape of candidate payload evidence that a later Phase 41G.2 binding step may consume.

## Authoritative Sources

- `programs/xxxl-svm/src/verifier/raw_payload.rs`
- `RAW_PAYLOAD_PHASE_23_FIELD_ORDER`
- `DecodedGuardianPayloadRaw<'a>`
- `programs/xxxl-svm/src/verifier/canonical_payload.rs`

## Boundary

41G.1 may define payload evidence shape.

41G.1 must not accept proof, validate guardian membership, count quorum, authorize minting, mark replay, mutate state, CPI, mint, handle live instruction flow, or unlock live route.

## Next Gate

After external acceptance, Phase 41G.2 payload hash binding may be planned separately.

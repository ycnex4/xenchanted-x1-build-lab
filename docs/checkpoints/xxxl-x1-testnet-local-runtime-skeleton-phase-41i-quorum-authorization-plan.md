# XXXL X1 Testnet Local Runtime Skeleton — Phase 41I Quorum Authorization Plan Checkpoint

Date: 2026-07-03

Status: planning only

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-plan`

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Primary plan:

`docs/xxxl/xxxl-phase-41i-quorum-authorization-plan.md`

## Purpose

This checkpoint records the start of Phase 41I.

Phase 41I is scoped to quorum counting and threshold authorization planning.

41I follows the accepted 41H.1 hardening, where decoded payload fields are derived only from the same raw payload bytes that were hash-bound by 41G.

## Boundary

41I must count only successful 41H guardian membership validations.

41I must not count caller-provided approval claims.

41I must not accept a free decoded payload.

41I must not accept a free payload-binding marker.

Preferred model:

41I composes 41H internally for each verification attempt, using the same raw payload bytes, signed message bytes, expected configured guardian set ID, and authoritative guardian set wrapper.

## Still Forbidden

41I planning does not introduce:

- replay writes;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- instruction handler;
- live route.

## Active Blockers

Active blockers remain unchanged:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Step

Request Theo and Audit Demon review for the 41I planning document before writing any 41I code.

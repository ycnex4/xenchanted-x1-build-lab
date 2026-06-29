# XXXL X1 Testnet Blocker Model Transition

Status: Completed
Branch: `stage-xxxl-x1-testnet-blocker-model-transition`
Base: `cf77671 Assess X1 testnet blocker transition after deployment`

## Purpose

This document records the dedicated blocker model transition after X1 testnet deployment.

This stage updates the current blocker model to distinguish:

- X1 testnet Program ID deployment status
- production Program ID readiness
- runtime live route readiness
- SPL CPI readiness
- guardian / proof-log / external-review readiness

This is a docs-only transition.

This stage does not change runtime code.

This stage does not execute an upgrade.

This stage does not submit a transaction.

This stage does not spend SOL.

This stage does not enable live gateway execution.

## Evidence Base

Deployment execution evidence:

- `6bda36d`

Post-deploy read-only verification:

- `7c9e777`

Blocker transition assessment:

- `cf77671`

X1 testnet Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Upgrade authority after deployment:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Gateway mint authority bump:

- `252`

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

## Retired Blanket Blocker

The blanket X1 testnet blocker name is retired:

- `PLACEHOLDER_PROGRAM_ID`

Reason:

- a real X1 testnet Program ID has been selected
- X1 testnet PDA fixtures were regenerated from that Program ID
- X1 testnet PDA fixtures were verified
- the runtime scaffold was deployed to X1 testnet
- post-deploy read-only verification confirmed the deployed program account

This retirement is scoped to the X1 testnet Program ID path.

It does not imply production Program ID readiness.

It does not imply live gateway readiness.

## New X1 Testnet Status

New X1 testnet status marker:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Meaning:

- the X1 testnet Program ID exists on-chain
- the deployed program is readable
- ProgramData and authority are observable
- the runtime remains scaffold-only
- the runtime remains locked
- the runtime is not a live gateway

This is a status marker, not a live capability.

## New Production Blocker

New production blocker:

- `PRODUCTION_PROGRAM_ID_UNSET`

Meaning:

- production Program ID remains separate from X1 testnet Program ID
- production PDA fixtures remain separate
- production authority model remains separate
- production release readiness remains separate

This keeps network identity explicit.

## Active Blockers After This Transition

Current active blockers:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Current X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Retired X1 testnet blanket blocker:

- `PLACEHOLDER_PROGRAM_ID`

## Runtime Status

The runtime remains scaffold-only.

The runtime remains locked.

No live route is enabled.

No SPL CPI execution is enabled.

No `invoke_signed` path is enabled.

No SPL Token `mint_to` path is enabled.

No production guardian set is configured.

No production proof log is configured.

External review remains incomplete.

## Updated Documents

This stage updates the current blocker/status docs:

- `docs/xxxl/xxxl-remaining-deployment-blockers-inventory.md`
- `docs/xxxl/xxxl-runtime-deployment-blocker-descriptions.md`
- `docs/xxxl/xxxl-runtime-deployment-blocker-resolution-guidance.md`
- `docs/xxxl/xxxl-runtime-deployment-status-report-boundary.md`
- `docs/xxxl/xxxl-runtime-predeploy-evidence-matrix.md`
- `docs/xxxl/xxxl-runtime-predeploy-readiness-checklist.md`

This stage also records:

- `docs/xxxl/xxxl-x1-testnet-blocker-model-transition.md`
- `docs/checkpoints/xxxl-x1-testnet-blocker-model-transition.md`

## Result

The blocker model is now network-aware.

X1 testnet deployment status is separated from production Program ID readiness.

Runtime blockers remain active.

No live gateway path is enabled.

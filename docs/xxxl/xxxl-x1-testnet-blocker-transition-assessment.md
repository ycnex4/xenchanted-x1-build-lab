# XXXL X1 Testnet Blocker Transition Assessment

Status: Completed
Branch: `stage-xxxl-x1-testnet-blocker-transition-assessment`
Base: `7c9e777 Add X1 testnet post-deploy read-only verification`

## Purpose

This document assesses how the blocker model should be interpreted after the first real X1 testnet deployment and the post-deploy read-only verification.

This is an assessment stage only.

This stage does not remove any blocker.

This stage does not transition any blocker.

This stage does not change runtime code.

This stage does not execute an upgrade.

This stage does not submit a transaction.

This stage does not spend SOL.

This stage does not enable the live gateway.

## Existing Evidence

Deployment execution evidence commit:

- `6bda36d Add X1 testnet deployment execution evidence`

Post-deploy read-only verification commit:

- `7c9e777 Add X1 testnet post-deploy read-only verification`

The deployed X1 testnet Program ID is:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Upgrade authority after deployment:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Gateway mint authority PDA fixture:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Gateway mint authority bump:

- `252`

Artifact:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

Artifact size:

- `38584 bytes`

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

## What The Deployment Changed

The X1 testnet now has a deployed XXXL SVM program account at the selected Program ID.

The Program ID is no longer merely an off-chain candidate for X1 testnet.

The Program ID is now observable on X1 testnet.

The deployed program data length matches the build artifact evidence.

The ProgramData address and upgrade authority are observable on X1 testnet.

## What The Deployment Did Not Change

The deployment did not enable a live gateway route.

The deployment did not enable SPL CPI execution.

The deployment did not enable `invoke_signed`.

The deployment did not enable SPL Token `mint_to`.

The deployment did not configure a production guardian set.

The deployment did not configure a production proof log.

The deployment did not complete external review.

The deployment did not freeze authority.

The deployment did not establish production-final immutability.

The runtime remains scaffold-only and locked.

## Assessment: `PLACEHOLDER_PROGRAM_ID`

The blocker name `PLACEHOLDER_PROGRAM_ID` is no longer fully accurate as a blanket statement for X1 testnet.

Reason:

- a real X1 testnet Program ID has been selected
- the corresponding PDA fixture was generated and verified
- the program was deployed to X1 testnet
- post-deploy read-only verification confirmed the deployed program account

However, this assessment does not remove or transition the blocker.

Reason:

- the current blocker model still needs a dedicated transition stage
- production/mainnet Program ID status remains separate from X1 testnet
- runtime remains locked and not live
- live route and SPL CPI blockers remain active
- the deployed Program ID alone does not imply gateway readiness

## Recommended Future Blocker Model

A future dedicated blocker transition stage should replace the overly broad `PLACEHOLDER_PROGRAM_ID` blocker with more precise network-aware status.

Recommended direction:

- retire `PLACEHOLDER_PROGRAM_ID` as a blanket X1 testnet blocker
- introduce or record `X1_TESTNET_PROGRAM_DEPLOYED`
- keep a separate production/mainnet blocker such as `PRODUCTION_PROGRAM_ID_UNSET`
- keep runtime blockers independent from Program ID blockers

Possible future active blocker set after a dedicated transition:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Optional non-blocker status flag:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

This optional status flag should not be confused with live gateway readiness.

## Assessment: Runtime Blockers

The following blockers remain fully valid:

- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Reason:

- no live route was enabled
- no SPL CPI execution was enabled
- no production guardian set was configured
- no production proof log was configured
- no external review was completed

## Assessment: Authority

Upgrade authority remains present after testnet deployment.

This is expected for the current X1 testnet phase.

This is not production-final immutability.

Authority lifecycle refinement is a separate future stage.

Final authority freeze remains gated by the existing authority freeze procedure model, including completion, review, testing, and documentation of deterministic X1-native mechanics.

## Current Stage Decision

This stage records the assessment only.

Current active blockers remain unchanged:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

A future dedicated stage may transition `PLACEHOLDER_PROGRAM_ID` into a more precise network-aware blocker/status model.

## Result

The blocker model has been assessed after the first X1 testnet deployment.

The correct next move is not to blindly remove `PLACEHOLDER_PROGRAM_ID`.

The correct next move is to perform a dedicated blocker model transition that separates:

- testnet Program ID deployment status
- production Program ID readiness
- runtime live route readiness
- SPL CPI readiness
- guardian/proof-log/external-review readiness

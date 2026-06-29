# XXXL X1 Testnet Deployment Execution Evidence

Status: Completed
Branch: `stage-xxxl-x1-testnet-deployment-execution-evidence`
Base: `2b7bf03 Add X1 testnet final deployment checklist`

## Purpose

This document records the first real X1 testnet deployment execution evidence for the XXXL SVM runtime scaffold.

This is deployment evidence.

This is not live gateway evidence.

This is not production-final immutability evidence.

The deployed runtime remains scaffold-only and locked.

## Deployment Summary

Deployment was executed on X1 testnet.

The deployment submitted a transaction.

The deployment spent testnet SOL.

The deployment created the on-chain program account for the selected XXXL X1 testnet Program ID.

## Deployment Result Flags

- `RPC_USED=true`
- `DEPLOYED=true`
- `TRANSACTION_SUBMITTED=true`
- `SOL_SPENT=true`

## Network

RPC:

- `https://rpc.testnet.x1.xyz`

Cluster version observed before deployment:

- `3.1.14`

## Program Identity

Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Gateway mint authority bump:

- `252`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Owner:

- `BPFLoaderUpgradeab1e11111111111111111111111`

Upgrade authority after deployment:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Fee payer:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

## Deployment Transaction

Signature:

- `5Ko88Gyduc2KWnA4BjGziTyD7UPYBV4N6dvHbGR8HVwj4V2885HwdfYtdi7kDC1bUoqfkWRrZenk29G3J447Vvtf`

UTC start:

- `2026-06-29T19:40:37Z`

UTC end:

- `2026-06-29T19:40:41Z`

Last deployed slot:

- `169365249`

## Artifact Evidence

Artifact path:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

Artifact size:

- `38584 bytes`

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

Program data length after deployment:

- `38584 (0x96b8) bytes`

## SOL Accounting

Fee payer balance before deployment:

- `24.79720708 SOL`

Fee payer balance after deployment:

- `24.52534222 SOL`

Estimated SOL spent:

- `0.27186486 SOL`

Program account balance after deployment:

- `0.26974872 SOL`

Interpretation:

- most of the estimated SOL spend is held as program/account balance
- the smaller remaining difference is deployment transaction / loader / service cost
- this is testnet SOL accounting only

## Deployment Log

Local deployment log path:

- `/tmp/xxxl-x1-testnet-deploy-20260629T194036Z.log`

The local log is not committed to the repository.

The repository records only public deployment evidence.

No private key, seed phrase, keypair content, mnemonic, or `.env` value is recorded.

## Authority Status

Upgrade authority is still present after the X1 testnet deployment.

This is expected for the current testnet phase.

This is not production-final immutability.

The authority is not the XXXL mint authority.

The authority does not mean the live gateway is enabled.

The final authority freeze remains a later lifecycle step.

Per the existing authority freeze procedure model, final freeze must not happen before deterministic X1-native mechanics are complete, reviewed, and documented.

## Runtime Status After Deployment

The program exists on X1 testnet.

The runtime remains scaffold-only.

The runtime remains locked.

The runtime is not a live mint gateway.

The runtime is not production final.

No live route was enabled.

No SPL CPI execution was enabled.

No `invoke_signed` path was enabled.

No SPL Token `mint_to` path was enabled.

## Blocker Status

This stage does not remove any blocker.

This stage does not transition any blocker.

`PLACEHOLDER_PROGRAM_ID` remains active until a dedicated blocker transition assessment explicitly changes the blocker model.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## What Changed

Changed:

- X1 testnet now has a deployed XXXL SVM program account at the selected Program ID.
- The repository now records deployment execution evidence.

Not changed:

- no runtime source code changed
- no Cargo files changed
- no keypair file committed
- no `.so` artifact committed
- no secret material recorded
- no blocker removed
- no live route enabled
- no mint authority migration completed
- no production guardian set configured
- no production proof log configured
- no external review completed

## Result

The first XXXL SVM runtime scaffold deployment to X1 testnet succeeded.

The deployment created the on-chain program account.

The artifact size matches the local build evidence.

The selected Program ID matches the deployed program.

The runtime remains scaffold-only, locked, unreleasable, and not a live gateway.

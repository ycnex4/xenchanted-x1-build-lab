# XXXL X1 Testnet Final Deployment Checklist

Status: Completed
Branch: `stage-xxxl-x1-testnet-final-deployment-checklist`
Base: `27685a1 Add X1 testnet build execution evidence`

## Purpose

This document records the final local checklist before a future X1 testnet deployment execution stage.

This is a checklist only.

This is not deployment evidence.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material is recorded.

No blocker is removed.

## Confirmed Deployment Inputs

Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Gateway mint authority bump:

- `252`

Local ignored program keypair:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

Build artifact:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

Artifact size:

- `38584 bytes`

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

Planned X1 testnet RPC:

- `https://rpc.testnet.x1.xyz`

## Final Checklist Before Real Deploy

Before any future deploy command is executed, confirm again:

- working tree is clean
- `main` equals `origin/main`
- local program keypair exists
- local program keypair remains ignored
- local public key equals selected Program ID
- artifact exists locally
- artifact size equals `38584 bytes`
- artifact SHA-256 equals `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`
- no keypair, secret, `.env`, `.so`, or `target/deploy` file is staged
- deploy command uses the selected program keypair
- deploy command uses the planned X1 testnet RPC
- deployment output Program ID equals `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

## Forbidden In This Checklist Stage

This stage did not execute:

- `solana program deploy`
- RPC deployment commands
- transaction submission
- SOL transfer
- keypair content printing
- secret file staging
- artifact staging
- blocker transition

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`PLACEHOLDER_PROGRAM_ID` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Result

The final local deployment checklist was recorded.

The Program ID was confirmed.

The ignored keypair public key was confirmed.

The build artifact path, size, and SHA-256 were confirmed.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material was recorded.

No blocker is removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

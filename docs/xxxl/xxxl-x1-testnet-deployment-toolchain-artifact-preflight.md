# XXXL X1 Testnet Deployment Toolchain Artifact Preflight

Status: Completed
Branch: `stage-xxxl-x1-testnet-deployment-toolchain-artifact-preflight`
Base: `8b8259f Add X1 testnet deployment command plan`

## Purpose

This document records a local toolchain and artifact-path preflight for a future X1 testnet deployment execution stage.

This is a local preflight record only.

This is not a build execution record.

This is not a deployment record.

This is not execution evidence.

This is not a mainnet readiness record.

This is not a production readiness record.

No build was executed by this stage.

No RPC was used by this stage.

No program was deployed by this stage.

No transaction was submitted by this stage.

No SOL was spent by this stage.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Selected X1 Testnet Program ID

Selected public X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

The local keypair file exists locally.

The local keypair is ignored through `.git/info/exclude`.

The local public key was checked and matches the selected Program ID candidate.

The keypair contents are not recorded.

The keypair contents are not committed.

## Verified PDA Fixture

PDA name:

- `gateway_mint_authority`

PDA derivation kind:

- `GatewayMintAuthority`

Seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Verified PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Verified bump:

- `252`

## Planned Network

Planned X1 testnet RPC:

- `https://rpc.testnet.x1.xyz`

The RPC endpoint must be re-confirmed before actual deployment execution.

This stage does not use the RPC endpoint.

## Local Toolchain Snapshot

The following local toolchain values were observed:

- `solana --version`: `solana-cli 4.0.0 (src:8de42dc0; feat:dda54cf7, client:Agave)`
- `cargo --version`: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `rustc --version`: `rustc 1.95.0 (59807616e 2026-04-14)`
- `rustup --version`: `rustup 1.29.0 (28d1352db 2026-03-05)`
- `cargo build-sbf --version`: `cargo-build-sbf 4.0.0 / platform-tools v1.53 / rustc 1.89.0`
- `cargo build-bpf --version`: `MISSING or failed: error: no such command: `build-bpf` /  / help: a command with a similar name exists: `build-sbf``

This snapshot is informational.

`cargo build-sbf` is the planned build command family.

`cargo build-bpf` is not required for the planned path.

Toolchain compatibility must still be confirmed during the future build/deployment execution stage.

## Program Build Input

Program manifest:

- `programs/xxxl-svm/Cargo.toml`

Expected crate/package name:

- `xxxl-svm`

Expected crate type:

- `cdylib`
- `lib`

## Planned Build Command

The planned future build command is:

- `cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml`

This command is recorded here as a plan only.

It was not executed in this stage.

## Expected Artifact Paths

Expected artifact path candidates after a future build:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`
- `target/deploy/xxxl_svm.so`

The future build execution stage must record the actual produced artifact path.

The future build execution stage must record the produced artifact hash.

The future deployment execution stage must deploy only the verified produced artifact.

## Test Evidence

Before recording this preflight, default Rust library tests were run locally.

Default Rust library test result:

- `201 passed`
- `0 failed`
- `1 ignored`

## Commands Forbidden In This Stage

This stage did not execute:

- `cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml`
- `cargo build-bpf`
- `solana program deploy`
- any command that sends a transaction
- any command that contacts RPC for deployment
- any command that transfers SOL
- any command that prints keypair contents
- any command that commits `.local-keys`
- any command that commits `.env`
- any command that commits a private key, seed phrase, mnemonic, or signer secret

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

## Important Boundary

A future successful local build does not by itself prove deployment readiness.

A future successful X1 testnet deployment does not by itself enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure
- production release readiness

## Required Future Work

Future stages must still record:

1. X1 testnet build execution evidence
2. produced artifact path and hash
3. X1 testnet deployment execution evidence
4. X1 testnet deployed Program ID verification evidence
5. X1 testnet SPL mint authority transfer plan
6. X1 testnet SPL mint authority verification evidence
7. Program ID readiness model update
8. placeholder Program ID blocker transition

The blocker transition must happen only after the required evidence exists.

## Result

The X1 testnet deployment toolchain/artifact preflight was recorded.

The selected Program ID was re-confirmed against the local public key.

The keypair remains ignored.

The local toolchain snapshot was recorded.

The planned build command was recorded.

Expected artifact paths were recorded.

Default Rust library tests pass.

No build was executed.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material was recorded.

No blocker is removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

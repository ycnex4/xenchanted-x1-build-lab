# XXXL X1 Testnet Build Execution Evidence

Status: Completed
Branch: `stage-xxxl-x1-testnet-build-execution-evidence`
Base: `2b3018b Add X1 testnet toolchain artifact preflight`

## Purpose

This document records local SBF build execution evidence for `programs/xxxl-svm`.

This is local build execution evidence only.

This is not deployment evidence.

This is not RPC evidence.

This is not transaction evidence.

This is not SOL spend evidence.

This is not release readiness evidence.

This is not production readiness evidence.

No program was deployed.

No RPC was used.

No transaction was submitted.

No SOL was spent.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Selected X1 Testnet Program ID

Selected public X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

Pre-build keypair safety checks:

- local keypair file exists
- `.local-keys/` is ignored through `.git/info/exclude`
- `solana-keygen pubkey .local-keys/xxxl-x1-testnet-program-keypair.json` returned `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- the local public key matches the selected Program ID candidate
- no keypair contents were printed
- no keypair or secret-looking files were tracked
- no keypair or secret-looking files were untracked

## Verified PDA Fixture

PDA name:

- `gateway_mint_authority`

Verified PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Verified bump:

- `252`

## Local Git Base

Build evidence was recorded from:

- `HEAD`: `2b3018b Add X1 testnet toolchain artifact preflight`
- `main`: `2b3018b`
- `origin/main`: `2b3018b`
- `FETCH_HEAD`: `2b3018b`

The working tree was clean before build execution.

## Local Toolchain Snapshot

The following local toolchain values were observed:

- `solana --version`: `solana-cli 4.0.0 (src:8de42dc0; feat:dda54cf7, client:Agave)`
- `cargo --version`: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `rustc --version`: `rustc 1.95.0 (59807616e 2026-04-14)`
- `rustup --version`: `rustup 1.29.0 (28d1352db 2026-03-05)`
- `cargo build-sbf --version`: `cargo-build-sbf 4.0.0 / platform-tools v1.53 / rustc 1.89.0`

## Test Evidence

Before the SBF build, default Rust library tests were run locally from `programs/xxxl-svm`.

Command:

- `cargo test --lib`

Result:

- `201 passed`
- `0 failed`
- `1 ignored`

## Build Execution

Build command:

- `cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml`

Build result:

- success
- `Finished release profile [optimized] target(s) in 2.52s`

Build execution flags:

- `BUILD_EXECUTED=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `TRANSACTION_SUBMITTED=false`
- `SOL_SPENT=false`

## Produced Artifact Evidence

Expected artifact path candidates:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`
- `target/deploy/xxxl_svm.so`

Actual produced artifact path:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

Artifact size:

- `38584` bytes

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

Artifact local listing:

- `-rwxrwxrwx 1 sergey sergey 38K Jun 29 18:19 programs/xxxl-svm/target/deploy/xxxl_svm.so`

The produced `.so` artifact is local build output only.

The produced `.so` artifact is not committed by this stage.

The produced `.so` artifact is not staged by this stage.

## Commands Not Executed

This stage did not execute:

- `solana program deploy`
- RPC deployment commands
- transaction submission commands
- SOL transfer commands
- commands that print keypair contents
- commands that stage `.local-keys`
- commands that stage `.env`
- commands that stage private keys, seed phrases, mnemonics, signer secrets, or wallet files
- blocker transition commands

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

## Safety Boundary

This stage does not change Rust source, Cargo files, tests, Program ID constants, PDA derivation logic, deployment status logic, safety invariant logic, live route execution, SPL CPI execution, `invoke_signed`, SPL Token `mint_to`, guardian production configuration, proof-log production configuration, external review status, or release readiness.

This successful local build does not by itself prove deployment readiness.

This successful local build does not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure
- production release readiness

## Result

The local SBF build was executed successfully.

The produced artifact path was recorded.

The produced artifact size was recorded.

The produced artifact SHA-256 hash was recorded.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material was recorded.

No blocker was removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

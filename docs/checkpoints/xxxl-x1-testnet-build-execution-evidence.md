# XXXL X1 Testnet Build Execution Evidence Checkpoint

Status: Completed
Branch: `stage-xxxl-x1-testnet-build-execution-evidence`
Base: `2b3018b Add X1 testnet toolchain artifact preflight`

## Summary

This checkpoint records local SBF build execution evidence for `programs/xxxl-svm`.

This checkpoint is docs-only.

The build was executed locally.

The produced artifact path, size, and SHA-256 hash were recorded.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material is recorded.

No deployment blocker is removed.

## Files Changed

Expected changed files:

- `docs/xxxl/xxxl-x1-testnet-build-execution-evidence.md`
- `docs/checkpoints/xxxl-x1-testnet-build-execution-evidence.md`
- `docs/checkpoints/current-design-checkpoint.md`

No Rust source changes are expected.

No Cargo changes are expected.

No keypair files are expected.

No `.so` artifact is expected to be staged or committed.

## Planned Deployment Identity

Selected X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Verified PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Verified bump:

- `252`

Local keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

Pre-build keypair safety checks:

- local keypair exists
- `.local-keys/` is ignored through `.git/info/exclude`
- local public key equals `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- no keypair contents were printed
- no keypair or secret-looking files were tracked or untracked

## Local Git Base

- `HEAD`: `2b3018b Add X1 testnet toolchain artifact preflight`
- `main`: `2b3018b`
- `origin/main`: `2b3018b`
- `FETCH_HEAD`: `2b3018b`

The working tree was clean before build execution.

## Local Toolchain Snapshot

- `solana --version`: `solana-cli 4.0.0 (src:8de42dc0; feat:dda54cf7, client:Agave)`
- `cargo --version`: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `rustc --version`: `rustc 1.95.0 (59807616e 2026-04-14)`
- `rustup --version`: `rustup 1.29.0 (28d1352db 2026-03-05)`
- `cargo build-sbf --version`: `cargo-build-sbf 4.0.0 / platform-tools v1.53 / rustc 1.89.0`

## Test Evidence

Default Rust library test command:

- `cargo test --lib`

Default Rust library test result:

- `201 passed`
- `0 failed`
- `1 ignored`

## Build Evidence

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

## Produced Artifact

Actual produced artifact path:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so`

Artifact size:

- `38584` bytes

Artifact SHA-256:

- `fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e`

The produced `.so` artifact is local build output only.

The produced `.so` artifact is not staged or committed by this stage.

## Forbidden In This Stage

This build-evidence stage did not execute:

- `solana program deploy`
- RPC deployment commands
- transaction submission
- SOL transfer
- keypair content printing
- secret file staging
- `.so` artifact staging
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

The local SBF build execution evidence was recorded.

The produced artifact path, size, and SHA-256 hash were recorded.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material was recorded.

No blocker was removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

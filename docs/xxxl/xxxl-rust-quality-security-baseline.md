# XXXL Rust Quality/Security Baseline

Status: BASELINE_CAPTURED_WITH_SOLANA_DEPENDENCY_AUDIT_BLOCKER.

This stage introduces the first Rust quality/security baseline for the XXXL SVM program.

## Checks

Hard checks attempted:

- `cargo fmt --check`
- `cargo test`
- `cargo audit`
- `cargo deny check`

Report-only:

- `cargo geiger`

## Results

### cargo fmt

`cargo fmt --check` initially failed because several Rust files were not formatted according to rustfmt.

Resolution in this stage:

- applied `cargo fmt`
- kept the formatted Rust files in the baseline commit

### cargo test

`cargo test` passes for the current SVM package.

Observed result:

- 63 Rust tests passed
- 0 failed

Known non-blocking warnings:

- `solana_program::entrypoint!` emits `unexpected cfg` warnings for `custom-heap`, `custom-panic`, and `solana`
- these warnings are not treated as `clippy -D warnings` blockers in this stage
- they are reserved for a later clippy warning cleanup stage

### cargo audit

`cargo audit` finds a real dependency-chain blocker:

- `RUSTSEC-2024-0344`
- crate: `curve25519-dalek`
- locked version: `3.2.1`
- fixed version: `>=4.1.3`

Dependency path:

- `xxxl-svm`
- `solana-program v1.18.26`
- `curve25519-dalek v3.2.1`

Dry-run update result:

- updating `curve25519-dalek` to `4.1.3` is not compatible with `solana-program = ^1.18.26`
- updating `solana-program` within the current semver range changes 0 packages
- updating `spl-token` within the current semver range changes 0 packages

Policy decision:

- do not add an audit ignore as a fake fix
- keep this as a documented audit blocker
- move Solana/SPL dependency upgrade into a separate stage

### cargo deny

`cargo deny check` initially failed with large license noise because no explicit `deny.toml` existed.

Resolution in this stage:

- added `programs/xxxl-svm/deny.toml`
- explicitly allowed standard dependency licenses used by the Solana/SPL dependency chain
- configured private crate handling for the local unpublished package
- confirmed `cargo deny check licenses` exits 0
- confirmed `cargo deny check bans` exits 0
- confirmed `cargo deny check sources` exits 0

Advisory checks intentionally remain non-green:

- `cargo deny check advisories` exits 1
- the advisory blocker matches the `cargo audit` result
- this is not ignored in this stage

### cargo geiger

`cargo geiger` is report-only in this stage.

Observed result:

- `cargo geiger` starts but does not produce a stable final report in this environment
- observed tool output includes package matching failures for registry packages such as `bytes@1.12.0` and `zerocopy-derive@0.8.52`
- this is recorded as a tool/reporting limitation, not as a runtime safety proof

Policy decision:

- unsafe statistics are not treated as an automatic pass/fail security verdict
- unsafe usage must be interpreted manually in a later audit stage
- a future stage may retry `cargo geiger` after dependency/toolchain cleanup

## Next stages

1. `stage-xxxl-solana-spl-dependency-upgrade-audit`
2. `stage-xxxl-rust-clippy-warning-cleanup`
3. `stage-xxxl-manual-account-constraint-audit-checklist`

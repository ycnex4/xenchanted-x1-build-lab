# Checkpoint: XXXL Add Mollusk Dev Dependency

Stage: stage-xxxl-add-mollusk-dev-dependency

Status: COMPLETED

## Goal

Add `mollusk-svm` as a dev-dependency separately from the first Mollusk harness implementation.

## Completed

- Added `mollusk-svm = "0.13.4"` as a dev-dependency.
- Preserved runtime dependency pins:
  - `solana-program = "2.3.0"`
  - `spl-token = "5.0.2"` with `no-entrypoint`
- Updated Cargo.lock.
- Verified cargo fmt.
- Verified cargo test.
- Verified cargo clippy with `-D warnings`.
- Verified cargo audit.
- Verified cargo deny licenses/bans/sources.
- Located local Mollusk source package.
- Captured compact API/source reconnaissance for future harness work.
- Documented dependency footprint increase.
- Documented audit warning increase.

## Verification

Hard checks passed:

- cargo fmt --check
- cargo test
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

Observed tests:

- 65 passed
- 0 failed

## Dependency footprint

Cargo audit scan after adding Mollusk:

- 404 crate dependencies

Previously observed baseline:

- 196 crate dependencies

## Audit warnings

Allowed warnings after adding Mollusk:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

## Decision

Mollusk is accepted as a dev-dependency.

Do not add the harness in this stage.

Do not change runtime behavior in this stage.

Do not activate live route execution in this stage.

## Next likely stage

Add the first scaffold-only Mollusk harness.

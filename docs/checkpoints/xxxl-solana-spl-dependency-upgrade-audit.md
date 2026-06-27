# Checkpoint: XXXL Solana/SPL Dependency Upgrade Audit

Stage: `stage-xxxl-solana-spl-dependency-upgrade-audit`

Status: `COMPLETED`

## Goal

Close the RustSec hard vulnerability from the Solana 1.18 dependency chain without blindly upgrading runtime dependencies.

## Completed

- Built a temporary upgrade matrix outside the repository.
- Confirmed Solana 1.18 candidates do not close the audit blocker.
- Confirmed Solana 2 + SPL Token 4 is not sufficient.
- Confirmed Solana 2 + SPL Token 5 is the first candidate with:
  - lock generation success
  - Rust tests passing
  - `cargo audit` passing
- Confirmed Solana 2 + SPL Token 6 also passes but is not minimal.
- Confirmed Solana 3 candidates close audit but fail current tests.
- Applied minimal selected upgrade:
  - `solana-program = 2.3.0`
  - `spl-token = 5.0.2`
- Confirmed resolved `curve25519-dalek = 4.1.3`.
- Confirmed Rust tests pass: 63 passed, 0 failed.
- Confirmed `cargo audit` exits 0.
- Confirmed `cargo deny check licenses/bans/sources` exits 0.

## Decision

Use `solana-program 2.3.0` and `spl-token 5.0.2` as the minimal working upgrade path.

Do not jump to Solana 3 in this stage.

Do not upgrade SPL Token beyond 5.0.2 in this stage.

## Remaining notes

`cargo audit` still reports allowed warnings for:

- `bincode 1.3.3`
- `libsecp256k1 0.6.0`
- `rand 0.7.3`

These are not hard vulnerabilities in this stage.

The known `entrypoint!` cfg warnings remain a later clippy/warning-cleanup concern.

The upgrade is still pre-live-runtime. It does not activate the live gateway route or change the staged finalization policy.

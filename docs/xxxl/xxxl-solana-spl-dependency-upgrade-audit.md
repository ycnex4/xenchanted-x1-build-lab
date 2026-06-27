# XXXL Solana/SPL Dependency Upgrade Audit

Status: COMPLETED.

This stage evaluates and applies the minimal dependency upgrade required to close the RustSec hard vulnerability found in the XXXL SVM program dependency chain.

## Baseline problem

Previous baseline:

- `solana-program = 1.18.26`
- `spl-token = 4.0.3` in lockfile
- `curve25519-dalek = 3.2.1`

`cargo audit` reported:

- `RUSTSEC-2024-0344`
- crate: `curve25519-dalek`
- locked version: `3.2.1`
- required fix: `>=4.1.3`

The vulnerable crate was pulled through `solana-program v1.18.26`.

## Upgrade matrix

The upgrade matrix was executed in temporary `/tmp` copies to avoid mutating the repository during candidate discovery.

Results:

| Candidate | Lock | Tests | Audit | Result |
|---|---:|---:|---:|---|
| Solana 1.18 current range | 0 | 0 | 1 | rejected: audit blocker remains |
| Solana 1.18 newer patch | 0 | 0 | 1 | rejected: audit blocker remains |
| Solana 2 + SPL Token 4 | 0 | 101 | 1 | rejected: tests fail and audit blocker remains |
| Solana 2 + SPL Token 5 | 0 | 0 | 0 | selected: minimal passing candidate |
| Solana 2 + SPL Token 6 | 0 | 0 | 0 | valid but not minimal |
| Solana 3 + SPL Token 6 | 0 | 101 | 0 | rejected: tests fail |
| Solana 3 + SPL Token 7 | 0 | 101 | 0 | rejected: tests fail |
| Solana 3 + SPL Token 8 | 0 | 101 | 0 | rejected: tests fail |

## Selected upgrade

Applied dependency upgrade:

- `solana-program: 1.18.26 -> 2.3.0`
- `spl-token: 4.0.x -> 5.0.2`

Resolved key packages after upgrade:

- `solana-program = 2.3.0`
- `spl-token = 5.0.2`
- `curve25519-dalek = 4.1.3`
- `bincode = 1.3.3`
- `libsecp256k1 = 0.6.0`
- `rand = 0.7.3 / 0.8.6`
- `borsh = 0.10.4 / 1.7.0`

## Verification

Hard checks passed:

- `cargo fmt --check`
- `cargo test`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

Observed results:

- Rust tests: 63 passed, 0 failed
- `cargo audit`: exit 0
- `cargo deny licenses`: exit 0
- `cargo deny bans`: exit 0
- `cargo deny sources`: exit 0

## Security result

The hard vulnerability `RUSTSEC-2024-0344` is resolved by upgrading the Solana dependency chain enough to pull `curve25519-dalek = 4.1.3`.

Remaining `cargo audit` findings are allowed warnings, not hard vulnerabilities in this stage:

- `bincode 1.3.3` unmaintained
- `libsecp256k1 0.6.0` unmaintained
- `rand 0.7.3` unsound advisory

## Policy decision

The selected upgrade is intentionally minimal.

`Solana 2 + SPL Token 6` also passed, but was not selected because `SPL Token 5.0.2` is the first passing candidate and therefore has a smaller compatibility surface.

Solana 3 candidates were rejected because the current program tests fail under the tested Solana 3 combinations.

No live gateway route was activated in this stage.

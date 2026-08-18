# Local audit summary — 2026-08-17

Status: SUMMARY ONLY.

Branch:

`audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`

Head:

`a3792607a5cca8d4563d092c3d635c3803e002ee`

## Scope

Local repository audit after Gate 6.1 handoff cleanup and npm audit lockfile fix.

No transaction sending.

No signature request distribution.

No signature collection.

No live mint.

No replay.

No rollback.

No production deployment.

## Passed checks

NPM typecheck: PASS.

NPM tests: PASS.

- 107 test files passed.
- 959 tests passed.

NPM build: PASS.

NPM audit after lockfile fix: PASS.

- 0 vulnerabilities.

Rust lib tests: PASS.

- 807 passed.
- 0 failed.
- 1 ignored.

Rust integration compile-only: PASS.

Non-Mollusk Rust integration runtime checks: PASS.

- disabled_cpi_reachability: PASS, 6 passed.
- instruction_reserved_bytes: PASS, 3 passed.
- phase_41k4_processed_event_marking_svm: PASS, 4 passed.
- phase_41k5_d15_atomic_mark_and_mint_svm: PASS.
- phase_41k5_d2_production_path_gated_mark_and_mint_e2e: PASS.
- phase_41k5_d3_negative_failure_modes: PASS.
- phase_41k6_b2_valid_quorum_live_gated_success: PASS.
- phase_41k6_b3_hostile_live_gated_matrix: PASS.

Repository status after checks: clean.

## Committed fixes

`106410aa65c9a89c92bee15068aaa9b61f076359`

- Kept Gate 6.1 handoff technical.

`a3792607a5cca8d4563d092c3d635c3803e002ee`

- Resolved npm audit lockfile vulnerabilities.
- Changed package-lock.json only.

## Known issue

`mollusk_consume_gateway_mint` contains artifact-runtime tests.

These tests depend on mutable local `.so` artifacts and must not run as part of default Rust tests.

Current fix:

- artifact-runtime tests are ignored by default.
- non-artifact boundary test remains active.
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml` now passes.
- `mollusk_consume_gateway_mint` default result: 1 passed, 64 ignored.

Observed local artifact mismatch:

- `programs/xxxl-svm/target/deploy/xxxl_svm.so` did not match U3 candidate.
- `programs/xxxl-svm/target/sbpf-solana-solana/release/deps/xxxl_svm.so` did not match U3 candidate.

Recorded U3 candidate artifact:

- sha256: `ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`
- size: `201160`

Do not run artifact-runtime Mollusk tests without an explicit artifact path/hash guard.

## Static scan notes

Forbidden phrase scan produced marker-string matches only.

Tracked risky filename scan produced evidence and policy file matches only.

No private key material, raw signatures, signature bundles, signed transactions, or environment dumps were added by this audit.

## Current operational state

v4 authorization payload hash:

`bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b`

Blocked v3 hash:

`0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d`

Signature request distribution: NOT STARTED.

Signature collection: NOT STARTED.

Live mint: NOT AUTHORIZED.

Transaction sending: NOT AUTHORIZED.

Replay: NOT AUTHORIZED.

Rollback: NOT AUTHORIZED.

Production deployment: NOT AUTHORIZED.

## Next technical blocker

Resolve `mollusk_consume_gateway_mint` artifact loading so local runtime tests execute the intended artifact and do not depend on stale `target/deploy/xxxl_svm.so`.

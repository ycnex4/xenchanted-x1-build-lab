# XXXL Mollusk Account Contract Negative Coverage

Status: COMPLETED.

This stage adds SBF/Mollusk negative coverage for the enforced `consume_gateway_mint` account contract.

## Goal

The previous stage enforced the runtime account contract in unit tests.

This stage proves through the SBF/Mollusk runtime harness that invalid account meta flags are rejected before the runtime reaches deeper validation or any live route behavior.

## What changed

Added ignored Mollusk tests for invalid account meta flags:

- readonly account passed as writable
- required writable account passed as readonly
- unexpected external signer

The tests mutate only the instruction account metas and expect the runtime to reject with:

    InvalidInstruction

## Covered cases

### Readonly account passed writable

`mint_state` is readonly in the manifest.

The test passes it as writable and expects rejection.

### Required writable account passed readonly

`processed_event` is writable in the manifest.

The test passes it as readonly and expects rejection.

### Unexpected signer

No external account signer is accepted by the manifest.

The test passes `recipient_balance` as signer and expects rejection.

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No runtime mutation behavior was changed.

This stage only adds negative SBF/Mollusk coverage for already-enforced account meta rules.

## Verification

Hard checks passed:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Decision

The Mollusk account contract negative coverage boundary is accepted.

Account meta enforcement is now covered by both unit tests and SBF/Mollusk tests.

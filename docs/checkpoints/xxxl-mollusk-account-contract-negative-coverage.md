# Checkpoint: XXXL Mollusk Account Contract Negative Coverage

Stage: stage-xxxl-mollusk-account-contract-negative-coverage

Status: COMPLETED

## Goal

Add SBF/Mollusk negative coverage for the enforced `consume_gateway_mint` account contract.

## Completed

Added ignored Mollusk tests for:

- readonly account passed writable
- required writable account passed readonly
- unexpected external signer

Each invalid account meta case is rejected with `InvalidInstruction`.

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployment behavior was changed.

No runtime mutation behavior was changed.

## Verification

Hard checks passed:

- cargo build-sbf
- cargo fmt --check
- cargo test
- cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

## Decision

The account contract enforcement path is now covered by unit tests and SBF/Mollusk negative tests.

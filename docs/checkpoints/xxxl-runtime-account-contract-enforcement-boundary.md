# Checkpoint: XXXL Runtime Account Contract Enforcement Boundary

Stage: stage-xxxl-runtime-account-contract-enforcement-boundary

Status: COMPLETED

## Goal

Enforce the explicit `consume_gateway_mint` account contract manifest at runtime validation boundary.

## Completed

Added runtime enforcement:

- `assert_consume_gateway_mint_account_contract`

Integrated enforcement into:

- `prepare_consume_gateway_mint_cpi_boundary`

The runtime now rejects account meta flags that do not match the manifest:

- readonly account passed writable
- writable account passed readonly
- unexpected signer account

## Mollusk update

The Mollusk fixture was updated to match the account contract:

- `mint_state` is readonly
- `gateway_config` is readonly
- `guardian_set` is readonly
- mutation-capable state/SPL accounts remain writable

## Test coverage

Tests verify rejection for:

- unnecessary writable flag on readonly account
- missing writable flag on required writable account
- unexpected external signer

Ignored Mollusk tests also pass after fixture alignment.

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployment behavior was changed.

`process_instruction` remains non-live.

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

The runtime account contract enforcement boundary is complete.

Account meta flags are now part of the enforced runtime contract.

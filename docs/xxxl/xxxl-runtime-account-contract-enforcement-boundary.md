# XXXL Runtime Account Contract Enforcement Boundary

Status: COMPLETED.

This stage makes the runtime enforce the explicit `consume_gateway_mint` account contract manifest.

## Goal

The previous stage documented the account contract manifest.

This stage enforces that manifest at runtime validation boundary.

The runtime now rejects:

- readonly accounts passed as writable
- writable accounts passed as readonly
- unexpected external signer accounts
- wrong account count

## What changed

The account contract module now exposes:

    assert_consume_gateway_mint_account_contract

The processor now calls this function inside:

    prepare_consume_gateway_mint_cpi_boundary

before deeper account validation.

## Enforced fields

For every `consume_gateway_mint` account, the runtime checks:

- account index
- writable flag
- signer flag

The owner/data/rent/PDA/SPL checks remain in the existing processor validation path.

## Account contract

Readonly:

- mint_state
- gateway_config
- guardian_set
- mint_authority_pda
- token_program

Writable:

- processed_event
- recipient_balance
- spl_token_mint
- recipient_token_account

Signer requirement:

- no external signer is accepted by this account contract

## Tests added

Unit tests prove that runtime validation rejects:

- unnecessary writable flag on a readonly account
- missing writable flag on a required writable account
- unexpected external signer

The Mollusk consume-gateway-mint fixture was updated so its account metas match the manifest:

- `mint_state` is readonly
- `gateway_config` is readonly
- mutation-capable accounts remain writable

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No mutation semantics were changed except rejecting account meta flags that do not match the manifest.

`process_instruction` remains non-live.

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

The runtime account contract enforcement boundary is accepted.

Future account meta changes must update the manifest, processor enforcement, unit tests, and Mollusk fixtures together.

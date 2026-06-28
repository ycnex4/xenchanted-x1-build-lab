# XXXL Runtime Account Contract Manifest Boundary

Status: COMPLETED.

This stage adds an explicit runtime account contract manifest for `consume_gateway_mint`.

## Goal

Freeze the account contract shape used by the XXXL runtime consume-gateway-mint path.

The manifest documents:

- account index
- account name
- writable vs readonly requirement
- signer requirement
- owner model

This is an activation-gate prerequisite.

## What changed

A new module was added:

    programs/xxxl-svm/src/account_contract.rs

It defines:

- `AccountWriteAccess`
- `AccountSignerRequirement`
- `AccountOwnerModel`
- `ConsumeGatewayMintAccountContractEntry`
- `CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT`
- `consume_gateway_mint_account_contract`
- `consume_gateway_mint_account_contract_entry`

The manifest covers 9 accounts:

0. mint_state
1. gateway_config
2. guardian_set
3. processed_event
4. recipient_balance
5. spl_token_mint
6. recipient_token_account
7. mint_authority_pda
8. token_program

## Writable accounts

Writable:

- processed_event
- recipient_balance
- spl_token_mint
- recipient_token_account

Readonly:

- mint_state
- gateway_config
- guardian_set
- mint_authority_pda
- token_program

## Signer model

No external account is required to be a signer for the consume-gateway-mint account contract.

The mint authority is a PDA and is not an external signer.

## Owner model

Program-owned accounts:

- mint_state
- gateway_config
- guardian_set
- processed_event
- recipient_balance

SPL Token-owned accounts:

- spl_token_mint
- recipient_token_account

PDA account:

- mint_authority_pda

Program account:

- token_program

## Tests added

The new tests verify:

- account contract length matches instruction account meta count
- account contract length matches processor required account count
- manifest indices are contiguous
- manifest names match processor indices
- manifest processor indices match instruction indices
- writable/readonly classification is fixed
- no external signer is required
- owner model is documented
- out-of-range lookup returns none

## Safety boundary

No live route was activated.

No SPL CPI behavior was changed.

No `process_instruction` behavior was changed.

No account mutation behavior was changed.

No deployment behavior was changed.

This is a manifest/documentation-and-test boundary.

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

The runtime account contract manifest boundary is accepted.

Any future account-order or mutability change must update this manifest and its tests.

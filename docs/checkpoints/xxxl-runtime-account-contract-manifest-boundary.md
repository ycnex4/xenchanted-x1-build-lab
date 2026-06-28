# Checkpoint: XXXL Runtime Account Contract Manifest Boundary

Stage: stage-xxxl-runtime-account-contract-manifest-boundary

Status: COMPLETED

## Goal

Add an explicit account contract manifest for the XXXL `consume_gateway_mint` runtime path.

## Completed

Added:

- `programs/xxxl-svm/src/account_contract.rs`

The manifest defines:

- account index
- account name
- writable vs readonly requirement
- signer requirement
- owner model

The manifest covers the 9-account runtime contract:

0. mint_state
1. gateway_config
2. guardian_set
3. processed_event
4. recipient_balance
5. spl_token_mint
6. recipient_token_account
7. mint_authority_pda
8. token_program

## Test coverage

Tests verify:

- manifest length matches instruction account meta count
- manifest length matches processor required account count
- manifest indices match processor constants
- manifest indices match instruction constants
- writable account set is fixed
- no external signer is required
- owner models are documented
- out-of-range lookup fails closed

## Safety boundary

No live route was activated.

No SPL CPI behavior was changed.

No `process_instruction` behavior was changed.

No account mutation behavior was changed.

No deployment behavior was changed.

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

The account contract manifest is now the explicit reference for the consume-gateway-mint account shape.

Future account-order or mutability changes must update this manifest and tests.

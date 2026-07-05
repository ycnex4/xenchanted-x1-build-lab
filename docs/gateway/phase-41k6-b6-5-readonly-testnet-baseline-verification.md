# Phase 41K.6 B6.5 — Read-only testnet baseline verification

## Purpose

This document records a read-only X1 testnet baseline verification for the public values discovered in the repository.

This verification uses read-only RPC account queries only.

This verification does not sign.

This verification does not submit transactions.

This verification does not spend SOL.

This verification does not access private keys.

This verification does not load keypair files.

This verification does not request airdrops.

This verification does not deploy.

This verification does not remove the B1C7 compile_error guard.

This verification does not weaken the B1C7 feature gate.

This verification does not open production or production-like activation.

## Current main checkpoint

Baseline values were previously merged on main:

e18c280 Merge phase 41K.6 B6.5 discovered public baseline values

Current decision remains:

NO-GO.

## RPC boundary

- rpc_label: x1_testnet_rpc_redacted
- rpc_url_recorded: false
- rpc_method_used: getAccountInfo
- commitment: confirmed
- observed_at_utc: 2026-07-05T17:46:44+00:00

The RPC endpoint value is not committed to this document.

## Expected public baseline

- expected_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- expected_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected_gateway_mint_authority_pda: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- expected_token_program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

## Observed program account

- address: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- exists: true
- owner: BPFLoaderUpgradeab1e11111111111111111111111
- executable: true
- lamports: 1141440
- data_len: 36
- decoded_loader_state: program
- decoded_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- program_data_matches_expected: true

## Observed ProgramData account

- address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- exists: true
- owner: BPFLoaderUpgradeab1e11111111111111111111111
- executable: false
- lamports: 269748720
- data_len: 38629
- decoded_loader_state: program_data
- decoded_slot: 169365249
- decoded_upgrade_authority_option: 1528936705
- decoded_upgrade_authority: None
- upgrade_authority_matches_expected: false

## Observed gateway mint authority PDA account

- address: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- exists: false
- owner: None
- executable: null
- lamports: None
- data_len: 0

Gateway mint authority PDA absence as an account is acceptable at this boundary if it is used only as a PDA signing authority.

This document does not infer submit readiness from PDA existence or absence.

## Observed SPL Token program account

- address: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- exists: true
- owner: BPFLoader2111111111111111111111111111111111
- executable: true
- lamports: 929020800
- data_len: 133352

## Verification result

- program_account_exists: true
- program_account_executable: true
- program_data_account_exists: true
- program_data_matches_expected: true
- upgrade_authority_matches_expected: false
- token_program_account_exists: true
- token_program_account_executable: true

## Values still not verified by this step

This read-only baseline verification does not verify:

- actual target_spl_mint
- actual mint_decimals
- actual mint_supply_before
- actual route_id
- actual gateway_config account
- actual guardian_set_id
- actual guardian_set_account
- actual guardian threshold
- actual guardian public keys
- actual recipient owner
- actual recipient token account
- actual processed_event PDA for a real event
- actual canonical_event_key for a real event
- actual fee payer public address
- actual payload hash for a GO package
- actual quorum package hash
- actual relayer submission package hash

## Current decision

Current decision:

NO-GO.

This read-only verification does not authorize live action.

# Phase 41K.6 B6.5 — Read-only baseline decoder correction

## Purpose

This document records a read-only decoder correction for the B6.5 X1 testnet baseline verification.

The previous read-only verification decoded the ProgramData upgrade authority option as a u32 at offset 12.

That produced an invalid option-like value and made the previous upgrade-authority mismatch inconclusive.

This correction probes both u32 option-tag and u8 option-tag layouts and records the selected layout.

This correction uses read-only RPC account queries only.

This correction does not sign.

This correction does not submit transactions.

This correction does not spend SOL.

This correction does not access private keys.

This correction does not load keypair files.

This correction does not request airdrops.

This correction does not deploy.

This correction does not remove the B1C7 compile_error guard.

This correction does not weaken the B1C7 feature gate.

This correction does not open production or production-like activation.

## Current main checkpoint

Previous read-only baseline verification is merged on main:

709cefb Merge phase 41K.6 B6.5 read-only testnet baseline verification

Current decision remains:

NO-GO.

## RPC boundary

- rpc_label: x1_testnet_rpc_redacted
- rpc_url_recorded: false
- rpc_method_used: getAccountInfo
- commitment: confirmed
- observed_at_utc: 2026-07-05T17:52:05+00:00

The RPC endpoint value is not committed to this document.

## Expected public baseline

- expected_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- expected_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

## Program account correction result

- program_account_exists: true
- program_account_owner: BPFLoaderUpgradeab1e11111111111111111111111
- program_account_executable: true
- decoded_loader_state: program
- decoded_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- program_data_matches_expected: true

## ProgramData authority decoder probes

- program_data_account_exists: true
- program_data_account_owner: BPFLoaderUpgradeab1e11111111111111111111111
- program_data_account_executable: false
- program_data_data_len: 38629
- decoded_loader_state: program_data
- decoded_slot: 169365249
- u32_option_probe: 1528936705
- u32_authority_probe: None
- u8_option_probe: 1
- u8_authority_probe: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- selected_layout: u8_option_tag
- selected_option: 1
- selected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- selected_upgrade_authority_matches_expected: true

## Interpretation

The previous value decoded_upgrade_authority_option: 1528936705 must not be treated as a real authority option value.

It was produced by reading four bytes at the option position.

This correction treats that previous mismatch as decoder-inconclusive.

The selected upgrade authority result above is the corrected read-only interpretation for this checkpoint.

## Current decision

Current decision:

NO-GO.

This decoder correction does not authorize live action.

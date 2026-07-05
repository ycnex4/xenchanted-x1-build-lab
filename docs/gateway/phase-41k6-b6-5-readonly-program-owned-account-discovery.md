# Phase 41K.6 B6.5 — Read-only program-owned account discovery

## Purpose

This document records a read-only discovery of accounts currently owned by the documented X1 testnet program.

This discovery uses getProgramAccounts only.

This discovery does not sign.

This discovery does not submit transactions.

This discovery does not spend SOL.

This discovery does not access private keys.

This discovery does not load keypair files.

This discovery does not request airdrops.

This discovery does not deploy.

This discovery does not remove the B1C7 compile_error guard.

This discovery does not weaken the B1C7 feature gate.

This discovery does not open production or production-like activation.

## Current main checkpoint

Read-only decoder correction is merged on main:

92bdb82 Merge phase 41K.6 B6.5 read-only baseline decoder correction

Current decision remains:

NO-GO.

## RPC boundary

- rpc_label: x1_testnet_rpc_redacted
- rpc_url_recorded: false
- rpc_method_used: getProgramAccounts
- commitment: confirmed
- observed_at_utc: 2026-07-05T17:54:07+00:00

The RPC endpoint value is not committed to this document.

## Program

- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

## Discovery summary

- total_program_owned_accounts_observed: 0

## Observed account counts by data length

- none

## Candidate account kind matching

### mint_state

- expected_len: 96
- purpose: target mint state / mint authority relationship
- candidate_count_by_length: 0
- candidates: none

### gateway_config

- expected_len: 152
- purpose: route id / source chain / guardian set / target mint
- candidate_count_by_length: 0
- candidates: none

### guardian_set

- expected_len: 400
- purpose: guardian threshold / guardian public keys
- candidate_count_by_length: 0
- candidates: none

### processed_event

- expected_len: 112
- purpose: per-event replay state, not expected before event selection
- candidate_count_by_length: 0
- candidates: none

### recipient_balance

- expected_len: 96
- purpose: local recipient accounting model
- candidate_count_by_length: 0
- candidates: none

## Interpretation

This discovery only classifies accounts by data length.

It does not prove that any candidate is the correct route, guardian set, mint state, recipient balance, or processed event account.

Any candidate account must be decoded and relationship-checked in a later read-only step before it can be promoted into the B6.5 packet.

If no suitable candidates exist, the missing values remain unknown and any account creation must require a separate explicit signed boundary.

## Current decision

Current decision:

NO-GO.

This account discovery does not authorize live action.

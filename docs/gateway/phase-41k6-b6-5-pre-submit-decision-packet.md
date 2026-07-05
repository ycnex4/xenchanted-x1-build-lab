# Phase 41K.6 B6.5 — Pre-submit decision packet

## Purpose

This document prepares the explicit decision packet required before opening any X1 testnet submit rehearsal.

B6.5 submit rehearsal is not opened by this document.

This document records that the submit boundary remains closed until all required values and approvals are explicit.

## Current main checkpoint

B6.4 operator approval boundary is merged on main:

9bb4897 Merge phase 41K.6 B6.4 operator approval boundary

B6.4 defined approval classes and kept signing, submission, SOL spend, private-key handling, deploy, gate removal, and production activation closed.

## Current B6.5 status

B6.5 explicit testnet submit rehearsal boundary:

NOT OPEN.

Testnet signing:

NOT APPROVED.

Testnet submit:

NOT APPROVED.

SOL spend:

NOT APPROVED.

Private-key handling:

NOT APPROVED.

Deploy:

NOT APPROVED.

B1C7 compile_error guard removal:

NOT APPROVED.

B1C7 feature gate weakening:

NOT APPROVED.

Production or production-like activation:

NOT APPROVED.

## Why B6.5 is not automatic

B6.5 is the first boundary that may discuss actual testnet submission.

Therefore B6.5 must not be treated as automatic engineering continuation.

B6.5 requires a separate written operator/project decision.

No earlier B6 step grants submit permission.

No earlier B6 step grants signing permission.

No earlier B6 step grants SOL spend permission.

No earlier B6 step grants key access permission.

## Required exact values before B6.5 can open

Before B6.5 can open, the following exact values must be known and recorded.

### Network

- network_name:
- rpc_url_label:
- chain_environment:
- read_only_mode_before_submit:
- submit_allowed_after_decision:

### Program

- xxxl_svm_program_id:
- program_deployment_status:
- program_owner:
- executable_status:
- build_profile:
- active_feature_gate_state:

### Mint

- target_spl_mint:
- token_program_id:
- mint_authority_pda:
- mint_decimals:
- mint_supply_before:
- mint_account_owner:
- mint_initialized_status:

### Route

- route_id:
- source_chain_id:
- source_token:
- target_mint:
- source_chain_weight_bps:
- amount_policy:
- route_enabled_status:

### Guardian set

- guardian_set_id:
- guardian_set_account:
- guardian_set_owner:
- guardian_count:
- threshold:
- guardian_public_keys_source:
- active_status:
- schema_version:

### Recipient

- recipient_owner:
- recipient_token_account:
- token_account_owner_program:
- token_account_mint:
- token_account_authority:
- token_account_balance_before:

### Processed event

- canonical_event_key:
- processed_event_pda:
- processed_event_account_owner:
- processed_event_account_state:
- processed_event_consumed_status:
- processed_event_lamports:
- processed_event_data_len:

### Package

- B5_package_source:
- payload_v2_hash:
- quorum_package_hash:
- relayer_submission_package_hash:
- prior_evidence_instruction_count:
- expected_instruction_order:

### Fee payer

- fee_payer_boundary:
- fee_payer_key_handling_policy:
- fee_payer_balance_check_policy:
- max_testnet_fee_allowed:
- log_redaction_policy:

## Required explicit approvals before B6.5 can open

The following approvals must be written explicitly before B6.5 opens.

- testnet-only scope approval:
- no-production-activation approval:
- signing approval:
- submit approval:
- SOL spend approval:
- fee payer boundary approval:
- guardian evidence boundary approval:
- B1C7 gate handling approval:
- abort condition approval:
- observation condition approval:

Any missing approval keeps B6.5 closed.

## B1C7 gate decision requirement

Before any B6.5 submit rehearsal, the project must explicitly decide how the B1C7 gate is handled.

Allowed decision states:

1. Gate remains closed; no submit rehearsal.
2. A testnet-only gated build is prepared under a separate explicit boundary.
3. Another reviewed mechanism is proposed before submit.

B6.5 cannot silently remove the compile_error guard.

B6.5 cannot silently weaken the feature gate.

B6.5 cannot hide gate changes inside refactors.

## Key and secret boundary

B6.5 cannot open until the key and secret boundary is explicit.

The boundary must define:

- who signs,
- what signs,
- where signing happens,
- what key material is used,
- what logs are allowed,
- what logs are forbidden,
- how secrets are prevented from entering repo,
- how secrets are prevented from entering command output,
- how secrets are prevented from entering prompts.

Private keys, seed phrases, mnemonic phrases, and raw secret keys must never be committed, printed, pasted, or logged.

## Abort conditions

B6.5 must not open if any of the following is true:

- program id unknown,
- mint unknown,
- guardian set unknown,
- route id unknown,
- recipient token account unknown,
- processed_event PDA unknown,
- fee payer boundary unknown,
- signing actor unknown,
- submit permission unclear,
- SOL spend permission unclear,
- B1C7 gate handling unclear,
- payload hash mismatch,
- quorum package mismatch,
- recipient token account mismatch,
- guardian threshold mismatch,
- processed_event already consumed,
- production activation ambiguity,
- any request to expose secrets.

## B6.5 opening checklist

B6.5 may open only when every item below is checked:

- [ ] Exact network recorded.
- [ ] Exact program id recorded.
- [ ] Exact mint recorded.
- [ ] Exact route id recorded.
- [ ] Exact guardian set id recorded.
- [ ] Exact recipient token account recorded.
- [ ] Exact processed_event PDA recorded.
- [ ] Exact payload hash recorded.
- [ ] Exact package hash recorded.
- [ ] Fee payer boundary recorded.
- [ ] Signing boundary approved.
- [ ] Submit boundary approved.
- [ ] SOL spend boundary approved.
- [ ] B1C7 gate handling approved.
- [ ] Production activation explicitly excluded.
- [ ] Abort conditions accepted.
- [ ] Observation plan accepted.
- [ ] Secret redaction policy accepted.

## Current decision

Current decision:

B6.5 remains closed.

The project may continue preparing documents and read-only inventory.

The project may not sign.

The project may not submit.

The project may not spend SOL.

The project may not handle private keys.

The project may not remove or weaken the B1C7 guard.

The project may not activate production or production-like runtime execution.

## Next valid step

The next valid step is to fill the B6.5 decision packet with exact testnet values and review the go/no-go state.

Until then, B6.5 remains unopened.

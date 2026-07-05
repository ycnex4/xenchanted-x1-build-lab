# Phase 41K.6 B6.2 — Testnet readiness inventory

## Purpose

B6.2 defines the X1 testnet readiness inventory required before any no-send dry-run package rehearsal.

B6.2 is an inventory boundary.

B6.2 does not deploy.

B6.2 does not submit transactions.

B6.2 does not sign transactions.

B6.2 does not spend SOL.

B6.2 does not access private keys.

B6.2 does not remove the B1C7 compile_error guard.

B6.2 does not weaken the B1C7 feature gate.

B6.2 does not open production gates.

B6.2 does not activate production runtime execution.

## Current main checkpoint

B6.1 opening boundary is merged on main:

74f5a33 Merge phase 41K.6 B6.1 testnet E2E opening boundary

B6.1 opened the testnet E2E rehearsal track as a no-send/no-sign/no-key/no-SOL/no-submit boundary.

B6.2 inherits that boundary.

## B6.2 scope

B6.2 asks:

What must be known before a no-send dry-run package can be prepared for X1 testnet?

B6.2 does not answer by touching the network.

B6.2 only defines the inventory.

Later B6 steps may fill the inventory using explicitly read-only checks.

## Readiness inventory categories

B6.2 inventory is grouped into:

1. Network inventory.
2. Program inventory.
3. Mint inventory.
4. Route inventory.
5. Guardian set inventory.
6. Processed event inventory.
7. Recipient token account inventory.
8. Instruction sysvar and Ed25519 inventory.
9. B5 package inheritance inventory.
10. Operator boundary inventory.
11. Stop-condition inventory.
12. Evidence capture inventory.

## 1. Network inventory

Required fields:

- network_name
- rpc_url_label
- chain_environment
- read_only_mode
- submit_allowed
- signing_allowed
- sol_spend_allowed

B6.2 expected values:

- network_name: X1 testnet
- read_only_mode: true
- submit_allowed: false
- signing_allowed: false
- sol_spend_allowed: false

B6.2 does not require live RPC access.

If later B6 steps introduce RPC calls, those calls must be read-only until a later explicit submit boundary is opened.

## 2. Program inventory

Required fields:

- xxxl_svm_program_id
- program_deployment_status
- program_owner
- executable_status
- build_profile
- active_feature_gate_state

B6.2 expected status:

- known or unknown is acceptable,
- no deployment happens in B6.2,
- no executable account is created in B6.2,
- no feature gate is opened in B6.2.

The B1C7 compile_error guard remains intact.

The B1C7 feature gate remains intact.

## 3. Mint inventory

Required fields:

- target_spl_mint
- token_program_id
- mint_authority_pda
- mint_decimals
- mint_supply_before
- mint_account_owner
- mint_initialized_status

B6.2 expected status:

- may be unknown at inventory-definition time,
- must be read-only when checked later,
- must not mint,
- must not initialize mint,
- must not change mint authority.

## 4. Route inventory

Required fields:

- route_id
- source_chain_id
- source_token
- target_mint
- amount_policy
- route_enabled_status
- source_chain_weight_bps

B6.2 note:

Route fields must match the B5 payload and the SVM handler boundary.

B6.2 does not define a new route model.

B6.2 must not silently introduce a new route id.

## 5. Guardian set inventory

Required fields:

- guardian_set_id
- guardian_set_account
- guardian_set_owner
- guardian_count
- threshold
- guardian_public_keys
- active_status
- schema_version

B6.2 expected status:

- guardian set may be represented as mock, fixture, or pre-generated metadata,
- no live guardian keys are accessed,
- no live guardian signatures are requested,
- no private key material is handled.

Future hardening note:

guardian_public_keys should eventually be linked to guardian_set_id through a read-only account load or descriptor check before live submission.

This is not required for B6.2 because B6.2 is inventory-only.

## 6. Processed event inventory

Required fields:

- canonical_event_key
- processed_event_pda
- processed_event_account_owner
- processed_event_account_state
- processed_event_consumed_status
- processed_event_lamports
- processed_event_data_len

B6.2 expected status:

- read-only only,
- no mark,
- no allocate,
- no assign,
- no rent transfer,
- no processed_event mutation.

The processed_event account must remain unmodified during B6.2.

## 7. Recipient token account inventory

Required fields:

- recipient_owner
- recipient_token_account
- token_account_owner_program
- token_account_mint
- token_account_authority
- token_account_balance_before

B6.2 expected status:

- read-only only,
- no account creation,
- no ATA creation,
- no token balance mutation.

B6.2 keeps the B5/B1C distinction between recipient owner and recipient token account.

## 8. Instruction sysvar and Ed25519 inventory

Required fields:

- instructions_sysvar_account
- ed25519_program_id
- prior_instruction_order_policy
- evidence_instruction_count
- source_instruction_indices
- signed_message_hash

B6.2 expected status:

- mock, fixture, or pre-generated evidence only,
- no live signing,
- no live guardian key access,
- no transaction construction requiring real signatures.

B6.2 does not verify live Ed25519 precompile behavior on X1 testnet.

That belongs to a later explicit rehearsal boundary.

## 9. B5 package inheritance inventory

B6.2 consumes the B5 package shape.

Required inherited fields:

- candidate
- sourceObservation
- handlerBinding
- payload_v2_hash
- quorum package
- relayer submission package
- processed_event
- route_id
- mint
- recipient token account
- amount
- guardian_set_id
- prior evidence instruction count

B6.2 must not redefine these fields.

B6.2 may only list what values are needed for testnet readiness.

## 10. Operator boundary inventory

Required fields:

- operator_role
- signer_boundary
- submit_boundary
- approval_boundary
- stop_authority
- log_redaction_policy

B6.2 expected status:

- operator role may be undefined,
- signer boundary remains closed,
- submit boundary remains closed,
- no approval for signing is requested,
- no approval for submission is requested.

Any testnet or production-like gate opening remains a separate deliberate operator/project decision.

## 11. Stop-condition inventory

B6.2 must preserve stop conditions for later phases.

Stop conditions include:

- unknown program id,
- unknown mint authority,
- unknown guardian set account,
- guardian threshold mismatch,
- processed_event already consumed,
- recipient token account mismatch,
- payload hash mismatch,
- stale signatures,
- unknown route id,
- ambiguous RPC result,
- any request for private keys,
- any attempt to submit before explicit submit boundary.

## 12. Evidence capture inventory

Required fields:

- branch
- commit
- command log path
- package hash
- payload hash
- test output summary
- read-only RPC transcript if later introduced
- no-send assertion
- no-key assertion
- no-SOL assertion
- no-gate-removal assertion

B6.2 does not require live evidence capture.

It defines what evidence later B6 steps should preserve.

## B6.2 readiness table

| Item | Required before B6.3 | Source | Mutation allowed |
|---|---:|---|---:|
| network_name | yes | config/doc | no |
| rpc_url_label | yes | config/doc | no |
| xxxl_svm_program_id | yes | read-only/config | no |
| target_spl_mint | yes | read-only/config | no |
| token_program_id | yes | read-only/config | no |
| route_id | yes | B5 package/config | no |
| guardian_set_id | yes | B5 package/config | no |
| guardian public keys | yes | fixture/config/read-only | no |
| processed_event_pda | yes | B5 package/PDA derivation | no |
| recipient token account | yes | B5 package/read-only | no |
| payload_v2_hash | yes | B5 package | no |
| quorum package | yes | B5 package | no |
| relayer submission package | yes | B5 package | no |
| signer boundary | yes | operator spec | no |
| stop conditions | yes | runbook/spec | no |

## B6.2 forbidden actions

B6.2 forbids:

- deploy,
- submit,
- sign,
- spend SOL,
- private key access,
- keypair file loading,
- seed phrase handling,
- transaction broadcast,
- requestAirdrop,
- createAccount,
- allocate,
- assign,
- transfer,
- mintTo,
- mark processed_event,
- feature gate weakening,
- compile_error guard removal,
- production activation.

## B6.2 closure requirements

B6.2 is closed when:

- readiness inventory categories are documented,
- required fields are documented,
- mutation boundaries are documented,
- no-send/no-sign/no-key/no-SOL/no-submit boundary is preserved,
- B1C7 compile_error guard preservation is explicit,
- B5 package inheritance is explicit,
- B6.3 entry criteria are clear,
- documentation diff check passes.

## B6.3 entry criteria

B6.3 may start after B6.2 is merged.

B6.3 target:

no-send dry-run package rehearsal.

B6.3 may construct or reuse a B5 relayer submission package using inventory values.

B6.3 must remain no-send, no-sign, no-key, no-SOL, no-submit, and no-gate-removal.

## Updated checkpoint list

✅ B1: guardian quorum authorization

✅ B2: valid quorum live-gated success test

✅ B3: hostile live-gated matrix

✅ B4: activation gate decision / production-readiness boundary

✅ B5: watcher/relayer integration path

✅ B5 external review closure

✅ B6.1: X1 testnet E2E opening boundary

👉 B6.2: testnet readiness inventory

⏭ B6.3: no-send dry-run package rehearsal

⏭ B6.4: external signer / operator approval boundary

⏭ B6.5: explicit testnet submit rehearsal boundary

⏭ B6.6: outcome observation

⏭ B6.7: B6 closure

## B6.3 no-send dry-run package rehearsal

B6.3 rehearses the B5 relayer submission package shape using readiness-style values.

B6.3 is documented in:

docs/gateway/phase-41k6-b6-3-no-send-dry-run-package-rehearsal.md

B6.3 adds focused TypeScript coverage in:

tests/phase41k6_b6_no_send_dry_run_package_rehearsal.test.ts

B6.3 remains no-send, no-sign, no-key, no-SOL, no-submit, no-RPC, and no-gate-removal.

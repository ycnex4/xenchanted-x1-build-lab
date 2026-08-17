# Gate P4 Testnet Provisioning Only Retry Simfix

Date UTC: 2026-08-17T14:05:52Z

## Scope

Retry of explicitly approved testnet provisioning only after local web3.js simulateTransaction argument failure.

Initialized exactly:
1. new guardian_set PDA
2. new gateway_config PDA

No guarded mint, replay, rollback, second mint, production deployment, mint_state reinitialization, target SPL mint replacement, old guardian_set reuse, old gateway_config reuse, or other transaction was authorized.

## Safety Flags

- provisioning_executed: `true`
- guardian_set_initialized: `true`
- gateway_config_initialized: `true`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- second_mint_executed: `false`
- production_deployment_executed: `false`
- raw_signatures_committed: `false`
- signed_transaction_committed: `false`

## Repository

- branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- head_sha: `bb23046535050727e479f7caa997b8bd3e999c8d`
- status_before_count: `0`

## Patched Program Verification

- deployed_sha256_before_p4_retry: `ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`
- deployed_size_before_p4_retry: `201160`

## New guardian_set

- guardian_set_id: `5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421`
- guardian_set_pda: `9fRJqk7DTkNhXwQEjtSg8ZhgVwt1D6a7VoZhHSMNuP25`
- guardian_set_active_status_after: `1`
- guardian_set_tx_signature: `22dT6kt4BcyuPv7E1dPd3AVeUKa46UZD1MdVbPzHeZQ2zjZshEquWqhZZGtxkxPRgz6akAuyzSC4MNihND2YEkkX`
- guardian_set_slot: `181262505`

## New gateway_config

- route_id: `aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f`
- gateway_config_pda: `3UFjhhHubGnE2xgdjNayaMQrkYnSRtE6ynxLteByVig5`
- gateway_config_decoded_route_id: `aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f`
- gateway_config_decoded_guardian_set_id: `5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421`
- gateway_config_tx_signature: `2rmwNnrpXbeQDMgQ1vXh6UmQ6Ap3nnUgY4NPeyfaxCQb2TdKk7FrwrR36Ln47wABxCQwWS1HMg9BBwEBfu8enfXk`
- gateway_config_slot: `181262506`

## Execution

- p4_node_code: `0`
- p4_result: `PASS_P4_TESTNET_PROVISIONING_ONLY_EXECUTED`

## Decision Boundary

P4 does not approve guarded mint, replay, rollback, or production deployment.

Next allowed step after P4 PASS is P5 read-only verification.

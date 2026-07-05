# Phase 41K.6 B6.5 — Redacted testnet value packet

## Purpose

This packet defines the redacted value format required before any later B6.5 GO decision can be considered.

This packet does not open B6.5 submit rehearsal.

This packet does not approve signing.

This packet does not approve transaction submission.

This packet does not approve SOL spend.

This packet does not approve private-key handling.

This packet does not approve deploy.

This packet does not remove the B1C7 compile_error guard.

This packet does not weaken the B1C7 feature gate.

This packet does not open production or production-like activation.

## Current main checkpoint

B6.5 boundary review closure notes are merged on main:

d0fd2e5 Merge phase 41K.6 B6.5 boundary review closure notes

Current decision remains:

NO-GO.

## Redaction policy

This packet must never contain:

- private keys,
- secret keys,
- seed phrases,
- mnemonic phrases,
- keypair file paths,
- wallet export data,
- unredacted operator secrets,
- unredacted guardian secrets,
- unredacted fee payer secrets.

Public addresses, public program ids, public mint addresses, public token accounts, and public transaction-independent hashes may be recorded if they are intended to be public testnet values.

If a value is not ready to publish, use:

- null
- redacted
- unknown
- not_applicable

## Operator sign-off fields

Current default values:

- testnet_submit_rehearsal_approved_by: null
- testnet_submit_rehearsal_approval_timestamp: null
- testnet_submit_rehearsal_approval_scope: null
- testnet_submit_rehearsal_approval_commit: null

Null means not approved.

Empty means not approved.

Missing means not approved.

These fields must remain null until a later explicit written GO decision is made.

## Current boundary state

- testnet signing: not approved
- testnet submit: not approved
- SOL spend: not approved
- private-key handling: not approved
- deploy: not approved
- B1C7 compile_error guard removal: not approved
- B1C7 feature gate weakening: not approved
- production activation: not approved

## Redacted network values

- network_name: X1 testnet
- rpc_url_label: redacted_or_not_set
- rpc_endpoint_public: null
- chain_environment: testnet
- read_only_mode_before_go: true
- submit_allowed_before_go: false
- signing_allowed_before_go: false
- sol_spend_allowed_before_go: false

## Redacted program values

- xxxl_svm_program_id: unknown
- program_deployment_status: unknown
- program_owner: unknown
- executable_status: unknown
- build_profile: unknown
- active_feature_gate_state: B1C7_guard_intact

## Redacted mint values

- target_spl_mint: unknown
- token_program_id: unknown
- mint_authority_pda: unknown
- mint_decimals: unknown
- mint_supply_before: unknown
- mint_account_owner: unknown
- mint_initialized_status: unknown

## Redacted route values

- route_id: unknown
- source_chain_id: unknown
- source_token: unknown
- target_mint: unknown
- source_chain_weight_bps: unknown
- amount_policy: unknown
- route_enabled_status: unknown

## Redacted guardian set values

- guardian_set_id: unknown
- guardian_set_account: unknown
- guardian_set_owner: unknown
- guardian_count: unknown
- threshold: unknown
- guardian_public_keys_source: fixture_or_read_only_descriptor_required
- active_status: unknown
- schema_version: unknown

## Redacted recipient values

- recipient_owner: unknown
- recipient_token_account: unknown
- token_account_owner_program: unknown
- token_account_mint: unknown
- token_account_authority: unknown
- token_account_balance_before: unknown

## Redacted processed event values

- canonical_event_key: unknown
- processed_event_pda: unknown
- processed_event_account_owner: unknown
- processed_event_account_state: unknown
- processed_event_consumed_status: unknown
- processed_event_lamports: unknown
- processed_event_data_len: unknown

## Redacted package values

- B5_package_source: existing_B5_package_shape
- payload_v2_hash: unknown
- quorum_package_hash: unknown
- relayer_submission_package_hash: unknown
- prior_evidence_instruction_count: unknown
- expected_instruction_order: unknown

## Redacted fee payer values

- fee_payer_boundary: not_approved
- fee_payer_public_address: null
- fee_payer_key_handling_policy: no_key_handling
- fee_payer_balance_check_policy: read_only_only_before_go
- max_testnet_fee_allowed: null
- log_redaction_policy: secrets_never_logged

## Approval class status

B6.4 defines five approval classes.

Current status:

- read-only inventory approval: planning only
- no-send package approval: no-send only
- testnet signing approval: not approved
- testnet submit approval: not approved
- production or production-like activation approval: not approved

No approval class that enables live action has been issued.

## Discovered public testnet baseline values

Public repository-discovered baseline values are recorded in:

docs/gateway/phase-41k6-b6-5-discovered-public-testnet-baseline-values.md

Discovered public baseline:

- documented_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- documented_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- documented_gateway_mint_authority_pda: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- documented_gateway_mint_authority_pda_bump: 252
- documented_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- token_program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
- documented_testnet_status: X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED

Source-level readiness remains blocked:

- source_runtime_status: SCAFFOLD_ONLY_NOT_DEPLOYABLE
- source_deployable: false
- source_program_id_boundary: PLACEHOLDER_PROGRAM_ID_BOUNDARY
- configured_source_placeholder_program_id: XXXLProgram111111111111111111111111111111111

Active blockers remain:

- PLACEHOLDER_PROGRAM_ID
- LIVE_ROUTE_DISABLED
- SPL_CPI_EXECUTION_DISABLED
- PRODUCTION_GUARDIAN_SET_UNSET
- PRODUCTION_PROOF_LOG_UNSET
- EXTERNAL_REVIEW_INCOMPLETE

The discovered public baseline does not approve signing, submission, SOL spend, private-key handling, deploy, B1C7 gate removal, feature gate weakening, or production activation.

Current decision remains:

NO-GO.

## Read-only testnet baseline verification

A read-only X1 testnet baseline verification is recorded in:

docs/gateway/phase-41k6-b6-5-readonly-testnet-baseline-verification.md

This verification uses read-only RPC account queries only.

It does not approve signing, submission, SOL spend, private-key handling, deploy, B1C7 gate removal, feature gate weakening, or production activation.

Current decision remains:

NO-GO.

## Read-only decoder correction

A read-only ProgramData decoder correction is recorded in:

docs/gateway/phase-41k6-b6-5-readonly-baseline-decoder-correction.md

The correction supersedes only the upgrade-authority option decoding from the previous baseline verification.

This correction does not approve signing, submission, SOL spend, private-key handling, deploy, B1C7 gate removal, feature gate weakening, or production activation.

Current decision remains:

NO-GO.

## Read-only program-owned account discovery

Read-only program-owned account discovery is recorded in:

docs/gateway/phase-41k6-b6-5-readonly-program-owned-account-discovery.md

This discovery uses getProgramAccounts only.

It does not approve signing, submission, SOL spend, private-key handling, deploy, B1C7 gate removal, feature gate weakening, or production activation.

Current decision remains:

NO-GO.

## Required transition before GO

Before this packet can support a later GO decision:

- unknown values must be replaced with exact public or redacted-safe values,
- operator sign-off fields must be explicitly filled,
- B1C7 gate handling must be explicitly decided,
- signing boundary must be explicitly approved,
- submit boundary must be explicitly approved,
- SOL spend boundary must be explicitly approved,
- production activation must remain explicitly excluded,
- abort conditions must be accepted,
- observation plan must be accepted,
- full test gate must be considered before submit boundary.

## Current decision

Current decision:

NO-GO.

This packet is a redacted preparation artifact only.

It does not authorize live action.

## Next safe step

The next safe step is to fill public, non-secret, redacted-safe values where available.

If exact values are unavailable, they must remain unknown.

If any field would expose a secret, it must remain redacted or null.

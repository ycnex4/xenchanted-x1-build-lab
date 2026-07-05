# Phase 41K.6 B6.32 — Local-validator fixture generator schema

Status:

LOCAL_VALIDATOR_ONLY_FIXTURE_GENERATOR_SCHEMA_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

## Purpose

This document defines the schema for a future local-validator-only fixture generator.

It extends:

- B6.29 local-validator dry-run design map
- B6.30 local-validator fixture inventory map
- B6.31 local-validator fixture generator design

This is docs-only.

It does not implement a fixture generator.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker H status

Blocker H:

local validator dry-run

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Schema boundary

The schemas in this document are for future local-validator-only fixtures.

They must not contain:

- production private keys
- production keypair paths
- production guardian material
- live RPC URLs
- X1 testnet submit commands
- production SPL mint addresses
- production recipient accounts
- production bridge messages

All generated data must be deterministic, disposable, local, and non-production.

## Schema files

A future fixture generator should emit the following schema-shaped files:

- manifest.json
- accounts.json
- instructions.json
- scenarios.json
- expected-snapshots.json
- failure-matrix.json
- mutation-invariance.json
- logs.json
- safety-report.json

B6.32 defines the shape only.

It does not generate these files.

## manifest.json schema

Required fields:

    {
      "manifest_version": "1",
      "status": "LOCAL_VALIDATOR_ONLY_FIXTURE_SET",
      "local_only": true,
      "testnet_allowed": false,
      "live_rpc_allowed": false,
      "production_keys_allowed": false,
      "fixture_set_id": "string",
      "fixture_set_name": "string",
      "deterministic_seed_label": "string",
      "program_fixture_id": "string",
      "account_fixture_ids": ["string"],
      "instruction_fixture_ids": ["string"],
      "success_scenario_ids": ["string"],
      "failure_scenario_ids": ["string"],
      "mutation_invariance_ids": ["string"],
      "safety_report_id": "string"
    }

Required invariants:

- local_only must be true.
- testnet_allowed must be false.
- live_rpc_allowed must be false.
- production_keys_allowed must be false.
- No field may contain live RPC URLs.
- No field may contain keypair paths.
- No field may contain private signing material.

## accounts.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "accounts": []
    }

Each account entry:

    {
      "account_id": "string",
      "account_kind": "gateway_config | guardian_set | mint_state | processed_event | spl_mint | recipient_token_account | program | sysvar | token_program | system_program",
      "pubkey": "local deterministic pubkey string",
      "owner": "local deterministic owner pubkey string",
      "is_signer": false,
      "is_writable": false,
      "lamports_model": "rent_exempt | low_rent | zero | fixture_specific",
      "data_encoding": "hex",
      "data_hex": "hex string",
      "negative_variant": "none | wrong_owner | wrong_discriminator | truncated | low_rent | uninitialized | semantic_mismatch"
    }

Required invariants:

- pubkey must be local deterministic fixture value.
- data_hex must match the account kind layout where applicable.
- Negative variants must be explicit.
- Production account addresses are forbidden.
- Live testnet account addresses are forbidden.

## instructions.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "instructions": []
    }

Each instruction entry:

    {
      "instruction_id": "string",
      "instruction_kind": "InitializeGatewayConfig | InitializeGuardianSet | InitializeMintState | ConsumeGatewayMint",
      "tag": 0,
      "payload_encoding": "hex",
      "payload_hex": "hex string",
      "instruction_data_encoding": "hex",
      "instruction_data_hex": "hex string",
      "expected_account_order_id": "string",
      "negative_variant": "none | empty_data | invalid_tag | truncated_payload | oversized_payload | wrong_context | malformed_amount"
    }

Required invariants:

- Tags must match the reserved local instruction tag map.
- Payloads must match typed instruction skeleton layouts.
- Negative variants must be explicit.
- No instruction may imply testnet submit.

## scenarios.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "success_scenarios": [],
      "failure_scenarios": []
    }

Success scenario entry:

    {
      "scenario_id": "string",
      "scenario_kind": "success",
      "description": "string",
      "instruction_id": "string",
      "account_ids_in_order": ["string"],
      "before_snapshot_id": "string",
      "after_snapshot_id": "string",
      "expected_result": "success",
      "expected_state_delta_id": "string",
      "local_validator_only": true
    }

Failure scenario entry:

    {
      "scenario_id": "string",
      "scenario_kind": "failure",
      "description": "string",
      "instruction_id": "string",
      "account_ids_in_order": ["string"],
      "before_snapshot_id": "string",
      "after_snapshot_id": "string",
      "expected_result": "failure",
      "expected_error_label": "string",
      "mutation_invariance_id": "string",
      "local_validator_only": true
    }

Required invariants:

- local_validator_only must be true.
- Failure scenarios must reference mutation-invariance expectations.
- Success scenarios must reference expected state deltas.
- No scenario may reference testnet execution.

## expected-snapshots.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "snapshots": []
    }

Snapshot entry:

    {
      "snapshot_id": "string",
      "scenario_id": "string",
      "snapshot_kind": "before | after",
      "account_snapshots": [
        {
          "account_id": "string",
          "lamports": "string",
          "owner": "string",
          "data_hash": "hex string",
          "data_hex": "hex string"
        }
      ]
    }

Required invariants:

- Before and after snapshots must exist for every scenario.
- Failure scenario after snapshots must match before snapshots for mutable accounts.
- Success scenario after snapshots must include only expected mutations.

## failure-matrix.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "failure_cases": []
    }

Failure case entry:

    {
      "failure_case_id": "string",
      "category": "account_order | account_meta | owner | discriminator | route | guardian | mint | recipient | replay | amount | payload | rent | state",
      "scenario_id": "string",
      "expected_error_label": "string",
      "expected_no_mutation": true,
      "mutable_account_ids": ["string"]
    }

Required failure categories:

- wrong account count
- wrong account order
- missing signer
- unexpected signer
- readonly mismatch
- writable mismatch
- wrong owner
- wrong PDA
- wrong discriminator
- wrong route id
- wrong source chain id
- wrong guardian set id
- wrong mint
- wrong mint authority PDA
- wrong mint authority bump
- wrong token program
- wrong recipient token account owner
- wrong recipient token account mint
- replayed processed_event
- zero amount
- amount overflow
- malformed instruction data
- truncated account data
- invalid guardian quorum
- invalid guardian signature package
- inactive mint_state
- inactive gateway_config
- low rent account

## mutation-invariance.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "mutation_invariance_checks": []
    }

Mutation invariance entry:

    {
      "mutation_invariance_id": "string",
      "scenario_id": "string",
      "expected_no_mutation": true,
      "checked_account_ids": ["string"],
      "before_snapshot_id": "string",
      "after_snapshot_id": "string",
      "comparison": "byte_identical"
    }

Required invariant:

For every failure scenario, mutable checked accounts must remain byte-identical unless a later explicitly approved design changes this rule.

Current default:

Failure leaves all mutable accounts byte-identical.

## logs.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "expected_logs": []
    }

Expected log entry:

    {
      "log_expectation_id": "string",
      "scenario_id": "string",
      "required_log_labels": ["string"],
      "forbidden_patterns": [
        "private_key",
        "secret_key",
        "seed_phrase",
        "mnemonic",
        "keypair_path",
        "BEGIN PRIVATE KEY",
        "https://"
      ]
    }

Required log labels:

- instruction decoded
- account order checked
- account meta validation checked
- owner validation checked
- route validation checked
- guardian validation checked
- replay validation checked
- mint planning checked
- CPI boundary checked
- success or failure result

## safety-report.json schema

Required top-level fields:

    {
      "schema_version": "1",
      "safety_report_id": "string",
      "local_only": true,
      "testnet_allowed": false,
      "live_rpc_detected": false,
      "production_keys_detected": false,
      "keypair_paths_detected": false,
      "private_material_detected": false,
      "submit_commands_detected": false,
      "deploy_commands_detected": false,
      "upgrade_commands_detected": false,
      "result": "PASS | FAIL"
    }

Required invariant:

The fixture set is invalid unless result is PASS.

## Validation rules for a future generator

A future generator implementation must reject output if:

- local_only is not true
- testnet_allowed is not false
- live RPC is detected
- production key material is detected
- keypair path is detected
- private material is detected
- submit command is detected
- deploy command is detected
- upgrade command is detected
- a failure scenario lacks mutation invariance
- a success scenario lacks expected state delta
- account data does not match schema
- instruction data does not match schema
- unknown scenario category is used

## Implementation non-goals

B6.32 does not implement:

- fixture generator code
- fixture files
- CLI command
- local validator execution
- runtime handler
- SPL CPI
- account initialization
- guardian package construction
- testnet submit
- upgrade flow

## Explicit non-closure

This checkpoint does not close blocker H.

It defines the future fixture generator schema only.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a local-only fixture generator skeleton with hard no-testnet guards.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.

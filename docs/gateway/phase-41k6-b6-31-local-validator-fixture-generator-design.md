# Phase 41K.6 B6.31 — Local-validator fixture generator design

Status:

LOCAL_VALIDATOR_ONLY_FIXTURE_GENERATOR_DESIGN_NOT_IMPLEMENTED

Current decision:

NO-GO

## Purpose

This document defines the design boundary for a future local-validator-only fixture generator.

It extends:

- B6.29 local-validator dry-run design map
- B6.30 local-validator fixture inventory map

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

## Generator boundary

A future fixture generator must be local-validator-only.

It must produce deterministic, disposable, non-production fixtures.

It must not use:

- X1 testnet
- live RPC
- real fee payer
- real upgrade authority signing
- production guardian keys
- production guardian packages
- production SPL mint
- production recipient accounts
- production bridge messages

## Generator output categories

A future generator should produce fixture outputs for:

1. Program fixture metadata.
2. SPL Token mint fixture.
3. Gateway config account fixture.
4. Guardian set account fixture.
5. Mint state account fixture.
6. Processed event account fixture.
7. Recipient token account fixture.
8. Instruction data fixtures.
9. Account meta fixtures.
10. Success scenario snapshots.
11. Failure scenario snapshots.
12. Mutation-invariance expectations.
13. Log expectation metadata.
14. Dry-run manifest.

## Dry-run manifest

A future generator should emit a local dry-run manifest containing:

- manifest_version
- local_only: true
- testnet_allowed: false
- live_rpc_allowed: false
- production_keys_allowed: false
- program_fixture_id
- fixture_set_id
- generated_at_source
- deterministic_seed_label
- instruction_fixture_ids
- account_fixture_ids
- success_scenario_ids
- failure_scenario_ids
- expected_mutation_invariance_ids

The manifest must not contain:

- private signing material
- keypair paths
- secrets
- live RPC URLs
- production guardian material

## Determinism requirements

A future generator must make fixture generation deterministic.

Required deterministic inputs:

- fixture_set_name
- scenario_name
- local_program_id_seed
- local_mint_seed
- local_owner_seed
- local_guardian_seed_label
- route_id seed
- canonical_event_key seed
- message_hash seed

Required deterministic outputs:

- local pubkeys
- local account data bytes
- instruction bytes
- expected before snapshots
- expected after snapshots
- expected error labels for negative scenarios

## Account data generation requirements

A future generator should use the existing local skeleton layouts as the source of truth where possible:

- state_account_layout_skeleton
- state_initialization_skeleton
- consume_state_transition_skeleton
- typed_instruction_skeleton
- account_order_skeleton
- account_validation_skeleton

The generator must not invent incompatible layouts.

If a future runtime layout differs from the local skeleton layout, the difference must be documented before local-validator execution.

## Success scenario generation

A future generator should produce at least one success scenario:

- initialized gateway_config
- initialized guardian_set
- initialized mint_state
- unconsumed processed_event
- initialized SPL mint
- initialized recipient token account
- valid ConsumeGatewayMint instruction data
- valid account order
- expected mint_state.total_minted delta
- expected recipient token balance delta
- expected processed_event consumed marker

Success scenario expected result:

- all preconditions pass
- SPL CPI is executed only if explicitly enabled in a future local-validator-only step
- all success mutations are atomic
- no unrelated account changes

## Failure scenario generation

A future generator should produce negative fixtures for:

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

Failure scenario expected result:

- expected error code or error label
- no mutation to gateway_config
- no mutation to guardian_set
- no mutation to mint_state
- no mutation to processed_event
- no mutation to SPL mint
- no mutation to recipient token account

## Mutation-invariance generation

A future generator should produce before and after comparison expectations.

For every failure fixture, it must define:

- mutable account list
- before byte snapshot
- expected after byte snapshot
- expected equality: true
- expected failure reason

The default invariant is:

Failure leaves all mutable accounts byte-identical.

## Local-only safety checks

A future generator must include checks that fail if:

- testnet program id is used
- live RPC URL is present
- production guardian material is present
- real keypair path is present
- production mint is present
- production recipient account is present
- submit command is present
- upgrade command is present
- deploy command is present

## Implementation non-goals

B6.31 does not implement:

- fixture generator code
- CLI command
- local validator execution
- runtime handler
- SPL CPI
- account initialization
- guardian package construction
- testnet submit
- upgrade flow

## Evidence required before moving from design to implementation

Before implementing a local fixture generator, the following must be explicit:

- generator file path
- output directory path
- no-testnet guard
- deterministic seed policy
- generated manifest schema
- generated snapshot schema
- generated failure matrix schema
- secret scanning boundary
- focused test command
- abort conditions

## Explicit non-closure

This checkpoint does not close blocker H.

It only defines the future local-validator fixture generator design.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is to create a local fixture generator schema document.

No generator implementation is approved by this checkpoint.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.

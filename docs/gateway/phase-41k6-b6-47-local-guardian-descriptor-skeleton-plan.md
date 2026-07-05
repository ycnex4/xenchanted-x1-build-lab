# Phase 41K.6 B6.47 — Local-only guardian descriptor skeleton plan

Status:

LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

## Purpose

This document defines the plan for a future local-only guardian descriptor skeleton.

It follows:

- B6.46 guardian set testnet descriptor map
- B6.41 local-validator-only GO form design
- B6.42 local fixture file emission plan
- B6.43 local fixture file emitter skeleton
- B6.44 local fixture file emitter safety checkpoint

This is docs-only.

It does not create a guardian descriptor.

It does not create guardian keys.

It does not use real guardian public keys.

It does not construct guardian packages.

It does not sign messages.

It does not run a local validator.

It does not build, deploy, upgrade, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, or perform SPL CPI minting.

## Current boundary

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

This checkpoint defines the future local-only guardian descriptor skeleton boundary only.

## Why a local-only guardian descriptor skeleton is needed

Before a future local-validator dry-run, guardian authorization must be represented in deterministic local fixtures.

The descriptor skeleton should model:

- guardian_set_id
- threshold
- guardian public key placeholders
- public key ordering rule
- descriptor integrity marker
- signature algorithm label
- message hash algorithm label
- route scope
- network scope
- activation status
- rotation policy
- no-signing-material guarantee

The skeleton must not include real signing material.

## Required local-only status

The future skeleton must be explicitly marked:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Required flags:

- local_only: true
- testnet_allowed: false
- live_route_allowed: false
- signing_enabled: false
- guardian_package_construction_enabled: false
- production_keys_allowed: false
- writes_to_disk: false unless a later fixture emission step explicitly enables local-only file writing

## Required skeleton fields

A future local-only descriptor skeleton should include:

- descriptor_version
- descriptor_status
- descriptor_scope
- network_scope
- route_id
- source_chain_id
- mint_token
- guardian_set_id
- threshold
- guardian_count
- guardian_public_key_fixtures
- public_key_encoding
- signature_algorithm
- message_hash_algorithm
- canonical_message_schema_version
- descriptor_integrity_hash_algorithm
- descriptor_integrity_hash_value
- descriptor_source
- activation_status
- rotation_policy
- emergency_disable_policy
- no_signing_material_statement

## Required local fixture public key policy

The future skeleton may use deterministic local public key fixtures.

They must be:

- obviously local
- deterministic
- reproducible
- not production keys
- not testnet guardian keys
- not private keys
- not keypair paths
- not mnemonic-derived material
- not suitable for real signing

The descriptor must say that the public keys are local fixtures only.

## Required threshold policy

The future skeleton must model threshold behavior.

Required test cases:

- threshold greater than zero
- threshold less than or equal to guardian_count
- threshold success boundary
- threshold failure boundary
- duplicate guardian rejection
- unknown guardian rejection
- inactive guardian rejection if modeled

## Required route scope

The future skeleton must bind descriptor scope to the intended local route.

Required fields:

- route_id
- source_chain_id
- mint_token
- schema_version
- message_hash_algorithm

The skeleton must not imply production route activation.

## Required descriptor integrity policy

The future skeleton should model descriptor integrity.

Required fields:

- descriptor_integrity_hash_algorithm
- descriptor_integrity_hash_value
- descriptor_source
- descriptor_review_status

The skeleton hash may be a deterministic local placeholder or derived from local skeleton content.

It must not be represented as production-ready.

## Required no-signing-material policy

The future skeleton must reject or avoid:

- private keys
- secret keys
- keypair paths
- mnemonic text
- seed phrase text
- production guardian material
- testnet guardian signing material
- live endpoint configuration
- submit command configuration

## Required relationship to local fixtures

The future guardian descriptor skeleton should be referenced by local fixture structures.

Expected relationships:

- manifest references guardian descriptor skeleton id
- guardian_set fixture references descriptor skeleton id
- failure matrix references guardian descriptor negative cases
- mutation-invariance references guardian failure scenarios
- logs expectation references guardian validation labels

## Required negative cases

The future skeleton should support planned negative cases for:

- threshold zero
- threshold greater than guardian_count
- empty guardian fixture list
- duplicate guardian fixture
- unknown guardian fixture
- malformed local public key fixture
- wrong guardian_set_id
- wrong route id
- wrong source_chain_id
- wrong mint_token
- wrong message hash label
- descriptor integrity mismatch
- signing material marker detected
- production-key marker detected
- live-route marker detected

## Required safety checks

A future implementation must abort if:

- descriptor id is empty
- descriptor scope is not local
- threshold is zero
- threshold is greater than guardian_count
- guardian_count is zero
- duplicate guardian fixture is detected
- unsafe text is detected
- signing material marker is detected
- keypair path marker is detected
- production marker is detected
- testnet submit marker is detected
- live route marker is detected

## Relationship to blocker F

Blocker F remains open.

B6.46 mapped the testnet descriptor requirements.

B6.47 maps a future local-only skeleton plan.

Neither creates a real testnet descriptor.

Neither initializes guardian state.

Neither constructs guardian packages.

Neither enables signing.

## Relationship to blocker H

A future local-validator dry-run may use a local-only guardian descriptor skeleton.

But this checkpoint does not approve local-validator execution.

Current blocker H state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

## Explicit non-approval

This B6.47 checkpoint does not implement a guardian descriptor skeleton.

It does not create a guardian descriptor.

It does not create keys.

It does not sign messages.

It does not approve local-validator execution.

It does not approve testnet action.

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a Rust local guardian descriptor skeleton with hard local-only and no-signing guards.

No guardian descriptor is created by this checkpoint.

No guardian package is constructed by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

## B6.48 local guardian descriptor skeleton

B6.48 local guardian descriptor skeleton is implemented in:

programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs

The module is explicitly marked:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Current safety flags:

- local_only: true
- testnet_allowed: false
- live_route_allowed: false
- signing_enabled: false
- guardian_package_construction_enabled: false
- production_keys_allowed: false
- writes_to_disk: false
- local_validator_execution_approved: false

Current decision remains:

NO-GO.

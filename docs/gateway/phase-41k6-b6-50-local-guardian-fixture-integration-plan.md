# Phase 41K.6 B6.50 — Local guardian descriptor fixture integration plan

Status:

LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

## Purpose

This document defines how the local guardian descriptor skeleton should later integrate with local fixture generation and local validator scenario planning.

It follows:

- B6.47 local-only guardian descriptor skeleton plan
- B6.48 local guardian descriptor skeleton
- B6.49 local guardian descriptor safety checkpoint
- B6.42 local fixture file emission plan
- B6.43 local fixture file emitter skeleton
- B6.44 local fixture file emitter safety checkpoint

This is docs-only.

It does not implement fixture integration.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not sign messages.

It does not run a local validator.

It does not build, deploy, upgrade, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, or perform SPL CPI minting.

## Current boundary

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

This checkpoint defines integration requirements only.

## Integration principle

The local guardian descriptor skeleton should become an input to future local-only fixture planning.

It must remain:

- local-only
- deterministic
- no-signing
- no-package-construction
- no-testnet
- no-live-route
- no-production-keys
- no-file-writing unless a later explicit local-only fixture emission step enables local writes

## Current local guardian descriptor skeleton

Implemented skeleton:

programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs

Current skeleton status:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Current skeleton guarantees:

- local_only: true
- testnet_allowed: false
- live_route_allowed: false
- signing_enabled: false
- guardian_package_construction_enabled: false
- production_keys_allowed: false
- writes_to_disk: false
- local_validator_execution_approved: false

## Future integration targets

The local guardian descriptor skeleton should later integrate with:

1. local fixture generator skeleton
2. local fixture file emitter skeleton
3. local guardian_set account fixture
4. local instruction fixture planning
5. local success scenario
6. local failure matrix
7. local mutation-invariance expectations
8. local logs expectations
9. local safety report

B6.50 does not implement these integrations.

## Future manifest integration

A future manifest should reference:

- local_guardian_descriptor_id
- local_guardian_descriptor_status
- local_guardian_descriptor_integrity_hash
- local_guardian_set_id
- local_guardian_threshold
- local_guardian_count
- local_guardian_fixture_ids

Required invariant:

The manifest must not imply testnet descriptor activation.

## Future accounts integration

A future accounts fixture should link:

- guardian_set account fixture
- guardian descriptor skeleton id
- guardian_set_id
- threshold
- guardian public key fixtures
- descriptor integrity marker

Required invariant:

The guardian_set account fixture must use only local public key fixtures.

It must not contain real guardian keys.

It must not contain signing material.

## Future instruction integration

A future instruction fixture should use guardian descriptor data only as local validation input.

Expected instruction relationships:

- InitializeGuardianSet references guardian_set_id
- InitializeGuardianSet references threshold
- InitializeGuardianSet references local guardian public key fixtures
- ConsumeGatewayMint references guardian_set_id
- ConsumeGatewayMint references expected approval count model
- ConsumeGatewayMint references descriptor route scope

Required invariant:

No instruction fixture may construct a real guardian package.

No instruction fixture may enable signing.

## Future success scenario integration

A future local success scenario should require:

- descriptor safety validation PASS
- guardian_set fixture matches descriptor
- threshold is satisfied by local approval model
- route id matches descriptor
- source chain id matches descriptor
- mint token matches descriptor
- message hash label matches descriptor
- no signing material detected
- no package construction detected

Expected success result:

- guardian authorization model passes
- consume plan may continue to later validation layers
- no descriptor mutation occurs

## Future failure matrix integration

The future failure matrix should include guardian descriptor failure cases:

- descriptor id mismatch
- descriptor integrity mismatch
- wrong guardian_set_id
- wrong threshold
- threshold zero
- threshold greater than guardian_count
- empty guardian fixture list
- duplicate guardian fixture
- unknown guardian fixture
- malformed guardian fixture
- wrong route id
- wrong source chain id
- wrong mint token
- wrong message hash label
- signing material marker detected
- package construction marker detected
- production key marker detected
- testnet activation marker detected
- live route marker detected

Required invariant:

Every descriptor-related failure must preserve no-mutation behavior.

## Future mutation-invariance integration

For every guardian descriptor failure case, the mutation-invariance plan must verify:

- gateway_config unchanged
- guardian_set unchanged
- mint_state unchanged
- processed_event unchanged
- SPL mint unchanged if present in fixture
- recipient token account unchanged if present in fixture

Expected comparison mode:

byte_identical

## Future logs integration

Future logs expectation should include labels for:

- guardian descriptor loaded
- guardian descriptor safety validated
- guardian_set fixture matched
- threshold checked
- duplicate guardian rejected
- unknown guardian rejected
- descriptor integrity mismatch rejected
- route mismatch rejected
- signing material marker rejected
- guardian validation failed without mutation

Forbidden logs:

- private material
- keypair path
- signing seed
- production guardian marker
- live endpoint marker
- testnet submit marker

## Future safety report integration

The future safety report should include guardian descriptor checks:

- local_guardian_descriptor_present
- local_guardian_descriptor_safety_passed
- guardian_descriptor_testnet_allowed
- guardian_descriptor_live_route_allowed
- guardian_descriptor_signing_enabled
- guardian_descriptor_package_construction_enabled
- guardian_descriptor_production_keys_allowed
- guardian_descriptor_writes_to_disk
- guardian_descriptor_private_material_detected
- guardian_descriptor_result

Required result for local-only fixture safety:

PASS

## Required integration abort conditions

A future implementation must abort if:

- descriptor skeleton status is not local-only no-signing
- descriptor safety validation fails
- descriptor id is empty
- guardian_set_id is empty
- threshold is zero
- threshold exceeds guardian_count
- guardian_count is zero
- duplicate guardian fixture is detected
- unsafe text is detected
- signing material marker is detected
- package construction marker is detected
- production key marker is detected
- testnet marker is detected
- live route marker is detected
- descriptor and guardian_set fixture diverge

## Relationship to blocker F

Blocker F remains open.

Reason:

The guardian descriptor is still local skeleton only.

No testnet guardian descriptor exists.

No testnet guardian_set account is initialized.

No real guardian package construction is enabled.

No signing is enabled.

## Relationship to blocker H

Blocker H remains open.

Reason:

No local-validator dry-run is executed.

The integration is only planned.

No fixture files are emitted.

No local scenarios are executed.

## Explicit non-approval

This checkpoint does not implement fixture integration.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a Rust local guardian fixture integration skeleton with hard local-only and no-signing guards.

No fixture integration is implemented by this checkpoint.

No fixture files are emitted by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

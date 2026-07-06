# Phase 41K.6 B6.53 — Local guardian failure matrix integration map

Status:

LOCAL_GUARDIAN_FAILURE_MATRIX_INTEGRATION_MAP_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps how local guardian descriptor and guardian fixture integration failures should later be represented in the local failure matrix.

It follows:

- B6.48 local guardian descriptor skeleton
- B6.49 local guardian descriptor safety checkpoint
- B6.51 local guardian fixture integration skeleton
- B6.52 local guardian fixture integration safety checkpoint
- B6.36 local-validator success/failure matrix design
- B6.42 local fixture file emission plan
- B6.43 local fixture file emitter skeleton

This is docs-only.

It does not execute the failure matrix.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not sign messages.

It does not run a local validator.

It does not build, deploy, upgrade, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, or perform SPL CPI minting.

## Current boundary

Current status:

LOCAL_GUARDIAN_FAILURE_MATRIX_INTEGRATION_MAP_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

This checkpoint defines failure matrix mapping only.

## Integration principle

Guardian descriptor failures must be first-class local failure cases.

Each guardian-related failure must prove:

- validation fails before mint execution
- no signing is enabled
- no guardian package is constructed
- no fixture files are emitted
- no local validator execution happens in this checkpoint
- no testnet action happens
- no mutable account changes in future execution
- all future failure-state comparisons are byte-identical

## Current local skeletons

Current local guardian descriptor skeleton:

programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs

Current status:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Current local guardian fixture integration skeleton:

programs/xxxl-svm/src/local_guardian_fixture_integration_skeleton.rs

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_SKELETON_NOT_EXECUTING

Both remain local-only and non-executing.

## Guardian failure groups

The future failure matrix should group guardian failures into these categories:

1. descriptor identity failures
2. descriptor integrity failures
3. guardian_set mapping failures
4. threshold failures
5. guardian fixture list failures
6. route scope failures
7. message boundary failures
8. signing material safety failures
9. package construction safety failures
10. live/testnet/prod safety failures

## Descriptor identity failures

Required planned cases:

- guardian_descriptor_id_mismatch
- missing_guardian_descriptor_id
- empty_guardian_descriptor_id
- malformed_guardian_descriptor_id
- descriptor_scope_not_local

Expected result:

- validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Descriptor integrity failures

Required planned cases:

- guardian_descriptor_integrity_mismatch
- missing_descriptor_integrity_hash
- wrong_descriptor_integrity_algorithm
- descriptor_source_commit_mismatch
- descriptor_file_path_mismatch_if_file_backed_later

Expected result:

- descriptor rejected
- no guardian authorization success
- no mutable state changes
- no package construction
- no signing
- no submit

## Guardian set mapping failures

Required planned cases:

- guardian_set_id_mismatch
- missing_guardian_set_id
- wrong_guardian_set_account_fixture
- descriptor_guardian_set_diverges_from_account_fixture
- inactive_guardian_set_if_modeled_later

Expected result:

- guardian_set validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Threshold failures

Required planned cases:

- guardian_threshold_zero
- guardian_threshold_exceeds_count
- guardian_count_zero
- insufficient_valid_approval_count
- duplicate_approval_does_not_increase_count
- unknown_guardian_does_not_increase_count

Expected result:

- threshold validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Guardian fixture list failures

Required planned cases:

- empty_guardian_fixture_list
- duplicate_guardian_fixture
- malformed_guardian_fixture
- unknown_guardian_fixture
- production_key_marker_detected
- real_guardian_marker_detected
- keypair_path_marker_detected
- private_material_marker_detected

Expected result:

- descriptor safety validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Route scope failures

Required planned cases:

- guardian_wrong_route_id
- guardian_wrong_source_chain_id
- guardian_wrong_mint_token
- guardian_wrong_network_scope
- guardian_live_route_marker_detected
- guardian_testnet_activation_marker_detected

Expected result:

- route-scope validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Message boundary failures

Required planned cases:

- guardian_wrong_message_hash_label
- guardian_wrong_schema_version
- guardian_wrong_canonical_message_boundary
- guardian_wrong_signed_payload_boundary_if_modeled_later

Expected result:

- message boundary validation fails
- no mutable state changes
- no package construction
- no signing
- no submit

## Signing material safety failures

Required planned cases:

- guardian_signing_material_marker_detected
- private_key_marker_detected
- secret_key_marker_detected
- mnemonic_marker_detected
- seed_phrase_marker_detected
- begin_private_key_marker_detected
- signing_key_marker_detected

Expected result:

- safety validation fails immediately
- no mutable state changes
- no package construction
- no signing
- no submit

## Package construction safety failures

Required planned cases:

- guardian_package_construction_marker_detected
- approval_package_marker_detected
- non_local_package_scope_detected
- package_construction_enabled_flag_true
- signing_enabled_flag_true

Expected result:

- safety validation fails immediately
- no mutable state changes
- no package construction
- no signing
- no submit

## Live, testnet, and production safety failures

Required planned cases:

- testnet_allowed_flag_true
- live_route_allowed_flag_true
- production_keys_allowed_flag_true
- fixture_file_emission_enabled_flag_true
- local_validator_execution_approved_flag_true_without_go
- testnet_submit_enabled_flag_true_without_go

Expected result:

- safety validation fails immediately
- no mutable state changes
- no package construction
- no signing
- no submit

## Required no-mutation invariant

Every guardian failure case must preserve:

- gateway_config unchanged
- guardian_set unchanged
- mint_state unchanged
- processed_event unchanged
- SPL mint unchanged if included in future fixture
- recipient token account unchanged if included in future fixture
- local descriptor object unchanged
- local fixture manifest unchanged
- local safety report unchanged

Expected comparison mode:

byte_identical

## Required future failure matrix fields

A future failure matrix entry should include:

- failure_case_id
- failure_group
- local_only
- descriptor_id
- guardian_set_id
- route_id
- expected_error_label
- expected_mutation
- expected_signing
- expected_package_construction
- expected_submit
- expected_testnet_action
- no_mutation_accounts
- log_expectation_id
- safety_report_expectation_id

Required defaults:

- local_only: true
- expected_mutation: false
- expected_signing: false
- expected_package_construction: false
- expected_submit: false
- expected_testnet_action: false

## Required future log expectations

Future local-only logs may include:

- guardian_descriptor_validation_started
- guardian_descriptor_rejected
- guardian_threshold_rejected
- guardian_fixture_rejected
- guardian_route_scope_rejected
- guardian_safety_marker_rejected
- guardian_failure_preserved_no_mutation

Logs must not include:

- private material
- keypair path
- mnemonic material
- seed phrase
- production key material
- live endpoint
- submit command
- transaction signature

## Relationship to blocker F

Blocker F remains open.

Reason:

The failure matrix integration is only mapped.

No testnet guardian descriptor exists.

No guardian_set account is initialized on testnet.

No guardian package construction is enabled.

Signing remains disabled.

## Relationship to blocker H

Blocker H remains open.

Reason:

The failure matrix is not executed.

No local-validator dry-run is approved.

No fixture files are emitted.

No snapshots are compared.

## Explicit non-approval

This checkpoint does not execute failure matrix.

It does not implement failure matrix runner.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

This checkpoint does not approve local-validator execution.

It does not approve testnet action.

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a Rust local guardian failure matrix skeleton with hard local-only and no-execution guards.

No failure matrix is executed by this checkpoint.

No fixture files are emitted by this checkpoint.

No descriptor files are created by this checkpoint.

No guardian packages are constructed by this checkpoint.

No signing is approved by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

## B6.54 local guardian failure matrix skeleton

B6.54 local guardian failure matrix skeleton is implemented in:

programs/xxxl-svm/src/local_guardian_failure_matrix_skeleton.rs

The module is explicitly marked:

LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_NOT_EXECUTING

Current safety flags:

- local_only: true
- testnet_allowed: false
- live_route_allowed: false
- signing_enabled: false
- guardian_package_construction_enabled: false
- production_keys_allowed: false
- writes_to_disk: false
- fixture_file_emission_enabled: false
- failure_matrix_execution_enabled: false
- local_validator_execution_approved: false

Current decision remains:

NO-GO.

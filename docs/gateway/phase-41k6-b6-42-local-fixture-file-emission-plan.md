# Phase 41K.6 B6.42 — Local fixture file emission plan

Status:

LOCAL_FIXTURE_FILE_EMISSION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

## Purpose

This document defines the future local fixture file emission plan.

It follows:

- B6.32 local-validator fixture generator schema
- B6.33 local-only fixture generator skeleton
- B6.34 local fixture generator safety checkpoint
- B6.41 local-validator-only GO form design

This is docs-only.

It does not implement fixture file emission.

It does not write fixture files.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current boundary

Current status:

LOCAL_FIXTURE_FILE_EMISSION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision:

NO-GO

This checkpoint defines the future emission boundary only.

## Why fixture file emission is needed

Before a future local-validator dry-run, the local fixtures should be materialized as files so that execution can be audited.

The future fixture files should make the local dry-run reproducible.

They should define:

- manifest
- accounts
- instructions
- scenarios
- expected snapshots
- failure matrix
- mutation-invariance checks
- expected logs
- safety report

This checkpoint does not create those files.

## Future output directory

Future local fixture output should use a disposable directory.

Recommended future directory pattern:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

The output directory must be local.

The output directory must be disposable.

The output directory must not contain production inputs.

The output directory must not be used as a testnet deployment directory.

## Future emitted files

A future fixture file emitter should create:

- manifest.json
- accounts.json
- instructions.json
- scenarios.json
- expected-snapshots.json
- failure-matrix.json
- mutation-invariance.json
- logs.json
- safety-report.json
- README.local-only.txt

B6.42 does not create these files.

## manifest.json emission requirement

The future manifest file must include:

- manifest_version
- status
- local_only
- testnet_allowed
- live_rpc_allowed
- production_keys_allowed
- fixture_set_id
- fixture_set_name
- deterministic_seed_label
- program_fixture_id
- account_fixture_ids
- instruction_fixture_ids
- success_scenario_ids
- failure_scenario_ids
- mutation_invariance_ids
- safety_report_id

Required invariant:

- local_only must be true
- testnet_allowed must be false
- live_rpc_allowed must be false
- production_keys_allowed must be false

## accounts.json emission requirement

The future accounts file must include local deterministic accounts only.

Required account groups:

- local program fixture
- local gateway_config fixture
- local guardian_set fixture
- local mint_state fixture
- local processed_event fixture
- local SPL mint fixture
- local recipient token account fixture
- local token program fixture
- local system program fixture
- local sysvar fixtures if needed

Required invariant:

No production account address is allowed.

No testnet account address is allowed.

## instructions.json emission requirement

The future instructions file must include local deterministic instruction data for:

- InitializeGatewayConfig
- InitializeGuardianSet
- InitializeMintState
- ConsumeGatewayMint

Required invariant:

Instruction data must be tied to the local skeleton layouts.

No instruction may imply live submit.

No instruction may imply testnet submit.

## scenarios.json emission requirement

The future scenarios file must include:

- one local success scenario
- local failure scenarios
- account order for each scenario
- before snapshot reference
- after snapshot reference
- expected result
- expected error label for failures
- mutation-invariance reference for failures

Required invariant:

Every scenario must be local-validator-only.

## expected-snapshots.json emission requirement

The future snapshot file must include:

- before snapshots
- after snapshots
- account data hashes
- account data bytes or deterministic references
- lamports model
- owner model

Required invariant:

Failure scenario after snapshots must match before snapshots for mutable accounts.

## failure-matrix.json emission requirement

The future failure matrix file must include:

- failure_case_id
- category
- scenario_id
- expected_error_label
- expected_no_mutation
- mutable_account_ids

Required invariant:

Every failure case must require no mutation unless a later explicitly approved design changes this rule.

## mutation-invariance.json emission requirement

The future mutation-invariance file must include:

- mutation_invariance_id
- scenario_id
- checked_account_ids
- before_snapshot_id
- after_snapshot_id
- comparison mode

Required comparison mode:

byte_identical

## logs.json emission requirement

The future logs expectation file must include:

- expected instruction decode labels
- expected account validation labels
- expected route validation labels
- expected guardian validation labels
- expected replay validation labels
- expected mint planning labels
- expected success labels
- expected failure labels
- forbidden text patterns

Required invariant:

Expected logs must not include private material or live endpoints.

## safety-report.json emission requirement

The future safety report must include:

- local_only
- testnet_allowed
- live_rpc_detected
- production_keys_detected
- key_material_paths_detected
- private_material_detected
- submit_commands_detected
- deploy_commands_detected
- upgrade_commands_detected
- result

Required invariant:

The fixture set is invalid unless result is PASS.

## README.local-only.txt emission requirement

The future local README must explain:

- fixtures are local only
- fixtures are disposable
- fixtures are not for testnet
- fixtures are not for production
- fixtures do not contain signing material
- fixtures do not approve local-validator execution by themselves
- fixtures do not approve testnet execution

## Future emitter safety checks

A future emitter must abort if:

- output directory is missing and cannot be created
- output directory points outside allowed local disposable area
- fixture_set_id is empty
- deterministic seed label is empty
- unsafe text pattern is detected
- testnet endpoint is detected
- live endpoint is detected
- production account is detected
- production mint is detected
- key material path is detected
- private material is detected
- submit command is detected
- deploy command is detected
- upgrade command is detected

## Future emitter success criteria

A future emitter succeeds only if:

- all expected files are emitted
- all emitted files are deterministic
- safety report result is PASS
- no forbidden text appears
- no testnet endpoint appears
- no live endpoint appears
- no signing material appears
- no production account appears
- no production mint appears
- source tree remains clean except explicitly generated disposable fixture files

## Future cleanup policy

Generated fixtures must be safe to delete.

Cleanup must preserve:

- command log if requested
- safety report if requested
- emitted fixture summary if requested

Cleanup must not remove source files.

## Relationship to local-validator GO

Fixture file emission is not local-validator execution.

A future fixture emission step may be allowed before local-validator execution, but it still requires its own scoped command boundary.

A future local-validator execution requires a separate explicit local-validator-only GO.

## Explicit non-approval

This B6.42 checkpoint does not approve fixture file emission.

It only defines the future fixture file emission plan.

It does not approve local-validator execution.

It does not approve testnet action.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a local fixture file emitter skeleton with hard local-only guards.

No fixture files are emitted by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

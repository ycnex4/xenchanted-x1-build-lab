# Current Design Checkpoint

Current branch target:

    stage-41k6-b2-valid-quorum-live-gated-success

Base:

    main @ c5e3c38

Closed:

    Phase 41K.6 B1 guardian quorum authorization
    D2 mint bypass fix
    B1 post-closure payload binding hardening

Current phase:

    B2: valid quorum live-gated success test

Current goal:

    Prove the positive gated ConsumeGatewayMint path:
    valid prior Ed25519 evidence
    -> payload v2 match
    -> guardian membership
    -> unique quorum
    -> B1C7 authorization
    -> CPI gate
    -> processed_event mark
    -> SPL mint

Production activation:

    Not part of B2.
    B2 remains test-gated.
    B4 remains the activation decision point.

Primary spec:

    docs/gateway/phase-41k6-b2-valid-quorum-live-gated-success.md

## Phase 41K.6 B6.26 local execution layer checkpoint

The B6.11-B6.25 local execution planning layer is now checkpointed in:

docs/gateway/phase-41k6-b6-26-local-execution-layer-checkpoint.md

Current status remains:

LOCAL_ONLY_NOT_DEPLOYABLE

Current decision remains:

NO-GO.

No deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, or submit rehearsal is approved.

## Phase 41K.6 B6.27 blocker closure readiness map

The B6 Strategy 2 blocker closure readiness map is recorded in:

docs/gateway/phase-41k6-b6-27-blocker-closure-readiness-map.md

It maps blockers A-H against the B6.11-B6.26 local execution layer.

No GO blocker is closed by this checkpoint.

Current status remains:

LOCAL_ONLY_NOT_DEPLOYABLE

Current decision remains:

NO-GO.

## Phase 41K.6 B6.28 B1C7 handler boundary map

The B1C7 handler boundary map is recorded in:

docs/gateway/phase-41k6-b6-28-b1c7-handler-boundary-map.md

It defines the future evidence required before blocker C can be considered for closure.

It does not close blocker C.

Current status remains:

LOCAL_ONLY_NOT_DEPLOYABLE

Current decision remains:

NO-GO.

## Phase 41K.6 B6.29 local-validator dry-run design map

The local-validator dry-run design map is recorded in:

docs/gateway/phase-41k6-b6-29-local-validator-dry-run-design-map.md

It defines the future evidence required before blocker H can be considered for closure.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_ONLY_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.30 local-validator fixture inventory map

The local-validator fixture inventory map is recorded in:

docs/gateway/phase-41k6-b6-30-local-validator-fixture-inventory-map.md

It defines the fixture groups required for a future local-validator-only dry-run.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_ONLY_FIXTURE_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.31 local-validator fixture generator design

The local-validator fixture generator design is recorded in:

docs/gateway/phase-41k6-b6-31-local-validator-fixture-generator-design.md

It defines the future design boundary for deterministic local-only fixture generation.

It does not implement a fixture generator.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_ONLY_FIXTURE_GENERATOR_DESIGN_NOT_IMPLEMENTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.32 local-validator fixture generator schema

The local-validator fixture generator schema is recorded in:

docs/gateway/phase-41k6-b6-32-local-validator-fixture-generator-schema.md

It defines the schema for deterministic local-only fixture generation outputs.

It does not implement a fixture generator.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_ONLY_FIXTURE_GENERATOR_SCHEMA_DEFINED_NOT_IMPLEMENTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.33 local-only fixture generator skeleton

The local-only fixture generator skeleton is implemented in:

programs/xxxl-svm/src/local_fixture_generator_skeleton.rs

It provides deterministic local fixture manifest, safety report, program fixture, and pubkey fixture skeletons.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_ONLY_FIXTURE_GENERATOR_SKELETON_NOT_EXECUTABLE_DRY_RUN

Current decision remains:

NO-GO.

## Phase 41K.6 B6.34 local fixture generator safety checkpoint

The local fixture generator safety checkpoint is recorded in:

docs/gateway/phase-41k6-b6-34-local-fixture-generator-safety-checkpoint.md

It confirms that the B6.33 fixture generator remains a local skeleton only.

It does not emit fixture files.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_ONLY_FIXTURE_GENERATOR_SAFETY_CHECKPOINT

Current decision remains:

NO-GO.

## Phase 41K.6 B6.35 local-validator command boundary map

The local-validator command boundary map is recorded in:

docs/gateway/phase-41k6-b6-35-local-validator-command-boundary-map.md

It defines future command boundaries for a local-validator-only dry-run.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_COMMAND_BOUNDARY_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.36 local-validator success/failure matrix design

The local-validator success/failure matrix design is recorded in:

docs/gateway/phase-41k6-b6-36-local-validator-success-failure-matrix.md

It defines the future success path, failure matrix, and no-mutation evidence requirements.

It does not execute a local validator dry-run.

It does not close blocker H.

Current status:

LOCAL_VALIDATOR_MATRIX_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.37 rollback and recovery plan map

The rollback and recovery plan map is recorded in:

docs/gateway/phase-41k6-b6-37-rollback-recovery-plan-map.md

It defines rollback and recovery requirements for local-validator, upgrade, state initialization, SPL mint authority, guardian set, submit rehearsal, and live route activation.

It does not execute rollback or recovery.

It does not close blocker G.

Current status:

ROLLBACK_RECOVERY_PLAN_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.38 upgrade authority custody map

The upgrade authority custody map is recorded in:

docs/gateway/phase-41k6-b6-38-upgrade-authority-custody-map.md

It records the public authority baseline and the remaining custody requirements before any future upgrade can be considered.

It does not execute an upgrade.

It does not close blocker A.

Known public baseline:

- program id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- upgrade authority public key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Current status:

UPGRADE_AUTHORITY_CUSTODY_MAP_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.39 expected post-upgrade ProgramData hash plan

The expected post-upgrade ProgramData hash plan is recorded in:

docs/gateway/phase-41k6-b6-39-post-upgrade-programdata-hash-plan.md

It defines what must be recorded before blocker B can close.

It does not compute or record the final expected hash.

It does not execute a build.

It does not execute an upgrade.

It does not close blocker B.

Current status:

POST_UPGRADE_PROGRAMDATA_HASH_PLAN_DEFINED_HASH_NOT_RECORDED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.40 Strategy 2 closure-readiness checkpoint

The Strategy 2 closure-readiness checkpoint is recorded in:

docs/gateway/phase-41k6-b6-40-strategy-2-closure-readiness-checkpoint.md

It summarizes B6.27 through B6.39.

It records that Strategy 2 is ready for the next decision, not for execution.

It does not close blockers A through H.

It does not execute a local validator dry-run.

It does not approve testnet action.

Current status:

STRATEGY_2_CLOSURE_READINESS_CHECKPOINT_NO_GO

Current decision remains:

NO-GO.

## Phase 41K.6 B6.41 local-validator-only GO form design

The local-validator-only GO form design is recorded in:

docs/gateway/phase-41k6-b6-41-local-validator-go-form-design.md

It defines the required future approval fields for a local-validator-only dry-run.

It does not approve local-validator execution.

It does not execute a local validator dry-run.

It does not approve testnet action.

Current status:

LOCAL_VALIDATOR_ONLY_GO_FORM_DESIGN_NOT_APPROVED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.42 local fixture file emission plan

The local fixture file emission plan is recorded in:

docs/gateway/phase-41k6-b6-42-local-fixture-file-emission-plan.md

It defines the future local-only fixture output files and safety requirements.

It does not implement fixture file emission.

It does not emit fixture files.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_FIXTURE_FILE_EMISSION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.43 local fixture file emitter skeleton

The local fixture file emitter skeleton is implemented in:

programs/xxxl-svm/src/local_fixture_file_emitter_skeleton.rs

It models future local fixture file emission without writing files to disk.

It does not emit fixture files.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_FIXTURE_FILE_EMITTER_SKELETON_NOT_WRITING_FILES

Current decision remains:

NO-GO.

## Phase 41K.6 B6.44 local fixture file emitter safety checkpoint

The local fixture file emitter safety checkpoint is recorded in:

docs/gateway/phase-41k6-b6-44-local-fixture-file-emitter-safety-checkpoint.md

It confirms that the B6.43 local fixture file emitter remains non-writing and non-executing.

It does not emit fixture files.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_FIXTURE_FILE_EMITTER_SAFETY_CHECKPOINT_NO_WRITE

Current decision remains:

NO-GO.

## Phase 41K.6 B6.45 SPL mint authority architecture map

The SPL mint authority architecture map is recorded in:

docs/gateway/phase-41k6-b6-45-spl-mint-authority-architecture-map.md

It defines the intended gateway mint authority PDA and SPL CPI boundary requirements.

It does not configure SPL mint authority.

It does not enable SPL CPI minting.

It does not approve local-validator execution.

It does not approve testnet action.

Known public baseline:

- gateway mint authority PDA: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- gateway mint authority bump: 252
- SPL Token program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Current status:

SPL_MINT_AUTHORITY_ARCHITECTURE_MAP_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.46 guardian set testnet descriptor map

The guardian set testnet descriptor map is recorded in:

docs/gateway/phase-41k6-b6-46-guardian-set-testnet-descriptor-map.md

It defines the future descriptor requirements for guardian set id, threshold, public keys, descriptor integrity, and runtime mapping.

It does not create a guardian descriptor.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

GUARDIAN_SET_TESTNET_DESCRIPTOR_MAP_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.47 local-only guardian descriptor skeleton plan

The local-only guardian descriptor skeleton plan is recorded in:

docs/gateway/phase-41k6-b6-47-local-guardian-descriptor-skeleton-plan.md

It defines the future local-only descriptor skeleton boundary for guardian_set_id, threshold, local public key fixtures, descriptor integrity, route scope, and no-signing policy.

It does not implement a guardian descriptor skeleton.

It does not create a guardian descriptor.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.48 local guardian descriptor skeleton

The local guardian descriptor skeleton is implemented in:

programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs

It models local guardian descriptor structure, threshold behavior, deterministic local public key fixtures, and no-signing safety checks.

It does not create a testnet guardian descriptor.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING

Current decision remains:

NO-GO.

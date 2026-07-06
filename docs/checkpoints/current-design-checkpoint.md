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

## Phase 41K.6 B6.49 local guardian descriptor safety checkpoint

The local guardian descriptor safety checkpoint is recorded in:

docs/gateway/phase-41k6-b6-49-local-guardian-descriptor-safety-checkpoint.md

It confirms that the B6.48 local guardian descriptor skeleton remains local-only, no-signing, no-package-construction, no-testnet, and non-executing.

It does not create a guardian descriptor.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_SAFETY_CHECKPOINT_NO_SIGNING

Current decision remains:

NO-GO.

## Phase 41K.6 B6.50 local guardian descriptor fixture integration plan

The local guardian descriptor fixture integration plan is recorded in:

docs/gateway/phase-41k6-b6-50-local-guardian-fixture-integration-plan.md

It defines how the local guardian descriptor skeleton should later integrate with local fixture generation, fixture file emission, guardian_set account fixtures, scenarios, failure matrix, mutation-invariance checks, logs, and safety reports.

It does not implement fixture integration.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_PLAN_DEFINED_NOT_IMPLEMENTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.51 local guardian fixture integration skeleton

The local guardian fixture integration skeleton is implemented in:

programs/xxxl-svm/src/local_guardian_fixture_integration_skeleton.rs

It links the local guardian descriptor skeleton to local fixture set identity, guardian_set account fixture, threshold model, descriptor failure cases, and mutation-invariance policy.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_SKELETON_NOT_EXECUTING

Current decision remains:

NO-GO.

## Phase 41K.6 B6.52 local guardian fixture integration safety checkpoint

The local guardian fixture integration safety checkpoint is recorded in:

docs/gateway/phase-41k6-b6-52-local-guardian-fixture-integration-safety-checkpoint.md

It confirms that the B6.51 local guardian fixture integration skeleton remains local-only, in-memory, no-signing, no-package-construction, no-file-emission, no-validator, and no-testnet.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_FIXTURE_INTEGRATION_SAFETY_CHECKPOINT_NO_EXECUTION

Current decision remains:

NO-GO.

## Phase 41K.6 B6.53 local guardian failure matrix integration map

The local guardian failure matrix integration map is recorded in:

docs/gateway/phase-41k6-b6-53-local-guardian-failure-matrix-integration-map.md

It maps guardian descriptor and guardian fixture integration failures into future local failure matrix groups, expected no-mutation behavior, log expectations, and safety report expectations.

It does not execute the failure matrix.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_FAILURE_MATRIX_INTEGRATION_MAP_DESIGN_NOT_EXECUTED

Current decision remains:

NO-GO.

## Phase 41K.6 B6.54 local guardian failure matrix skeleton

The local guardian failure matrix skeleton is implemented in:

programs/xxxl-svm/src/local_guardian_failure_matrix_skeleton.rs

It models guardian descriptor and guardian fixture integration failure cases in memory, including no-mutation policy, log expectation ids, and safety report expectation ids.

It does not execute the failure matrix.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_NOT_EXECUTING

Current decision remains:

NO-GO.

## Phase 41K.6 B6.55 guardian local safety lane consolidation

The guardian local safety lane consolidation checkpoint is recorded in:

docs/gateway/phase-41k6-b6-55-guardian-local-safety-lane-consolidation.md

It consolidates B6.45 through B6.54 and records that the guardian descriptor, guardian fixture integration, and guardian failure matrix skeleton work has reached a safe decision boundary.

It does not add another Rust skeleton.

It does not execute the failure matrix.

It does not emit fixture files.

It does not create descriptor files.

It does not construct guardian packages.

It does not enable signing.

It does not approve local-validator execution.

It does not approve testnet action.

Current status:

GUARDIAN_LOCAL_SAFETY_LANE_CONSOLIDATED_READY_FOR_DECISION_NOT_EXECUTION

Current decision remains:

NO-GO.

The next safe step is a decision, not another automatic guardian skeleton.

## Phase 41K.6 B6.56 local-only fixture emission readiness decision map

The local-only fixture emission readiness decision map is recorded in:

docs/gateway/phase-41k6-b6-56-local-only-fixture-emission-readiness-decision-map.md

It defines the future local-only fixture bundle inventory, selected output directory, naming policy, determinism policy, safety checks, cleanup policy, and explicit GO boundary.

It does not implement fixture file emission.

It does not emit fixture files.

It does not create an output directory.

It does not run a local validator.

It does not approve testnet action.

It does not enable signing.

It does not construct guardian packages.

It does not create descriptor files.

It does not approve submit, upgrade, state initialization, SPL mint authority setup, or SPL CPI minting.

Current status:

LOCAL_ONLY_FIXTURE_EMISSION_READINESS_DECISION_MAP_READY_FOR_GO_FORM_NOT_EXECUTION

Current decision remains:

NO-GO.

The next safe step is B6.57 local-only fixture emission GO form, still unapproved.

## Phase 41K.6 B6.57 local-only fixture emission GO form

The local-only fixture emission GO form is recorded in:

docs/gateway/phase-41k6-b6-57-local-only-fixture-emission-go-form.md

It defines the exact future approval form required before actual local-only fixture file emission can be implemented or executed.

It does not grant approval.

It does not implement fixture file emission.

It does not emit fixture files.

It does not create an output directory.

It does not run a local validator.

It does not approve testnet action.

It does not enable signing.

It does not construct guardian packages.

It does not create descriptor files.

It does not approve submit, upgrade, state initialization, SPL mint authority setup, or SPL CPI minting.

Current status:

LOCAL_ONLY_FIXTURE_EMISSION_GO_FORM_DEFINED_NOT_APPROVED

Current decision remains:

NO-GO.

The next safe step is a decision from Sergey. Actual fixture emission remains forbidden without a separate explicit scoped GO.

## Phase 41K.6 B6.57 Theo safety boundary review

Theo's B6.57 safety boundary review is recorded in:

docs/gateway/phase-41k6-b6-57-theo-safety-boundary-review.md

Theo confirmed that the NO-GO boundary after B6.7 through B6.57 remains clean.

Theo approved B6.58 local-only fixture emission with mandatory constraints:

- mock/deterministic data only
- no real private keys
- no seed phrases
- no credentials
- no real upgrade authority keypair
- no authenticated testnet RPC endpoints
- guardian descriptors may contain public mock/deterministic data only
- no local-validator execution
- no testnet action
- no signing
- no guardian package construction
- no SPL mint authority setup
- no SPL CPI minting
- no upgrade
- no state initialization
- no submit

Theo's approval does not replace Sergey explicit scoped GO.

Current status:

THEO_SAFETY_BOUNDARY_REVIEW_RECORDED_B6_58_APPROVED_MOCK_DATA_ONLY_SERGEY_GO_STILL_REQUIRED

Current decision remains:

NO-GO UNTIL SERGEY EXPLICIT B6.58 GO.

The next safe step is a Sergey decision.

## Phase 41K.6 B6.58 actual local-only fixture file emission

The actual local-only fixture file emission checkpoint is recorded in:

docs/gateway/phase-41k6-b6-58-actual-local-only-fixture-file-emission.md

B6.58 adds a host-only local fixture emitter example:

programs/xxxl-svm/examples/emit_local_fixtures_b6_58.rs

B6.58 emits the approved local-only mock fixture bundle to:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

The emitted bundle contains exactly the approved 10 fixture files.

The emitted data is mock/deterministic only.

B6.58 does not run a local validator.

B6.58 does not use testnet.

B6.58 does not use live RPC.

B6.58 does not enable signing.

B6.58 does not construct guardian packages.

B6.58 does not create testnet descriptor files.

B6.58 does not configure SPL mint authority.

B6.58 does not perform SPL CPI minting.

B6.58 does not execute upgrade, state initialization, or submit.

Current status:

LOCAL_ONLY_FIXTURE_FILE_EMISSION_COMPLETED_MOCK_DATA_ONLY_NO_EXECUTION

B6.58 scoped decision:

GO EXECUTED FOR LOCAL MOCK FIXTURE FILE EMISSION ONLY

Global execution decision remains:

NO-GO FOR VALIDATOR_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is B6.59 emitted fixture bundle safety checkpoint.


## Phase 41K.6 B6.59 emitted fixture bundle safety checkpoint

The emitted fixture bundle safety checkpoint is recorded in:

docs/gateway/phase-41k6-b6-59-emitted-fixture-bundle-safety-checkpoint.md

B6.59 verifies the local fixture bundle emitted by B6.58.

Verification result:

- file count: 10
- JSON parse check: OK
- forbidden material scan: OK
- local validator execution: NOT_EXECUTED
- testnet action: NOT_EXECUTED
- signing: NOT_EXECUTED
- SPL setup: NOT_EXECUTED
- upgrade/init/submit: NOT_EXECUTED

Current status:

EMITTED_FIXTURE_BUNDLE_SAFETY_CHECKPOINT_COMPLETED_NO_EXECUTION

Current decision remains:

NO-GO FOR VALIDATOR_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is B6.60 local-validator dry-run GO form / command boundary.


## Phase 41K.6 B6.60 local-validator dry-run GO form / command boundary

The local-validator dry-run GO form and command boundary is recorded in:

docs/gateway/phase-41k6-b6-60-local-validator-dry-run-go-form.md

B6.60 is form-only.

B6.60 does not run a local validator.

B6.60 does not use testnet, live RPC, real signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

Current status:

LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NOT_APPROVED

Current decision remains:

NO-GO FOR LOCAL_VALIDATOR_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is a Sergey decision before any B6.61 local-validator dry-run planning or execution.


## Phase 41K.6 B6.61 local-validator dry-run planning-only

The local-validator dry-run planning-only boundary is recorded in:

docs/gateway/phase-41k6-b6-61-local-validator-dry-run-planning-only.md

B6.61 converts the B6.60 GO form into a planning-only boundary.

B6.61 does not provide a runnable validator command.

B6.61 does not run a local validator.

B6.61 does not use testnet, live RPC, real signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

Current status:

LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_DEFINED_NO_EXECUTION

Current decision remains:

NO-GO FOR LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is either Theo review of B6.58 through B6.61 or a separate B6.62 command-boundary checkpoint with no execution.


## Phase 41K.6 B6.62 Theo review package for B6.58 through B6.61

The Theo review package is recorded in:

docs/gateway/phase-41k6-b6-62-theo-review-package-b6-58-through-b6-61.md

B6.62 prepares a review package covering:

- B6.58 actual local-only fixture file emission
- B6.59 emitted fixture bundle safety checkpoint
- B6.60 local-validator dry-run GO form / command boundary
- B6.61 local-validator dry-run planning-only boundary

B6.62 is review-package only.

B6.62 does not run a local validator.

B6.62 does not provide a runnable validator command.

B6.62 does not use testnet, live RPC, real signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

Current status:

THEO_REVIEW_PACKAGE_PREPARED_FOR_B6_58_THROUGH_B6_61_NO_EXECUTION

Current decision remains:

NO-GO FOR LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is to send B6.62 to Theo and wait for review before any B6.63 command-boundary work.


## Phase 41K.6 B6.62R Theo verdict record

Theo's verdict for B6.58 through B6.62 is recorded in:

docs/gateway/phase-41k6-b6-62r-theo-verdict-record.md

Theo assessed B6.58 through B6.61 as clean and confirmed:

- no validator execution
- no runnable command
- no testnet
- no live RPC
- no signing
- no real keys
- no guardian packages
- no SPL setup
- no upgrade/init/submit

Theo approved:

APPROVE B6.63 COMMAND-BOUNDARY NO-EXECUTION

Theo also noted a non-blocking B6.59 documentation gap: the forbidden-material scan taxonomy should be documented more explicitly.

Mandatory B6.63 guards:

- execution prevention by default
- mock data only from tmp/local-validator-fixtures/
- Blocker H gate preserved with BLOCKER_H_NOT_CLOSED log and exit
- no implicit testnet fallback; fail closed, not open

Current status:

THEO_VERDICT_RECORDED_B6_63_COMMAND_BOUNDARY_NO_EXECUTION_APPROVED

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is B6.63 command-boundary definition with no execution.


## Phase 41K.6 B6.63 command-boundary no-execution

The B6.63 command-boundary no-execution checkpoint is recorded in:

docs/gateway/phase-41k6-b6-63-command-boundary-no-execution.md

B6.63 adds the command-boundary script:

scripts/gateway/b6_63_local_validator_command_boundary_no_execution.sh

B6.63 carries Theo's mandatory guards:

- execution prevention by default
- mock data only from tmp/local-validator-fixtures/
- Blocker H gate preserved with BLOCKER_H_NOT_CLOSED
- no implicit testnet fallback; fail closed, not open

Verification result:

- bash syntax check: OK
- default no-execution run: OK
- fixture file count: 10
- JSON check: OK
- forbidden-material taxonomy scan: OK
- fixture boundary: LOCAL_TMP_ONLY
- --execute refusal: OK
- --execute refusal exit code: 63

Current status:

COMMAND_BOUNDARY_DEFINED_NO_EXECUTION_BLOCKER_H_STILL_GATED

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is B6.64 command-boundary safety checkpoint / Theo review package for B6.63.


## Phase 41K.6 B6.64 command-boundary safety checkpoint / Theo review package

The B6.64 command-boundary safety checkpoint and Theo review package is recorded in:

docs/gateway/phase-41k6-b6-64-command-boundary-safety-checkpoint-theo-review-package.md

B6.64 verifies the B6.63 command-boundary script:

scripts/gateway/b6_63_local_validator_command_boundary_no_execution.sh

Verification result:

- syntax check: OK
- default blocker H gate: OK
- default local-validator execution: NOT_EXECUTED
- no testnet fallback: OK
- --execute refusal: OK
- --execute refusal exit code: 63
- execute blocker H gate: OK
- testnet action: NOT_EXECUTED
- signing: NOT_EXECUTED
- SPL setup: NOT_EXECUTED
- upgrade/init/submit: NOT_EXECUTED

Current status:

COMMAND_BOUNDARY_SAFETY_CHECKPOINT_READY_FOR_THEO_REVIEW_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is to send B6.64 to Theo for review.


## Phase 41K.6 B6.64R Theo verdict record and exit-code comment

Theo's verdict for B6.63/B6.64 is recorded in:

docs/gateway/phase-41k6-b6-64r-theo-verdict-record-exit-code-comment.md

Theo approved B6.63/B6.64 and confirmed that the B6.63 command-boundary script is fail-closed.

Theo requested one documentation comment:

# Exit 63 = BLOCKER_H_NOT_CLOSED

B6.64R adds that comment to:

scripts/gateway/b6_63_local_validator_command_boundary_no_execution.sh

Verification result after the comment:

- syntax check: OK
- default blocker H gate: OK
- default local-validator execution: NOT_EXECUTED
- no testnet fallback: OK
- --execute refusal: OK
- --execute refusal exit code: 63
- exit 63 comment: OK
- testnet action: NOT_EXECUTED
- signing: NOT_EXECUTED
- SPL setup: NOT_EXECUTED
- upgrade/init/submit: NOT_EXECUTED

Current status:

THEO_VERDICT_RECORDED_B6_63_B6_64_APPROVED_EXIT_63_COMMENT_ADDED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is Blocker H.1 local-validator dry-run planning step.


## Blocker H.1 local-validator dry-run planning-only

The Blocker H.1 planning-only checkpoint is recorded in:

docs/gateway/blocker-h-1-local-validator-dry-run-planning-only.md

H.1 opens the Blocker H local-validator dry-run lane as planning-only.

H.1 does not run a local validator.

H.1 does not provide an actual runnable validator command.

H.1 does not use testnet, live RPC, signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

H.1 records that B6.63/B6.64/B6.64R are ready and that the B6.63 command-boundary script is fail-closed.

Blocker H remains OPEN and GATED.

Current status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_PLANNING_ONLY_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is Blocker H.2 preflight checklist definition with no execution.


## Blocker H.2 local-validator preflight checklist, no execution

The Blocker H.2 preflight checklist checkpoint is recorded in:

docs/gateway/blocker-h-2-local-validator-preflight-checklist-no-execution.md

H.2 defines and verifies the preflight checklist required before any future actual Blocker H local-validator dry-run can be considered.

Verification result:

- fixture directory: OK
- fixture file count: 10
- JSON parse: OK
- forbidden-material taxonomy scan: OK
- B6.63 script exists: OK
- B6.63 script syntax: OK
- default blocker H gate: OK
- default local-validator execution: NOT_EXECUTED
- no testnet fallback: OK
- --execute refusal: OK
- --execute exit code: 63
- exit 63 comment: OK
- testnet action: NOT_EXECUTED
- signing: NOT_EXECUTED
- SPL setup: NOT_EXECUTED
- upgrade/init/submit: NOT_EXECUTED

Current status:

BLOCKER_H_PREFLIGHT_CHECKLIST_DEFINED_AND_VERIFIED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is Blocker H.3 dry-run execution plan / GO form, still no execution unless separately approved.


## Blocker H.3 local-validator dry-run GO form, no execution

The Blocker H.3 dry-run GO form checkpoint is recorded in:

docs/gateway/blocker-h-3-local-validator-dry-run-go-form-no-execution.md

H.3 defines the future explicit GO form and execution boundary for a possible actual local-validator dry-run.

H.3 does not run a local validator.

H.3 does not add an actual runnable validator execution command.

H.3 does not use testnet, live RPC, signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

Future explicit GO phrase:

I approve Blocker H actual local-validator dry-run only, scoped to local disposable validator state and the verified mock fixture bundle, with no testnet RPC, no live RPC, no real signing keys, no real guardian packages, no SPL mint authority setup against real assets, no SPL CPI minting against real assets, no program upgrade, no persistent state initialization outside the local validator, and no submit to any network.

Current status:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_GO_FORM_DEFINED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is either Theo review of H.1 through H.3 or Blocker H.4 execution-readiness review package with no execution.


## Blocker H.3R Theo verdict record for H.1 through H.3

Theo's verdict for Blocker H.1 through H.3 is recorded in:

docs/gateway/blocker-h-3r-theo-verdict-record-h1-through-h3.md

Theo approved H.1 through H.3 and confirmed that each H sub-blocker is planning-only, correctly scoped, and preserves the execution gate.

Theo confirmed that the B6.63/B6.64 fail-closed command-boundary script remains the only runnable artifact and that H.1 through H.3 did not add any new executable path.

Theo approved proceeding to:

Blocker H.4 execution-readiness review package with no execution.

H.4 scope expectation:

- readiness assessment against the H.2 preflight checklist
- no execution
- not a GO decision

Items to carry into H.4:

- solana-test-validator binary present and correct version
- fixture bundle loaded and JSON-valid
- mock accounts deterministically generated
- no real private keys in fixture directory
- no testnet RPC endpoints in config
- program binary hash matches expected if testing upgrade path
- SPL Token program present in validator genesis
- requester identity
- specific execution scope: local validator dry-run only
- fixture bundle version/hash
- expected program binary hash if upgrade path is tested
- Blocker A through G status: still open, noted
- rollback plan if dry-run produces unexpected state
- sign-off field: empty until explicit GO

Current status:

THEO_VERDICT_RECORDED_H1_THROUGH_H3_APPROVED_H4_REVIEW_PACKAGE_ALLOWED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is Blocker H.4 execution-readiness review package with no execution.


## Blocker H.4 execution-readiness review package, no execution

The Blocker H.4 execution-readiness review package is recorded in:

docs/gateway/blocker-h-4-execution-readiness-review-package-no-execution.md

H.4 reviews readiness against the H.2 preflight checklist.

H.4 is not a GO decision.

H.4 does not run a local validator.

H.4 does not add an actual runnable validator execution command.

H.4 does not use testnet, live RPC, signing, real keys, guardian packages, SPL setup, upgrade, state initialization, or submit.

Readiness evidence:

- solana-test-validator binary: PRESENT
- solana-test-validator version: 4.0.0
- fixture directory: OK
- fixture file count: 10
- JSON parse: OK
- forbidden-material taxonomy scan: OK
- fixture bundle SHA256: 0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7
- B6.63 script exists: OK
- B6.63 script syntax: OK
- default blocker H gate: OK
- default local-validator execution: NOT_EXECUTED
- no testnet fallback: OK
- --execute refusal: OK
- --execute exit code: 63
- exit 63 comment: OK
- testnet action: NOT_EXECUTED
- signing: NOT_EXECUTED
- SPL setup: NOT_EXECUTED
- upgrade/init/submit: NOT_EXECUTED

Current status:

BLOCKER_H_EXECUTION_READINESS_REVIEW_PACKAGE_COMPLETED_NO_EXECUTION_NOT_GO_DECISION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is to send H.4 to Theo for review.


## Blocker H.4R Theo verdict record for H.4 execution-readiness

Theo's verdict for Blocker H.4 is recorded in:

docs/gateway/blocker-h-4r-theo-verdict-record-h4-readiness.md

Theo approved H.4 as an execution-readiness review package.

Theo confirmed H.4 covers the H.2 preflight checklist and adds a fixture bundle fingerprint.

Integrity anchor:

0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7

Theo stated that any fixture modification between H.4 and actual execution requires a new H.4 cycle.

Theo approved proceeding to:

Blocker H.5 GO decision step, separately gated.

H.5 is not automatic execution.

H.5 must reference the exact fixture SHA256 above.

Current status:

THEO_VERDICT_RECORDED_H4_READINESS_APPROVED_H5_GO_DECISION_STEP_ALLOWED_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is Blocker H.5 GO decision step.


## Blocker H.5 GO decision step, no execution

The Blocker H.5 GO decision checkpoint is recorded in:

docs/gateway/blocker-h-5-go-decision-step-no-execution.md

H.5 defines the decision surface for a possible future actual local-validator dry-run.

H.5 is not actual execution approval.

H.5 does not run a local validator.

H.5 does not add an actual runnable validator execution command.

Fixture integrity anchor:

0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7

Requester:

Sergey Stepanenko

Actual execution sign-off:

EMPTY — no explicit GO for actual local-validator execution has been given in H.5.

Current status:

BLOCKER_H_GO_DECISION_SURFACE_DEFINED_SIGN_OFF_EMPTY_NO_EXECUTION

Current decision remains:

NO-GO FOR BLOCKER_H_LOCAL_VALIDATOR_EXECUTION_TESTNET_SIGNING_SPL_UPGRADE_INIT_SUBMIT

The next safe step is a Sergey decision. Actual Blocker H local-validator dry-run requires a separate explicit scoped GO referencing the fixture SHA256.


## Blocker H.5R explicit scoped GO record

The Blocker H.5R explicit scoped GO record is recorded in:

docs/gateway/blocker-h-5r-explicit-scoped-go-record.md

Sergey provided the explicit scoped GO phrase for future H.6 local-validator dry-run.

Approved fixture bundle SHA256:

0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7

Approved future scope:

- Blocker H actual local-validator dry-run only
- local disposable validator state
- verified mock fixture bundle only
- no testnet RPC
- no live RPC
- no real signing keys
- no real guardian packages
- no SPL mint authority setup against real assets
- no SPL CPI minting against real assets
- no program upgrade
- no persistent state initialization outside the local validator
- no submit to any network

H.5R does not run a local validator.

H.5R does not add an actual runnable validator execution command.

Current status:

EXPLICIT_SCOPED_GO_RECORDED_FOR_H6_LOCAL_VALIDATOR_DRY_RUN_NO_EXECUTION_IN_H5R

Current decision:

GO RECORDED FOR H6 LOCAL VALIDATOR DRY-RUN ONLY WITH STRICT LOCAL DISPOSABLE SCOPE

NO-GO REMAINS FOR TESTNET_SIGNING_REAL_KEYS_GUARDIAN_PACKAGES_SPL_REAL_SETUP_PROGRAM_UPGRADE_PERSISTENT_INIT_NETWORK_SUBMIT

The next step may be Blocker H.6 actual local-validator dry-run execution, strictly within the recorded H.5R scope and only after preflight verification.


## Blocker H.6 actual local-validator dry-run

The Blocker H.6 execution evidence checkpoint is recorded in:

docs/gateway/blocker-h-6-actual-local-validator-dry-run.md

H.6 executed the first actual local-validator dry-run under the H.5R explicit scoped GO.

Execution evidence:

- local validator execution: EXECUTED
- RPC URL: http://127.0.0.1:8899
- ledger directory: tmp/local-validator-ledgers/blocker-h-6-disposable-ledger
- solana-test-validator version: 4.0.0
- cluster version: 4.0.0
- health check: OK
- validator stopped: OK
- fixture bundle SHA256: 0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7
- testnet action: NOT_EXECUTED
- live RPC action: NOT_EXECUTED
- signing: NOT_EXECUTED
- real keys: NOT_USED
- guardian packages: NOT_CONSTRUCTED
- SPL setup: NOT_EXECUTED
- program upgrade: NOT_EXECUTED
- state initialization: NOT_EXECUTED
- network submit: NOT_EXECUTED

Current status:

BLOCKER_H_ACTUAL_LOCAL_VALIDATOR_DRY_RUN_EXECUTED_SUCCESSFULLY_LOCAL_ONLY

Current decision:

BLOCKER_H_LOCAL_VALIDATOR_DRY_RUN_EXECUTED_WITHIN_H5R_SCOPED_GO

NO-GO REMAINS FOR TESTNET_SIGNING_REAL_KEYS_GUARDIAN_PACKAGES_SPL_REAL_SETUP_PROGRAM_UPGRADE_PERSISTENT_INIT_NETWORK_SUBMIT

The next safe step is Blocker H.6R execution result review package.


## Blocker H.6R execution result review package

The Blocker H.6R execution result review package is recorded in:

docs/gateway/blocker-h-6r-execution-result-review-package.md

H.6R reviews and records the H.6 actual local-validator dry-run result.

H.6R does not run the validator again.

Reviewed result:

- local validator execution: EXECUTED in H.6
- health check: OK
- validator stopped: OK
- solana-test-validator version: 4.0.0
- cluster version: 4.0.0
- fixture bundle SHA256: 0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7
- testnet action: NOT_EXECUTED
- live RPC action: NOT_EXECUTED
- signing: NOT_EXECUTED
- real keys: NOT_USED
- guardian packages: NOT_CONSTRUCTED
- SPL setup: NOT_EXECUTED
- program upgrade: NOT_EXECUTED
- state initialization: NOT_EXECUTED
- network submit: NOT_EXECUTED

Current status:

BLOCKER_H6_EXECUTION_RESULT_REVIEW_PACKAGE_COMPLETED_NO_FURTHER_EXECUTION

Current decision:

H6_LOCAL_VALIDATOR_HEALTH_DRY_RUN_COMPLETED_SUCCESSFULLY_WITHIN_H5R_SCOPE

NO-GO REMAINS FOR TESTNET_SIGNING_REAL_KEYS_GUARDIAN_PACKAGES_SPL_REAL_SETUP_PROGRAM_UPGRADE_PERSISTENT_INIT_NETWORK_SUBMIT

The next safe step is to send H.6/H.6R to Theo for review.


## Blocker H.6RV Theo verdict and Blocker H closure

The Blocker H.6RV Theo verdict record is recorded in:

docs/gateway/blocker-h-6rv-theo-verdict-close-blocker-h.md

Theo reviewed H.6/H.6R and gave the narrow verdict:

Blocker H local-validator health dry-run is complete.

Closure decision:

BLOCKER H CLOSED.

Closed scope:

- local validator started
- local validator responded to health check
- local validator stopped cleanly
- fixture bundle SHA256 verified: 0a3290df47782008f72b441d9b9bf36964003324dde5baaf03f5fb0a04b19da7
- no testnet
- no live RPC
- no signing
- no real keys
- no state mutation
- all forbidden paths remained NOT_EXECUTED

Current status:

BLOCKER_H_CLOSED_LOCAL_VALIDATOR_HEALTH_DRY_RUN_COMPLETED

Current decision:

BLOCKER_H_CLOSED_FOR_NARROW_LOCAL_VALIDATOR_HEALTH_DRY_RUN_SCOPE

NO-GO REMAINS FOR TESTNET_SIGNING_REAL_KEYS_GUARDIAN_PACKAGES_SPL_REAL_SETUP_PROGRAM_UPGRADE_PERSISTENT_INIT_NETWORK_SUBMIT

Still open:

- Blocker A: upgrade authority
- Blocker B: ProgramData hash
- Blocker C: B1C7 handler
- Blocker D: state initialization design
- Blocker E: SPL mint architecture
- Blocker F: guardian descriptor
- Blocker G: rollback plan

Future program-load testing, state initialization simulation, fixture consumption testing, SPL testing, signing, or testnet work must be opened as separately scoped work with its own GO boundary.


## Blocker H post-closure tmp ignore safety

The post-closure tmp ignore safety record is recorded in:

docs/gateway/blocker-h-post-closure-tmp-ignore-safety.md

Purpose:

Protect local disposable runtime output from accidental commit after Blocker H closure.

Git ignore rule:

tmp/

Rule status:

ADDED_TMP_IGNORE_RULE

This safety step does not run the validator, does not modify fixtures, and does not modify disposable ledger contents.

Blocker H remains closed.

NO-GO remains for testnet, signing, SPL setup, program upgrade, persistent initialization, and network submit.


## Blocker A.1 upgrade authority discovery planning-only

The Blocker A.1 planning record is recorded in:

docs/gateway/blocker-a-1-upgrade-authority-discovery-planning-only.md

Purpose:

Define the safe discovery and evidence boundary for Blocker A upgrade authority.

A.1 is planning-only.

A.1 does not call RPC, does not use testnet, does not use live RPC, does not use keys, does not sign, does not inspect live ProgramData, does not modify state, and does not submit to any network.

Blocker A remains open.

Current status:

BLOCKER_A_OPEN_DISCOVERY_PLANNING_ONLY_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

The next safe step is Blocker A.2 repo-only upgrade authority source discovery.


## Blocker A.2 repo-grounded upgrade authority status reconciliation

The Blocker A.2 repo-grounded reconciliation record is recorded in:

docs/gateway/blocker-a-2-repo-grounded-upgrade-authority-status-reconciliation.md

A.2 reviewed existing tracked repository anchors instead of doing a blind scan.

Repo-grounded public baseline from B6.38:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- x1_testnet_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Repo-grounded runtime status:

- runtime remains scaffold-only
- Program ID placeholder boundary remains active
- deployable_path_ready is false
- deployment status remains deployable=false
- live route remains disabled
- SPL CPI execution remains disabled
- production guardian set remains unset
- production proof log remains unset
- external review remains incomplete

Relationship to Blocker H:

Blocker H is closed only for narrow local-validator health dry-run. It does not approve program-load testing, state initialization simulation, fixture consumption, SPL testing, signing, testnet RPC, live RPC, upgrade, persistent initialization, or submit.

A.2 conclusion:

Blocker A remains open.

Current status:

BLOCKER_A_OPEN_REPO_GROUNDED_RECONCILIATION_COMPLETED_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

Corrected next safe step:

A.3 repo-only authority model decision record.


## Blocker A.3 repo-only authority model decision record

The Blocker A.3 authority model decision record is recorded in:

docs/gateway/blocker-a-3-repo-only-authority-model-decision-record.md

Selected repo-grounded authority model:

TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION

Candidate future Blocker A closure state after live read-only evidence:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Meaning:

Upgrade authority is tolerated only as a temporary staged-finalization mechanism, not as mint authority and not as discretionary supply control.

Carried-forward public baseline:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- x1_testnet_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

A.3 does not call RPC, does not use testnet, does not use live RPC, does not use keys, does not sign, does not inspect live ProgramData, does not deploy, does not upgrade, does not initialize state, does not configure SPL, and does not submit to any network.

A.3 does not close Blocker A.

Runtime blockers remain active:

- PLACEHOLDER_PROGRAM_ID
- deployable_path_ready=false
- deployable=false
- live route disabled
- SPL CPI execution disabled
- production guardian set unset
- production proof log unset
- external review incomplete

Current status:

BLOCKER_A_OPEN_REPO_ONLY_AUTHORITY_MODEL_SELECTED_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

AUTHORITY_MODEL_SELECTED_TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION_BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

The next safe step is A.4 read-only live ProgramData evidence GO form, still with no RPC execution.


## Blocker A.4 read-only live ProgramData evidence GO form

The Blocker A.4 GO form is recorded in:

docs/gateway/blocker-a-4-read-only-programdata-evidence-go-form.md

A.4 defines the explicit scoped GO phrase required before A.5 may perform read-only live ProgramData evidence.

A.4 does not call RPC, does not use testnet, does not use live RPC, does not use keys, does not sign, does not inspect live ProgramData, does not deploy, does not upgrade, does not initialize state, does not configure SPL, does not construct guardian packages, and does not submit to any network.

Future A.5 target:

- network: X1 testnet
- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected_observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Required explicit GO phrase for A.5:

I approve Blocker A.5 read-only live ProgramData evidence only, scoped to X1 testnet RPC https://rpc.testnet.x1.xyz, program id D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my, ProgramData account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T, and expected observed upgrade authority DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc, using read-only ProgramData inspection only, with no signing, no keypair use, no deploy, no write-buffer, no set-upgrade-authority, no close, no upgrade, no state initialization, no SPL setup, no guardian package construction, and no submit or mutation.

Current sign-off:

NOT PROVIDED IN A.4

Current status:

BLOCKER_A_OPEN_READ_ONLY_PROGRAMDATA_EVIDENCE_GO_FORM_DEFINED_NO_RPC_NO_EXECUTION

Current decision:

A5_READ_ONLY_LIVE_PROGRAMDATA_EVIDENCE_REQUIRES_EXPLICIT_SCOPED_GO

NO-GO REMAINS_FOR_TESTNET_RPC_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT


## Blocker A.5 read-only live ProgramData evidence

The Blocker A.5 read-only live ProgramData evidence record is recorded in:

docs/gateway/blocker-a-5-read-only-live-programdata-evidence.md

Evidence directory:

docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence

A.5 executed only read-only RPC inspection using explicit --url.

A.5 did not use signing, did not use keypairs, did not deploy, did not write-buffer, did not set-upgrade-authority, did not close, did not upgrade, did not initialize state, did not configure SPL, did not construct guardian packages, did not submit, and did not mutate network state.

ProgramData evidence status:

READ_ONLY_PROGRAMDATA_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Observed:

- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true
- program_show_exit_code: 0
- program_account_exit_code: 0

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_REVIEW

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

A.5R review package for Theo / closure assessment


## Blocker A.5R read-only evidence review package

The Blocker A.5R review package is recorded in:

docs/gateway/blocker-a-5r-read-only-evidence-review-package.md

A.5R reviews the matched A.5 read-only live ProgramData evidence.

A.5 evidence result:

- evidence_status: READ_ONLY_PROGRAMDATA_EVIDENCE_MATCHED_EXPECTED_AUTHORITY
- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true
- program_show_exit_code: 0
- program_account_exit_code: 0
- signing_used: false
- mutation_executed: false

A.5R does not call RPC, does not use keys, does not sign, does not deploy, does not upgrade, does not initialize state, does not configure SPL, does not construct guardian packages, does not submit, and does not mutate network state.

A.5R prepares the closure assessment for Blocker A.

Candidate closure state:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Current status:

BLOCKER_A_REVIEW_READY_READ_ONLY_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

External review / closure decision for Blocker A.


## Blocker A.6 closure decision record

The Blocker A.6 closure decision record is recorded in:

docs/gateway/blocker-a-6-closure-decision-record.md

Blocker A is now CLOSED narrowly as:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Closure basis:

- A.1 planning boundary completed
- A.2 repo-grounded reconciliation completed
- A.3 selected TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION
- A.4 defined explicit read-only GO form
- A.5 executed read-only live ProgramData evidence
- A.5R reviewed matched evidence

Accepted live read-only evidence:

- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true
- program_show_exit_code: 0
- program_account_exit_code: 0
- signing_used: false
- mutation_executed: false

A.6 does not approve signing, keypair use, deploy, write-buffer, set-upgrade-authority, close, upgrade, state initialization, SPL setup, SPL CPI minting, guardian package construction, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash
- C — OPEN: B1C7 handler production/testnet boundary
- D — OPEN: state initialization design
- E — OPEN: SPL mint authority architecture
- F — OPEN: guardian descriptor
- G — OPEN: rollback / recovery plan

Current status:

BLOCKER_A_CLOSED_AS_UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Current decision:

BLOCKER_A_CLOSED_NARROW_AUTHORITY_MODEL_ONLY

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker C.1 — B1C7 handler production/testnet boundary planning.


## Blocker C.1 B1C7 handler production/testnet boundary planning

The Blocker C.1 planning record is recorded in:

docs/gateway/blocker-c-1-b1c7-handler-production-testnet-boundary-planning.md

C.1 opens the B1C7 handler production/testnet boundary track after Blocker A was closed narrowly.

Current repo-grounded C status:

- B1C7 integration exists behind feature gate: phase-41k6-b1c7-handler-integration-test-gate
- dangerous SBF build allow feature is separately named and explicit
- default consume_gateway_mint path rejects with CpiBoundaryNotReady when the B1C7 gate is not enabled
- LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED remains false
- deployment_status.rs still records deployable=false
- program_id_status.rs still records PLACEHOLDER_PROGRAM_ID_BOUNDARY
- lib.rs still records SCAFFOLD_ONLY_NOT_DEPLOYABLE

C.1 does not activate the handler, does not change code, does not call RPC, does not use testnet, does not use keys, does not sign, does not deploy, does not upgrade, does not initialize state, does not configure SPL, does not construct guardian packages, does not submit, and does not mutate.

Blocker C remains open.

Current status:

BLOCKER_C_OPEN_B1C7_HANDLER_BOUNDARY_PLANNING_ONLY_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker C.2 — repo-grounded B1C7 handler inventory.


## Blocker C.2 repo-grounded B1C7 handler inventory

The Blocker C.2 inventory record is recorded in:

docs/gateway/blocker-c-2-repo-grounded-b1c7-handler-inventory.md

Evidence directory:

docs/gateway/evidence/blocker-c-2-repo-grounded-b1c7-handler-inventory

C.2 inventoried the B1C7 handler path in tracked repository files only.

Static inventory result:

- cargo_b1c7_feature_present: true
- cargo_b1c7_dangerous_allow_present: true
- processor_compile_error_for_b1c7_without_dangerous_allow: true
- processor_live_route_flag_false: true
- processor_default_path_fails_cpi_not_ready: true
- processor_b1c7_handler_boundary_present: true
- processor_b1c7_authorization_from_inputs_present: true
- processor_b1c7_atomic_boundary_present: true
- account_contract_b1_v3_12_account_contract_present: true
- cpi_execution_false_default_present: true
- cpi_execution_true_requires_b1c7_and_dangerous_allows: true
- deployment_still_not_deployable: true
- program_id_placeholder_active: true
- handler_calls_authorization_before_atomic_mark_and_mint: true
- atomic_boundary_checks_cpi_gate_before_atomic_mark_and_mint_call: true
- atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: true

C.2 interpretation:

- the repo has a meaningful B1C7 handler path
- the path remains integration/test-gated
- default consume_gateway_mint fails closed with CpiBoundaryNotReady
- live route activation remains false
- SPL CPI execution remains false by default
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active

C.2 does not close Blocker C.

C.2 does not activate the handler, does not change code, does not call RPC, does not use testnet, does not use keys, does not sign, does not deploy, does not upgrade, does not initialize state, does not configure SPL, does not construct guardian packages, does not submit, and does not mutate.

Current status:

BLOCKER_C_OPEN_REPO_GROUNDED_B1C7_HANDLER_INVENTORY_COMPLETED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker C.3 — B1C7 production/testnet activation decision model.

## C.2R order-check correction

The original C.2 static check reported:

atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: false

This was a tooling artifact, not a runtime gap.

Reason:

The check used a whole-file string index and matched the import/use occurrence of guarded_mint_to_cpi_execution_gate_boundary before the function body.

Corrected function-scoped check:

- function: atomic_mark_and_mint_boundary
- mark_processed_event_atomic_call_line: 556
- guarded_mint_to_cpi_execution_gate_boundary_call_line: 571
- mark_before_guarded_cpi_call: true

Corrected status:

atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: true

C.2 remains inventory-only and still does not close Blocker C.



## Blocker C.3 B1C7 production/testnet activation decision model

The Blocker C.3 activation decision model is recorded in:

docs/gateway/blocker-c-3-b1c7-production-testnet-activation-decision-model.md

Evidence directory:

docs/gateway/evidence/blocker-c-3-b1c7-production-testnet-activation-decision-model

C.3 decision:

B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

Selected future model:

REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT

C.3 rejects direct activation of the existing dangerous test-gate feature set as a deployable or testnet-intended route.

Reason:

The current feature names and compile-error text intentionally describe the path as non-production integration/test-gated and dangerous for deploy artifacts.

C.3 requires a future reviewed testnet-intended B1C7 handler route/boundary before any deployable artifact can be built or accepted.

C.3 does not close Blocker C.

C.3 does not activate the handler, does not change code, does not call RPC, does not use testnet, does not use keys, does not sign, does not deploy, does not upgrade, does not initialize state, does not configure SPL, does not construct guardian packages, does not submit, and does not mutate.

Current status:

BLOCKER_C_OPEN_B1C7_ACTIVATION_DECISION_MODEL_RECORDED_NO_CODE_CHANGE_NO_RPC_NO_EXECUTION

Current decision:

B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker C.4 — B1C7 invariant review package.


## Blocker C.4 B1C7 invariant review package

The Blocker C.4 invariant review package is recorded in:

docs/gateway/blocker-c-4-b1c7-invariant-review-package.md

Evidence directory:

docs/gateway/evidence/blocker-c-4-b1c7-invariant-review-package

Invariant result:

- handler_entrypoint_present: true
- handler_authorization_call_before_atomic_boundary_call: true
- authorization_account_contract_asserted: true
- authorization_guardian_set_loaded: true
- authorization_prior_instructions_loaded: true
- authorization_payload_context_constructed: true
- authorization_established_before_status_gate: true
- authorization_status_gate_before_mutation: true
- atomic_boundary_rechecks_authorized_status: true
- atomic_boundary_rechecks_fail_fast_before_mutation: true
- atomic_boundary_rechecks_prior_ed25519_evidence: true
- atomic_boundary_rechecks_payload_hash_binding: true
- atomic_boundary_rechecks_guardian_membership: true
- atomic_boundary_rechecks_quorum: true
- atomic_boundary_checks_cpi_gate_before_mark_and_mint_boundary: true
- atomic_mark_boundary_marks_before_guarded_cpi: true
- default_non_b1c7_path_fails_closed: true
- b1_v3_account_contract_present: true
- b1_v3_account_contract_has_instructions_sysvar: true
- b1_v3_account_contract_asserts_sysvar_key: true
- cpi_gate_false_default_present: true
- cpi_gate_true_requires_b1c7_and_dangerous_allows: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

Line map:

- handler_line: 206
- auth_call_line: 213
- atomic_call_line: 221
- auth_fn_line: 232
- account_contract_assert_line: 238
- guardian_load_line: 254
- prior_load_line: 273
- payload_context_line: 285
- auth_establish_line: 294
- auth_status_gate_line: 300
- atomic_fn_line: 310
- atomic_status_gate_line: 318
- fail_fast_line: 320
- evidence_line: 321
- payload_hash_line: 322
- guardian_membership_line: 323
- quorum_line: 324
- cpi_gate_line: 329
- atomic_mark_mint_call_line: 334
- default_fail_closed_line: 392
- default_fail_error_line: 393
- atomic_mark_fn_line: 525
- mark_line: 556
- guarded_cpi_line: 571

all_invariants_passed: true

C.4 supports the conclusion that the B1C7 handler boundary has a coherent invariant structure in repo source.

C.4 does not approve direct dangerous test-gate deployment.

C.4 does not replace the need for a future reviewed testnet-intended handler route before any deployable artifact.

C.4 does not close Blocker C.

Current status:

BLOCKER_C_REVIEW_READY_B1C7_INVARIANTS_RECORDED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker C.5 — B1C7 boundary closure decision record.

## C.4R authorization status gate call-graph correction

The original C.4 static check reported:

authorization_status_gate_before_mutation: false

This was a tooling artifact, not a runtime gap.

Reason:

The check compared source line numbers across different functions. It compared the authorization status gate inside establish_b1c7_consume_gateway_mint_authorization_from_handler_inputs with the atomic boundary call inside b1c7_authorized_consume_gateway_mint_handler_boundary.

Corrected call-graph check:

- handler_line: 206
- handler_auth_call_line: 213
- handler_atomic_boundary_call_line: 221
- handler_calls_authorization_before_atomic_boundary: true
- authorization_function_line: 232
- authorization_establish_line: 294
- authorization_status_gate_line: 300
- authorization_return_line: 306
- authorization_function_gates_status_before_return: true
- atomic_boundary_function_line: 310
- atomic_boundary_authorized_recheck_line: 318
- atomic_boundary_fail_fast_recheck_line: 320
- atomic_boundary_rechecks_authorization_before proceeding: true

Corrected invariant:

authorization_status_gate_before_mutation: true

Corrected aggregate:

all_invariants_passed: true

C.4 remains repo-only invariant review evidence and still does not close Blocker C.



## Blocker C.5 B1C7 boundary closure decision record

The Blocker C.5 closure decision record is recorded in:

docs/gateway/blocker-c-5-b1c7-boundary-closure-decision-record.md

Blocker C is now CLOSED narrowly as:

B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Closure basis:

- C.1 opened B1C7 production/testnet boundary planning
- C.2 completed repo-grounded handler inventory
- C.2R corrected the order-check tooling artifact
- C.3 rejected direct dangerous test-gate activation
- C.3 selected future model REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT
- C.4 completed B1C7 invariant review
- C.4R corrected the authorization status gate call-graph artifact

Accepted C.4 invariant result:

all_invariants_passed: true

C.5 does not approve handler activation, live route activation, direct dangerous test-gate deployment, signing, keypair use, deploy, write-buffer, set-upgrade-authority, close, upgrade, state initialization, SPL setup, SPL CPI minting, guardian package construction, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash
- D — OPEN: state initialization design
- E — OPEN: SPL mint authority architecture
- F — OPEN: guardian descriptor
- G — OPEN: rollback / recovery plan

Current status:

BLOCKER_C_CLOSED_NARROW_B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Current decision:

BLOCKER_C_CLOSED_NARROW_BOUNDARY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker D.1 — state initialization design planning.


## Blocker D.1 state initialization design planning

The Blocker D.1 planning record is recorded in:

docs/gateway/blocker-d-1-state-initialization-design-planning.md

D.1 opens the state initialization design track after:

- Blocker A closed narrowly as upgrade authority present but accepted for test phase
- Blocker C closed narrowly as B1C7 handler boundary / invariants only

Current repo-grounded state facts:

- MINT_STATE_ACCOUNT_LEN = 176
- GATEWAY_CONFIG_ACCOUNT_LEN = 256
- GUARDIAN_SET_ACCOUNT_LEN = 320
- PROCESSED_EVENT_ACCOUNT_LEN = 144
- RECIPIENT_BALANCE_ACCOUNT_LEN = 144
- fixed discriminators exist for MintState, GatewayConfig, GuardianSet, ProcessedEvent, and RecipientBalance
- gateway_mint_authority PDA is explicitly inventoried with seeds xxxl / gateway-mint-authority / v1
- legacy pre-41K.4 processed-event helper is not a live replay-protection initialization model

D.1 separates state into:

- long-lived protocol state
- derived authority state
- per-event replay state
- per-recipient accounting state
- SPL token state

D.1 does not close Blocker D.

D.1 does not initialize state, does not create accounts, does not call RPC, does not use testnet, does not use keys, does not sign, does not deploy, does not upgrade, does not configure SPL, does not construct guardian packages, does not submit, and does not mutate.

Current status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_DESIGN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker D.2 — repo-grounded state layout and PDA inventory.


## Blocker D.2 repo-grounded state layout and PDA inventory

The Blocker D.2 inventory record is recorded in:

docs/gateway/blocker-d-2-repo-grounded-state-layout-and-pda-inventory.md

Evidence directory:

docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory

Account layout inventory:

- MINT_STATE_ACCOUNT_LEN: 176
- GATEWAY_CONFIG_ACCOUNT_LEN: 256
- GUARDIAN_SET_ACCOUNT_LEN: 320
- PROCESSED_EVENT_ACCOUNT_LEN: 144
- RECIPIENT_BALANCE_ACCOUNT_LEN: 144

Discriminator inventory:

- MINT_STATE_ACCOUNT_DISCRIMINATOR
- GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR
- GUARDIAN_SET_ACCOUNT_DISCRIMINATOR
- PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
- RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR

PDA inventory:

- gateway_mint_authority
  - seeds: xxxl / gateway-mint-authority / v1
  - depends_on_program_id: true
  - purpose: SPL Token mint authority for gateway-backed XXXL minting

State category inventory:

- MintState: long-lived protocol state
- GatewayConfig: long-lived route/config state
- GuardianSet: long-lived guardian quorum state
- ProcessedEvent: per-event replay/consumption state
- RecipientBalance: per-recipient accounting state
- gateway_mint_authority: derived authority PDA
- SPL mint and token accounts: Blocker E scope

Inventory checks:

- all_expected_account_lengths_present: true
- all_expected_account_lengths_match_current_values: true
- runtime_layout_version_is_1: true
- all_expected_discriminators_present: true
- mint_state_view_present: true
- gateway_config_view_present: true
- guardian_set_view_present: true
- processed_event_view_present: true
- recipient_balance_view_present: true
- legacy_processed_event_helper_marked_not_live: true
- gateway_mint_authority_pda_inventory_present: true
- gateway_mint_authority_seeds_present: true
- gateway_mint_authority_depends_on_program_id: true
- processed_event_marking_boundary_report_present: true
- processed_event_marking_requires_system_owned_empty_entry: true
- processed_event_marking_accepts_lamport_dusted_empty_pda: true
- processed_event_marking_writes_final_consumed_image: true
- processed_event_marking_redecodes_after_write: true
- processed_event_marking_spl_mint_disabled: true
- processed_event_marking_live_route_disabled: true
- processed_event_marking_function_present: true
- account_contract_processed_event_system_or_program_pda: true
- account_contract_mint_authority_pda_program_derived: true
- account_contract_rent_payer_signer_present: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

all_inventory_checks_passed: true

D.2 confirms that the current repo defines state layout constants, discriminators, account views, gateway_mint_authority PDA inventory, and a Phase 41K.4 ProcessedEvent marking boundary.

D.2 does not close Blocker D.

D.2 does not initialize state, create accounts, call RPC, use testnet, use keys, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_D_OPEN_REPO_GROUNDED_STATE_LAYOUT_AND_PDA_INVENTORY_COMPLETED_NO_INITIALIZATION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker D.3 — state initialization authority and one-time guard decision model.


## Blocker D.3 state initialization authority and one-time guard decision model

The Blocker D.3 decision model is recorded in:

docs/gateway/blocker-d-3-state-initialization-authority-one-time-guard-decision-model.md

Evidence directory:

docs/gateway/evidence/blocker-d-3-state-initialization-authority-one-time-guard-decision-model

D.3 decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

Selected boundary:

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

D.3 rejects:

- direct manual state creation with unbounded admin discretion
- any initializer that can rewrite already-initialized protocol state
- any hidden admin mint or balance-write pathway
- treating ProcessedEvent marking as general protocol initialization
- using Blocker D to approve SPL mint setup
- executing initialization before a reviewed package and final scoped GO

Long-lived protocol state:

- MintState
- GatewayConfig
- GuardianSet

Not part of long-lived protocol initialization in D:

- ProcessedEvent: per-event replay protection, initialized/marked through Phase 41K.4 boundary
- RecipientBalance: per-recipient accounting state, requires later lazy-init model
- gateway_mint_authority PDA: derived authority boundary, but SPL authority architecture is Blocker E
- SPL mint and token accounts: Blocker E scope

Required future initializer properties:

- explicit initializer entrypoint or package
- explicit long-lived account list
- fixed account lengths
- fixed account discriminators
- runtime_layout_version written and checked
- one-time initialization guard
- reinitialization rejection
- public config values recorded before execution
- no hidden admin mint authority
- no admin recipient balance write
- no processed-event prepopulation as substitute for replay protection
- no SPL mint setup inside D
- separate final scoped GO before execution

D.3 does not close Blocker D.

D.3 does not initialize state, create accounts, call RPC, use testnet, use keys, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_AUTHORITY_ONE_TIME_GUARD_DECISION_MODEL_RECORDED_NO_EXECUTION

Current decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker D.4 — state initialization invariant review package.


## Blocker D.4 state initialization invariant review package

The Blocker D.4 invariant review package is recorded in:

docs/gateway/blocker-d-4-state-initialization-invariant-review-package.md

Evidence directory:

docs/gateway/evidence/blocker-d-4-state-initialization-invariant-review-package

Reviewed invariants:

- long-lived state scope is limited to MintState, GatewayConfig, and GuardianSet
- state layouts, lengths, discriminators, account views, and runtime layout version are inventoried
- future initializer must have a one-time guard
- future initializer must reject reinitialization
- direct manual unbounded admin initialization is rejected
- ProcessedEvent remains per-event replay protection and is not general protocol initialization
- RecipientBalance initialization model remains an explicit open design gap
- SPL mint setup and SPL mint authority architecture remain Blocker E
- gateway_mint_authority PDA is inventoried but not activated by D
- future initializer must not introduce hidden admin mint or balance-write pathways
- D.4 approves no execution, no account creation, no RPC, no signing, no deploy, no upgrade, no SPL setup, no submit, no mutation

Review result:

all_invariants_reviewed: true

blocker_d_closure_ready: true

closure_type: narrow_design_boundary_only

Prepared closure candidate:

STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Remaining open items outside D closure:

- RecipientBalance lazy/precreate initialization mechanics
- actual initializer instruction/package implementation
- local evidence that reinitialization fails
- local evidence that partial initialization cannot be treated as valid
- SPL mint authority architecture in Blocker E
- expected post-upgrade ProgramData hash in Blocker B
- guardian descriptor in Blocker F
- rollback/recovery plan in Blocker G

D.4 does not close Blocker D.

D.4 does not initialize state, create accounts, call RPC, use testnet, use keys, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_D_REVIEW_READY_STATE_INITIALIZATION_INVARIANTS_RECORDED_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker D.5 — state initialization design closure decision record.


## Blocker D.5 state initialization design closure decision record

The Blocker D.5 closure decision record is recorded in:

docs/gateway/blocker-d-5-state-initialization-design-closure-decision-record.md

Blocker D is now CLOSED narrowly as:

STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Closure basis:

- D.1 opened state initialization design planning
- D.2 completed repo-grounded state layout and PDA inventory
- D.3 recorded the state initialization authority and one-time guard decision model
- D.4 completed state initialization invariant review

Accepted D.2 inventory result:

all_inventory_checks_passed: true

Accepted D.3 decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

Accepted D.3 boundary:

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

Accepted D.4 invariant result:

all_invariants_reviewed: true

blocker_d_closure_ready: true

closure_type: narrow_design_boundary_only

D.5 does not approve state initialization execution, account creation, PDA creation, initializer execution, SPL mint setup, SPL CPI minting, signing, keypair use, deploy, write-buffer, set-upgrade-authority, close, upgrade, guardian package construction, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash
- E — OPEN: SPL mint authority architecture
- F — OPEN: guardian descriptor
- G — OPEN: rollback / recovery plan

Current status:

BLOCKER_D_CLOSED_NARROW_STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_D_CLOSED_NARROW_DESIGN_INVARIANTS_ONLY

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker E.1 — SPL mint authority architecture planning.


## Blocker E.1 SPL mint authority architecture planning

The Blocker E.1 planning record is recorded in:

docs/gateway/blocker-e-1-spl-mint-authority-architecture-planning.md

E.1 opens the SPL mint authority architecture track after:

- Blocker A closed narrowly as upgrade authority present but accepted for test phase
- Blocker C closed narrowly as B1C7 handler boundary / invariants only
- Blocker D closed narrowly as state initialization design / invariants only

Current repo-grounded SPL facts:

- gateway_mint_authority PDA is inventoried
- MintToCpi boundary references token_program, mint, recipient_token_account, and mint_authority_pda
- token_program is asserted as spl_token::id()
- mint_authority_pda is asserted against gateway_mint_authority PDA derivation
- SPL mint_to instruction can be built
- gateway_mint_authority signer seeds exist
- SPL mint CPI execution remains disabled by default
- guarded SPL CPI path returns CpiBoundaryNotReady when not enabled
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active

E.1 does not close Blocker E.

E.1 does not create an SPL mint, configure mint authority, transfer mint authority, set freeze authority, mint tokens, initialize state, call RPC, use testnet, use keys, sign, deploy, upgrade, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_ARCHITECTURE_PLANNING_ONLY_NO_SPL_SETUP_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker E.2 — repo-grounded SPL mint authority and CPI inventory.


## Blocker E.2 repo-grounded SPL mint authority and CPI inventory

The Blocker E.2 inventory record is recorded in:

docs/gateway/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory.md

Evidence directory:

docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory

Authority inventory:

- gateway_mint_authority PDA
  - seeds: xxxl / gateway-mint-authority / v1
  - depends_on_program_id: true
  - expected role: SPL Token mint authority for gateway-backed XXXL minting
  - current status: inventoried, not created, not activated by E.2

CPI inventory:

- token program model: classic SPL Token via spl_token::id()
- mint instruction model: spl_token::instruction::mint_to
- signer model: invoke_signed with gateway_mint_authority signer seeds
- mint authority check: assert_gateway_mint_authority_pda
- default CPI execution: disabled
- closed-gate result: CpiBoundaryNotReady
- open-gate route: requires D2 production-path test gate plus B1C7 handler integration test gate and both dangerous allow features
- E.2 execution status: no SPL mint setup, no mint authority transfer, no mint_to execution

Account contract inventory:

- spl_token_mint: writable, not signer, SplTokenOwned
- recipient_token_account: writable, not signer, SplTokenOwned
- mint_authority_pda: readonly, not signer, ProgramDerivedAddress
- token_program: readonly, not signer, SplTokenProgram

MintState relationship inventory:

- MintState records mint_pubkey
- MintState records gateway_mint_authority_pda
- MintState records gateway_mint_authority_bump
- MintState records total_supply
- E.2 does not prove SPL total supply reconciliation; this remains a future E invariant

Inventory checks:

- gateway_mint_authority_pda_inventory_present: true
- gateway_mint_authority_seeds_present: true
- gateway_mint_authority_depends_on_program_id: true
- gateway_mint_authority_derivation_function_present: true
- mint_to_cpi_accounts_present: true
- mint_to_cpi_boundary_present: true
- mint_to_cpi_planning_boundary_present: true
- classic_spl_token_program_asserted: true
- spl_token_mint_to_instruction_built: true
- mint_authority_pda_asserted_against_program_derivation: true
- gateway_mint_authority_signer_seeds_present: true
- mint_to_cpi_uses_invoke_signed: true
- spl_cpi_execution_disabled_by_default: true
- spl_cpi_gate_open_requires_d2_and_b1c7_dangerous_allows: true
- guarded_cpi_returns_cpi_boundary_not_ready_when_gate_closed: true
- account_contract_has_spl_mint: true
- account_contract_has_recipient_token_account: true
- account_contract_has_mint_authority_pda: true
- account_contract_has_token_program: true
- mint_state_records_mint_pubkey: true
- mint_state_records_gateway_mint_authority_pda: true
- mint_state_records_gateway_mint_authority_bump: true
- mint_state_records_total_supply: true
- execution_plan_keeps_live_route_flags_explicit: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

all_inventory_checks_passed: true

E.2 confirms that the current repo contains a planned gateway_mint_authority PDA, classic SPL Token CPI mint_to boundary, guarded CPI gate, and account contract entries for spl_token_mint, recipient_token_account, mint_authority_pda, and token_program.

E.2 does not close Blocker E.

E.2 does not create an SPL mint, configure mint authority, transfer mint authority, set freeze authority, mint tokens, initialize state, call RPC, use testnet, use keys, sign, deploy, upgrade, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_E_OPEN_REPO_GROUNDED_SPL_MINT_AUTHORITY_AND_CPI_INVENTORY_COMPLETED_NO_SPL_SETUP

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker E.3 — SPL mint authority setup decision model.


## Blocker E.3 SPL mint authority setup decision model

The Blocker E.3 decision model is recorded in:

docs/gateway/blocker-e-3-spl-mint-authority-setup-decision-model.md

Evidence directory:

docs/gateway/evidence/blocker-e-3-spl-mint-authority-setup-decision-model

E.3 decision:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Selected token program model:

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

Selected setup path:

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Selected freeze authority direction:

FREEZE_AUTHORITY_NONE_PREFERRED

Selected initial supply rule:

ZERO_INITIAL_SUPPLY_REQUIRED

Execution boundary:

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

E.3 rejects:

- retained human/admin mint authority
- retained human/admin freeze authority as default
- Token-2022 as current repo model
- SPL setup execution before reviewed setup package and final scoped GO

Remaining open items before E closure:

- exact decimals value
- exact canonical mint account address/model
- exact setup package
- exact freeze authority proof
- exact mint authority proof
- exact total supply reconciliation evidence
- exact local/testnet evidence package
- final scoped GO before execution

E.3 does not close Blocker E.

E.3 does not create an SPL mint, configure mint authority, transfer mint authority, set/disable freeze authority, mint tokens, initialize state, call RPC, use testnet, use keys, sign, deploy, upgrade, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_SETUP_DECISION_MODEL_RECORDED_NO_SPL_SETUP_NO_EXECUTION

Current decision:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker E.4 — SPL mint authority invariant review package.


## Blocker E.4 SPL mint authority invariant review package

The Blocker E.4 invariant review package is recorded in:

docs/gateway/blocker-e-4-spl-mint-authority-invariant-review-package.md

Evidence directory:

docs/gateway/evidence/blocker-e-4-spl-mint-authority-invariant-review-package

Reviewed invariants:

- classic SPL Token is the current repo model
- gateway_mint_authority PDA is the canonical mint authority
- no retained human/admin mint authority is allowed after canonical setup
- zero initial supply is required
- freeze authority none / disabled is preferred
- retained human/admin freeze authority is rejected as default
- SPL CPI minting remains fail-closed by default
- SPL CPI minting must remain downstream of gateway authorization
- MintState relationship fields are recorded
- SPL total supply reconciliation proof remains future execution evidence
- no SPL setup execution is approved

Review result:

all_invariants_reviewed: true

blocker_e_closure_ready: true

closure_type: narrow_architecture_boundary_only

Prepared closure candidate:

SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Remaining open items outside E closure:

- exact decimals value
- exact canonical mint account address/model
- exact future SPL setup package
- exact future authority handoff proof if a temporary setup authority is used
- exact future freeze authority proof
- exact future total supply reconciliation evidence
- final scoped GO before any SPL setup execution

E.4 does not close Blocker E.

E.4 does not create an SPL mint, configure mint authority, transfer mint authority, set/disable freeze authority, mint tokens, initialize state, call RPC, use testnet, use keys, sign, deploy, upgrade, construct guardian packages, submit, or mutate.

Current status:

BLOCKER_E_REVIEW_READY_SPL_MINT_AUTHORITY_INVARIANTS_RECORDED_NO_SPL_SETUP_NO_EXECUTION

Current decision:

BLOCKER_E_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker E.5 — SPL mint authority architecture closure decision record.


## Blocker E.5 SPL mint authority architecture closure decision record

The Blocker E.5 closure decision record is recorded in:

docs/gateway/blocker-e-5-spl-mint-authority-architecture-closure-decision-record.md

Blocker E is now CLOSED narrowly as:

SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Closure basis:

- E.1 opened SPL mint authority architecture planning
- E.2 completed repo-grounded SPL mint authority and CPI inventory
- E.3 recorded the SPL mint authority setup decision model
- E.4 completed SPL mint authority invariant review

Accepted E.2 inventory result:

all_inventory_checks_passed: true

Accepted E.3 decision:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Accepted token program model:

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

Accepted setup path:

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Accepted freeze authority direction:

FREEZE_AUTHORITY_NONE_PREFERRED

Accepted initial supply rule:

ZERO_INITIAL_SUPPLY_REQUIRED

Accepted execution boundary:

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

Accepted E.4 invariant result:

all_invariants_reviewed: true

blocker_e_closure_ready: true

closure_type: narrow_architecture_boundary_only

E.5 does not approve SPL mint creation, SPL mint initialization, mint authority assignment, mint authority transfer, freeze authority assignment, freeze authority disablement, SPL CPI minting, state initialization execution, signing, keypair use, deploy, write-buffer, set-upgrade-authority, close, upgrade, guardian package construction, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash
- F — OPEN: guardian descriptor
- G — OPEN: rollback / recovery plan

Current status:

BLOCKER_E_CLOSED_NARROW_SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_E_CLOSED_NARROW_ARCHITECTURE_INVARIANTS_ONLY

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker F.1 — guardian descriptor planning.


## Blocker F.1 guardian descriptor planning

The Blocker F.1 planning record is recorded in:

docs/gateway/blocker-f-1-guardian-descriptor-planning.md

Evidence directory:

docs/gateway/evidence/blocker-f-1-guardian-descriptor-planning

F.1 opens the guardian descriptor track after:

- Blocker A closed narrowly as upgrade authority present but accepted for test phase
- Blocker C closed narrowly as B1C7 handler boundary / invariants only
- Blocker D closed narrowly as state initialization design / invariants only
- Blocker E closed narrowly as SPL mint authority architecture / invariants only

Preferred future direction:

- static public guardian descriptor in repo
- public keys only
- no private keys
- deterministic descriptor hash/id
- explicit testnet vs production labeling
- explicit quorum rule
- descriptor binding to route/state/message expectations
- final scoped GO before any guardian package construction or submit

F.1 does not close Blocker F.

F.1 does not add guardian keys, add private keys, finalize a descriptor, construct guardian packages, sign, initialize state, configure SPL, call RPC, use testnet, deploy, upgrade, submit, or mutate.

Current status:

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_PLANNING_ONLY_NO_KEYS_NO_PACKAGES_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker F.2 — repo-grounded guardian/quorum inventory.


## Blocker F.2 repo-grounded guardian/quorum inventory

The Blocker F.2 inventory record is recorded in:

docs/gateway/blocker-f-2-repo-grounded-guardian-quorum-inventory.md

Evidence directory:

docs/gateway/evidence/blocker-f-2-repo-grounded-guardian-quorum-inventory

Inventory summary:

- guardian_set_account_len_present: true
- guardian_set_discriminator_or_view_present: true
- guardian_set_account_contract_entry_present: true
- repo_has_guardian_or_quorum_references: true
- repo_has_quorum_approval_signature_references: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true
- f2_no_keys_no_packages_no_execution: true

all_inventory_checks_passed: true

Descriptor inventory:

- descriptor_status: not finalized
- keys_added_by_f2: false
- private_keys_added_by_f2: false
- package_constructed_by_f2: false
- signing_executed_by_f2: false
- expected_future_model: static public guardian descriptor in repo, public keys only, deterministic descriptor hash/id, explicit testnet/production label, explicit threshold/quorum rule

State inventory summary:

- GuardianSet is part of the state/account inventory.
- guardian_set is part of the consume-gateway-mint account contract.
- F.2 records inventory only and does not initialize GuardianSet state.
- F.2 does not activate a guardian descriptor.

Quorum inventory summary:

- F.2 inventories repo references to quorum, threshold, approval, signature, and ed25519.
- F.2 does not select final guardian public keys.
- F.2 does not select production guardian keys.
- F.2 does not construct approvals or packages.
- F.2 does not sign anything.

F.2 does not close Blocker F.

F.2 does not add guardian keys, add private keys, finalize a descriptor, construct guardian packages, sign, initialize state, configure SPL, call RPC, use testnet, deploy, upgrade, submit, or mutate.

Current status:

BLOCKER_F_OPEN_REPO_GROUNDED_GUARDIAN_QUORUM_INVENTORY_COMPLETED_NO_KEYS_NO_PACKAGES

Current decision:

BLOCKER_F_NOT_CLOSED

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker F.3 — guardian descriptor decision model.


## Blocker F.3 guardian descriptor decision model

The Blocker F.3 decision model is recorded in:

docs/gateway/blocker-f-3-guardian-descriptor-decision-model.md

Evidence directory:

docs/gateway/evidence/blocker-f-3-guardian-descriptor-decision-model

F.3 decision:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Descriptor model:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_IN_REPO_PUBLIC_KEYS_ONLY_NO_PRIVATE_KEYS

Descriptor scope:

TESTNET_DESCRIPTOR_ALLOWED_ONLY_AS_EXPLICIT_TESTNET_ARTIFACT_AFTER_FURTHER_REVIEW

Key material boundary:

PUBLIC_KEYS_ONLY_PRIVATE_KEYS_NEVER_COMMITTED_NEVER_REQUESTED_NEVER_PRINTED

Key type model:

ED25519_PUBLIC_KEY_DESCRIPTOR_MODEL

Quorum model:

EXPLICIT_THRESHOLD_OVER_DISTINCT_GUARDIAN_APPROVALS

Descriptor id model:

DETERMINISTIC_DESCRIPTOR_HASH_ID_REQUIRED

Binding model:

DESCRIPTOR_BOUND_TO_GUARDIAN_SET_ID_ROUTE_ID_SOURCE_CHAIN_MINT_TOKEN_MESSAGE_SCHEMA_AND_CANONICAL_ENCODING

Rotation model:

ROTATION_REQUIRES_NEW_DESCRIPTOR_ID_AND_SEPARATE_REVIEW

Package boundary:

GUARDIAN_PACKAGE_CONSTRUCTION_REQUIRES_FUTURE_REVIEWED_DESCRIPTOR_AND_FINAL_SCOPED_GO

Execution boundary:

FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING

Required future descriptor fields:

- descriptor_version
- descriptor_scope
- network_label
- guardian_set_id
- descriptor_id_or_hash
- key_type
- guardian_public_keys
- threshold
- distinct_approval_rule
- route_id
- source_chain_id
- mint_token
- message_schema_version
- canonical_encoding_id
- activation_boundary
- rotation_boundary
- expiry_or_supersession_rule

Rejected models:

- implicit or undocumented guardian set
- ad hoc relayer/admin signature model
- hidden off-repo guardian set
- production descriptor finalization in current phase
- private keys in repo
- guardian package construction before final scoped GO
- signing before final scoped GO

Remaining open items before F closure:

- exact descriptor schema file path
- exact descriptor hash canonicalization rule
- exact testnet guardian public key list
- exact threshold value
- exact guardian_set_id value
- exact route/state/message binding fields
- exact failure matrix for invalid/duplicate/unknown/under-threshold approvals
- no-private-key repo scan evidence
- guardian descriptor invariant review package
- final scoped GO before package construction or signing

F.3 does not close Blocker F.

F.3 does not add guardian keys, add private keys, finalize a live descriptor, construct guardian packages, sign, initialize state, configure SPL, call RPC, use testnet, deploy, upgrade, submit, or mutate.

Current status:

BLOCKER_F_OPEN_GUARDIAN_DESCRIPTOR_DECISION_MODEL_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker F.4 — guardian descriptor invariant review package.


## Blocker F.4 guardian descriptor invariant review package

The Blocker F.4 invariant review package is recorded in:

docs/gateway/blocker-f-4-guardian-descriptor-invariant-review-package.md

Evidence directory:

docs/gateway/evidence/blocker-f-4-guardian-descriptor-invariant-review-package

Reviewed invariants:

- static public descriptor model
- public keys only
- private keys never committed, requested, printed, or stored in repo
- ed25519 public key descriptor model
- explicit threshold/quorum rule
- distinct guardian approvals
- deterministic descriptor hash/id
- guardian_set_id / route / source chain / mint token / message schema / canonical encoding binding
- rotation requires new descriptor id and separate review
- guardian package construction and signing remain blocked
- no RPC, testnet, submit, or mutation approved

Private key material scan summary:

private_key_scan_match_count: 0

private_key_material_scan_result: NO_PRIVATE_KEY_MATERIAL_PATTERNS_FOUND

Review result:

all_invariants_reviewed: true

blocker_f_closure_ready: true

closure_type: narrow_descriptor_boundary_only

Prepared closure candidate:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Remaining open items outside F closure:

- exact descriptor schema file path
- exact canonical descriptor hash rule
- exact testnet guardian public key list
- exact threshold value
- exact guardian_set_id value
- exact route/state/message binding values
- exact invalid/duplicate/unknown/under-threshold failure matrix
- future reviewed descriptor artifact
- final scoped GO before package construction or signing

F.4 does not close Blocker F.

F.4 does not add guardian keys, add private keys, finalize a live descriptor, construct guardian packages, sign, initialize state, configure SPL, call RPC, use testnet, deploy, upgrade, submit, or mutate.

Current status:

BLOCKER_F_REVIEW_READY_GUARDIAN_DESCRIPTOR_INVARIANTS_RECORDED_NO_KEYS_NO_PACKAGES_NO_EXECUTION

Current decision:

BLOCKER_F_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker F.5 — guardian descriptor closure decision record.


## Blocker F.5 guardian descriptor closure decision record

The Blocker F.5 closure decision record is recorded in:

docs/gateway/blocker-f-5-guardian-descriptor-closure-decision-record.md

Blocker F is now CLOSED narrowly as:

GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Closure basis:

- F.1 opened guardian descriptor planning
- F.2 completed repo-grounded guardian/quorum inventory
- F.3 recorded the guardian descriptor decision model
- F.4 completed guardian descriptor invariant review

Accepted F.2 inventory result:

all_inventory_checks_passed: true

Accepted F.3 decision:

STATIC_PUBLIC_GUARDIAN_DESCRIPTOR_PUBLIC_KEYS_ONLY_EXPLICIT_THRESHOLD_NO_PRIVATE_KEYS_NO_PACKAGES_NO_SIGNING

Accepted key material boundary:

PUBLIC_KEYS_ONLY_PRIVATE_KEYS_NEVER_COMMITTED_NEVER_REQUESTED_NEVER_PRINTED

Accepted key type model:

ED25519_PUBLIC_KEY_DESCRIPTOR_MODEL

Accepted quorum model:

EXPLICIT_THRESHOLD_OVER_DISTINCT_GUARDIAN_APPROVALS

Accepted package/signing boundary:

FUTURE_REVIEWED_GUARDIAN_DESCRIPTOR_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_GUARDIAN_PACKAGE_CONSTRUCTION_OR_SIGNING

Accepted F.4 invariant result:

all_invariants_reviewed: true

blocker_f_closure_ready: true

closure_type: narrow_descriptor_boundary_only

Private key material scan accepted:

private_key_scan_match_count: 0

private_key_material_scan_result: NO_PRIVATE_KEY_MATERIAL_PATTERNS_FOUND

F.5 does not approve guardian descriptor finalization, guardian public key selection, production key selection, private key handling, signing, guardian package construction, state initialization execution, SPL setup, deploy, write-buffer, set-upgrade-authority, close, upgrade, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash
- G — OPEN: rollback / recovery plan

Current status:

BLOCKER_F_CLOSED_NARROW_GUARDIAN_DESCRIPTOR_MODEL_REVIEWED_KEYS_AND_PACKAGES_NOT_APPROVED

Current decision:

BLOCKER_F_CLOSED_NARROW_DESCRIPTOR_INVARIANTS_ONLY

NO-GO REMAINS_FOR_GUARDIAN_DESCRIPTOR_FINALIZATION_GUARDIAN_KEYS_PRODUCTION_KEYS_SIGNING_PACKAGES_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker G.1 — rollback / recovery plan planning.


## Blocker G.1 rollback / recovery plan planning

The Blocker G.1 planning record is recorded in:

docs/gateway/blocker-g-1-rollback-recovery-plan-planning.md

Evidence directory:

docs/gateway/evidence/blocker-g-1-rollback-recovery-plan-planning

G.1 opens rollback / recovery planning after:

- Blocker A closed narrowly as upgrade authority present but accepted for test phase
- Blocker C closed narrowly as B1C7 handler boundary / invariants only
- Blocker D closed narrowly as state initialization design / invariants only
- Blocker E closed narrowly as SPL mint authority architecture / invariants only
- Blocker F closed narrowly as guardian descriptor model / invariants only

Preferred future direction:

- full stage-gated recovery plan
- explicit pre-mutation abort points
- no automatic retry after failure
- explicit post-submit observation requirements
- explicit user GO before any mutation or recovery action
- explicit abandon/redeploy path when rollback is not possible
- evidence saved for every branch

G.1 does not close Blocker G.

G.1 does not run build, deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, call RPC, use testnet, submit, or mutate.

Current status:

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_PLAN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker G.2 — repo-grounded rollback / recovery inventory.


## Blocker G.2 repo-grounded rollback / recovery inventory

The Blocker G.2 inventory record is recorded in:

docs/gateway/blocker-g-2-repo-grounded-rollback-recovery-inventory.md

Evidence directory:

docs/gateway/evidence/blocker-g-2-repo-grounded-rollback-recovery-inventory

Inventory summary:

- g1_planning_recorded: true
- stage_gated_recovery_candidate_present: true
- abandon_redeploy_candidate_present: true
- user_final_go_candidate_present: true
- no_automatic_retry_scope_present: true
- pre_mutation_abort_scope_present: true
- post_submit_observation_scope_present: true
- rollback_recovery_references_found: true
- deploy_upgrade_references_found: true
- state_spl_guardian_references_found: true
- final_go_no_go_references_found: true
- evidence_observation_references_found: true
- g2_no_execution: true

all_inventory_checks_passed: true

Inventory counts:

- rollback_recovery: files=193, sampled_lines=160
- deploy_upgrade: files=490, sampled_lines=160
- state_spl_guardian: files=700, sampled_lines=160
- final_go_no_go: files=108, sampled_lines=160
- evidence_observation: files=959, sampled_lines=160

Stage-gated recovery inventory:

- pre_build_abort: required future recovery branch
- post_build_pre_deploy_abort: required future recovery branch
- post_deploy_pre_state_init_observation: required future recovery branch
- post_state_init_stop_condition: required future recovery branch
- post_spl_setup_stop_condition: required future recovery branch
- post_guardian_descriptor_pre_package_abort: required future recovery branch
- post_package_pre_submit_abort: required future recovery branch
- post_submit_observation: required future evidence branch
- non_reversible_action_policy: abandon/redeploy decision path required
- automatic_retry_policy: automatic retry rejected
- user_go_policy: explicit scoped user GO required before mutation/recovery action

G.2 does not close Blocker G.

G.2 does not run build, deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, call RPC, use testnet, submit, or mutate.

Current status:

BLOCKER_G_OPEN_REPO_GROUNDED_ROLLBACK_RECOVERY_INVENTORY_COMPLETED_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker G.3 — rollback / recovery decision model.


## Blocker G.3 rollback / recovery decision model

The Blocker G.3 decision model is recorded in:

docs/gateway/blocker-g-3-rollback-recovery-decision-model.md

Evidence directory:

docs/gateway/evidence/blocker-g-3-rollback-recovery-decision-model

G.3 decision:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Rollback model:

STAGE_GATED_ABORT_OBSERVE_RECOVER_OR_ABANDON_REDEPLOY

Automatic retry policy:

AUTOMATIC_RETRY_REJECTED

Non-reversible action policy:

ABANDON_OR_REDEPLOY_IF_SAFE_ROLLBACK_NOT_POSSIBLE

User GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_MUTATION_OR_RECOVERY_ACTION

Evidence policy:

EVIDENCE_REQUIRED_BEFORE_NEXT_STAGE_OR_RECOVERY_BRANCH

Execution boundary:

FUTURE_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_OR_RECOVERY_MUTATION

Required stage gates:

- pre-build / pre-artifact gate
- post-build / pre-deploy gate
- post-deploy-or-upgrade / pre-state-init gate
- post-state-init / pre-SPL-setup gate
- post-SPL-setup / pre-guardian-package gate
- post-guardian-package / pre-submit gate
- post-submit observation gate
- non-reversible action abandon/redeploy gate

Rejected actions:

- automatic retry after failed mutation
- continuing after missing evidence
- continuing after ambiguous post-submit state
- pretending non-reversible mutation can always be rolled back
- recovery action without explicit scoped user GO
- build/deploy/upgrade/state-init/SPL/package/signing/submit inside G.3

Remaining open items before G closure:

- rollback / recovery invariant review package
- closure decision record
- final scoped GO package after B and G are closed
- actual future pre-mutation evidence bundle
- actual future post-submit observation bundle

G.3 does not close Blocker G.

G.3 does not run build, deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, call RPC, use testnet, submit, or mutate.

Current status:

BLOCKER_G_OPEN_ROLLBACK_RECOVERY_DECISION_MODEL_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker G.4 — rollback / recovery invariant review package.


## Blocker G.4 rollback / recovery invariant review package

The Blocker G.4 invariant review package is recorded in:

docs/gateway/blocker-g-4-rollback-recovery-invariant-review-package.md

Evidence directory:

docs/gateway/evidence/blocker-g-4-rollback-recovery-invariant-review-package

Reviewed invariants:

- full stage-gated recovery model
- explicit pre-mutation abort points
- post-submit observation evidence required
- automatic retry rejected
- explicit scoped user GO required before any mutation or recovery action
- abandon/redeploy path required when safe rollback is not possible
- evidence required before next stage or recovery branch
- no build/deploy/upgrade/state-init/SPL/package/signing/RPC/testnet/submit/mutation approved

Review result:

all_invariants_reviewed: true

blocker_g_closure_ready: true

closure_type: narrow_recovery_boundary_only

Prepared closure candidate:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Remaining open items outside G closure:

- Blocker G closure decision record
- Blocker B expected post-upgrade ProgramData hash
- future final scoped GO package
- future pre-mutation evidence bundle
- future post-submit observation bundle

G.4 does not close Blocker G.

G.4 does not run build, deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, call RPC, use testnet, submit, or mutate.

Current status:

BLOCKER_G_REVIEW_READY_ROLLBACK_RECOVERY_INVARIANTS_RECORDED_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_G_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker G.5 — rollback / recovery closure decision record.


## Blocker G.5 rollback / recovery closure decision record

The Blocker G.5 closure decision record is recorded in:

docs/gateway/blocker-g-5-rollback-recovery-closure-decision-record.md

Blocker G is now CLOSED narrowly as:

ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Closure basis:

- G.1 opened rollback / recovery planning
- G.2 completed repo-grounded rollback / recovery inventory
- G.3 recorded the rollback / recovery decision model
- G.4 completed rollback / recovery invariant review

Accepted G.3 decision:

FULL_STAGE_GATED_RECOVERY_NO_AUTOMATIC_RETRY_USER_GO_REQUIRED

Accepted rollback model:

STAGE_GATED_ABORT_OBSERVE_RECOVER_OR_ABANDON_REDEPLOY

Accepted automatic retry policy:

AUTOMATIC_RETRY_REJECTED

Accepted non-reversible action policy:

ABANDON_OR_REDEPLOY_IF_SAFE_ROLLBACK_NOT_POSSIBLE

Accepted user GO policy:

EXPLICIT_SCOPED_USER_GO_REQUIRED_BEFORE_ANY_MUTATION_OR_RECOVERY_ACTION

Accepted evidence policy:

EVIDENCE_REQUIRED_BEFORE_NEXT_STAGE_OR_RECOVERY_BRANCH

Accepted G.4 invariant result:

all_invariants_reviewed: true

blocker_g_closure_ready: true

closure_type: narrow_recovery_boundary_only

G.5 does not approve build, deploy, upgrade, write buffer, authority change, state initialization execution, SPL setup, guardian package construction, signing, RPC, testnet, transaction submit, mutation, or production activation.

Remaining blockers:

- B — OPEN: expected post-upgrade ProgramData hash

Current status:

BLOCKER_G_CLOSED_NARROW_ROLLBACK_RECOVERY_PLAN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_G_CLOSED_NARROW_RECOVERY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_BUILD_DEPLOY_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker B.1 — expected post-upgrade ProgramData hash planning.


## Blocker B.1 expected post-upgrade ProgramData hash planning

The Blocker B.1 planning record is recorded in:

docs/gateway/blocker-b-1-expected-post-upgrade-programdata-hash-planning.md

Evidence directory:

docs/gateway/evidence/blocker-b-1-expected-post-upgrade-programdata-hash-planning

B.1 opens the expected post-upgrade ProgramData hash track after narrow closure of A, C, D, E, F, G, and H.

Preferred future direction:

- full hash bundle
- canonical runtime hash over ProgramData executable bytes
- source commit binding
- build command binding
- toolchain/version binding
- feature flag binding
- artifact hash binding
- baseline ProgramData evidence binding
- post-upgrade read-only verification plan
- mismatch stop condition
- no automatic retry
- explicit scoped user GO before any build/hash/upgrade step that is outside pure planning

B.1 does not close Blocker B.

B.1 does not run build, compute artifact hash, compute ProgramData hash, deploy, upgrade, write buffer, change authority, initialize state, configure SPL, construct guardian packages, sign, call RPC, use testnet, submit, or mutate.

Current status:

BLOCKER_B_OPEN_EXPECTED_POST_UPGRADE_PROGRAMDATA_HASH_PLANNING_ONLY_NO_BUILD_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_B_NOT_CLOSED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

Next safe step:

Blocker B.2 — repo-grounded ProgramData hash inventory.

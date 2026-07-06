# Scoped Package.1 — First future operation class selection planning

Status:

SCOPED_PACKAGE_1_OPEN_FIRST_OPERATION_CLASS_SELECTED_FOR_PLANNING_ONLY_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_SELECTED_FOR_PLANNING_ONLY_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Scoped Package.1 selects the first future operation class for planning only.

This is not an actual execution package.

This does not grant GO.

This does not draft a runnable package.

This does not run build.

This does not compute artifact hash.

This does not compute ProgramData executable-bytes hash.

This does not call RPC.

This does not use testnet.

This does not deploy.

This does not upgrade.

This does not write a buffer.

This does not change authority.

This does not initialize state.

This does not configure SPL.

This does not construct guardian packages.

This does not sign.

This does not submit or mutate any network.

## Evidence basis

- Final GO.5 — final scoped GO model closure decision record
- Final GO.4 — final scoped GO package invariant review
- Final GO.3 — final scoped GO package decision model
- Final GO.2 — repo-grounded final GO package inventory
- Final GO.1 — final scoped GO package planning
- Blocker B.5 — expected post-upgrade ProgramData hash closure

## Selection rationale

The selected first future operation class is:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Reason:

Expected-hash and build/hash evidence are prerequisites before any upgrade/write-buffer GO.

Selecting this class for planning does not authorize build or hash computation.

It only identifies the first future class for a later requirements inventory.

## Operation class selection matrix

```text
# Scoped Package.1 operation class selection matrix

SP1_CANDIDATE_00_NO_OPERATION_SELECTED
status: rejected
meaning: Do not select any future operation class.
reason_rejected: Final GO.5 explicitly identifies Scoped Package.1 as the next safe planning step.

SP1_CANDIDATE_01_UPGRADE_OR_WRITE_BUFFER
status: rejected_for_now
meaning: Select upgrade/write-buffer as the first future operation class.
reason_rejected: Expected-hash/build-hash evidence package must exist before any upgrade/write-buffer GO.

SP1_CANDIDATE_02_READ_ONLY_NETWORK_PRECHECK
status: rejected_for_now
meaning: Select read-only RPC/network precheck as the first future operation class.
reason_rejected: Read-only precheck is needed before mutation, but expected-hash/build-hash evidence planning is the safer first dependency for upgrade safety.

SP1_CANDIDATE_03_STATE_INIT
status: rejected_for_now
meaning: Select state initialization as the first future operation class.
reason_rejected: State initialization execution is not approved and should not precede expected-hash/build-hash evidence planning.

SP1_CANDIDATE_04_SPL_SETUP
status: rejected_for_now
meaning: Select SPL setup as the first future operation class.
reason_rejected: SPL setup execution is not approved and should not precede expected-hash/build-hash evidence planning.

SP1_CANDIDATE_05_GUARDIAN_PACKAGE_OR_SIGNING
status: rejected_for_now
meaning: Select guardian package construction or signing as the first future operation class.
reason_rejected: Guardian package construction and signing are not approved and should remain separate later boundaries.

SP1_CANDIDATE_06_PRODUCTION_ACTIVATION
status: rejected
meaning: Select production activation as the first future operation class.
reason_rejected: Production activation is far outside the current safety boundary.

SP1_CANDIDATE_07_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE
status: selected_for_planning_only
meaning: Select expected-hash/build-hash evidence package as the first future operation class to plan.
reason_selected: Blocker B requires expected hash package and build/hash evidence before any upgrade/write-buffer GO.

SELECTED_SP1_OPERATION_CLASS
EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

SELECTED_SP1_OPERATION_CLASS_KIND
PLANNING_ONLY_NOT_EXECUTION

SELECTED_SP1_GO_STATE
FINAL_GO_NOT_GRANTED

SELECTED_SP1_DOES_NOT_AUTHORIZE
BUILD_HASH_RPC_TESTNET_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGE_SIGNING_SUBMIT_MUTATION

SELECTED_SP1_NEXT_SAFE_STEP
SCOPED_PACKAGE_2_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_REQUIREMENTS_INVENTORY
```

## Selected operation class boundary

```text
# Selected operation class boundary

selected_operation_class: EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

selection_scope:
- planning only
- first future operation class target
- no actual package drafted
- no build
- no artifact hash computation
- no ProgramData executable-bytes hash computation
- no RPC
- no testnet
- no deploy
- no upgrade
- no write-buffer
- no state initialization
- no SPL setup
- no guardian package construction
- no signing
- no submit
- no mutation

future_meaning:
A later separate scoped package may define exact requirements for a future expected-hash/build-hash evidence package.

future_requirement:
Even that later package must not execute build or hash computation unless a separate exact scoped user GO phrase is recorded.

current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
```

## Non-GO statement

Scoped Package.1 does not grant GO.

Scoped Package.1 does not approve:

- build
- local hash computation
- ProgramData executable-bytes hash computation
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- signing
- RPC
- testnet
- transaction submit
- mutation
- production activation

## Result

Current status:

SCOPED_PACKAGE_1_OPEN_FIRST_OPERATION_CLASS_SELECTED_FOR_PLANNING_ONLY_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_SELECTED_FOR_PLANNING_ONLY_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Scoped Package.2 — expected-hash/build-hash evidence package requirements inventory.

Scoped Package.2 should inventory the exact requirements for a future expected-hash/build-hash evidence package.

Scoped Package.2 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=scoped-package-1-first-operation-class-selection-planning
timestamp_utc=2026-07-06T21:36:53Z
repo_only=true
planning_only=true
operation_class_selected_for_planning_only=true
future_go_granted=false
build_executed=false
artifact_hash_computed=false
programdata_hash_computed=false
rpc_used=false
testnet_used=false
deploy_executed=false
upgrade_executed=false
write_buffer_executed=false
authority_change_executed=false
state_initialized=false
spl_setup_executed=false
guardian_package_constructed=false
signing_executed=false
submit_executed=false
mutation_executed=false
```

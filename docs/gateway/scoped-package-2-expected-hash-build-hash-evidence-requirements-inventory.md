# Scoped Package.2 — Expected-hash/build-hash evidence package requirements inventory

Status:

SCOPED_PACKAGE_2_OPEN_EXPECTED_HASH_BUILD_HASH_REQUIREMENTS_INVENTORY_COMPLETED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_REQUIREMENTS_INVENTORIED_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Scoped Package.2 inventories requirements for a future expected-hash/build-hash evidence package.

This is requirements inventory only.

It does not draft a runnable package.

It does not grant GO.

It does not run build.

It does not compute artifact hash.

It does not compute ProgramData executable-bytes hash.

It does not call RPC.

It does not use testnet.

It does not deploy.

It does not upgrade.

It does not write a buffer.

It does not change authority.

It does not initialize state.

It does not configure SPL.

It does not construct guardian packages.

It does not sign.

It does not submit or mutate any network.

## Evidence basis

- Scoped Package.1 — first operation class selection planning
- Final GO.5 — final scoped GO model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record
- Upgrade authority custody map
- xxxl-svm Cargo/program scaffold files

## Requirements inventory

```text
# Expected-hash/build-hash evidence package requirements inventory

SP2_REQ_01_REPO_SOURCE_BINDING
status: required
meaning: future package must bind repo full name, branch, source commit, and clean working tree status

SP2_REQ_02_BUILD_COMMAND_BINDING
status: required
meaning: future package must define exact build command before any build GO

SP2_REQ_03_TOOLCHAIN_LOCKFILE_BINDING
status: required
meaning: future package must bind Rust/Solana/SBF toolchain and lockfiles

SP2_REQ_04_FEATURE_FLAG_BINDING
status: required
meaning: future package must bind feature flags and dangerous gate status

SP2_REQ_05_LOCAL_ARTIFACT_PATH
status: required
meaning: future package must define exact local SBF artifact path

SP2_REQ_06_LOCAL_ARTIFACT_SHA256
status: required_future_value
meaning: future package must record local SBF artifact SHA256 after separately approved build/hash execution

SP2_REQ_07_CANONICAL_PROGRAMDATA_HASH_DOMAIN
status: required
meaning: canonical runtime hash domain is ProgramData executable bytes excluding loader metadata

SP2_REQ_08_CANONICAL_PROGRAMDATA_SHA256
status: required_future_value
meaning: future package must record canonical ProgramData executable-bytes SHA256 after separately approved hash execution

SP2_REQ_09_BASELINE_PROGRAM_BINDING
status: required
meaning: future package must bind current program id, ProgramData account, and upgrade authority baseline

SP2_REQ_10_EXPECTED_HASH_PACKAGE_ID
status: required_future_value
meaning: future package must have a unique expected-hash package id

SP2_REQ_11_NO_RPC_FOR_LOCAL_BUILD_HASH_PACKAGE
status: required_boundary
meaning: requirements inventory does not permit RPC/testnet; later read-only precheck is separate

SP2_REQ_12_STOP_ON_MISMATCH
status: required
meaning: future mismatch policy must be stop, not retry

SP2_REQ_13_EXACT_USER_GO_REQUIRED_BEFORE_EXECUTION
status: required
meaning: future build/hash execution requires exact scoped user GO phrase

SP2_REQ_14_NO_SECRET_MATERIAL
status: required
meaning: future evidence must not include private keys, seed phrases, or secret material

SP2_REQ_15_EVIDENCE_STORAGE_PATH
status: required
meaning: future package must define evidence directory and required files
```

## Requirements summary

- sp1_selected_operation_class_recorded: true
- final_go_not_granted: true
- blocker_b_expected_hash_requirements_present: true
- repo_source_binding_requirement_inventoried: true
- build_command_requirement_inventoried: true
- toolchain_lockfile_requirement_inventoried: true
- feature_flag_requirement_inventoried: true
- artifact_path_requirement_inventoried: true
- artifact_sha256_requirement_inventoried: true
- canonical_programdata_hash_domain_requirement_inventoried: true
- canonical_programdata_sha256_requirement_inventoried: true
- baseline_program_binding_requirement_inventoried: true
- expected_hash_package_id_requirement_inventoried: true
- stop_on_mismatch_requirement_inventoried: true
- exact_user_go_requirement_inventoried: true
- no_secret_material_requirement_inventoried: true
- evidence_storage_requirement_inventoried: true
- sp2_no_build_no_hash_no_rpc_no_execution: true

all_requirements_inventoried: true

## Repo file inventory

```text
# Repo file inventory for future expected-hash/build-hash package requirements

Cargo files:
- programs/xxxl-svm/Cargo.toml

Lockfiles:
- package-lock.json
- programs/xxxl-svm/Cargo.lock

Build-related tracked files sample:
- programs/xxxl-svm/Cargo.lock
- programs/xxxl-svm/Cargo.toml

Gateway docs count:
- 439

xxxl-svm program files count:
- 93
```

## Remaining gaps

- actual future expected-hash/build-hash package not drafted
- exact source commit for future execution not selected
- exact build command for future execution not selected
- exact toolchain versions for future execution not selected
- exact feature flag set for future execution not selected
- exact local artifact path for future execution not selected
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- expected-hash package id not selected
- future exact user GO phrase not selected
- build/hash execution remains not approved
- RPC/testnet remains not approved
- deploy/upgrade/write-buffer remains not approved
- mutation remains not approved

## Interpretation

Scoped Package.2 identifies what a future expected-hash/build-hash evidence package must contain.

It does not select actual future values for source commit, build command, toolchain versions, feature flags, artifact path, artifact hash, ProgramData hash, or exact user GO phrase.

It does not approve build or hash computation.

## Non-GO statement

Scoped Package.2 does not grant GO.

Scoped Package.2 does not approve:

- build
- local artifact hash computation
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

SCOPED_PACKAGE_2_OPEN_EXPECTED_HASH_BUILD_HASH_REQUIREMENTS_INVENTORY_COMPLETED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_REQUIREMENTS_INVENTORIED_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

all_requirements_inventoried: true

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Scoped Package.3 — expected-hash/build-hash evidence package decision model.

Scoped Package.3 should select the strict model for a future expected-hash/build-hash evidence package.

Scoped Package.3 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=scoped-package-2-expected-hash-build-hash-evidence-requirements-inventory
timestamp_utc=2026-07-06T21:42:29Z
repo_only=true
requirements_inventory_only=true
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

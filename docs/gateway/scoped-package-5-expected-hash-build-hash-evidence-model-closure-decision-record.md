# Scoped Package.5 — Expected-hash/build-hash evidence model closure decision record

Status:

SCOPED_PACKAGE_5_CLOSED_NARROW_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_CLOSED_NARROW_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Scoped Package.5 records the narrow closure decision for the expected-hash/build-hash evidence model.

This closure does not grant GO.

This closure does not approve execution.

It closes only the model boundary for a future expected-hash/build-hash evidence package.

## Evidence basis

- Scoped Package.1 — first operation class selection planning
- Scoped Package.2 — expected-hash/build-hash evidence package requirements inventory
- Scoped Package.3 — expected-hash/build-hash evidence package decision model
- Scoped Package.4 — expected-hash/build-hash evidence package invariant review
- Final GO.5 — final scoped GO model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Closure decision

EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_CLOSED_NARROW_FINAL_GO_NOT_GRANTED

Meaning:

The expected-hash/build-hash evidence model is closed narrowly.

The project does not yet have an actual future execution package.

The project does not yet have approval for build, hash computation, RPC, testnet, submit, or mutation.

## Closed model points

- Scoped Package.1 selected expected-hash/build-hash evidence package as first future operation class for planning only
- Scoped Package.2 inventoried requirements
- Scoped Package.3 selected strict expected-hash/build-hash evidence package model
- Scoped Package.4 reviewed invariants and prepared closure candidate
- strict local evidence model required
- full hash bundle required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- SHA256 required
- repo/source/build/toolchain/lockfile/feature flag bindings required
- baseline program id, ProgramData, and authority bindings required
- local package must not include RPC/testnet
- local package must not authorize upgrade/write-buffer or mutation
- exact scoped user GO required before any future build/hash execution
- no secret material allowed in evidence
- any mismatch requires stop
- automatic retry rejected
- current GO state remains FINAL_GO_NOT_GRANTED
- no execution approved

## Closure summary

```text
# Expected-hash/build-hash evidence model closure summary

closure_status: SCOPED_PACKAGE_5_CLOSED_NARROW_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED
closure_decision: EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_CLOSED_NARROW_FINAL_GO_NOT_GRANTED
selected_model: STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED
selected_operation_class: EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY
current_go_state: FINAL_GO_NOT_GRANTED
closure_scope: narrow_expected_hash_build_hash_evidence_model_boundary_only

strict_model_required: true
full_hash_bundle_required: true
canonical_runtime_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
hash_algorithm: SHA256
source_build_toolchain_lockfile_feature_binding_required: true
baseline_program_programdata_authority_binding_required: true
local_package_no_rpc_testnet: true
local_package_no_upgrade_write_buffer_mutation: true
exact_scoped_user_go_required_before_build_hash: true
no_secret_material_required: true
stop_on_mismatch_required: true
automatic_retry_rejected: true

execution_approved: false
build_approved: false
artifact_hash_computation_approved: false
programdata_hash_computation_approved: false
rpc_approved: false
testnet_approved: false
deploy_approved: false
upgrade_approved: false
write_buffer_approved: false
authority_change_approved: false
state_init_approved: false
spl_setup_approved: false
guardian_package_approved: false
signing_approved: false
submit_approved: false
mutation_approved: false
production_activation_approved: false
```

## Future package boundary

```text
# Future expected-hash/build-hash package boundary

This closure does not create a runnable future package.
This closure does not approve build or hash computation.

A later package may draft exact execution requirements only if it preserves:
- strict expected-hash/build-hash evidence package model
- single operation boundary
- local build/hash boundary
- no RPC/testnet inside local evidence package
- no deploy/upgrade/write-buffer
- no state init
- no SPL setup
- no guardian package construction
- no signing
- no submit
- no mutation
- no secrets
- stop-on-mismatch
- no automatic retry
- exact scoped user GO phrase required before execution

Required future package values remain unset:
- package id
- source commit
- clean repo status
- build command
- toolchain versions
- lockfiles
- feature flags
- artifact path
- local SBF artifact SHA256
- canonical ProgramData executable-bytes SHA256
- exact user GO phrase
- evidence path
```

## Still not approved

- actual future execution package drafting
- actual source commit selection
- actual build command selection
- actual toolchain version selection
- actual feature flag selection
- actual artifact path selection
- actual local SBF artifact SHA256 computation
- actual canonical ProgramData executable-bytes SHA256 computation
- actual expected-hash package id generation
- actual exact scoped user GO phrase selection
- build/hash execution
- RPC/testnet
- deploy/upgrade/write-buffer
- state initialization
- SPL setup
- guardian package construction
- signing
- submit
- mutation
- production activation

## Non-GO statement

Scoped Package.5 does not grant GO.

Scoped Package.5 does not approve:

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

SCOPED_PACKAGE_5_CLOSED_NARROW_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_CLOSED_NARROW_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Evidence Package.1 — exact expected-hash/build-hash execution package planning only.

Evidence Package.1 may draft exact future package fields, but must still not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

Any actual build/hash execution still requires a later exact scoped user GO phrase.

## Evidence preview

metadata:

```text
phase=scoped-package-5-expected-hash-build-hash-evidence-model-closure-decision-record
timestamp_utc=2026-07-06T22:23:28Z
repo_only=true
closure_decision_only=true
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

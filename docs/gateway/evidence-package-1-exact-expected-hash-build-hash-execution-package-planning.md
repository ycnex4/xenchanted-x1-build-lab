# Evidence Package.1 — Exact expected-hash/build-hash execution package planning

Status:

EVIDENCE_PACKAGE_1_OPEN_EXACT_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_SHAPE_SELECTED_FINAL_GO_NOT_GRANTED

Selected package shape:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Evidence Package.1 defines the shape of a future exact expected-hash/build-hash execution package.

This is planning only.

It does not fill exact execution values.

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

- Scoped Package.5 — expected-hash/build-hash evidence model closure decision record
- Scoped Package.4 — expected-hash/build-hash evidence invariant review
- Scoped Package.3 — expected-hash/build-hash evidence decision model
- Scoped Package.2 — requirements inventory
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Selected package shape

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

Meaning:

A future exact execution package may be drafted as a strict local build/hash package, but only inside the closed Scoped Package.5 model.

This step only selects the package shape and required fields.

## Required future fields

- package id
- operation class
- repo full name
- branch
- source commit
- clean working tree status
- build command
- Rust toolchain version
- Solana/SBF toolchain version
- lockfiles
- feature flags
- local artifact path
- local SBF artifact SHA256 output field
- canonical ProgramData executable-bytes SHA256 output field
- canonical hash domain
- SHA256 algorithm
- baseline program id
- baseline ProgramData account
- baseline upgrade authority
- evidence directory
- no secret material statement
- exact scoped user GO phrase gate
- stop-on-mismatch rule
- no automatic retry rule

## Values not selected yet

- exact source commit
- exact build command
- exact toolchain versions
- exact feature flags
- exact artifact path
- exact evidence path
- exact user GO phrase
- local SBF artifact SHA256
- canonical ProgramData executable-bytes SHA256

## Execution package shape

```text
# Evidence Package.1 execution package shape

selected_package_shape: STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY
selected_model: STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED
selected_operation_class: EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false

purpose:
Define the shape of a future exact build/hash execution package without executing it.

package_must_include_sections:
- package identity
- operation class
- repo/source binding
- clean working tree binding
- build command binding
- toolchain binding
- lockfile binding
- feature flag binding
- local artifact path binding
- local artifact SHA256 output field
- canonical ProgramData executable-bytes SHA256 output field
- canonical hash domain statement
- SHA256 algorithm statement
- baseline program id / ProgramData / authority binding
- evidence directory binding
- no secret material statement
- exact scoped user GO phrase gate
- stop-on-mismatch rule
- no automatic retry rule
- non-RPC/testnet boundary
- no deploy/upgrade/write-buffer boundary
- no mutation boundary
- post-execution repo status check

package_must_not_include:
- RPC call
- testnet call
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- signing
- transaction submit
- mutation
- production activation

current_step_does_not_select_values:
- exact source commit
- exact build command
- exact toolchain versions
- exact feature flags
- exact artifact path
- exact evidence path
- exact user GO phrase
- local SBF artifact SHA256
- canonical ProgramData executable-bytes SHA256
```

## Planning decision matrix

```text
# Evidence Package.1 planning decision matrix

EP1_MODEL_0_NO_EXECUTION_PACKAGE_PLANNING
status: rejected
meaning: Stop without defining the future execution package shape.
reason_rejected: Scoped Package.5 identifies Evidence Package.1 as next safe planning step.

EP1_MODEL_1_DIRECT_BUILD_HASH_EXECUTION
status: rejected
meaning: Execute build/hash now.
reason_rejected: Current GO state is FINAL_GO_NOT_GRANTED and build/hash is not approved.

EP1_MODEL_2_COMBINED_BUILD_HASH_RPC_UPGRADE_PACKAGE
status: rejected
meaning: Combine build/hash, RPC precheck, upgrade/write-buffer, and mutation.
reason_rejected: Violates single-boundary scoped package model and no-RPC/no-mutation boundary.

EP1_MODEL_3_LOCAL_BUILD_HASH_PLANNING_WITHOUT_GO_GATE
status: rejected
meaning: Draft a package that can later run without exact scoped GO.
reason_rejected: Exact scoped user GO phrase is required before any build/hash execution.

EP1_MODEL_4_STRICT_LOCAL_EXECUTION_PACKAGE_SHAPE_ONLY
status: selected
meaning: Define a strict local build/hash execution package shape, but do not fill values and do not execute.
reason_selected: Allows safe progression toward a future exact package while preserving NO-GO.

SELECTED_EP1_MODEL
STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

SELECTED_EP1_CURRENT_GO_STATE
FINAL_GO_NOT_GRANTED

SELECTED_EP1_BOUNDARY
NO_BUILD_NO_HASH_NO_RPC_NO_TESTNET_NO_DEPLOY_NO_UPGRADE_NO_WRITE_BUFFER_NO_STATE_INIT_NO_SPL_SETUP_NO_GUARDIAN_PACKAGE_NO_SIGNING_NO_SUBMIT_NO_MUTATION

SELECTED_EP1_NEXT_SAFE_STEP
EVIDENCE_PACKAGE_2_EXACT_VALUES_INVENTORY
```

## Non-GO boundary

```text
# Evidence Package.1 non-GO boundary

Evidence Package.1 does not grant GO.

Evidence Package.1 does not approve:
- build
- local artifact hash computation
- ProgramData executable-bytes hash computation
- RPC
- testnet
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- signing
- transaction submit
- mutation
- production activation

Any actual build/hash execution still requires a later exact scoped user GO phrase.
```

## Non-GO statement

Evidence Package.1 does not grant GO.

Evidence Package.1 does not approve:

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

EVIDENCE_PACKAGE_1_OPEN_EXACT_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_SHAPE_SELECTED_FINAL_GO_NOT_GRANTED

Selected package shape:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Evidence Package.2 — exact expected-hash/build-hash package values inventory.

Evidence Package.2 may inventory candidate exact values for source commit, build command, toolchain, feature flags, artifact path, and evidence path.

Evidence Package.2 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=evidence-package-1-exact-expected-hash-build-hash-execution-package-planning
timestamp_utc=2026-07-06T22:30:13Z
repo_only=true
execution_package_planning_only=true
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

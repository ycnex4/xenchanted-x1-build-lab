# Evidence Package.2 — Exact expected-hash/build-hash package values inventory

Status:

EVIDENCE_PACKAGE_2_OPEN_EXACT_EXPECTED_HASH_BUILD_HASH_VALUES_INVENTORY_COMPLETED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

CANDIDATE_EXPECTED_HASH_BUILD_HASH_EXECUTION_VALUES_INVENTORIED_FINAL_GO_NOT_GRANTED

Selected package shape:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Evidence Package.2 inventories candidate exact values for a future expected-hash/build-hash execution package.

This is values inventory only.

It does not approve the candidate values for execution.

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

- Evidence Package.1 — exact expected-hash/build-hash execution package planning
- Scoped Package.5 — expected-hash/build-hash evidence model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record
- Upgrade authority custody map
- programs/xxxl-svm/Cargo.toml
- programs/xxxl-svm/Cargo.lock

## Values inventory summary

```text
# Evidence Package.2 values inventory summary

package_id_candidate: EP2-CANDIDATE-cbb205a0f950
repo_full_name: ycnex4/xenchanted-x1-build-lab
branch: evidence-package-2-exact-expected-hash-build-hash-package-values-inventory
source_commit: cbb205a0f9500608eda70ab533492dbf64f6c69f
repo_clean_status: false

build_command_candidate: cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features
selected_feature_flags_candidate: --no-default-features
dangerous_features_selected: false

local_artifact_path_candidate: programs/xxxl-svm/target/deploy/xxxl_svm.so
evidence_path_candidate: docs/gateway/evidence/evidence-package-2-exact-expected-hash-build-hash-package-values-inventory-execution

local_artifact_sha256: UNSET_NOT_COMPUTED
canonical_programdata_executable_bytes_sha256: UNSET_NOT_COMPUTED

canonical_runtime_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
hash_algorithm: SHA256

baseline_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
baseline_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
baseline_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

exact_user_go_phrase_candidate: UNSET_NOT_SELECTED
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
```

## Source binding inventory

```text
# Evidence Package.2 source binding inventory

package_id_candidate: EP2-CANDIDATE-cbb205a0f950
repo_full_name: ycnex4/xenchanted-x1-build-lab
remote_url: https://github.com/ycnex4/xenchanted-x1-build-lab.git
branch: evidence-package-2-exact-expected-hash-build-hash-package-values-inventory
source_commit: cbb205a0f9500608eda70ab533492dbf64f6c69f
repo_clean_status: false
selected_package_shape: STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY
selected_model: STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED
operation_class: EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
```

## Build binding inventory

```text
# Evidence Package.2 build binding inventory

build_command_candidate: cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features
build_command_status: candidate_selected_for_review_not_approved_for_execution
build_command_executed: false

manifest_path: programs/xxxl-svm/Cargo.toml
lockfile_path: programs/xxxl-svm/Cargo.lock
lockfile_present: true

default_features_line: default = []
feature_policy_candidate: NO_DEFAULT_FEATURES_AND_NO_DANGEROUS_FEATURES
selected_feature_flags_candidate: --no-default-features
dangerous_features_present_in_manifest: true
dangerous_features_selected: false

non_dangerous_features_detected:
- default
- phase-41k4-svm-test-harness
- phase-41k5-spl-mint-to-cpi-test-gate
- phase-41k5-d2-production-path-test-gate
- phase-41k6-b1-v3-account-contract-test-gate
- phase-41k6-b1b-guardian-set-loading-test-gate
- phase-41k6-b1c-ed25519-evidence-wiring-test-gate
- phase-41k6-b1c7-handler-integration-test-gate

dangerous_features_detected:
- dangerously-allow-phase-41k4-svm-test-harness-sbf-build
- dangerously-allow-phase-41k5-spl-mint-to-cpi-test-gate-sbf-build
- dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build
- dangerously-allow-phase-41k6-b1-v3-account-contract-test-gate-sbf-build
- dangerously-allow-phase-41k6-b1b-guardian-set-loading-test-gate-sbf-build
- dangerously-allow-phase-41k6-b1c-ed25519-evidence-wiring-test-gate-sbf-build
- dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build
```

## Artifact/hash binding inventory

```text
# Evidence Package.2 artifact/hash binding inventory

local_artifact_path_candidate: programs/xxxl-svm/target/deploy/xxxl_svm.so
local_artifact_sha256: UNSET_NOT_COMPUTED
canonical_programdata_executable_bytes_sha256: UNSET_NOT_COMPUTED

canonical_runtime_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
hash_algorithm: SHA256

baseline_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
baseline_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
baseline_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

artifact_hash_computed: false
programdata_hash_computed: false
```

## Toolchain binding inventory

```text
# Evidence Package.2 toolchain binding inventory

toolchain_version_capture_status: deferred_to_future_execution_package_or_pre_execution_probe
rust_toolchain_version: UNSET_NOT_CAPTURED
cargo_version: UNSET_NOT_CAPTURED
solana_cli_version: UNSET_NOT_CAPTURED
cargo_build_sbf_version: UNSET_NOT_CAPTURED

reason:
Evidence Package.2 is repo-only values inventory.
It does not run toolchain commands.
Future package must capture these values before build/hash execution.
```

## GO boundary

```text
# Evidence Package.2 exact GO boundary

exact_user_go_phrase_candidate: UNSET_NOT_SELECTED

future_go_rule:
Any actual build/hash execution requires a later exact scoped user GO phrase.

current_go_state: FINAL_GO_NOT_GRANTED
execution_approved: false
build_approved: false
artifact_hash_computation_approved: false
programdata_hash_computation_approved: false
rpc_approved: false
testnet_approved: false
mutation_approved: false
```

## Remaining gaps

```text
# Evidence Package.2 remaining gaps

- Evidence Package.3 execution decision model not recorded
- Evidence Package.4 invariant review not recorded
- Evidence Package.5 closure decision not recorded
- exact scoped user GO phrase not selected
- toolchain versions not captured
- build command not approved for execution
- build not executed
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- RPC/testnet not approved
- deploy/upgrade/write-buffer not approved
- mutation not approved
```

## Non-GO boundary

```text
# Evidence Package.2 non-GO boundary

Evidence Package.2 does not grant GO.

Evidence Package.2 does not approve:
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
```

## Result

Current status:

EVIDENCE_PACKAGE_2_OPEN_EXACT_EXPECTED_HASH_BUILD_HASH_VALUES_INVENTORY_COMPLETED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

CANDIDATE_EXPECTED_HASH_BUILD_HASH_EXECUTION_VALUES_INVENTORIED_FINAL_GO_NOT_GRANTED

Selected package shape:

STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Evidence Package.3 — expected-hash/build-hash execution decision model.

Evidence Package.3 should define what a later exact build/hash execution step may do, what counts as success, and what causes stop.

Evidence Package.3 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=evidence-package-2-exact-expected-hash-build-hash-package-values-inventory
timestamp_utc=2026-07-06T22:33:53Z
repo_only=true
values_inventory_only=true
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

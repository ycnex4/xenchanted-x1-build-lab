# BuildHash Execution.1.3 — Exact scoped local build/hash GO package closure record

Status:

BUILDHASH_EXECUTION_1_3_CLOSED_EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_READY_FOR_USER_EXACT_GO_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_CLOSED_FINAL_GO_NOT_GRANTED_UNTIL_USER_EXACT_PHRASE

Selected execution model:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

Final package id:

BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49

Final program source commit:

ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6

Exact GO phrase selected:

true

User exact GO phrase provided:

false

Execution approved now:

false

NO-GO REMAINS_FOR_RPC_TESTNET_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_SUBMIT_MUTATION

## Purpose

BuildHash Execution.1.3 closes the exact scoped local build/hash GO package.

It selects the exact GO phrase for a future BuildHash Execution.2 user message.

It does not execute build/hash by itself.

It does not call RPC.

It does not use testnet.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- BuildHash Execution.1.1 — exact scoped local build/hash GO package draft
- BuildHash Execution.1.2 — exact scoped local build/hash GO package invariant review
- Evidence Package.5 — expected-hash/build-hash execution decision closure record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Closure summary

```text
# BuildHash Execution.1.3 closure summary

closure_status: BUILDHASH_EXECUTION_1_3_CLOSED_EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_READY_FOR_USER_EXACT_GO_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION
closure_decision: EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_CLOSED_FINAL_GO_NOT_GRANTED_UNTIL_USER_EXACT_PHRASE
selected_execution_model: STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY
current_go_state: FINAL_GO_NOT_GRANTED

final_package_id: BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49
final_program_source_commit: ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6
final_program_source_short: ee0cb44f7d49

exact_go_phrase_selected: true
exact_go_phrase: GO_BHX2_LOCAL_BUILD_HASH_ONLY_BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49_SOURCE_ee0cb44f7d49
user_exact_go_phrase_provided: false
execution_approved_now: false

repo_full_name: ycnex4/xenchanted-x1-build-lab
remote_url: https://github.com/ycnex4/xenchanted-x1-build-lab.git

build_command: cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features
feature_flags: --no-default-features
dangerous_features_selected: false
artifact_path: programs/xxxl-svm/target/deploy/xxxl_svm.so
evidence_path: docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go

hash_algorithm: SHA256
canonical_runtime_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

baseline_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
baseline_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
baseline_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

program_source_binding_policy:
Future BuildHash Execution.2 must verify that programs/xxxl-svm source content matches final_program_source_commit before executing build/hash.
Documentation-only commits after this closure do not change the bound program source if programs/xxxl-svm is unchanged.

execution_authorization_policy:
BuildHash Execution.2 may execute local build/hash only if the user provides exact_go_phrase verbatim.
```

## Exact GO phrase

```text
# BuildHash Execution.1.3 exact GO phrase

final_package_id: BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49
final_program_source_commit: ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6
final_program_source_short: ee0cb44f7d49

exact_go_phrase_selected: true
exact_go_phrase: GO_BHX2_LOCAL_BUILD_HASH_ONLY_BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49_SOURCE_ee0cb44f7d49

user_exact_go_phrase_provided: false
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved_now: false

meaning:
This phrase is selected for a future BuildHash Execution.2 local build/hash execution only.
It does not authorize RPC, testnet, deploy, upgrade, write-buffer, authority change, state initialization, SPL setup, guardian package construction, signing, transaction submit, mutation, or production activation.
```

## Final execution preconditions

```text
# BuildHash Execution.1.3 final execution preconditions

required_before_buildhash_execution_2:
- user provides exact GO phrase verbatim: GO_BHX2_LOCAL_BUILD_HASH_ONLY_BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49_SOURCE_ee0cb44f7d49
- repo is clean before execution
- programs/xxxl-svm source content matches final_program_source_commit: ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6
- build command equals: cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features
- feature flags equal: --no-default-features
- dangerous features selected: false
- artifact path equals: programs/xxxl-svm/target/deploy/xxxl_svm.so
- evidence path equals: docs/gateway/evidence/buildhash-execution-2-local-build-hash-execution-after-exact-go
- hash algorithm equals: SHA256
- canonical hash domain equals: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
- toolchain versions are captured before build
- no RPC/testnet is attempted
- no mutation is attempted
- no secret material is requested, printed, or required

stop_if:
- exact GO phrase is missing or differs
- repo is dirty before execution
- programs/xxxl-svm differs from final_program_source_commit
- toolchain capture fails
- build command differs
- dangerous feature is selected
- build fails
- artifact is missing or empty
- local artifact SHA256 fails
- canonical hash method is missing or ambiguous
- canonical ProgramData executable-bytes SHA256 fails
- RPC/testnet is attempted
- deploy/upgrade/write-buffer/signing/submit/mutation is attempted
- secret material is requested, printed, or required
- final git status has unexplained changes

automatic_retry: rejected
```

## Allowed next block

```text
# BuildHash Execution.1.3 allowed next block

next_block: BuildHash Execution.2 — local build/hash execution after exact GO

BuildHash Execution.2 may only:
- verify exact GO phrase
- verify clean repo
- verify bound program source content
- capture toolchain versions
- run the exact local build command
- verify artifact exists and is non-empty
- compute local SBF artifact SHA256
- compute canonical ProgramData executable-bytes SHA256
- write local evidence
- capture final git status

BuildHash Execution.2 must not:
- call RPC
- use testnet
- deploy
- upgrade
- write buffer
- change authority
- initialize state
- configure SPL
- construct guardian packages
- sign
- submit transaction
- mutate network/state
- activate production
- request, print, or require secret material
```

## Remaining gaps

```text
# BuildHash Execution.1.3 remaining gaps

- user has not provided exact GO phrase
- BuildHash Execution.2 not executed
- toolchain versions not captured
- build not executed
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- RPC/testnet not approved
- deploy/upgrade/write-buffer not approved
- state initialization not approved
- SPL setup not approved
- guardian package construction not approved
- signing not approved
- submit not approved
- mutation not approved
- production activation not approved
```

## Non-GO boundary

```text
# BuildHash Execution.1.3 non-GO boundary

BuildHash Execution.1.3 does not grant GO by itself.

BuildHash Execution.1.3 selects the exact GO phrase for a future user message.

BuildHash Execution.1.3 does not approve now:
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

Actual local build/hash execution still requires the user to provide the exact GO phrase verbatim.
```

## Result

Current status:

BUILDHASH_EXECUTION_1_3_CLOSED_EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_READY_FOR_USER_EXACT_GO_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

EXACT_SCOPED_LOCAL_BUILD_HASH_GO_PACKAGE_CLOSED_FINAL_GO_NOT_GRANTED_UNTIL_USER_EXACT_PHRASE

Final package id:

BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49

Final program source commit:

ee0cb44f7d496e010e784608d0c7ccc8e84e7fb6

Exact GO phrase:

GO_BHX2_LOCAL_BUILD_HASH_ONLY_BHX2_LOCAL_BUILD_HASH_ee0cb44f7d49_SOURCE_ee0cb44f7d49

Current GO state:

FINAL_GO_NOT_GRANTED

user_exact_go_phrase_provided: false

execution_approved_now: false

## Next safe step

BuildHash Execution.2 — local build/hash execution after exact GO.

BuildHash Execution.2 may start only if the user provides the exact GO phrase verbatim.

BuildHash Execution.2 must remain local build/hash only and must not call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=buildhash-execution-1-3-exact-scoped-local-build-hash-go-package-closure-record
timestamp_utc=2026-07-06T23:10:15Z
repo_only=true
closure_record_only=true
exact_go_phrase_selected=true
user_exact_go_phrase_provided=false
final_go_granted=false
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

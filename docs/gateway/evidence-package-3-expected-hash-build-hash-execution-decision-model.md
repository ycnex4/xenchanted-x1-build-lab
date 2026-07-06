# Evidence Package.3 — Expected-hash/build-hash execution decision model

Status:

EVIDENCE_PACKAGE_3_OPEN_EXPECTED_HASH_BUILD_HASH_EXECUTION_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected execution model:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Evidence Package.3 records the decision model for a future local build/hash execution step.

This is decision-model only.

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
- Evidence Package.2 — exact expected-hash/build-hash package values inventory
- Scoped Package.5 — expected-hash/build-hash evidence model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record

## Selected execution model

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Meaning:

A later execution step may run only a strict local build/hash flow after exact scoped user GO phrase.

It must remain separate from RPC/testnet, deploy/upgrade/write-buffer, signing, submit, and mutation.

## Future actions allowed only after exact GO

- capture git branch and source commit
- verify clean working tree before build
- capture Rust/Cargo/Solana/cargo-build-sbf versions
- run exact package-bound local build command
- verify expected local artifact path
- compute local SBF artifact SHA256
- compute canonical ProgramData executable-bytes SHA256 using declared local canonicalization method
- write evidence files
- capture final git status

## Explicitly not allowed in future build/hash execution package

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
- private keys, seed phrases, or secret material

## Execution decision model

```text
# Evidence Package.3 execution decision model

selected_execution_model: STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY
selected_package_shape: STRICT_LOCAL_EXPECTED_HASH_BUILD_HASH_EXECUTION_PACKAGE_PLANNING_ONLY
selected_evidence_model: STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED
current_go_state: FINAL_GO_NOT_GRANTED
execution_approved_now: false

future_execution_allowed_only_after:
- Evidence Package.4 invariant review
- Evidence Package.5 closure decision
- exact scoped user GO phrase
- clean working tree
- exact source commit binding
- exact build command binding
- exact toolchain version capture
- exact feature flag binding
- no dangerous features
- exact artifact path binding
- exact evidence path binding

future_execution_may_do_after_exact_go:
- capture git branch and source commit
- verify clean working tree before build
- capture rust/cargo/solana/cargo-build-sbf versions
- run exact local build command
- verify local artifact exists
- compute local SBF artifact SHA256
- compute canonical ProgramData executable-bytes SHA256 using declared local canonicalization method
- write evidence files
- verify no RPC/testnet/deploy/upgrade/write-buffer/state/SPL/guardian/signing/submit/mutation occurred
- capture final git status

future_execution_must_not_do:
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
- access private keys, seed phrases, or secret material

source_commit_policy:
EP2 source_commit is an inventory candidate only.
A future execution package must bind the then-current source commit exactly.
If source commit differs from the bound commit, stop.

repo_clean_status_policy:
Future execution requires clean working tree before build.
If working tree is dirty before execution, stop.
If build creates ignored target outputs, this is acceptable only if git status remains clean or the package explicitly records the expected ignored-output behavior.

hash_policy:
local_sbf_artifact_sha256 must be computed from the exact artifact path.
canonical_programdata_executable_bytes_sha256 must be computed using the declared canonical ProgramData executable-bytes method.
If canonicalization method is missing, ambiguous, or fails, stop.

success_policy:
Success requires all success criteria to be true and no stop condition to be triggered.

stop_policy:
Any mismatch requires stop.
Automatic retry is forbidden.
```

## Future execution success criteria

```text
# Future build/hash execution success criteria

SUCCESS_CRITERION_01_EXACT_GO
The exact scoped user GO phrase is present and matches the package phrase.

SUCCESS_CRITERION_02_CLEAN_REPO_BEFORE
Working tree is clean before execution.

SUCCESS_CRITERION_03_SOURCE_COMMIT_BOUND
The current source commit matches the package-bound source commit.

SUCCESS_CRITERION_04_TOOLCHAIN_CAPTURED
Rust, Cargo, Solana CLI, and cargo-build-sbf versions are captured in evidence.

SUCCESS_CRITERION_05_EXACT_BUILD_COMMAND
Only the package-bound build command is executed.

SUCCESS_CRITERION_06_NO_DANGEROUS_FEATURES
No dangerous feature flags are selected.

SUCCESS_CRITERION_07_ARTIFACT_EXISTS
The expected local artifact path exists after build.

SUCCESS_CRITERION_08_ARTIFACT_NON_EMPTY
The local artifact is non-empty.

SUCCESS_CRITERION_09_LOCAL_ARTIFACT_SHA256_RECORDED
The local SBF artifact SHA256 is recorded.

SUCCESS_CRITERION_10_CANONICAL_PROGRAMDATA_SHA256_RECORDED
The canonical ProgramData executable-bytes SHA256 is recorded.

SUCCESS_CRITERION_11_NO_RPC_TESTNET
No RPC or testnet call is made.

SUCCESS_CRITERION_12_NO_MUTATION
No deploy, upgrade, write-buffer, authority change, state init, SPL setup, guardian construction, signing, submit, or mutation occurs.

SUCCESS_CRITERION_13_NO_SECRETS
No private keys, seed phrases, or secret material are requested or printed.

SUCCESS_CRITERION_14_EVIDENCE_COMPLETE
All required evidence files are written.

SUCCESS_CRITERION_15_FINAL_STATUS_CAPTURED
Final git status is captured.

all_success_criteria_required: true
```

## Future execution stop conditions

```text
# Future build/hash execution stop conditions

STOP_01_MISSING_OR_WRONG_GO_PHRASE
Exact scoped user GO phrase is missing or does not match.

STOP_02_DIRTY_REPO_BEFORE_EXECUTION
Working tree is dirty before execution.

STOP_03_SOURCE_COMMIT_MISMATCH
Current source commit differs from package-bound source commit.

STOP_04_TOOLCHAIN_MISSING
Required toolchain or version capture fails.

STOP_05_BUILD_COMMAND_MISMATCH
The command to be run differs from the package-bound command.

STOP_06_DANGEROUS_FEATURE_SELECTED
A dangerous feature is selected.

STOP_07_BUILD_FAILURE
The build command fails.

STOP_08_ARTIFACT_MISSING_OR_EMPTY
Expected local artifact is missing or empty.

STOP_09_LOCAL_HASH_FAILURE
Local artifact SHA256 computation fails.

STOP_10_CANONICAL_HASH_METHOD_MISSING
Canonical ProgramData executable-bytes method is missing or ambiguous.

STOP_11_CANONICAL_HASH_FAILURE
Canonical ProgramData executable-bytes SHA256 computation fails.

STOP_12_RPC_OR_TESTNET_ATTEMPT
Any RPC or testnet call is attempted.

STOP_13_MUTATION_ATTEMPT
Deploy, upgrade, write-buffer, authority change, state init, SPL setup, guardian construction, signing, submit, or mutation is attempted.

STOP_14_SECRET_MATERIAL
Private keys, seed phrases, or secret material are requested, printed, or required.

STOP_15_UNEXPECTED_OUTPUT
Unexpected artifact path, unexpected feature set, or unexpected output appears.

STOP_16_POST_STATUS_UNEXPLAINED
Final git status contains unexplained changes.

stop_policy: any_stop_condition_requires_stop
automatic_retry: rejected
```

## Future execution evidence file list

```text
# Future execution evidence file list

Required future execution evidence files:
- metadata.txt
- source-binding.txt
- pre-execution-git-status.txt
- toolchain-versions.txt
- build-command.txt
- build-output.txt
- artifact-info.txt
- local-artifact-sha256.txt
- canonical-programdata-executable-bytes-sha256.txt
- hash-method.txt
- success-criteria-report.txt
- stop-condition-report.txt
- post-execution-git-status.txt
- non-go-boundary.txt

Required future values:
- exact package id
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

## Remaining gaps

- Evidence Package.4 invariant review not recorded
- Evidence Package.5 closure decision not recorded
- exact scoped user GO phrase not selected
- future execution package not closed
- toolchain versions not captured
- build not executed
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- RPC/testnet not approved
- deploy/upgrade/write-buffer not approved
- mutation not approved

## Non-GO boundary

```text
# Evidence Package.3 non-GO boundary

Evidence Package.3 does not grant GO.

Evidence Package.3 does not approve:
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

Evidence Package.3 records the decision model only.
Any actual build/hash execution still requires a later exact scoped user GO phrase.
```

## Result

Current status:

EVIDENCE_PACKAGE_3_OPEN_EXPECTED_HASH_BUILD_HASH_EXECUTION_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected execution model:

STRICT_LOCAL_BUILD_HASH_EXECUTION_WITH_EXACT_GO_ONLY

Current GO state:

FINAL_GO_NOT_GRANTED

execution_approved: false

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Evidence Package.4 — expected-hash/build-hash execution decision invariant review.

Evidence Package.4 should review the exact-GO gate, clean repo requirement, source commit binding, toolchain capture, build command binding, no dangerous features, artifact/hash success criteria, stop conditions, no-RPC/testnet boundary, no-mutation boundary, and no-execution boundary.

Evidence Package.4 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=evidence-package-3-expected-hash-build-hash-execution-decision-model
timestamp_utc=2026-07-06T22:39:15Z
repo_only=true
execution_decision_model_only=true
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

# Scoped Package.3 — Expected-hash/build-hash evidence package decision model

Status:

SCOPED_PACKAGE_3_OPEN_EXPECTED_HASH_BUILD_HASH_EVIDENCE_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Scoped Package.3 records the decision model for a future expected-hash/build-hash evidence package.

This is decision-model only.

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
- Scoped Package.2 — expected-hash/build-hash evidence package requirements inventory
- Final GO.5 — final scoped GO model closure decision record
- Blocker B.5 — expected post-upgrade ProgramData hash closure decision record
- Upgrade authority custody map
- xxxl-svm Cargo/program scaffold files

## Selected decision model

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Meaning:

A future expected-hash/build-hash evidence package must be strict, bounded, local, non-RPC, and separate from upgrade/write-buffer or any network mutation.

A later separate package may execute build/hash only after exact scoped user GO.

## Selected rules

- strict expected-hash/build-hash evidence package model required
- current package is decision-model only
- future package must be separate from RPC/testnet read-only precheck
- future package must be separate from deploy/upgrade/write-buffer
- full hash bundle required
- canonical runtime hash domain is ProgramData executable bytes excluding loader metadata
- SHA256 required
- repo/source/build/toolchain/lockfile/feature flag bindings required
- baseline program id, ProgramData, and authority bindings required
- local artifact SHA256 is required future evidence but not computed here
- canonical ProgramData executable-bytes SHA256 is required future evidence but not computed here
- exact scoped user GO phrase required before any build/hash execution
- no secrets in evidence
- any mismatch requires stop
- automatic retry rejected
- current GO state remains FINAL_GO_NOT_GRANTED
- no build/hash/RPC/testnet/submit/mutation approved

## Decision matrix

```text
# Expected-hash/build-hash evidence package decision matrix

SP3_MODEL_0_NO_EVIDENCE_PACKAGE_MODEL
status: rejected
meaning: Do not define a model for expected-hash/build-hash evidence.
reason_rejected: Blocker B requires expected-hash package and build/hash evidence before any upgrade/write-buffer GO.

SP3_MODEL_1_MONOLITHIC_BUILD_HASH_UPGRADE_PACKAGE
status: rejected
meaning: One package authorizes build, hash computation, expected-hash evidence, RPC precheck, upgrade/write-buffer, and post-upgrade verification.
reason_rejected: violates staged single-operation scoped GO model and bundles local execution, network read-only checks, and mutation.

SP3_MODEL_2_RUNNABLE_PACKAGE_WITHOUT_EXACT_USER_GO
status: rejected
meaning: A future evidence package can run build/hash without exact scoped user GO phrase.
reason_rejected: violates Final GO.5 exact user GO requirement.

SP3_MODEL_3_RPC_TESTNET_INCLUDED_IN_LOCAL_EVIDENCE_PACKAGE
status: rejected
meaning: Expected-hash/build-hash evidence package includes RPC/testnet observations.
reason_rejected: local build/hash evidence and read-only network prechecks are separate operation boundaries.

SP3_MODEL_4_ARTIFACT_HASH_ONLY
status: rejected
meaning: Local artifact SHA256 alone is enough as runtime hash evidence.
reason_rejected: Blocker B requires full hash bundle and canonical ProgramData executable-bytes SHA256 domain.

SP3_MODEL_5_UNBOUND_TOOLCHAIN_OR_FEATURE_FLAGS
status: rejected
meaning: Evidence package does not bind toolchain, lockfiles, build command, or feature flags.
reason_rejected: non-reproducible and incompatible with Blocker B requirements.

SP3_MODEL_6_STRICT_NON_EXECUTING_REQUIREMENTS_PACKAGE
status: selected
meaning: This package records the strict model for a future expected-hash/build-hash evidence package without running build/hash.
reason_selected: keeps current state safe while defining the future package shape.

SP3_MODEL_7_FUTURE_EXECUTION_PACKAGE_WITH_EXACT_GO_ONLY
status: selected_future_allowed_form
meaning: A later separate package may execute build/hash only after exact scoped user GO phrase and only inside the selected evidence model.
reason_selected: preserves staged approval while allowing future evidence generation.

SELECTED_SP3_MODEL
STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

SELECTED_SP3_CURRENT_GO_STATE
FINAL_GO_NOT_GRANTED

SELECTED_SP3_CURRENT_PACKAGE_KIND
DECISION_MODEL_ONLY_NOT_EXECUTION

SELECTED_SP3_FUTURE_PACKAGE_SHAPE
TWO_PHASE_REQUIREMENTS_THEN_EXPLICIT_GO_BEFORE_BUILD_HASH_EXECUTION

SELECTED_SP3_REQUIRED_EVIDENCE_BUNDLE
PACKAGE_ID_REPO_BRANCH_SOURCE_COMMIT_CLEAN_STATUS_BUILD_COMMAND_TOOLCHAIN_LOCKFILES_FEATURE_FLAGS_ARTIFACT_PATH_ARTIFACT_SHA256_CANONICAL_PROGRAMDATA_HASH_DOMAIN_CANONICAL_PROGRAMDATA_SHA256_BASELINE_PROGRAM_ID_PROGRAMDATA_AUTHORITY_STOP_RULE_USER_GO_PHRASE_EVIDENCE_PATH

SELECTED_SP3_CANONICAL_RUNTIME_HASH_DOMAIN
PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

SELECTED_SP3_HASH_ALGORITHM
SHA256

SELECTED_SP3_BOUNDARY
NO_BUILD_NO_HASH_NO_RPC_NO_TESTNET_NO_DEPLOY_NO_UPGRADE_NO_WRITE_BUFFER_NO_STATE_INIT_NO_SPL_SETUP_NO_GUARDIAN_PACKAGE_NO_SIGNING_NO_SUBMIT_NO_MUTATION

SELECTED_SP3_STOP_RULE
ANY_MISMATCH_REQUIRES_STOP_NO_AUTOMATIC_RETRY

SELECTED_SP3_NEXT_SAFE_STEP
SCOPED_PACKAGE_4_EXPECTED_HASH_BUILD_HASH_EVIDENCE_MODEL_INVARIANT_REVIEW
```

## Future evidence package schema

```text
# Future expected-hash/build-hash evidence package schema

This schema is for a future package only.
Scoped Package.3 does not fill runtime values and does not grant GO.

REQUIRED_STATIC_BINDINGS:
- package_id
- operation_class
- repo_full_name
- branch
- source_commit
- repo_clean_status
- build_command
- rust_toolchain_version
- solana_toolchain_version
- sbf_toolchain_version
- lockfiles
- feature_flags
- dangerous_feature_gate_status
- local_sbf_artifact_path
- canonical_runtime_hash_domain
- canonical_hash_algorithm
- baseline_program_id
- baseline_programdata_account
- baseline_upgrade_authority
- evidence_directory
- no_secret_material_statement
- stop_on_mismatch_statement
- no_automatic_retry_statement
- exact_scoped_user_go_phrase_template

REQUIRED_FUTURE_VALUES_AFTER_SEPARATE_EXECUTION_GO:
- local_sbf_artifact_sha256
- canonical_programdata_executable_bytes_sha256
- generated_expected_hash_package_id
- command_output_evidence_file
- repo_clean_status_after_execution

EXPLICITLY_OUT_OF_SCOPE_FOR_THIS_LOCAL_EVIDENCE_PACKAGE:
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

EXECUTION_RULE:
Build/hash execution is not allowed unless a later separate scoped package records exact values and the user provides the exact scoped GO phrase.

STOP_RULE:
Any mismatch requires stop. Automatic retry is forbidden.
```

## Selected evidence model rules

```text
# Selected evidence model rules

SP3_RULE_01_STRICT_MODEL_REQUIRED
The future expected-hash/build-hash evidence package must use the strict selected model.

SP3_RULE_02_NO_EXECUTION_IN_MODEL_STEP
Scoped Package.3 is decision-model only and cannot execute build or hash computation.

SP3_RULE_03_FULL_HASH_BUNDLE_REQUIRED
The future package must bind both local SBF artifact SHA256 and canonical ProgramData executable-bytes SHA256.

SP3_RULE_04_CANONICAL_DOMAIN_REQUIRED
Canonical runtime hash domain is ProgramData executable bytes excluding loader metadata.

SP3_RULE_05_SHA256_REQUIRED
Canonical hash algorithm is SHA256.

SP3_RULE_06_SOURCE_AND_BUILD_BINDING_REQUIRED
Repo, branch, source commit, clean status, build command, toolchain, lockfiles, and feature flags must be bound.

SP3_RULE_07_BASELINE_BINDING_REQUIRED
Program id, ProgramData account, and upgrade authority baseline must be bound.

SP3_RULE_08_NO_RPC_TESTNET_IN_LOCAL_EVIDENCE_PACKAGE
Local build/hash evidence package must not include RPC/testnet; read-only network precheck is separate.

SP3_RULE_09_EXACT_USER_GO_REQUIRED_BEFORE_BUILD_HASH
No build/hash execution may happen without a later exact scoped user GO phrase.

SP3_RULE_10_NO_UPGRADE_OR_MUTATION
Expected-hash/build-hash evidence package does not authorize upgrade/write-buffer or mutation.

SP3_RULE_11_NO_SECRETS
Evidence must not include private keys, seed phrases, or secret material.

SP3_RULE_12_STOP_NO_RETRY
Any mismatch requires stop. Automatic retry is forbidden.
```

## Remaining gaps

- Scoped Package.4 invariant review not recorded
- Scoped Package.5 closure decision not recorded
- actual future execution package not drafted
- exact source commit not selected
- exact build command not selected
- exact toolchain versions not selected
- exact feature flags not selected
- exact artifact path not selected
- local SBF artifact SHA256 not computed
- canonical ProgramData executable-bytes SHA256 not computed
- expected-hash package id not generated
- exact scoped user GO phrase not selected
- build/hash execution remains not approved
- RPC/testnet remains not approved
- deploy/upgrade/write-buffer remains not approved
- mutation remains not approved

## Non-GO statement

Scoped Package.3 does not grant GO.

Scoped Package.3 does not approve:

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

SCOPED_PACKAGE_3_OPEN_EXPECTED_HASH_BUILD_HASH_EVIDENCE_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected operation class:

EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_PLANNING_ONLY

Selected model:

STRICT_EXPECTED_HASH_BUILD_HASH_EVIDENCE_PACKAGE_MODEL_REQUIRED

Current GO state:

FINAL_GO_NOT_GRANTED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Scoped Package.4 — expected-hash/build-hash evidence package invariant review.

Scoped Package.4 should review the strict evidence package model, full hash bundle rule, canonical hash domain, exact bindings, no-RPC boundary, no-upgrade boundary, exact user GO requirement, stop-on-mismatch rule, and no-execution boundary.

Scoped Package.4 must not run build, compute hashes, call RPC, use testnet, deploy, upgrade, initialize state, configure SPL, construct packages, sign, submit, or mutate.

## Evidence preview

metadata:

```text
phase=scoped-package-3-expected-hash-build-hash-evidence-decision-model
timestamp_utc=2026-07-06T22:07:16Z
repo_only=true
decision_model_only=true
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

# Final GO.3 — Final scoped GO package decision model

Status:

FINAL_GO_3_OPEN_FINAL_SCOPED_GO_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected model:

STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Purpose

Final GO.3 records the decision model for future final scoped GO packages.

This is decision-model only.

It does not grant GO.

It does not run build.

It does not compute artifact hash.

It does not compute ProgramData hash.

It does not deploy.

It does not upgrade.

It does not write a buffer.

It does not change authority.

It does not initialize state.

It does not configure SPL.

It does not construct guardian packages.

It does not sign.

It does not call RPC.

It does not use testnet.

It does not submit or mutate any network.

## Evidence basis

- Final GO.1 — final scoped GO package planning
- Final GO.2 — repo-grounded final GO package inventory
- Blocker A-H narrow closure records

## Selected decision model

STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED

Meaning:

A future final scoped GO package must be staged, exact, and limited to one operation class.

A future GO package must not bundle build/hash, read-only RPC, upgrade/write-buffer, state init, SPL setup, guardian package/signing/submit, and production activation into one broad approval.

## Selected rules

- one future GO package authorizes exactly one operation class only
- general GO is rejected
- multi-action chained GO is rejected
- expected-hash package is required before any upgrade/write-buffer GO
- read-only baseline precheck is required before any network mutation GO
- identity/hash/authority/network/cost/verification mismatch requires stop
- automatic retry is rejected
- exact scoped user GO phrase is required for each future action
- post-action read-only verification is required after any mutation
- Final GO.3 itself does not grant GO
- Final GO.3 does not approve build/hash/RPC/testnet/submit/mutation

## Decision matrix

```text
# Final scoped GO package decision matrix

FG3_MODEL_0_GENERAL_GO
status: rejected
meaning: A broad permission to proceed with multiple actions.
reason_rejected: too broad, not evidence-bounded, and incompatible with stop-on-mismatch.

FG3_MODEL_1_CHAINED_MULTI_ACTION_GO
status: rejected
meaning: One GO authorizes build, hash, upgrade, state init, SPL setup, guardian package, signing, submit, and post-verification as a chained sequence.
reason_rejected: too many irreversible boundaries in one approval.

FG3_MODEL_2_MUTATION_GO_WITHOUT_EXPECTED_HASH_PACKAGE
status: rejected
meaning: A network mutation is allowed before an expected-hash package exists.
reason_rejected: incompatible with Blocker B closure.

FG3_MODEL_3_MUTATION_GO_WITHOUT_READ_ONLY_PRECHECKS
status: rejected
meaning: A network mutation is allowed without current read-only baseline observation.
reason_rejected: unsafe because program id, ProgramData, authority, network, and cost boundaries must be confirmed first.

FG3_MODEL_4_AUTOMATIC_RETRY_GO
status: rejected
meaning: Future package permits automatic retry after mismatch or failed verification.
reason_rejected: incompatible with Blocker G and Blocker B mismatch policy.

FG3_MODEL_5_SINGLE_OPERATION_SCOPED_GO
status: selected
meaning: A future GO package may authorize exactly one scoped operation class, with exact identity, evidence, cost, abort rules, and post-verification.
reason_selected: smallest safe approval unit.

FG3_MODEL_6_STAGED_SINGLE_OPERATION_GO_SEQUENCE
status: selected_required_structure
meaning: Future work must be split into separate GO packages when crossing boundaries: build/hash evidence, read-only network precheck, upgrade/write-buffer, state init, SPL setup, guardian package/signing/submit, and post-action verification.
reason_selected: prevents accidental bundling of review-only, read-only, and mutation steps.

SELECTED_FG3_DECISION
STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED

SELECTED_FG3_CURRENT_GO_STATE
FINAL_GO_NOT_GRANTED

SELECTED_FG3_SINGLE_OPERATION_RULE
ONE_FINAL_GO_PACKAGE_AUTHORIZES_EXACTLY_ONE_OPERATION_CLASS_ONLY

SELECTED_FG3_REQUIRED_BINDINGS
OPERATION_NETWORK_PROGRAM_ID_PROGRAMDATA_AUTHORITY_SOURCE_COMMIT_BUILD_HASH_COST_ABORT_POST_VERIFY_USER_PHRASE

SELECTED_FG3_EXPECTED_HASH_RULE
EXPECTED_HASH_PACKAGE_REQUIRED_BEFORE_ANY_UPGRADE_OR_WRITE_BUFFER_GO

SELECTED_FG3_READ_ONLY_PRECHECK_RULE
READ_ONLY_BASELINE_PRECHECK_REQUIRED_BEFORE_ANY_NETWORK_MUTATION_GO

SELECTED_FG3_STOP_RULE
ANY_IDENTITY_HASH_AUTHORITY_NETWORK_COST_OR_VERIFICATION_MISMATCH_REQUIRES_STOP

SELECTED_FG3_RETRY_RULE
AUTOMATIC_RETRY_REJECTED

SELECTED_FG3_USER_GO_RULE
EXACT_USER_GO_PHRASE_REQUIRED_FOR_EACH_FUTURE_SCOPED_GO

SELECTED_FG3_NON_GO_RULE
FINAL_GO_3_DOES_NOT_GRANT_GO_AND_DOES_NOT_APPROVE_BUILD_HASH_RPC_TESTNET_SUBMIT_OR_MUTATION
```

## Future GO state machine

```text
# Final GO state machine

STATE_0_NO_GO
meaning: No execution permission exists.
current_state_after_final_go_3: true

STATE_1_PLANNING_READY
meaning: Planning records exist, but no action is authorized.

STATE_2_INVENTORY_READY
meaning: Repo evidence inventory exists, but no action is authorized.

STATE_3_DECISION_MODEL_READY
meaning: Decision model exists, but no action is authorized.

STATE_4_FUTURE_PACKAGE_DRAFTED
meaning: A future exact package may be drafted for one operation class only.

STATE_5_FUTURE_PACKAGE_REVIEWED
meaning: A future exact package may be reviewed, still no action unless explicit user phrase is present.

STATE_6_EXACT_SCOPED_USER_GO_PRESENT
meaning: User has explicitly approved exactly one operation class with exact scope.

STATE_7_ALLOWED_ACTION_RUNNING
meaning: Only the approved operation class may be performed.

STATE_8_POST_ACTION_READ_ONLY_VERIFICATION
meaning: After action, required read-only verification bundle must be recorded.

STATE_9_COMPLETE_OR_STOP
meaning: If verification passes, close the operation package. If mismatch occurs, stop. No automatic retry.

DISALLOWED_TRANSITIONS:
- STATE_3_DECISION_MODEL_READY -> STATE_7_ALLOWED_ACTION_RUNNING
- STATE_4_FUTURE_PACKAGE_DRAFTED -> STATE_7_ALLOWED_ACTION_RUNNING
- STATE_5_FUTURE_PACKAGE_REVIEWED -> STATE_7_ALLOWED_ACTION_RUNNING without exact scoped user GO
- any state -> automatic retry after mismatch
- any state -> broader action than the exact GO scope
```

## Future GO package strict rules

```text
# Future final scoped GO package strict rules

RULE_01_SINGLE_OPERATION_CLASS_ONLY
A future GO package may authorize exactly one operation class.

RULE_02_NO_GENERAL_GO
A future GO package must not authorize broad project execution.

RULE_03_EXACT_IDENTITIES_REQUIRED
Network, program id, ProgramData account, authority, source commit, build command, feature flags, artifact path, hashes, cost boundary, and user GO phrase must be exact.

RULE_04_EXPECTED_HASH_PACKAGE_REQUIRED_FOR_UPGRADE
Upgrade/write-buffer GO cannot exist without a prior expected-hash package.

RULE_05_READ_ONLY_PRECHECK_REQUIRED_BEFORE_MUTATION
Any network mutation GO requires read-only baseline prechecks.

RULE_06_STOP_ON_MISMATCH
Any mismatch in identity, hash, authority, network, ProgramData, program id, cost boundary, or verification result requires stop.

RULE_07_NO_AUTOMATIC_RETRY
Automatic retry is forbidden.

RULE_08_POST_ACTION_VERIFICATION_REQUIRED
Any mutation package must define post-action read-only verification before it can be considered complete.

RULE_09_EXACT_USER_GO_PHRASE_REQUIRED
No action may run unless the user provides the exact future scoped GO phrase.

RULE_10_NO_SECRET_MATERIAL_IN_REPO
Private keys, seed phrases, and secret material must not be recorded in repo evidence.

RULE_11_NO_SCOPE_EXPANSION
The operator must not expand the action beyond the exact GO package scope.

RULE_12_SEPARATE_GO_FOR_SEPARATE_BOUNDARY
Build/hash, read-only RPC, upgrade/write-buffer, state init, SPL setup, guardian package/signing/submit, and production activation each require separate scoped approval if ever pursued.
```

## Future GO phrase template

```text
# Future exact scoped user GO phrase template

This is a template only.
Final GO.3 does not select an actual phrase and does not grant GO.

A future phrase must include:

I explicitly approve exactly this scoped operation:
<OPERATION_CLASS>

Network:
<NETWORK_NAME_AND_RPC_IDENTITY_IF_USED>

Program id:
<PROGRAM_ID>

ProgramData account:
<PROGRAMDATA_ACCOUNT>

Expected authority:
<AUTHORITY_PUBLIC_KEY>

Source commit:
<SOURCE_COMMIT>

Expected hash package:
<EXPECTED_HASH_PACKAGE_ID>

Maximum cost:
<MAX_COST>

Abort rule:
Stop on any mismatch. No automatic retry.

I understand this GO does not approve any action outside this exact scope.
```

## Remaining gaps

- actual final scoped GO package not drafted
- actual expected-hash package not generated
- actual build/hash evidence not generated
- actual read-only network precheck not executed
- actual operation class not selected
- actual user GO phrase not selected
- actual max cost boundary not selected
- actual post-action verification bundle not generated
- execution remains NO-GO

## Non-GO statement

Final GO.3 does not grant GO.

Final GO.3 does not approve:

- build
- local hash computation
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

FINAL_GO_3_OPEN_FINAL_SCOPED_GO_DECISION_MODEL_RECORDED_NO_BUILD_NO_HASH_NO_RPC_NO_EXECUTION

Current decision:

STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED_FINAL_GO_NOT_GRANTED

Selected model:

STAGED_SINGLE_OPERATION_SCOPED_GO_MODEL_REQUIRED

NO-GO REMAINS_FOR_BUILD_HASH_DEPLOY_UPGRADE_WRITE_BUFFER_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_SIGNING_RPC_TESTNET_NETWORK_SUBMIT_MUTATION

## Next safe step

Final GO.4 — final scoped GO package invariant review.

Final GO.4 should review the staged single-operation model, exact binding rules, expected-hash gate, read-only precheck gate, stop-on-mismatch, no-automatic-retry, exact user GO phrase requirement, and no-execution boundary.

Final GO.4 must not run build, compute hashes, deploy, upgrade, initialize state, configure SPL, construct packages, sign, call RPC, use testnet, submit, or mutate.

## Evidence preview

metadata:

```text
phase=final-go-3-final-scoped-go-package-decision-model
timestamp_utc=2026-07-06T21:02:03Z
repo_only=true
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

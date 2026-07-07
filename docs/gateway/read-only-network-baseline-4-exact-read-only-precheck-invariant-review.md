# Read-only Network Baseline.4 — Exact read-only precheck invariant review

Status:

READ_ONLY_NETWORK_BASELINE_4_REVIEW_READY_EXACT_READ_ONLY_PRECHECK_INVARIANTS_RECORDED_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_PENDING_CLOSURE

Closure candidate prepared:

STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_EXECUTION_NOT_APPROVED

Selected model:

STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY

Current GO state:

READ_ONLY_NETWORK_GO_NOT_GRANTED

## Purpose

Read-only Network Baseline.4 records invariant review for the strict read-only network precheck decision model.

This step is invariant review only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- Read-only Network Baseline.3 decision model
- Read-only Network Baseline.2 requirements inventory
- Read-only Network Baseline.1 precheck planning
- BuildHash Execution.2 local build/hash evidence

## Invariant review matrix

~~~text
# Read-only Network Baseline.4 invariant review matrix

RONB4_INVARIANT_01_CURRENT_GO_NOT_GRANTED:
status: reviewed
result: true
meaning: Current GO state remains READ_ONLY_NETWORK_GO_NOT_GRANTED.

RONB4_INVARIANT_02_SELECTED_MODEL_STRICT_READ_ONLY:
status: reviewed
result: true
meaning: Selected model is STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY.

RONB4_INVARIANT_03_RONB3_NOT_EXECUTABLE:
status: reviewed
result: true
meaning: RONB3 is decision-model-only and not executable.

RONB4_INVARIANT_04_RPC_NOT_APPROVED_NOW:
status: reviewed
result: true
meaning: RPC remains not approved in RONB4.

RONB4_INVARIANT_05_TESTNET_NOT_APPROVED_NOW:
status: reviewed
result: true
meaning: Testnet call remains not approved in RONB4.

RONB4_INVARIANT_06_PROGRAMDATA_READ_NOT_APPROVED_NOW:
status: reviewed
result: true
meaning: ProgramData read remains not approved in RONB4.

RONB4_INVARIANT_07_LIVE_HASH_COMPARISON_NOT_APPROVED_NOW:
status: reviewed
result: true
meaning: Live hash comparison remains not approved in RONB4.

RONB4_INVARIANT_08_MUTATION_NOT_APPROVED_NOW:
status: reviewed
result: true
meaning: Mutation remains not approved in RONB4.

RONB4_INVARIANT_09_EXACT_GO_REQUIRED_FOR_FUTURE_PRECHECK:
status: reviewed
result: true
meaning: A later exact scoped user GO phrase remains required before any read-only network precheck.

RONB4_INVARIANT_10_EXACT_RPC_ENDPOINT_REQUIRED:
status: reviewed
result: true
meaning: A future package must bind an exact RPC endpoint.

RONB4_INVARIANT_11_X1_TESTNET_BINDING_REQUIRED:
status: reviewed
result: true
meaning: A future package must bind X1_TESTNET network identity.

RONB4_INVARIANT_12_PROGRAM_ID_AND_PROGRAMDATA_BINDINGS_REQUIRED:
status: reviewed
result: true
meaning: Future precheck must bind program id and ProgramData account.

RONB4_INVARIANT_13_EXPECTED_AUTHORITY_BINDING_REQUIRED:
status: reviewed
result: true
meaning: Future precheck must verify observed upgrade authority.

RONB4_INVARIANT_14_EXPECTED_HASH_BINDING_REQUIRED:
status: reviewed
result: true
meaning: Expected hash remains e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1.

RONB4_INVARIANT_15_HASH_DOMAIN_AND_ALGORITHM_REQUIRED:
status: reviewed
result: true
meaning: Hash domain and SHA256 algorithm remain required.

RONB4_INVARIANT_16_NO_SIGNER_KEYPAIR_SECRET:
status: reviewed
result: true
meaning: Signer flags, keypair paths, and secret material remain forbidden.

RONB4_INVARIANT_17_NO_TRANSACTION_SUBMIT:
status: reviewed
result: true
meaning: Transaction submit remains forbidden.

RONB4_INVARIANT_18_NO_DEPLOY_UPGRADE_WRITE_BUFFER:
status: reviewed
result: true
meaning: Deploy, upgrade, and write-buffer remain forbidden.

RONB4_INVARIANT_19_NO_STATE_SPL_GUARDIAN_MUTATION:
status: reviewed
result: true
meaning: State initialization, SPL setup, guardian package construction, and mutation remain forbidden.

RONB4_INVARIANT_20_STOP_ON_MISMATCH:
status: reviewed
result: true
meaning: Any identity, authority, hash, method, command, or boundary mismatch remains a stop condition.

RONB4_INVARIANT_21_NO_AUTOMATIC_RETRY:
status: reviewed
result: true
meaning: Automatic retry remains rejected.

RONB4_AGGREGATE:
all_invariants_reviewed: true
read_only_precheck_model_closure_ready: true
closure_type: narrow_exact_read_only_network_precheck_model_boundary_only
current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED
rpc_approved_now: false
testnet_approved_now: false
programdata_read_approved_now: false
live_hash_comparison_approved_now: false
execution_approved: false
mutation_approved_now: false
~~~

## Read-only boundary review

~~~text
# Read-only Network Baseline.4 read-only boundary review

reviewed:
- selected model
- current GO state
- exact future GO requirement
- exact RPC endpoint requirement
- X1 testnet network binding
- program id binding
- ProgramData account binding
- expected upgrade authority binding
- expected hash binding
- canonical hash domain
- hash algorithm
- success criteria
- stop conditions
- no signer/keypair/secret boundary
- no transaction submit boundary
- no deploy/upgrade/write-buffer boundary
- no state/SPL/guardian/mutation boundary
- stop on mismatch rule
- no automatic retry rule

review_result:
The RONB3 decision model is ready for narrow closure in RONB5.

closure_candidate:
STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_EXECUTION_NOT_APPROVED

closure_must_not_approve:
- RPC
- testnet call
- ProgramData read
- live hash comparison
- signing
- transaction submit
- mutation
~~~

## Review result

all_invariants_reviewed: true

read_only_precheck_model_closure_ready: true

closure_type: narrow_exact_read_only_network_precheck_model_boundary_only

current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED

rpc_approved_now: false

testnet_approved_now: false

programdata_read_approved_now: false

live_hash_comparison_approved_now: false

execution_approved: false

mutation_approved_now: false

## Closure candidate prepared

STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_EXECUTION_NOT_APPROVED

Meaning:

- the strict read-only precheck model has been reviewed
- exact future GO remains required
- exact RPC endpoint remains unselected
- exact read-only command set remains unselected
- no RPC/testnet/ProgramData read is approved now
- no live hash comparison is approved now
- no signing/submit/mutation is approved now

## Remaining gaps

~~~text
# Read-only Network Baseline.4 remaining gaps

- RONB5 closure record not recorded
- exact read-only precheck package not drafted
- RPC endpoint not selected
- exact read-only command set not selected
- exact evidence path not selected
- exact GO phrase not selected
- ProgramData read not executed
- observed upgrade authority not re-read
- observed executable bytes not extracted
- observed executable bytes SHA256 not computed
- observed-vs-expected hash comparison not executed
- RPC/testnet not approved
- signing/submit/mutation not approved
~~~

## Non-GO boundary

~~~text
# Read-only Network Baseline.4 non-GO boundary

Read-only Network Baseline.4 does not grant GO.

Read-only Network Baseline.4 does not approve:
- RPC
- testnet call
- ProgramData read
- live account inspection
- live hash comparison
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

Read-only Network Baseline.4 is invariant review only.
A later RONB5 closure record is required before a future exact read-only precheck package.
A later exact scoped user GO phrase is required before any RPC/testnet read.
~~~

## Result

invariant_review_only: true

all_invariants_reviewed: true

read_only_precheck_model_closure_ready: true

rpc_used: false

testnet_used: false

programdata_read_executed: false

live_hash_comparison_executed: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Next safe step

Read-only Network Baseline.5 — exact read-only precheck model closure record.

Read-only Network Baseline.5 must still not call RPC, use testnet, read ProgramData, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-baseline-4-exact-read-only-precheck-invariant-review
timestamp_utc=2026-07-07T04:58:42Z
repo_only=true
invariant_review_only=true
rpc_used=false
testnet_used=false
programdata_read_executed=false
live_hash_comparison_executed=false
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
~~~

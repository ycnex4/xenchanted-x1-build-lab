# Read-only Network Baseline.3 — Exact read-only precheck decision model

Status:

READ_ONLY_NETWORK_BASELINE_3_OPEN_EXACT_READ_ONLY_PRECHECK_DECISION_MODEL_RECORDED_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY_MODEL_REQUIRED_RPC_NOT_APPROVED

Selected model:

STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY

Current GO state:

READ_ONLY_NETWORK_GO_NOT_GRANTED

## Purpose

Read-only Network Baseline.3 records the decision model for a future exact scoped read-only network precheck.

This step is decision-model-only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- Read-only Network Baseline.1 precheck package planning
- Read-only Network Baseline.2 requirements inventory
- BuildHash Execution.2 local build/hash evidence
- Upgrade authority custody map

## Decision model

~~~text
# Read-only Network Baseline.3 decision model

operation_class: READ_ONLY_NETWORK_BASELINE_PRECHECK_DECISION_MODEL_ONLY
model_status: DECISION_MODEL_RECORDED_NOT_EXECUTABLE
selected_model: STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY
current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED
rpc_approved_now: false
testnet_approved_now: false
programdata_read_approved_now: false
live_hash_comparison_approved_now: false
mutation_approved_now: false

selected_model_meaning:
A future read-only network precheck may be prepared only as a separate exact scoped package.
It may execute only after a later exact scoped user GO phrase.
It must be read-only.
It must not include any signer, keypair, transaction submit, deploy, upgrade, write-buffer, authority change, state initialization, SPL setup, guardian package construction, mutation, or production activation.

required_future_precheck_bindings:
- package id
- exact GO phrase
- network: X1_TESTNET
- exact RPC endpoint
- program id
- ProgramData account
- expected upgrade authority
- expected canonical ProgramData executable-bytes SHA256
- canonical hash domain
- hash algorithm
- exact read-only command set
- evidence path
- success criteria
- stop conditions

future_precheck_allowed_only_after:
- RONB4 invariant review
- RONB5 closure record
- exact scoped user GO phrase
- clean working tree
- exact command verification
- no signer/keypair/submit/mutation boundary verification
~~~

## Decision matrix

~~~text
# Read-only Network Baseline.3 decision matrix

OPTION_01_NO_PRECHECK_MODEL:
decision: rejected
reason: Future live network read requires explicit read-only model and stop rules.

OPTION_02_IMMEDIATE_RPC_NOW:
decision: rejected
reason: RONB3 is decision-model-only and RPC is not approved.

OPTION_03_READ_AND_MUTATE_COMBINED:
decision: rejected
reason: Read-only baseline must not combine with deploy, upgrade, write-buffer, signing, submit, or mutation.

OPTION_04_READ_ONLY_WITHOUT_EXACT_GO:
decision: rejected
reason: Future read-only network access requires exact scoped user GO.

OPTION_05_READ_ONLY_WITH_SIGNER_OR_KEYPAIR:
decision: rejected
reason: Read-only precheck must not require signer flags, keypair paths, secret material, or transaction submission.

OPTION_06_READ_ONLY_HASH_CHECK_ONLY:
decision: selected
model: STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY
reason: This preserves local hash evidence, checks live ProgramData read-only, and forbids mutation.

OPTION_07_AUTOMATIC_RETRY_ON_MISMATCH:
decision: rejected
reason: Hash/identity/authority mismatch is a stop condition; automatic retry remains rejected.
~~~

## Future success criteria

~~~text
# Future read-only precheck success criteria

future_precheck_success_requires:
- exact scoped GO phrase matches
- clean working tree before precheck
- exact RPC endpoint selected
- network identity is X1_TESTNET
- program id equals D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account equals 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed upgrade authority equals DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- executable bytes or canonical equivalent extracted successfully
- observed executable-bytes SHA256 computed successfully
- observed executable-bytes SHA256 equals e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- canonical hash domain remains PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
- hash algorithm remains SHA256
- no signer flag
- no keypair path
- no secret material
- no transaction submit
- no deploy/upgrade/write-buffer
- no authority change
- no state initialization
- no SPL setup
- no guardian package construction
- no mutation
- evidence files written
- final status captured

all_success_criteria_required: true
~~~

## Future stop conditions

~~~text
# Future read-only precheck stop conditions

stop_if:
- exact GO phrase is missing or differs
- working tree is dirty before precheck
- RPC endpoint is missing, ambiguous, or differs from selected endpoint
- network identity is not X1_TESTNET
- program id differs
- ProgramData account differs
- ProgramData owner/loader relationship cannot be validated when required
- observed upgrade authority differs
- ProgramData cannot be read
- executable bytes cannot be extracted or canonicalized
- observed executable-bytes SHA256 cannot be computed
- observed executable-bytes SHA256 differs from expected hash
- canonical hash domain differs
- hash algorithm differs
- command contains signer flag
- command contains keypair path
- command requests or prints secret material
- command can submit a transaction
- command can deploy, upgrade, write-buffer, close, transfer, initialize, configure SPL, construct guardian packages, or mutate
- final status is missing or unexplained

automatic_retry: rejected
mutation_after_precheck: not_authorized
~~~

## Forbidden actions

~~~text
# Read-only Network Baseline.3 forbidden actions

forbidden_now_and_in_future_read_only_precheck:
- signer flag
- keypair path
- secret material
- transaction submit
- deploy
- upgrade
- write-buffer
- set-upgrade-authority
- close program/buffer/account
- transfer
- state initialization
- SPL setup
- guardian package construction
- mutation
- production activation

future_precheck_must_remain_read_only: true
~~~

## Remaining gaps

~~~text
# Read-only Network Baseline.3 remaining gaps

- RONB4 invariant review not recorded
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
# Read-only Network Baseline.3 non-GO boundary

Read-only Network Baseline.3 does not grant GO.

Read-only Network Baseline.3 does not approve:
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

Read-only Network Baseline.3 records the decision model only.
A later exact scoped read-only precheck package and exact user GO phrase are required before any RPC/testnet read.
~~~

## Result

decision_model_only: true

selected_model: STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY

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

Read-only Network Baseline.4 — exact read-only precheck invariant review.

Read-only Network Baseline.4 must still not call RPC, use testnet, read ProgramData, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-baseline-3-exact-read-only-precheck-decision-model
timestamp_utc=2026-07-07T02:58:27Z
repo_only=true
decision_model_only=true
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

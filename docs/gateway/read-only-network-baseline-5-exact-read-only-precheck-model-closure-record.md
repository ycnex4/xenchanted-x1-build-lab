# Read-only Network Baseline.5 — Exact read-only precheck model closure record

Status:

READ_ONLY_NETWORK_BASELINE_5_CLOSED_EXACT_READ_ONLY_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_EXECUTION_NOT_APPROVED

Current decision:

STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_CLOSED_NARROW_FINAL_READ_ONLY_GO_NOT_GRANTED

Selected model:

STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY

Current GO state:

READ_ONLY_NETWORK_GO_NOT_GRANTED

## Purpose

Read-only Network Baseline.5 closes the strict read-only network precheck model narrowly.

This step is closure-record-only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Evidence basis

- Read-only Network Baseline.4 invariant review
- Read-only Network Baseline.3 decision model
- Read-only Network Baseline.2 requirements inventory
- Read-only Network Baseline.1 precheck planning
- BuildHash Execution.2 local build/hash evidence

## Closure summary

~~~text
# Read-only Network Baseline.5 closure summary

status: READ_ONLY_NETWORK_BASELINE_5_CLOSED_EXACT_READ_ONLY_PRECHECK_MODEL_REVIEWED_RPC_NOT_APPROVED_EXECUTION_NOT_APPROVED
decision: STRICT_READ_ONLY_NETWORK_PRECHECK_MODEL_CLOSED_NARROW_FINAL_READ_ONLY_GO_NOT_GRANTED
selected_model: STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY
current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED

closed_scope:
- RONB1 precheck package planning recorded
- RONB2 exact read-only precheck requirements inventoried
- RONB3 strict read-only precheck decision model recorded
- RONB4 invariant review recorded
- RONB5 closure records narrow model closure

closure_meaning:
The strict read-only network precheck model is closed narrowly.
Future read-only network access is not approved by this closure.
A separate exact scoped read-only precheck package is still required.
A later exact user GO phrase is still required before any RPC/testnet read.

not_approved_by_closure:
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
~~~

## Future exact read-only precheck package requirements

~~~text
# Future exact read-only precheck package requirements

future_package_must_bind:
- operation class: READ_ONLY_NETWORK_PRECHECK_ONLY
- package id
- exact user GO phrase
- repository and branch
- source commit
- clean working tree requirement
- network: X1_TESTNET
- exact RPC endpoint
- program id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected upgrade authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- expected canonical ProgramData executable-bytes SHA256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- canonical hash domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
- hash algorithm: SHA256
- exact read-only command set
- evidence directory
- success criteria
- stop conditions
- no signer/keypair/secret boundary
- no transaction submit boundary
- no mutation boundary
- no automatic retry boundary

future_package_must_not_include:
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
~~~

## Future exact GO requirements

~~~text
# Future exact GO requirements

current_exact_go_phrase: UNSET_NOT_SELECTED
current_read_only_precheck_go_granted: false

future_exact_go_phrase_must_bind:
- package id
- operation class READ_ONLY_NETWORK_PRECHECK_ONLY
- source commit
- X1 testnet network
- exact RPC endpoint
- program id
- ProgramData account
- expected hash
- read-only boundary

future_exact_go_phrase_must_not_be_interpreted_as:
- deploy GO
- upgrade GO
- write-buffer GO
- authority-change GO
- signing GO
- transaction-submit GO
- mutation GO
- production activation GO

future_exact_go_phrase_status: deferred_to_future_exact_read_only_precheck_package
~~~

## Allowed next step

~~~text
# Allowed next step after RONB5

next_safe_step:
Read-only Network Precheck Package.1 — exact scoped read-only precheck package draft.

next_safe_step_scope:
- repo-only package draft
- select package id
- bind exact operation class
- propose exact RPC endpoint
- propose exact read-only command set
- propose exact evidence path
- propose exact GO phrase
- no RPC
- no testnet call
- no ProgramData read
- no live hash comparison
- no signing
- no transaction submit
- no mutation

not_allowed_next_without_later_exact_go:
- RPC call
- testnet ProgramData read
- live hash comparison
- deploy
- upgrade
- write-buffer
- sign
- submit
- mutation
~~~

## Remaining gaps

~~~text
# Read-only Network Baseline.5 remaining gaps

- exact read-only precheck package not drafted
- package id for actual read-only precheck not selected
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
# Read-only Network Baseline.5 non-GO boundary

Read-only Network Baseline.5 does not grant GO.

Read-only Network Baseline.5 does not approve:
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

Read-only Network Baseline.5 closes the model only.
Future read-only network access requires a separate exact scoped package and a later exact user GO phrase.
~~~

## Result

closure_record_only: true

read_only_precheck_model_closed_narrowly: true

selected_model: STRICT_READ_ONLY_NETWORK_PRECHECK_WITH_EXACT_GO_ONLY

current_go_state: READ_ONLY_NETWORK_GO_NOT_GRANTED

read_only_precheck_go_granted: false

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

Read-only Network Precheck Package.1 — exact scoped read-only precheck package draft.

This next step must still not call RPC, use testnet, read ProgramData, perform live hash comparison, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-baseline-5-exact-read-only-precheck-model-closure-record
timestamp_utc=2026-07-07T05:13:40Z
repo_only=true
closure_record_only=true
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

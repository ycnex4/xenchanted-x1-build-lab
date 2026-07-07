# Read-only Network Precheck Package.1 — Exact scoped read-only precheck package draft

Status:

READ_ONLY_NETWORK_PRECHECK_PACKAGE_1_OPEN_EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_DRAFT_NO_RPC_NO_TESTNET_NO_PROGRAMDATA_READ_NO_MUTATION

Current decision:

EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_DRAFT_RECORDED_READ_ONLY_GO_NOT_GRANTED

Current GO state:

READ_ONLY_PRECHECK_GO_NOT_GRANTED

Draft package id:

RONPP1_DRAFT_f3bff313af20

Source commit:

f3bff313af205285d4635fa709fc04f162edbb05

## Purpose

Read-only Network Precheck Package.1 drafts the future exact scoped read-only network precheck package.

This step is repo-only draft.

It does not call RPC.

It does not use testnet.

It does not read ProgramData from network.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Draft bindings

~~~text
# Read-only Network Precheck Package.1 draft bindings

package_status: DRAFT_NOT_EXECUTABLE
operation_class: READ_ONLY_NETWORK_PRECHECK_ONLY
draft_package_id: RONPP1_DRAFT_f3bff313af20
source_commit: f3bff313af205285d4635fa709fc04f162edbb05
source_short: f3bff313af20

network: X1_TESTNET
proposed_rpc_endpoint: https://rpc.testnet.x1.xyz
rpc_endpoint_status: PROPOSED_NOT_USED_NOT_VERIFIED

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
canonical_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
hash_algorithm: SHA256

exact_go_phrase_status: UNSET_PENDING_FUTURE_CLOSURE
read_only_precheck_go_granted_now: false

rpc_used_now: false
testnet_used_now: false
programdata_read_executed_now: false
live_hash_comparison_executed_now: false
mutation_executed_now: false
~~~

## Proposed read-only command set

~~~text
# Proposed future read-only command set

command_set_status: PROPOSED_NOT_EXECUTABLE
rpc_endpoint_status: PROPOSED_NOT_USED_NOT_VERIFIED
proposed_rpc_endpoint: https://rpc.testnet.x1.xyz

proposed_future_commands:
1. solana account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T --url https://rpc.testnet.x1.xyz --output json
2. solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json
3. solana program dump D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my docs/gateway/evidence/FUTURE_EXACT_PRECHECK_EXECUTION/observed-xxxl-svm.so --url https://rpc.testnet.x1.xyz
4. sha256sum docs/gateway/evidence/FUTURE_EXACT_PRECHECK_EXECUTION/observed-xxxl-svm.so

proposed_future_command_purpose:
- read ProgramData account metadata
- read program account metadata
- dump executable bytes read-only
- compute observed executable bytes SHA256 locally
- compare observed SHA256 to expected SHA256

expected_sha256:
e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

must_not_execute_in_this_phase: true
executed_now: false
rpc_used_now: false
testnet_used_now: false
~~~

## Proposed success criteria

~~~text
# Proposed future precheck success criteria

future_success_requires:
- exact GO phrase matches final selected phrase
- clean working tree before execution
- source commit matches final package binding
- RPC endpoint equals final selected endpoint
- network is X1_TESTNET
- program id equals D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account equals 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed upgrade authority equals DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- observed executable bytes are extracted successfully
- observed executable bytes SHA256 is computed successfully
- observed executable bytes SHA256 equals e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- canonical hash domain remains PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA
- hash algorithm remains SHA256
- no signer flag
- no keypair path
- no secret material
- no transaction submit
- no deploy
- no upgrade
- no write-buffer
- no authority change
- no state initialization
- no SPL setup
- no guardian package construction
- no mutation
- evidence files written
- final status captured

all_success_criteria_required: true
~~~

## Proposed stop conditions

~~~text
# Proposed future precheck stop conditions

stop_if:
- exact GO phrase is missing or differs
- working tree is dirty before execution
- source commit differs from final package binding
- RPC endpoint is missing, ambiguous, or differs from final selected endpoint
- network is not X1_TESTNET
- program id differs from D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- ProgramData account differs from 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed upgrade authority differs from DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- ProgramData cannot be read
- executable bytes cannot be dumped or canonicalized
- observed executable bytes SHA256 cannot be computed
- observed executable bytes SHA256 differs from e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- command contains signer flag
- command contains keypair path
- command requests or prints secret material
- command can submit a transaction
- command can deploy, upgrade, write-buffer, close, transfer, initialize, configure SPL, construct guardian packages, or mutate

automatic_retry: rejected
mutation_after_precheck: not_authorized
~~~

## Proposed exact GO phrase draft

~~~text
# Proposed exact GO phrase draft

phrase_status: DRAFT_NOT_FINAL_NOT_EXECUTABLE

proposed_draft_phrase:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP1_DRAFT_f3bff313af20_SOURCE_f3bff313af20

meaning_if_finalized_later:
- authorize only read-only network precheck
- bind package id
- bind source commit
- bind X1 testnet
- bind exact RPC endpoint
- bind program id
- bind ProgramData account
- bind expected hash
- forbid signer/keypair/submit/mutation

not_final_now: true
read_only_precheck_go_granted_now: false
~~~

## Remaining gaps

~~~text
# Read-only Network Precheck Package.1 remaining gaps

- draft package not reviewed
- final package id not selected
- final exact GO phrase not selected
- proposed RPC endpoint not verified
- exact command set not reviewed
- exact evidence path not finalized
- execution package not closed
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
# Read-only Network Precheck Package.1 non-GO boundary

Read-only Network Precheck Package.1 does not grant GO.

Read-only Network Precheck Package.1 does not approve:
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

This phase is repo-only draft.
The proposed RPC endpoint and command set are not executed.
~~~

## Result

draft_only: true

draft_package_id: RONPP1_DRAFT_f3bff313af20

source_commit: f3bff313af205285d4635fa709fc04f162edbb05

source_short: f3bff313af20

proposed_rpc_endpoint: https://rpc.testnet.x1.xyz

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

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

Read-only Network Precheck Package.2 — exact scoped read-only precheck package requirements/invariant review.

Read-only Network Precheck Package.2 must still not call RPC, use testnet, read ProgramData, perform live hash comparison, sign, submit, or mutate.

## Evidence preview

metadata:

~~~text
phase=read-only-network-precheck-package-1-exact-scoped-read-only-precheck-package-draft
timestamp_utc=2026-07-07T05:20:17Z
repo_only=true
draft_only=true
source_commit=f3bff313af205285d4635fa709fc04f162edbb05
source_short=f3bff313af20
draft_package_id=RONPP1_DRAFT_f3bff313af20
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

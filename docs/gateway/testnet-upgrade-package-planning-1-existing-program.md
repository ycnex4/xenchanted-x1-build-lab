# Testnet Upgrade Package Planning.1 — Existing program only

Status:

TESTNET_UPGRADE_PACKAGE_PLANNING_1_EXISTING_PROGRAM_ONLY_EXECUTION_BLOCKED_NO_RPC_NO_MUTATION

Path decision:

UPGRADE_EXISTING_PROGRAM_ONLY

Execution:

BLOCKED

## Summary

Theo approved package planning only.

Execution remains blocked until a separate exact GO phrase.

The selected path is upgrade existing program only.

Redeploy/new ProgramData is not selected.

## Target binding

~~~text
source_commit=17a1576e7addfa753f9569c49f8736ef55614d7d
source_short=17a1576e7add
program_id=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
programdata_account=9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
upgrade_authority=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
expected_local_artifact_hash=e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
expected_local_artifact_size=20840
observed_live_hash_before_upgrade=fd4d3b659ccaea4f5e24eca4d9e80ff808c43de1bf1ecef7315961751a085a7e
observed_live_size_before_upgrade=38584
upgrade_path=UPGRADE_EXISTING_PROGRAM_ONLY
future_go_phrase=GO_TESTNET_UPGRADE_EXISTING_PROGRAM_ONLY_TDUP1_SOURCE_17a1576_EXPECTED_E68ADA36_PROGRAM_D7AQMZNT
~~~

## Path decision

~~~text
path_decision=UPGRADE_EXISTING_PROGRAM_ONLY

reason:
Program ID, ProgramData account, and upgrade authority already match expected values.
The live binary differs from the expected local build.
Therefore the planning path is upgrade existing program, preserving Program ID.

not_selected:
REDEPLOY_NEW_PROGRAM_OR_NEWDATA

redeploy_new_program_reason_rejected:
A new deploy would change the target assumptions and would not preserve the intended Program ID boundary.
~~~

## Preconditions

~~~text
All must be true before any future execution GO:

[ ] Fresh rebuild from source commit 17a1576e7addfa753f9569c49f8736ef55614d7d produces hash e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
[ ] Fresh local artifact size is 20840
[ ] Immediate read-only precheck confirms ProgramData account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
[ ] Immediate read-only precheck confirms upgrade authority DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
[ ] Immediate read-only precheck confirms target Program ID D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
[ ] Runtime blockers are explicitly scoped as inert deploy only, not functional route activation
[ ] Exact execution GO phrase is provided by user after package closure
[ ] No command outside the closed package is used
~~~

## Stop conditions

~~~text
Stop immediately if any condition occurs:

- Fresh rebuild hash differs from e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- Fresh artifact size differs from 20840
- ProgramData account mismatch
- Upgrade authority mismatch
- Target Program ID mismatch
- Any RPC/read-only precheck error before upgrade
- Buffer/write hash cannot be verified before final upgrade step
- Buffer/write hash differs from e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
- Unexpected CLI behavior
- Any command asks for an authority not explicitly scoped
- Any signing prompt appears outside the exact GO execution package
- Any transaction would target a different program or ProgramData account
~~~

## Allowed command classes for future execution package

~~~text
Allowed only in future execution package after separate exact GO:

1. Local build/hash commands:
   cargo build-sbf --manifest-path programs/xxxl-svm/Cargo.toml --no-default-features
   sha256sum programs/xxxl-svm/target/deploy/xxxl_svm.so
   stat -c%s programs/xxxl-svm/target/deploy/xxxl_svm.so

2. Immediate read-only verification before upgrade:
   solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json
   solana account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T --url https://rpc.testnet.x1.xyz --output json
   solana program dump D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my <evidence-path>/pre-upgrade-observed.so --url https://rpc.testnet.x1.xyz
   sha256sum <evidence-path>/pre-upgrade-observed.so

3. Upgrade existing program only:
   solana program write-buffer <expected-local-artifact> ...
   solana program upgrade <buffer-address> D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my ...

4. Post-upgrade read-only verification:
   solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json
   solana account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T --url https://rpc.testnet.x1.xyz --output json
   solana program dump D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my <evidence-path>/post-upgrade-observed.so --url https://rpc.testnet.x1.xyz
   sha256sum <evidence-path>/post-upgrade-observed.so

Important:
Exact command lines, signer paths, buffer handling, and CLI v4.0 syntax must be finalized in the execution package.
This planning checkpoint does not authorize running any of them.
~~~

## Forbidden commands

~~~text
Forbidden by this planning checkpoint:

- Any execution command now
- Any RPC call now
- Any deploy to new Program ID
- Any redeploy/new ProgramData path
- Any upgrade execution without separate exact GO
- Any write-buffer without separate exact GO
- Any signing without separate exact GO
- Any transaction submit without separate exact GO
- Any authority change
- Any state initialization
- Any SPL setup
- Any guardian package construction
- Any production activation
~~~

## Future exact GO phrase

~~~text
future_exact_go_phrase=GO_TESTNET_UPGRADE_EXISTING_PROGRAM_ONLY_TDUP1_SOURCE_17a1576_EXPECTED_E68ADA36_PROGRAM_D7AQMZNT

status:
DEFINED_BUT_NOT_GRANTED

meaning:
This phrase is defined for a future execution package.
It is not active until the planning package is merged and a separate execution package is closed.
~~~

## Post-upgrade verification plan

~~~text
Post-upgrade verification plan after future exact GO execution:

1. Read Program account.
2. Read ProgramData account.
3. Confirm ProgramData account remains 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T.
4. Confirm upgrade authority remains DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc unless a separately approved authority decision exists.
5. Dump deployed program bytes from D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my.
6. Compute SHA256.
7. Require post-upgrade hash == e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1.
8. Require post-upgrade binary size == 20840.
9. Record transaction signature and all verification outputs.
10. If any check fails, stop and do not proceed to activation.
~~~

## Non-GO boundary

~~~text
This planning checkpoint is not an execution GO.

execution_authorized=false
rpc_used=false
testnet_used=false
programdata_read_executed=false
executable_bytes_dumped=false
live_hash_comparison_executed=false
deploy_executed=false
upgrade_executed=false
write_buffer_executed=false
signing_executed=false
submit_executed=false
mutation_executed=false
~~~

## Progress

~~~text
✅ 0: repo sanity review before GO
✅ 1: local build/hash evidence
✅ 2: RONB — read-only network baseline model
✅ 3: RONPP1 — read-only precheck package draft
✅ 4: RONPP2 — requirements / invariant review
✅ 5: RONPP3 — exact read-only package closure
✅ 6: checkpoint + Theo review package
✅ 6R: Theo repo-grounded verdict
✅ 7: RONPP3 alignment to current main merge commit
✅ 8: Read-only Network Precheck Execution.1
✅ 9: Precheck Result Decision
✅ 10: Local Rebuild Investigation.1
✅ 11: Investigation Result Decision
✅ 12: Testnet Upgrade Package Planning.1

👉 13: Upgrade execution package only after separate closure and exact GO
⏭ 14: Post-upgrade verification
⏭ 15: Separate activation path

blocked:
execution/RPC/write-buffer/upgrade/sign/submit/mutation
~~~

## Result

planning_checkpoint_only: true

execution_authorized: false

path_decision: UPGRADE_EXISTING_PROGRAM_ONLY

source_commit: 17a1576e7addfa753f9569c49f8736ef55614d7d

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_hash: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

future_go_phrase_defined_but_not_granted: GO_TESTNET_UPGRADE_EXISTING_PROGRAM_ONLY_TDUP1_SOURCE_17a1576_EXPECTED_E68ADA36_PROGRAM_D7AQMZNT

rpc_used: false

testnet_used: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

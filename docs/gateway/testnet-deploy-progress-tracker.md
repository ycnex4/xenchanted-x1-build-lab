# X1 Testnet Deploy Progress Tracker

Last updated by:

testnet-deploy-checkpoint-1-before-theo-review

Current status:

TESTNET_DEPLOY_CHECKPOINT_1_BEFORE_THEO_REVIEW_RECORDED_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

PAUSE_FOR_THEO_REVIEW_BEFORE_READ_ONLY_PRECHECK_EXECUTION

## Progress

~~~text
✅ 0: repo sanity review before GO
✅ 1: local build/hash evidence
✅ 2: RONB — read-only network baseline model
✅ 3: RONPP1 — read-only precheck package draft
✅ 4: RONPP2 — requirements / invariant review
✅ 5: RONPP3 — exact read-only package closure

👉 6: checkpoint + Theo review package

⏭ 7: RONPP3 alignment to current main merge commit
⏭ 8: Read-only Network Precheck Execution.1
⏭ 9: Precheck result decision
⏭ 10: Deploy-readiness cleanup
⏭ 11: New build/hash after readiness changes
⏭ 12: Testnet deploy/upgrade package
⏭ 13: Testnet deploy/upgrade execution
⏭ 14: Post-deploy verification
⏭ 15: Separate activation path
~~~

## Current known facts

Current main commit:

~~~text
6db0483583d0d1cd9beb0b02ed28a6d949fc4f2e
~~~

RONPP3 bound source commit:

~~~text
fead873b9d8d4e018106d1167e6b27494b03d89e
~~~

RONPP3 final GO phrase currently recorded:

~~~text
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
~~~

Known alignment gap:

~~~text
RONPP3 is bound to fead873b9d8d, while current main is 6db0483583d0 after merge.
Before any read-only RPC execution, add a repo-only alignment checkpoint or ask Theo to confirm a different approach.
~~~

## Bound read-only precheck values

Network:

~~~text
X1_TESTNET
~~~

RPC endpoint:

~~~text
https://rpc.testnet.x1.xyz
~~~

Program id:

~~~text
D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
~~~

ProgramData account:

~~~text
9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
~~~

Expected upgrade authority:

~~~text
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
~~~

Expected canonical ProgramData executable-bytes SHA256:

~~~text
e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1
~~~

## Still forbidden

~~~text
- RPC before exact GO
- testnet call before exact GO
- ProgramData read before exact GO
- executable bytes dump before exact GO
- live hash comparison before exact GO
- signer/keypair
- transaction submit
- deploy
- upgrade
- write-buffer
- authority change
- state initialization
- SPL setup
- guardian package construction
- mutation
- production activation
~~~

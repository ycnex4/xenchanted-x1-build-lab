# Theo Review Package — X1 testnet deploy boundary after RONPP3

Status:

THEO_REVIEW_PACKAGE_PREPARED_X1_TESTNET_DEPLOY_BOUNDARY_AFTER_RONPP3_NO_EXECUTION

Decision requested:

REVIEW_BEFORE_RONPP3_ALIGNMENT_AND_READ_ONLY_PRECHECK_EXECUTION

## Context

We are preparing the path from read-only network precheck toward possible X1 testnet deploy/upgrade.

The latest closed package is RONPP3:

~~~text
READ_ONLY_NETWORK_PRECHECK_PACKAGE_3_CLOSED_EXACT_SCOPED_READ_ONLY_PRECHECK_PACKAGE_READY_FOR_USER_EXACT_GO_NO_RPC_NO_TESTNET_NO_PROGRAMDATA_READ_NO_MUTATION
~~~

RONPP3 final package id:

~~~text
RONPP3_READ_ONLY_PRECHECK_fead873b9d8d
~~~

RONPP3 final GO phrase:

~~~text
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d
~~~

## Current issue before continuing

RONPP3 was closed with source commit:

~~~text
fead873b9d8d4e018106d1167e6b27494b03d89e
~~~

After merging RONPP3, current main is:

~~~text
6db0483583d0d1cd9beb0b02ed28a6d949fc4f2e
~~~

Question for Theo:

~~~text
Should we insert a repo-only alignment checkpoint and issue a new exact GO phrase bound to current main before any read-only RPC execution?
~~~

## Proposed next steps

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

## Safety facts still active

~~~text
- Runtime still records SCAFFOLD_ONLY_NOT_DEPLOYABLE.
- Program ID readiness still records PLACEHOLDER_PROGRAM_ID_BOUNDARY.
- Deployment report remains deployable: false.
- Live route activation remains disabled.
- Read-only precheck does not authorize signer/keypair/submit/deploy/upgrade/mutation.
~~~

## Questions for Theo

~~~text
# Theo review questions

1. Are we still on the correct path from RONPP3 toward a safe X1 testnet deploy track?

2. Is it correct to insert a repo-only alignment checkpoint before using the RONPP3 exact GO phrase, because RONPP3 is bound to source commit fead873b9d8d while current main is 6db0483583d0?

3. Is Read-only Network Precheck Execution.1 the correct next execution step after alignment, limited to:
   - RPC read
   - ProgramData/account metadata
   - executable bytes dump
   - SHA256 compare
   - evidence only?

4. Should we treat deploy/upgrade as a separate operation class after read-only precheck, with a separate scoped package and separate exact GO?

5. Before any testnet deploy/upgrade, must we first resolve or explicitly scope around:
   - SCAFFOLD_ONLY_NOT_DEPLOYABLE
   - PLACEHOLDER_PROGRAM_ID_BOUNDARY
   - LIVE_ROUTE_DISABLED
   - SPL_CPI_EXECUTION_DISABLED
   - production guardian/proof-log/external review blockers?

6. If the next target is only an inert testnet artifact deployment, what blockers may remain open but must be explicitly recorded as out of scope?

7. If the next target is a functional gateway testnet route, what blockers must close before deploy/activation?

8. Are we over-documenting in a way that creates circular process, or is the current separation of read-only precheck, deploy/upgrade, and activation still justified?
~~~

## Non-GO boundary

~~~text
# Non-GO boundary

This checkpoint does not grant GO.

This checkpoint does not approve:
- RPC
- testnet call
- ProgramData read
- executable bytes dump
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

This checkpoint pauses execution and prepares a Theo review package.
~~~

## Requested Theo verdict format

Please answer with one of:

~~~text
APPROVE_RONPP3_ALIGNMENT_BEFORE_READ_ONLY_PRECHECK
APPROVE_READ_ONLY_PRECHECK_WITH_EXISTING_RONPP3_BINDING
REQUEST_ADDITIONAL_REPO_ONLY_CHECKPOINT_BEFORE_ANY_RPC
REJECT_READ_ONLY_PRECHECK_PATH_AS_CURRENTLY_SCOPED
~~~

And add comments on:
- whether we are walking in circles;
- whether read-only precheck is the correct next execution step;
- whether deploy/upgrade must remain a separate package;
- whether placeholder/scaffold deployment blockers must be resolved before any testnet deploy/upgrade.

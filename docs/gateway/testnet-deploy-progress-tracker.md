# X1 Testnet Deploy Progress Tracker

Last updated by:

precheck-result-decision-1-hash-mismatch-investigation-required

Current status:

PRECHECK_RESULT_DECISION_1_HASH_MISMATCH_INVESTIGATION_REQUIRED_DEPLOY_UPGRADE_BLOCKED_NO_RPC_NO_MUTATION

Current decision:

STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK_INVESTIGATION_REQUIRED

## Progress

~~~text
# X1 Testnet Deploy Track progress state after Precheck Result Decision.1

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
✅ 9: Precheck Result Decision — hash mismatch stopped correctly

👉 10: local rebuild / observed-binary investigation package

⏭ 11: New build/hash after investigation
⏭ 12: Testnet deploy/upgrade package only after classification
⏭ 13: Testnet deploy/upgrade execution only after separate exact GO
⏭ 14: Post-deploy verification
⏭ 15: Separate activation path

blocked:
deploy/upgrade/write-buffer/sign/submit/mutation
~~~

## Decision summary

~~~text
# Precheck Result Decision.1

status:
PRECHECK_RESULT_DECISION_1_HASH_MISMATCH_INVESTIGATION_REQUIRED_DEPLOY_UPGRADE_BLOCKED_NO_RPC_NO_MUTATION

decision:
STOPPED_CORRECTLY_BY_READ_ONLY_PRECHECK

classification:
HASH_MISMATCH_INVESTIGATION_REQUIRED_BEFORE_CATEGORIZATION

not_classified_yet_as:
- expected stale testnet binary
- stale local expected hash
- wrong build artifact/hash domain
- wrong deployment target

current_likelihood_notes_from_theo:
- expected stale/different testnet binary: medium
- local expected hash stale: medium-high
- wrong build artifact/hash domain: low-medium
- wrong deployment target: low

reason:
ProgramData account and upgrade authority match expected values, but live testnet executable bytes hash differs from the expected local build hash.

critical_observation:
The expected hash originated from BuildHash Execution.2, not from a fresh rebuild from current main.

next_required_checkpoint:
local rebuild / observed-binary investigation package

deploy_upgrade_blocked:
true
~~~

## Next safe step

~~~text
Local rebuild / observed-binary investigation package.
~~~

## Still forbidden

~~~text
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

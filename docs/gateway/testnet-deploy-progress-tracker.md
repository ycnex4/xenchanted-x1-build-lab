# X1 Testnet Deploy Progress Tracker

Last updated by:

testnet-deploy-checkpoint-2-theo-verdict-ronpp3-alignment

Current status:

TESTNET_DEPLOY_CHECKPOINT_2_THEO_VERDICT_RONPP3_ALIGNMENT_RECORDED_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

RONPP3_ALIGNMENT_ACCEPTED_AFTER_THEO_VERDICT_READ_ONLY_PRECHECK_GO_NOT_GRANTED

## Progress

~~~text
# X1 Testnet Deploy Track progress state after alignment

✅ 0: repo sanity review before GO
✅ 1: local build/hash evidence
✅ 2: RONB — read-only network baseline model
✅ 3: RONPP1 — read-only precheck package draft
✅ 4: RONPP2 — requirements / invariant review
✅ 5: RONPP3 — exact read-only package closure
✅ 6: checkpoint + Theo review package
✅ 6R: Theo repo-grounded verdict — APPROVE_RONPP3_ALIGNMENT_BEFORE_READ_ONLY_PRECHECK
✅ 7: RONPP3 alignment to current main merge commit

👉 8: Read-only Network Precheck Execution.1 — after exact user GO only

⏭ 9: Precheck result decision
⏭ 10: Deploy-readiness cleanup
⏭ 11: New build/hash after readiness changes
⏭ 12: Testnet deploy/upgrade package
⏭ 13: Testnet deploy/upgrade execution
⏭ 14: Post-deploy verification
⏭ 15: Separate activation path

current_main_commit: 18ff5149e8507c11a64e3bbcdc3349b0abde4ef4
current_main_short: 18ff5149e850
alignment_package_id: RONPP3A_READ_ONLY_PRECHECK_18ff5149e850
final_exact_go_phrase: GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850

read_only_precheck_go_granted_now: false
~~~

## Theo verdict

~~~text
primary_verdict: APPROVE_RONPP3_ALIGNMENT_BEFORE_READ_ONLY_PRECHECK
recommended_option: Option A — new exact GO phrase bound to current main
~~~

## Diff classification

~~~text
# Diff scope classification

base_commit: fead873b9d8d4e018106d1167e6b27494b03d89e
base_short: fead873b9d8d
current_main_commit: 18ff5149e8507c11a64e3bbcdc3349b0abde4ef4
current_main_short: 18ff5149e850

diff_line_count: 18
diff_scope: DOCS_ONLY
material_code_changes: false

classification:
The diff from RONPP3 bound source commit to current main was inspected by path.

Theo's repo-grounded verdict states that the diff is documentation-only, with zero code changes and no build/hash/program-id/programdata changes.

local_path_classification:
DOCS_ONLY

non_doc_paths_file:
docs/gateway/evidence/testnet-deploy-checkpoint-2-theo-verdict-ronpp3-alignment/non-doc-paths.txt
~~~

## Final exact GO phrase after alignment

~~~text
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850
~~~

## Bound read-only precheck values

network: X1_TESTNET

rpc_endpoint: https://rpc.testnet.x1.xyz

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

canonical_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

hash_algorithm: SHA256

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

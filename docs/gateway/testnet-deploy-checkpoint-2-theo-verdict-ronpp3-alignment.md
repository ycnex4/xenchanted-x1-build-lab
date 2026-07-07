# Testnet Deploy Checkpoint.2 — Theo verdict and RONPP3 alignment

Status:

TESTNET_DEPLOY_CHECKPOINT_2_THEO_VERDICT_RONPP3_ALIGNMENT_RECORDED_NO_RPC_NO_TESTNET_NO_MUTATION

Current decision:

RONPP3_ALIGNMENT_ACCEPTED_AFTER_THEO_VERDICT_READ_ONLY_PRECHECK_GO_NOT_GRANTED

Current GO state:

READ_ONLY_PRECHECK_GO_NOT_GRANTED

Alignment package id:

RONPP3A_READ_ONLY_PRECHECK_18ff5149e850

Current main commit:

18ff5149e8507c11a64e3bbcdc3349b0abde4ef4

Final exact GO phrase after alignment:

GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850

## Purpose

This checkpoint records Theo's repo-grounded verdict and aligns the RONPP3 read-only precheck package to the current main commit.

This step is repo-only.

It does not call RPC.

It does not use testnet.

It does not read ProgramData.

It does not dump executable bytes.

It does not perform live hash comparison.

It does not deploy, upgrade, write buffer, sign, submit, or mutate.

## Theo verdict record

~~~text
# Theo repo-grounded verdict record

verdict_timestamp_user_supplied: 2026-07-07 14:59

primary_verdict:
APPROVE_RONPP3_ALIGNMENT_BEFORE_READ_ONLY_PRECHECK

recommended_option:
Option A — new exact GO phrase bound to current main.

repo_grounded_findings_from_theo:
- diff fead873b9d8d -> 18ff514 is docs-only
- code changes: ZERO
- build artifacts: no change
- expected SHA256: no change
- Program ID: no change
- ProgramData account: no change
- RONPP3 bindings remain valid
- alignment checkpoint is still recommended because execution will happen from current main
- read-only precheck is the correct next execution step after alignment
- deploy/upgrade must remain a separate operation class
- inert deploy may keep scaffold/placeholder/live-route/SPL-CPI blockers explicitly out of scope
- functional route requires blockers closed
- we are not walking in circles

selected_path:
APPROVE_RONPP3_ALIGNMENT_BEFORE_READ_ONLY_PRECHECK_WITH_NEW_GO_PHRASE_BOUND_TO_CURRENT_MAIN
~~~

## Diff scope classification

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

## Diff stat

~~~text
 docs/checkpoints/current-design-checkpoint.md      | 122 ++++++
 .../allowed-next-step.txt                          |  36 ++
 .../closure-summary.txt                            |  52 +++
 .../final-exact-go-phrase.txt                      |  19 +
 .../final-read-only-command-set.txt                |  37 ++
 .../final-stop-conditions.txt                      |  25 ++
 .../final-success-criteria.txt                     |  34 ++
 .../metadata.txt                                   |  23 ++
 .../non-go-boundary.txt                            |  25 ++
 .../remaining-gap-summary.txt                      |  11 +
 .../metadata.txt                                   |  21 +
 .../non-go-boundary.txt                            |  23 ++
 .../progress-state.txt                             |  32 ++
 .../theo-review-questions.txt                      |  27 ++
 ...ed-read-only-precheck-package-closure-record.md | 424 +++++++++++++++++++++
 ...stnet-deploy-checkpoint-1-before-theo-review.md | 219 +++++++++++
 docs/gateway/testnet-deploy-progress-tracker.md    | 122 ++++++
 ...package-testnet-deploy-boundary-after-ronpp3.md | 199 ++++++++++
 18 files changed, 1451 insertions(+)
~~~

## Final exact GO phrase after alignment

~~~text
# Final exact GO phrase after RONPP3 alignment

phrase_status: FINAL_SELECTED_NOT_GRANTED_UNTIL_USER_REPEATS_VERBATIM

old_ronpp3_go_phrase:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3_READ_ONLY_PRECHECK_fead873b9d8d_SOURCE_fead873b9d8d

old_ronpp3_go_phrase_status:
SUPERSEDED_BY_ALIGNMENT_OPTION_A

alignment_package_id:
RONPP3A_READ_ONLY_PRECHECK_18ff5149e850

current_main_commit:
18ff5149e8507c11a64e3bbcdc3349b0abde4ef4

current_main_short:
18ff5149e850

final_exact_go_phrase:
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850

meaning:
- authorize only READ_ONLY_NETWORK_PRECHECK_ONLY
- bind alignment package id
- bind current main short
- bind X1 testnet
- bind exact RPC endpoint
- bind program id
- bind ProgramData account
- bind expected hash
- forbid signer/keypair/submit/mutation/deploy/upgrade

read_only_precheck_go_granted_now: false
~~~

## Progress state

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

## Bound read-only precheck values

network: X1_TESTNET

rpc_endpoint: https://rpc.testnet.x1.xyz

program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T

expected_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

expected_canonical_programdata_executable_bytes_sha256: e68ada36e1914584c2dcc186afbdfcba608b286fc2cd404015a7a8c28764daa1

canonical_hash_domain: PROGRAMDATA_EXECUTABLE_BYTES_EXCLUDING_LOADER_METADATA

hash_algorithm: SHA256

## Non-GO boundary

~~~text
# Non-GO boundary

This alignment checkpoint does not grant GO.

This alignment checkpoint does not approve:
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

This checkpoint only records Theo verdict, diff classification, and aligned read-only precheck package binding.
~~~

## Result

checkpoint_only: true

theo_verdict_recorded: true

ronpp3_alignment_recorded: true

alignment_package_id: RONPP3A_READ_ONLY_PRECHECK_18ff5149e850

ronpp3_bound_source_commit: fead873b9d8d4e018106d1167e6b27494b03d89e

ronpp3_bound_source_short: fead873b9d8d

current_main_commit: 18ff5149e8507c11a64e3bbcdc3349b0abde4ef4

current_main_short: 18ff5149e850

diff_line_count: 18

diff_scope: DOCS_ONLY

material_code_changes: false

final_exact_go_phrase: GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850

read_only_precheck_go_granted: false

rpc_used: false

testnet_used: false

programdata_read_executed: false

executable_bytes_dumped: false

live_hash_comparison_executed: false

deploy_executed: false

upgrade_executed: false

write_buffer_executed: false

signing_executed: false

submit_executed: false

mutation_executed: false

## Next safe step

Read-only Network Precheck Execution.1 — after exact user GO phrase only.

Required exact user GO phrase:

~~~text
GO_RONPP_READ_ONLY_PRECHECK_ONLY_RONPP3A_READ_ONLY_PRECHECK_18ff5149e850_SOURCE_18ff5149e850
~~~

Until the exact phrase is provided, no RPC/testnet/ProgramData read/executable bytes dump/live hash comparison is authorized.

# Blocker A.3 — Repo-only authority model decision record

Status:

BLOCKER_A_OPEN_REPO_ONLY_AUTHORITY_MODEL_SELECTED_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

AUTHORITY_MODEL_SELECTED_TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION_BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Purpose

Blocker A.3 records the repo-only authority model decision for the next phase.

A.3 does not prove live ProgramData state.

A.3 does not close Blocker A.

A.3 selects the intended authority model that future read-only evidence must verify.

## Boundary

A.3 is repo-only.

A.3 does not call RPC.

A.3 does not use testnet.

A.3 does not use live RPC.

A.3 does not use keys.

A.3 does not sign.

A.3 does not inspect live ProgramData.

A.3 does not run solana program show.

A.3 does not deploy, upgrade, initialize state, configure SPL, or submit to any network.

## Reviewed repo anchors

A.3 uses these existing repo anchors:

- docs/gateway/blocker-a-1-upgrade-authority-discovery-planning-only.md
- docs/gateway/blocker-a-2-repo-grounded-upgrade-authority-status-reconciliation.md
- docs/gateway/phase-41k6-b6-38-upgrade-authority-custody-map.md
- docs/xxxl/xxxl-program-v1-deployment-readiness.md
- docs/xxxl/xxxl-authority-freeze-procedure-model.md
- programs/xxxl-svm/src/deployment_status.rs
- programs/xxxl-svm/src/program_id_status.rs

## Selected authority model

Selected model:

TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION

Candidate future Blocker A closure state after live read-only evidence:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Meaning:

- upgrade authority may remain present during a bounded staged-finalization / test phase
- upgrade authority is tolerated only as a temporary staged-finalization mechanism
- upgrade authority is not admin mint authority
- upgrade authority is not discretionary supply control
- upgrade authority is not production immutability
- upgrade authority requires public disclosure
- upgrade authority requires a future freeze/removal plan
- upgrade authority requires fresh read-only ProgramData evidence before Blocker A can close

## Why authority-none is not selected now

A.3 does not select immediate UPGRADE_AUTHORITY_NONE_CONFIRMED because the repo's own deployment readiness documents allow temporary upgradeability during staged protocol finalization.

The repo also records that authority removal/freeze becomes eligible only after final deterministic X1-native emission mechanics are complete, reviewed, documented, publicly explained, and rehearsed.

Therefore, immediate authority-none is not the current repo-grounded model.

## Covenant for temporary upgradeability

Temporary upgradeability is acceptable only under the following covenant:

- no manual mint
- no premine
- no founder allocation
- no hidden emission
- no arbitrary supply control
- no gateway authorization bypass
- no processed-event history rewrite
- no balance rewrite
- no Build-derived supply rights during Genesis Phase
- no X1-native emission before deterministic mechanics are designed and tested
- no live route activation unless separately reviewed
- no SPL setup unless separately reviewed
- no upgrade unless separately approved with explicit scoped GO
- public disclosure must explain temporary upgradeability before deployment
- freeze/removal plan must remain part of the lifecycle path

## Repo-grounded public baseline carried forward

The known public baseline carried forward from B6.38/A.2 is:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- x1_testnet_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

A.3 interpretation:

- these are public identifiers only
- they are not signing material
- they do not prove fresh live ProgramData state
- they do not close Blocker A

## Current runtime blockers remain active

A.3 does not override current runtime blockers:

- PLACEHOLDER_PROGRAM_ID boundary remains active in code
- deployable_path_ready remains false
- deployment status remains deployable=false
- live route remains disabled
- SPL CPI execution remains disabled
- production guardian set remains unset
- production proof log remains unset
- external review remains incomplete

## What future read-only evidence must verify

Before Blocker A can close as UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE, a separately gated read-only evidence step must verify:

- exact network
- exact RPC URL
- exact program id
- exact ProgramData account
- exact observed upgrade authority
- whether the observed authority matches the repo-grounded expected public key
- whether the program is owned by the expected upgradeable loader
- whether the evidence command required signing
- whether any mutation command was executed
- slot or equivalent observation context if available

No signing or mutation is allowed for this future read-only evidence step.

## Non-closure statement

A.3 does not close Blocker A.

A.3 only selects the repo-grounded authority model for the next phase.

Blocker A remains open until read-only ProgramData evidence and review confirm the selected model.

## Result

Current status:

BLOCKER_A_OPEN_REPO_ONLY_AUTHORITY_MODEL_SELECTED_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

AUTHORITY_MODEL_SELECTED_TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION_BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Next safe step

The next safe step is A.4 read-only live ProgramData evidence GO form.

A.4 must still not call RPC.

A.4 should define the exact read-only evidence command boundary and require explicit user approval before any live RPC read.

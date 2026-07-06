# Blocker A.6 — Closure decision record

Status:

BLOCKER_A_CLOSED_AS_UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Current decision:

BLOCKER_A_CLOSED_NARROW_AUTHORITY_MODEL_ONLY

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker A.6 records the closure decision for Blocker A.

The closure is narrow.

It closes only the upgrade authority custody / authority-model blocker for the currently observed X1 testnet ProgramData state.

It does not approve signing, upgrade, state initialization, SPL setup, guardian package construction, submit, or mutation.

## Closure state

Blocker A is closed as:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Meaning:

- upgrade authority is present
- the observed authority matches the repo-grounded expected public authority
- the authority is accepted only for a bounded staged-finalization test phase
- this is not production immutability
- this is not admin mint authority
- this is not discretionary supply control
- future freeze/removal remains required by the lifecycle model

## Evidence chain

A.6 is based on:

1. A.1 — upgrade authority discovery planning-only
2. A.2 — repo-grounded authority status reconciliation
3. A.3 — repo-only authority model decision record
4. A.4 — read-only live ProgramData evidence GO form
5. A.5 — read-only live ProgramData evidence
6. A.5R — read-only evidence review package

## Live read-only evidence accepted

Accepted A.5 evidence:

- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true
- program_show_exit_code: 0
- program_account_exit_code: 0
- signing_used: false
- mutation_executed: false

## What this closure allows

This closure allows future planning to treat Blocker A as closed for the narrow authority-model question.

It allows the project to move to the next separately scoped blocker.

Recommended next blockers:

- C — B1C7 handler production/testnet boundary
- D/E — state initialization and SPL mint authority architecture
- F — guardian descriptor
- B — build artifact and expected ProgramData hash
- G — rollback / recovery plan

## What this closure does not allow

This closure does not approve:

- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- state initialization
- SPL mint setup
- SPL CPI minting
- guardian package construction
- transaction submit
- mutation
- production activation

## Remaining blockers

After A.6:

- Blocker A — CLOSED narrowly
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — OPEN: B1C7 handler production/testnet boundary
- Blocker D — OPEN: state initialization design
- Blocker E — OPEN: SPL mint authority architecture
- Blocker F — OPEN: guardian descriptor
- Blocker G — OPEN: rollback / recovery plan
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker A must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until B, C, D, E, F, and G are closed and a final scoped GO package is recorded.

## Result

Current status:

BLOCKER_A_CLOSED_AS_UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Current decision:

BLOCKER_A_CLOSED_NARROW_AUTHORITY_MODEL_ONLY

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to the next separately scoped blocker.

Recommended next step:

Blocker C.1 — B1C7 handler production/testnet boundary planning.

Do not proceed to deploy, upgrade, state init, SPL setup, guardian package construction, or submit.

# Blocker C.5 — B1C7 boundary closure decision record

Status:

BLOCKER_C_CLOSED_NARROW_B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Current decision:

BLOCKER_C_CLOSED_NARROW_BOUNDARY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker C.5 records the closure decision for Blocker C.

The closure is narrow.

It closes only the B1C7 handler boundary / invariant review blocker.

It does not approve handler activation, live route activation, deploy, upgrade, state initialization, SPL setup, guardian package construction, submit, or mutation.

## Closure state

Blocker C is closed as:

B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Meaning:

- the B1C7 handler boundary has been reviewed at repo/invariant level
- the default non-B1C7 path fails closed
- the current B1C7 path remains integration/test-gated
- direct dangerous test-gate activation is rejected
- a future reviewed testnet-intended handler route is required before any deployable artifact
- this closure is not testnet activation
- this closure is not production activation
- this closure is not approval to deploy or upgrade

## Evidence chain

C.5 is based on:

1. C.1 — B1C7 handler production/testnet boundary planning
2. C.2 — repo-grounded B1C7 handler inventory
3. C.2R — order-check tooling artifact correction
4. C.3 — B1C7 production/testnet activation decision model
5. C.4 — B1C7 invariant review package
6. C.4R — authorization status gate call-graph correction

## Accepted C.3 decision

C.3 decision accepted:

B1C7_DIRECT_DANGEROUS_TEST_GATE_ACTIVATION_REJECTED

Selected future model:

REVIEWED_TESTNET_INTENDED_HANDLER_ROUTE_REQUIRED_BEFORE_ANY_DEPLOYABLE_ARTIFACT

Therefore, the existing dangerous test-gate feature set must not be treated as a deployable or testnet-intended route.

## Accepted C.4 invariant result

C.4 invariant result accepted:

all_invariants_passed: true

Accepted invariant categories:

- handler entrypoint present
- handler calls authorization before atomic boundary
- authorization account contract asserted
- guardian set loaded
- prior instructions loaded through instructions sysvar boundary
- payload context constructed
- authorization status gate before returning Ok authorization
- atomic boundary rechecks Authorized status
- atomic boundary rechecks fail_fast_before_mutation
- atomic boundary rechecks prior Ed25519 evidence
- atomic boundary rechecks payload hash binding
- atomic boundary rechecks guardian membership
- atomic boundary rechecks quorum
- SPL CPI execution gate checked before atomic mark+mint boundary
- processed_event marking occurs before guarded SPL CPI inside atomic_mark_and_mint_boundary
- default path fails closed with CpiBoundaryNotReady
- B1 V3 account contract includes instructions_sysvar
- CPI execution remains false by default
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active

## What this closure allows

This closure allows future planning to treat Blocker C as closed for the narrow handler-boundary/invariant question.

It allows the project to proceed to the next separately scoped blocker.

Recommended next blockers:

- D — state initialization design
- E — SPL mint authority architecture
- F — guardian descriptor
- B — expected post-upgrade ProgramData hash
- G — rollback / recovery plan

## What this closure does not allow

This closure does not approve:

- handler activation
- live route activation
- direct dangerous test-gate deployment
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

After C.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — OPEN: state initialization design
- Blocker E — OPEN: SPL mint authority architecture
- Blocker F — OPEN: guardian descriptor
- Blocker G — OPEN: rollback / recovery plan
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker C must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until B, D, E, F, and G are closed and a final scoped GO package is recorded.

A future testnet-intended B1C7 handler route remains required before any deployable artifact can be accepted.

## Result

Current status:

BLOCKER_C_CLOSED_NARROW_B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Current decision:

BLOCKER_C_CLOSED_NARROW_BOUNDARY_INVARIANTS_ONLY

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to the next separately scoped blocker.

Recommended next step:

Blocker D.1 — state initialization design planning.

Do not proceed to deploy, upgrade, state init execution, SPL setup, guardian package construction, or submit.

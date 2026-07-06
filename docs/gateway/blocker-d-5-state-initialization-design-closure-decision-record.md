# Blocker D.5 — State initialization design closure decision record

Status:

BLOCKER_D_CLOSED_NARROW_STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_D_CLOSED_NARROW_DESIGN_INVARIANTS_ONLY

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker D.5 records the closure decision for Blocker D.

The closure is narrow.

It closes only the state initialization design / invariant review blocker.

It does not approve state initialization execution.

It does not approve account creation.

It does not approve PDA creation.

It does not approve SPL mint setup.

It does not approve deploy, upgrade, signing, submit, or mutation.

## Closure state

Blocker D is closed as:

STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- state initialization design boundary has been reviewed
- long-lived protocol state scope is defined
- one-time guard requirement is recorded
- reinitialization rejection requirement is recorded
- direct unbounded manual admin initialization is rejected
- ProcessedEvent is separated from long-lived protocol initialization
- RecipientBalance initialization remains an explicit future implementation detail
- SPL mint setup and SPL authority architecture remain Blocker E
- no state initialization execution is approved
- no account creation is approved
- no mutation is approved

## Evidence chain

D.5 is based on:

1. D.1 — state initialization design planning
2. D.2 — repo-grounded state layout and PDA inventory
3. D.3 — state initialization authority and one-time guard decision model
4. D.4 — state initialization invariant review package

## Accepted D.2 inventory result

D.2 inventory accepted:

all_inventory_checks_passed: true

Accepted inventory categories:

- fixed account lengths
- fixed discriminators
- runtime layout version
- account views
- gateway_mint_authority PDA inventory
- ProcessedEvent marking boundary
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active

## Accepted D.3 decision

D.3 decision accepted:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

D.3 boundary accepted:

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

## Accepted D.4 invariant result

D.4 invariant result accepted:

all_invariants_reviewed: true

blocker_d_closure_ready: true

closure_type: narrow_design_boundary_only

Accepted invariant categories:

- long-lived state scope is limited to MintState, GatewayConfig, and GuardianSet
- layouts, lengths, discriminators, views, and runtime layout version are inventoried
- future initializer must have one-time guard
- future initializer must reject reinitialization
- direct manual unbounded admin initialization is rejected
- ProcessedEvent remains per-event replay protection
- RecipientBalance lazy/precreate model remains explicit
- SPL mint setup remains Blocker E
- gateway_mint_authority PDA is inventoried but not activated by D
- future initializer must not introduce hidden admin mint or balance-write pathways
- no execution is approved

## What this closure allows

This closure allows future planning to treat Blocker D as closed for the narrow state-initialization design/invariant question.

It allows the project to proceed to the next separately scoped blocker.

Recommended next blockers:

- E — SPL mint authority architecture
- F — guardian descriptor
- B — expected post-upgrade ProgramData hash
- G — rollback / recovery plan

## What this closure does not allow

This closure does not approve:

- state initialization execution
- account creation
- PDA creation
- initializer execution
- SPL mint setup
- SPL CPI minting
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- guardian package construction
- transaction submit
- mutation
- production activation

## Remaining blockers

After D.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — CLOSED narrowly: state initialization design / invariants only
- Blocker E — OPEN: SPL mint authority architecture
- Blocker F — OPEN: guardian descriptor
- Blocker G — OPEN: rollback / recovery plan
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker D must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until B, E, F, and G are closed and a final scoped GO package is recorded.

A future reviewed initializer package remains required before any state initialization execution.

SPL mint authority architecture remains Blocker E.

## Result

Current status:

BLOCKER_D_CLOSED_NARROW_STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_D_CLOSED_NARROW_DESIGN_INVARIANTS_ONLY

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to the next separately scoped blocker.

Recommended next step:

Blocker E.1 — SPL mint authority architecture planning.

Do not proceed to deploy, upgrade, state init execution, SPL setup, guardian package construction, or submit.

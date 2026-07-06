# Blocker D.3 — State initialization authority and one-time guard decision model

Status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_AUTHORITY_ONE_TIME_GUARD_DECISION_MODEL_RECORDED_NO_EXECUTION

Current decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

Selected boundary:

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker D.3 records the authority and one-time guard decision model for state initialization.

D.3 is decision-model only.

It does not initialize any account.

It does not create any account.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate any network.

## Background

D.1 opened the state initialization design track.

D.2 inventoried state layouts, discriminators, account views, PDA inventory, and ProcessedEvent marking boundaries.

D.2 confirmed that layout and PDA inventory exists, but it did not approve initialization execution.

## D.3 decision

D.3 records the selected decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

This means no long-lived protocol state can be initialized until a later reviewed initializer package exists and a final scoped GO is recorded.

## Selected boundary

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

Long-lived protocol state:

- MintState
- GatewayConfig
- GuardianSet

Not part of long-lived protocol initialization in D:

- ProcessedEvent: per-event replay protection, initialized/marked through Phase 41K.4 boundary
- RecipientBalance: per-recipient accounting state, requires later lazy-init model
- gateway_mint_authority PDA: derived authority boundary, but SPL authority architecture is Blocker E
- SPL mint and token accounts: Blocker E scope

## Rejected models

D.3 rejects:

- direct manual state creation with unbounded admin discretion
- any initializer that can rewrite already-initialized protocol state
- any hidden admin mint or balance-write pathway
- treating ProcessedEvent marking as general protocol initialization
- using Blocker D to approve SPL mint setup
- executing initialization before a reviewed package and final scoped GO

## Required future initializer properties

- explicit initializer entrypoint or package
- explicit long-lived account list
- fixed account lengths
- fixed account discriminators
- runtime_layout_version written and checked
- one-time initialization guard
- reinitialization rejection
- public config values recorded before execution
- no hidden admin mint authority
- no admin recipient balance write
- no processed-event prepopulation as substitute for replay protection
- no SPL mint setup inside D
- separate final scoped GO before execution

## One-time guard model

A future initializer must prove:

- uninitialized accounts are recognized only by expected owner/data/lamport state
- initialized accounts contain expected discriminator and runtime layout version
- initialization writes the expected final image exactly once
- a second initialization attempt fails
- partial initialization cannot be treated as valid final state
- no admin path can later rewrite MintState, GatewayConfig, GuardianSet, RecipientBalance, or ProcessedEvent outside reviewed protocol paths

## Relationship to ProcessedEvent

ProcessedEvent remains per-event replay protection.

Its Phase 41K.4 boundary can allocate/assign/write the consumed final image for a specific canonical_event_key.

ProcessedEvent must not be pre-initialized as a mutable registry by a general state initializer.

## Relationship to RecipientBalance

RecipientBalance remains per-recipient accounting state.

D.3 does not decide the lazy initialization mechanics for RecipientBalance.

A later D step must decide whether RecipientBalance is pre-created, lazily initialized, or created through a separate reviewed boundary.

## Relationship to SPL mint setup

SPL mint setup is not approved by D.3.

SPL mint authority architecture remains Blocker E.

D may reference the target mint pubkey in MintState/GatewayConfig, but D must not create/configure the SPL mint or approve SPL mint authority.

## Decision matrix

```text
# State initialization decision matrix

D3_MODEL_0_NO_INITIALIZATION_YET
status: current_default
meaning: No state initialization execution is approved.
allows_state_init_execution_now: false
allows_account_creation_now: false
allows_mutation_now: false

D3_MODEL_1_DIRECT_MANUAL_STATE_CREATION_WITH_UNBOUNDED_ADMIN
status: rejected
meaning: A human/admin creates or rewrites protocol state without a reviewed initializer, one-time guard, and public config boundary.
reason_rejected: incompatible with immutable/no-hidden-admin-mint/no-admin-balance-write principles.
allows_state_init_execution_now: false
allows_mutation_now: false

D3_MODEL_2_REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD
status: selected_future_model
meaning: A future reviewed initializer package may initialize long-lived protocol state once, with explicit account list, fixed layouts, fixed discriminators, reinitialization rejection, public config values, and final scoped GO.
allows_state_init_execution_now: false
allows_future_state_init_after_required_evidence: true
allows_mutation_now: false

D3_MODEL_3_LAZY_PROCESSED_EVENT_MARKING_AS_GENERAL_STATE_INIT
status: rejected
meaning: Treat per-event ProcessedEvent marking as general protocol state initialization.
reason_rejected: ProcessedEvent marking is per-event replay protection and must stay separate from long-lived protocol initialization.
allows_state_init_execution_now: false
allows_mutation_now: false

D3_MODEL_4_SPL_MINT_SETUP_INSIDE_BLOCKER_D
status: rejected_for_D_scope
meaning: Use D to approve SPL mint creation/configuration/authority setup.
reason_rejected: SPL mint authority architecture is Blocker E scope.
allows_spl_setup_now: false
allows_mutation_now: false

SELECTED_D3_DECISION
REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

SELECTED_D3_BOUNDARY
LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP
```

## Non-closure statement

D.3 does not close Blocker D.

D.3 does not approve:

- state initialization execution
- account creation
- PDA creation
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

## Result

Current status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_AUTHORITY_ONE_TIME_GUARD_DECISION_MODEL_RECORDED_NO_EXECUTION

Current decision:

REVIEWED_TESTNET_INITIALIZER_WITH_ONE_TIME_GUARD_REQUIRED_BEFORE_ANY_STATE_INIT_EXECUTION

Selected boundary:

LONG_LIVED_PROTOCOL_STATE_INIT_SEPARATED_FROM_PROCESSED_EVENT_MARKING_AND_SPL_SETUP

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker D.4 — state initialization invariant review package.

D.4 should record the closure criteria for long-lived protocol state initialization, ProcessedEvent separation, RecipientBalance gap, and D/E SPL boundary.

D.4 must not initialize state, call RPC, use testnet, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-d-3-state-initialization-authority-one-time-guard-decision-model
timestamp_utc=2026-07-06T18:05:32Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
state_initialized=false
accounts_created=false
initializer_executed=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```

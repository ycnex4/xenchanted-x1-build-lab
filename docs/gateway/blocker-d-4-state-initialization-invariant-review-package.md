# Blocker D.4 — State initialization invariant review package

Status:

BLOCKER_D_REVIEW_READY_STATE_INITIALIZATION_INVARIANTS_RECORDED_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker D.4 records the state initialization invariant review package.

D.4 is review-only.

It does not initialize any account.

It does not create any account.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate any network.

## Evidence basis

D.4 is based on:

- D.1 state initialization planning
- D.2 repo-grounded state layout and PDA inventory
- D.3 state initialization authority and one-time guard decision model

## Reviewed invariants

- long-lived state scope is limited to MintState, GatewayConfig, and GuardianSet
- state layouts, lengths, discriminators, account views, and runtime layout version are inventoried
- future initializer must have a one-time guard
- future initializer must reject reinitialization
- direct manual unbounded admin initialization is rejected
- ProcessedEvent remains per-event replay protection and is not general protocol initialization
- RecipientBalance initialization model remains an explicit open design gap
- SPL mint setup and SPL mint authority architecture remain Blocker E
- gateway_mint_authority PDA is inventoried but not activated by D
- future initializer must not introduce hidden admin mint or balance-write pathways
- D.4 approves no execution, no account creation, no RPC, no signing, no deploy, no upgrade, no SPL setup, no submit, no mutation

## Invariant review matrix

```text
# State initialization invariant review matrix

D4_INVARIANT_01_LONG_LIVED_STATE_SCOPE_DEFINED
status: reviewed
result: true
meaning: Long-lived protocol state is MintState, GatewayConfig, and GuardianSet.

D4_INVARIANT_02_LAYOUTS_AND_DISCRIMINATORS_INVENTORIED
status: reviewed
result: true
meaning: D.2 records fixed lengths, fixed discriminators, account views, and runtime layout version.

D4_INVARIANT_03_ONE_TIME_GUARD_REQUIRED
status: required_future_initializer_property
result: true
meaning: Future initializer must reject reinitialization and must not rewrite initialized protocol state.

D4_INVARIANT_04_NO_DIRECT_MANUAL_UNBOUNDED_ADMIN_INIT
status: rejected_model
result: true
meaning: Direct manual state creation with unbounded admin discretion is rejected.

D4_INVARIANT_05_PROCESSED_EVENT_SEPARATED
status: reviewed
result: true
meaning: ProcessedEvent is per-event replay protection and not general protocol initialization.

D4_INVARIANT_06_RECIPIENT_BALANCE_GAP_RECORDED
status: open_design_gap
result: true
meaning: RecipientBalance lazy/precreate model is not closed by D.4 and must remain explicit.

D4_INVARIANT_07_SPL_SETUP_SEPARATED_TO_BLOCKER_E
status: reviewed
result: true
meaning: SPL mint setup and SPL mint authority architecture remain Blocker E.

D4_INVARIANT_08_GATEWAY_MINT_AUTHORITY_PDA_INVENTORIED_BUT_NOT_ACTIVATED
status: reviewed
result: true
meaning: gateway_mint_authority PDA is inventoried but not created or activated by D.4.

D4_INVARIANT_09_NO_ADMIN_MINT_OR_BALANCE_WRITE
status: required_future_initializer_property
result: true
meaning: Future state initialization must not introduce hidden admin mint or recipient-balance write paths.

D4_INVARIANT_10_NO_EXECUTION_APPROVED
status: reviewed
result: true
meaning: D.4 does not approve state init execution, account creation, RPC, testnet, signing, deploy, upgrade, SPL setup, submit, or mutation.

D4_AGGREGATE
all_invariants_reviewed: true
blocker_d_closure_ready: true
closure_type: narrow_design_boundary_only
```

## Review result

all_invariants_reviewed: true

blocker_d_closure_ready: true

closure_type: narrow_design_boundary_only

## Closure candidate prepared

D.4 prepares, but does not itself record, a narrow closure candidate for Blocker D:

STATE_INITIALIZATION_DESIGN_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- state initialization design boundary is reviewed
- long-lived state scope is defined
- one-time guard requirement is recorded
- reinitialization rejection requirement is recorded
- ProcessedEvent is separated from long-lived initialization
- SPL setup is separated to Blocker E
- no execution is approved

## Remaining open items outside D closure

- RecipientBalance lazy/precreate initialization mechanics
- actual initializer instruction/package implementation
- local evidence that reinitialization fails
- local evidence that partial initialization cannot be treated as valid
- SPL mint authority architecture in Blocker E
- expected post-upgrade ProgramData hash in Blocker B
- guardian descriptor in Blocker F
- rollback/recovery plan in Blocker G

## Non-closure statement

D.4 does not close Blocker D.

D.4 does not approve:

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

BLOCKER_D_REVIEW_READY_STATE_INITIALIZATION_INVARIANTS_RECORDED_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker D.5 — state initialization design closure decision record.

D.5 may close Blocker D narrowly as a design/invariant closure only.

D.5 must not initialize state, call RPC, use testnet, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-d-4-state-initialization-invariant-review-package
timestamp_utc=2026-07-06T18:47:19Z
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

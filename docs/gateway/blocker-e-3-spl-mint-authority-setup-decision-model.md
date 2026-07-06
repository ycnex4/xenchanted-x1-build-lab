# Blocker E.3 — SPL mint authority setup decision model

Status:

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_SETUP_DECISION_MODEL_RECORDED_NO_SPL_SETUP_NO_EXECUTION

Current decision:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Selected token program model:

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

Selected setup path:

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Selected freeze authority direction:

FREEZE_AUTHORITY_NONE_PREFERRED

Selected initial supply rule:

ZERO_INITIAL_SUPPLY_REQUIRED

Execution boundary:

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker E.3 records the SPL mint authority setup decision model.

E.3 is decision-model only.

It does not create an SPL mint.

It does not configure mint authority.

It does not transfer mint authority.

It does not disable or set freeze authority.

It does not mint tokens.

It does not initialize state.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, construct guardian packages, submit, or mutate any network.

## Background

E.1 opened the SPL mint authority architecture track.

E.2 completed a repo-grounded inventory of the gateway_mint_authority PDA, SPL Token mint_to CPI boundary, account contract entries, MintState relationship fields, and fail-closed CPI gate.

## E.3 selected model

E.3 selects the following model:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Meaning:

- gateway_mint_authority PDA is the canonical mint authority
- no human/admin mint authority may remain after canonical setup
- direct retained human/admin mint authority is rejected
- SPL mint setup execution requires a future reviewed setup package
- SPL mint setup execution requires a final scoped GO

## Token program model

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

The current repo CPI path is based on classic SPL Token.

Token-2022 is not selected by E.3.

A Token-2022 path would require an explicit future design/code change and a separate review.

## Setup path

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Preferred future model:

- initialize the mint with gateway_mint_authority PDA as mint authority from the start

Conditionally acceptable fallback:

- temporary setup authority may exist only for setup
- temporary setup authority must publicly hand off mint authority to gateway_mint_authority PDA
- temporary setup authority must not remain capable of minting
- handoff evidence must be recorded before any live path is accepted

## Freeze authority direction

FREEZE_AUTHORITY_NONE_PREFERRED

Preferred future direction:

- freeze authority disabled / none

Rejected default:

- retained human/admin freeze authority

## Initial supply rule

ZERO_INITIAL_SUPPLY_REQUIRED

The SPL mint must start with zero supply.

Supply should increase only through gateway-authorized minting after all required blockers and final GO are complete.

## Execution boundary

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

E.3 does not approve any SPL setup execution.

## Decision matrix

```text
# SPL mint authority setup decision matrix

E3_MODEL_0_NO_SPL_SETUP_YET
status: current_default
meaning: No SPL mint setup execution is approved.
allows_spl_mint_creation_now: false
allows_authority_configuration_now: false
allows_mint_to_now: false
allows_mutation_now: false

E3_MODEL_1_RETAIN_HUMAN_ADMIN_MINT_AUTHORITY
status: rejected
meaning: A human/admin key remains capable of minting after setup.
reason_rejected: incompatible with no hidden admin mint and no admin supply control.
allows_spl_mint_creation_now: false
allows_mutation_now: false

E3_MODEL_2_TEMPORARY_SETUP_AUTHORITY_THEN_PUBLIC_HANDOFF_TO_GATEWAY_PDA
status: conditionally_acceptable_future_testnet_fallback
meaning: A temporary setup authority may exist only long enough to initialize the mint and publicly hand off mint authority to gateway_mint_authority PDA.
conditions_required_before_execution: exact setup package, exact authority handoff proof, exact freeze authority proof, public evidence, final scoped GO.
allows_spl_mint_creation_now: false
allows_authority_configuration_now: false
allows_mutation_now: false

E3_MODEL_3_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_FROM_START
status: selected_preferred_future_model
meaning: SPL mint is initialized with gateway_mint_authority PDA as canonical mint authority from the start.
reason_selected: avoids retained human/admin mint authority and matches current PDA/CPI design.
allows_spl_mint_creation_now: false
allows_authority_configuration_now: false
allows_mutation_now: false

E3_MODEL_4_RETAIN_HUMAN_ADMIN_FREEZE_AUTHORITY
status: rejected
meaning: A human/admin key remains capable of freezing accounts.
reason_rejected: incompatible with immutable/non-admin protocol direction unless separately justified and disclosed; not selected for this path.
allows_mutation_now: false

E3_MODEL_5_FREEZE_AUTHORITY_NONE
status: selected_preferred_future_model
meaning: Freeze authority is disabled / none after canonical setup.
allows_freeze_authority_setup_now: false
allows_mutation_now: false

E3_MODEL_6_TOKEN_2022_MINT
status: not_selected_current_repo_model
meaning: Token-2022 is not the current repo CPI model.
reason_not_selected: current CPI inventory is classic SPL Token via spl_token::id() and spl_token::instruction::mint_to.
allows_mutation_now: false

SELECTED_E3_TOKEN_PROGRAM_MODEL
CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

SELECTED_E3_CANONICAL_MINT_AUTHORITY
GATEWAY_MINT_AUTHORITY_PDA

SELECTED_E3_SETUP_PATH
PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

SELECTED_E3_FREEZE_AUTHORITY_DIRECTION
FREEZE_AUTHORITY_NONE_PREFERRED

SELECTED_E3_INITIAL_SUPPLY_RULE
ZERO_INITIAL_SUPPLY_REQUIRED

SELECTED_E3_DECISION
GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

SELECTED_E3_EXECUTION_BOUNDARY
FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION
```

## Remaining open items before E closure

- exact decimals value
- exact canonical mint account address/model
- exact setup package
- exact freeze authority proof
- exact mint authority proof
- exact total supply reconciliation evidence
- exact local/testnet evidence package
- final scoped GO before execution

## Non-closure statement

E.3 does not close Blocker E.

E.3 does not approve:

- SPL mint creation
- SPL mint initialization
- mint authority assignment
- mint authority transfer
- freeze authority assignment
- freeze authority disablement
- SPL CPI minting
- state initialization execution
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

BLOCKER_E_OPEN_SPL_MINT_AUTHORITY_SETUP_DECISION_MODEL_RECORDED_NO_SPL_SETUP_NO_EXECUTION

Current decision:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Selected token program model:

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

Selected setup path:

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Selected freeze authority direction:

FREEZE_AUTHORITY_NONE_PREFERRED

Selected initial supply rule:

ZERO_INITIAL_SUPPLY_REQUIRED

Execution boundary:

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker E.4 — SPL mint authority invariant review package.

E.4 should review no-human-mint-authority, freeze authority direction, zero initial supply, fail-closed CPI, total supply reconciliation gap, and no-execution boundary.

E.4 must not create an SPL mint, configure authority, call RPC, use testnet, sign, deploy, upgrade, initialize state, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-e-3-spl-mint-authority-setup-decision-model
timestamp_utc=2026-07-06T19:15:56Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
spl_mint_created=false
spl_authority_configured=false
spl_mint_to_executed=false
state_initialized=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```

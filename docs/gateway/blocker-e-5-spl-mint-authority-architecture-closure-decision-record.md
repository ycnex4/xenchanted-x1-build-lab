# Blocker E.5 — SPL mint authority architecture closure decision record

Status:

BLOCKER_E_CLOSED_NARROW_SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_E_CLOSED_NARROW_ARCHITECTURE_INVARIANTS_ONLY

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker E.5 records the closure decision for Blocker E.

The closure is narrow.

It closes only the SPL mint authority architecture / invariant review blocker.

It does not approve SPL mint setup execution.

It does not approve SPL mint creation.

It does not approve mint authority assignment or transfer.

It does not approve freeze authority assignment or disablement.

It does not approve SPL CPI minting.

It does not approve state initialization execution.

It does not approve deploy, upgrade, signing, submit, or mutation.

## Closure state

Blocker E is closed as:

SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Meaning:

- SPL mint authority architecture has been reviewed
- classic SPL Token is the current repo model
- gateway_mint_authority PDA is selected as canonical mint authority
- retained human/admin mint authority is rejected
- zero initial supply is required
- freeze authority none / disabled is preferred
- retained human/admin freeze authority is rejected as default
- SPL CPI minting remains fail-closed by default
- SPL CPI minting must remain downstream of gateway authorization
- MintState relationship fields are recorded
- live SPL total supply reconciliation proof remains future evidence
- no SPL setup execution is approved
- no mutation is approved

## Evidence chain

E.5 is based on:

1. E.1 — SPL mint authority architecture planning
2. E.2 — repo-grounded SPL mint authority and CPI inventory
3. E.3 — SPL mint authority setup decision model
4. E.4 — SPL mint authority invariant review package

## Accepted E.2 inventory result

E.2 inventory accepted:

all_inventory_checks_passed: true

Accepted inventory categories:

- gateway_mint_authority PDA inventory
- classic SPL Token CPI mint_to boundary
- gateway_mint_authority signer seeds
- fail-closed SPL CPI gate
- account contract entries for spl_token_mint, recipient_token_account, mint_authority_pda, and token_program
- MintState fields for mint_pubkey, gateway_mint_authority_pda, gateway_mint_authority_bump, and total_supply
- deployment_status remains deployable=false
- Program ID placeholder boundary remains active

## Accepted E.3 decision

E.3 decision accepted:

GATEWAY_MINT_AUTHORITY_PDA_CANONICAL_MINT_AUTHORITY_NO_RETAINED_HUMAN_ADMIN_MINT_AUTHORITY

Accepted token program model:

CLASSIC_SPL_TOKEN_CURRENT_REPO_MODEL

Accepted setup path:

PREFER_INITIALIZE_MINT_WITH_GATEWAY_MINT_AUTHORITY_PDA_AS_AUTHORITY_ELSE_REVIEWED_TEMP_SETUP_AUTHORITY_HANDOFF_TO_PDA

Accepted freeze authority direction:

FREEZE_AUTHORITY_NONE_PREFERRED

Accepted initial supply rule:

ZERO_INITIAL_SUPPLY_REQUIRED

Accepted execution boundary:

FUTURE_REVIEWED_SPL_SETUP_PACKAGE_AND_FINAL_SCOPED_GO_REQUIRED_BEFORE_ANY_SPL_MINT_SETUP_EXECUTION

## Accepted E.4 invariant result

E.4 invariant result accepted:

all_invariants_reviewed: true

blocker_e_closure_ready: true

closure_type: narrow_architecture_boundary_only

Accepted invariant categories:

- classic SPL Token is the current repo model
- gateway_mint_authority PDA is canonical
- no retained human/admin mint authority
- zero initial supply
- freeze authority none / disabled preferred
- retained human/admin freeze authority rejected as default
- SPL CPI fail-closed by default
- SPL CPI downstream of gateway authorization
- MintState relationship recorded
- total supply reconciliation remains future evidence
- no SPL setup execution approved

## What this closure allows

This closure allows future planning to treat Blocker E as closed for the narrow SPL mint authority architecture/invariant question.

It allows the project to proceed to the next separately scoped blocker.

Recommended next blockers:

- F — guardian descriptor
- B — expected post-upgrade ProgramData hash
- G — rollback / recovery plan

## What this closure does not allow

This closure does not approve:

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

## Remaining blockers

After E.5:

- Blocker A — CLOSED narrowly: upgrade authority present but accepted for test phase
- Blocker B — OPEN: expected post-upgrade ProgramData hash
- Blocker C — CLOSED narrowly: B1C7 handler boundary / invariants only
- Blocker D — CLOSED narrowly: state initialization design / invariants only
- Blocker E — CLOSED narrowly: SPL mint authority architecture / invariants only
- Blocker F — OPEN: guardian descriptor
- Blocker G — OPEN: rollback / recovery plan
- Blocker H — CLOSED narrowly: local-validator health dry-run only

## Safety invariant

Closing Blocker E must not weaken the overall NO-GO boundary.

Overall testnet mutation remains NO-GO until B, F, and G are closed and a final scoped GO package is recorded.

A future reviewed SPL setup package remains required before any SPL mint setup execution.

A future final scoped GO remains required before any SPL setup execution.

## Result

Current status:

BLOCKER_E_CLOSED_NARROW_SPL_MINT_AUTHORITY_ARCHITECTURE_REVIEWED_EXECUTION_NOT_APPROVED

Current decision:

BLOCKER_E_CLOSED_NARROW_ARCHITECTURE_INVARIANTS_ONLY

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Proceed to the next separately scoped blocker.

Recommended next step:

Blocker F.1 — guardian descriptor planning.

Do not proceed to deploy, upgrade, state init execution, SPL setup, guardian package construction, or submit.

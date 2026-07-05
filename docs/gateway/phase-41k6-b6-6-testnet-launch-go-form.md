# Phase 41K.6 B6.6 — Testnet launch GO form

## Purpose

This form defines the minimum fields required before any live X1 testnet action may be approved.

This form is currently unapproved.

This form does not approve signing.

This form does not approve transaction submission.

This form does not approve SOL spend.

This form does not approve private-key handling.

This form does not approve deploy.

This form does not approve program upgrade.

This form does not approve account initialization.

This form does not approve production or production-like activation.

## Current status

- decision: NO-GO
- approved_by: null
- approved_at_utc: null
- approved_commit: null
- approved_scope: null
- approved_network: null
- approved_program_id: null
- approved_fee_payer_public_address: null
- approved_max_sol_spend: null
- approved_strategy: null
- approved_live_action_class: null
- approved_commands_or_procedure: null
- approved_abort_conditions: null
- approved_post_action_observation: null
- explicit_no_production_activation: true

Null means not approved.

Empty means not approved.

Missing means not approved.

## Strategy selection

- Strategy 1 existing program state initialization only: not approved
- Strategy 2 program upgrade then state initialization: not approved
- Strategy 3 new testnet deployment: not approved
- Strategy 4 stop and redesign: not approved

## Live action classes

- Class A build-only local artifact: not approved
- Class B testnet program upgrade: not approved
- Class C testnet state initialization: not approved
- Class D testnet SPL mint setup: not approved
- Class E testnet guardian evidence package: not approved
- Class F testnet submit rehearsal: not approved

## Required before any GO

Before any GO, the following must be filled:

- exact approved scope
- exact approved network
- exact approved program id
- exact approved strategy
- exact approved live action class
- exact fee payer public address if SOL can be spent
- exact max SOL spend if SOL can be spent
- exact commands or procedure
- exact abort conditions
- exact post-action read-only observation
- explicit statement that production activation remains excluded
- explicit statement that private keys must never be printed
- explicit statement that keypair paths must not be committed

## Current decision

Current decision:

NO-GO.

This form does not authorize live action.

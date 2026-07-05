# Phase 41K.6 B6.38 — Upgrade authority custody map

Status:

UPGRADE_AUTHORITY_CUSTODY_MAP_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps blocker A:

upgrade authority custody map.

It records what is known, what remains unknown, and what must be proven before any future program upgrade can be considered.

This is docs-only.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker A status

Blocker A:

upgrade authority custody map

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Known public baseline

Current known public testnet baseline:

- x1_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- x1_testnet_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority_public_key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

These are public identifiers only.

No signing material is stored in this repository.

No signing material is printed by this checkpoint.

No signing action is approved by this checkpoint.

## Custody map principle

Before any future upgrade can be approved, the authority custody boundary must answer:

1. Who controls the upgrade authority?
2. Where is the signing capability kept?
3. How is access controlled?
4. What approval is required before use?
5. What exact network is authorized?
6. What exact program id is authorized?
7. What exact expected ProgramData hash is authorized?
8. What maximum cost boundary applies?
9. What abort conditions apply?
10. What post-upgrade verification must pass?
11. What rollback and recovery rule applies if verification fails?

This checkpoint does not answer all of these.

It maps the requirements only.

## Current custody state

Current known state:

- public upgrade authority address is known
- ProgramData account is known
- program id is known
- local docs and skeletons are present
- post-upgrade expected hash is not yet recorded
- scoped written upgrade GO is not present
- local-validator dry-run is not executed
- blocker B is not closed
- blocker C is not closed as runtime-ready
- blocker D is not closed as executable state initialization
- blocker E is not closed
- blocker F is not closed
- blocker G is not closed
- blocker H is not closed

Conclusion:

Blocker A cannot be closed yet.

## Required custody fields before closure

Before blocker A can close, the following fields must be explicitly recorded:

- authority_public_key
- authority_role
- authority_holder_confirmation
- signing_boundary_description
- approval_required_before_use
- allowed_network
- allowed_program_id
- allowed_programdata_account
- allowed_operation_type
- max_cost_boundary
- allowed_time_window_if_any
- expected_build_artifact
- expected_programdata_hash
- pre_upgrade_read_only_verification
- post_upgrade_read_only_verification
- abort_conditions
- recovery_conditions
- evidence_retention_policy

## Required human approval boundary

Any future upgrade requires explicit scoped written GO.

The approval must include:

- network
- program id
- ProgramData account
- authority public key
- operation type
- expected build artifact
- expected ProgramData hash
- fee payer public address if applicable
- maximum cost boundary
- exact command scope
- abort conditions
- post-upgrade verification commands
- statement that no production activation is included unless separately scoped

Generic intent is not enough.

Examples that are not enough:

- continue
- go ahead
- test it
- run testnet
- upgrade it
- launch it

## Forbidden until scoped GO

The following remain forbidden:

- using upgrade authority
- signing an upgrade
- submitting an upgrade transaction
- deploying a new program to testnet
- initializing testnet state accounts
- setting SPL mint authority
- minting through SPL CPI on testnet
- constructing non-local guardian packages
- submitting gateway messages
- enabling live route
- enabling production activation

## Read-only verification required before any future GO

Before any future upgrade GO can be considered, read-only verification must confirm:

- current program id
- current ProgramData account
- current ProgramData upgrade authority
- current ProgramData executable state
- current program binary identity if available
- current deployed slot or last deployment metadata if available
- current account ownership
- current balance and rent context if relevant

Read-only verification must not require signing.

## Post-upgrade verification required after any future GO

If a future upgrade is explicitly approved and executed, post-upgrade verification must confirm:

- program id unchanged
- ProgramData account unchanged
- upgrade authority unchanged unless explicitly scoped
- ProgramData hash equals expected hash
- program remains executable
- no state initialization occurred unless separately scoped
- no SPL mint authority change occurred unless separately scoped
- no gateway submit occurred unless separately scoped
- no live route activation occurred unless separately scoped

If any verification fails, all follow-up actions must stop.

## Abort conditions

Any future upgrade path must abort if:

- wrong branch
- dirty working tree
- missing expected hash
- missing local-validator evidence if required
- missing blocker closure evidence
- wrong program id
- wrong ProgramData account
- wrong authority public key
- authority holder ambiguity
- fee payer ambiguity
- network ambiguity
- build artifact mismatch
- command scope ambiguity
- unexpected cost boundary
- post-upgrade verification plan missing
- rollback plan missing

## Recovery condition

If an upgrade attempt ever fails or verification is ambiguous:

- stop further actions
- do not initialize state
- do not set mint authority
- do not submit gateway message
- do not retry blindly
- preserve logs
- record observed program id
- record observed ProgramData account
- record observed ProgramData hash if available
- create a separate recovery decision

## Relationship to other blockers

Blocker A depends on or interacts with:

- blocker B: expected post-upgrade ProgramData hash
- blocker C: B1C7 handler presence verification
- blocker D: state initialization instruction design
- blocker E: SPL mint authority architecture
- blocker F: guardian set testnet descriptor
- blocker G: rollback and recovery plan
- blocker H: local validator dry-run

Current state:

All remain not closed for testnet upgrade readiness.

## Explicit non-closure

This checkpoint does not close blocker A.

It maps upgrade authority custody requirements only.

Current blocker A state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only expected post-upgrade ProgramData hash plan for blocker B.

No upgrade action is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

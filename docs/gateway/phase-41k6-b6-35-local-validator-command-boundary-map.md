# Phase 41K.6 B6.35 — Local-validator command boundary map

Status:

LOCAL_VALIDATOR_COMMAND_BOUNDARY_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document defines the command boundary for a future local-validator-only dry-run.

It extends:

- B6.29 local-validator dry-run design map
- B6.30 local-validator fixture inventory map
- B6.31 local-validator fixture generator design
- B6.32 local-validator fixture generator schema
- B6.33 local-only fixture generator skeleton
- B6.34 local fixture generator safety checkpoint

This is docs-only.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker H status

Blocker H:

local validator dry-run

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Command boundary principle

A future local-validator-only command plan must be explicitly separated from any live or testnet flow.

Allowed future command classes may include only:

- local environment inspection
- local-only fixture generation
- local-only test execution
- local-validator-only process startup
- local-validator-only process shutdown
- local-only account fixture loading
- local-only transaction simulation
- local-only snapshot comparison
- local-only log inspection
- local cleanup

The current checkpoint does not approve any of these commands for execution.

It only defines the boundary for a later explicit local-validator-only GO.

## Forbidden command classes

The following command classes are forbidden until a later explicit written GO changes the boundary:

- testnet submit
- live RPC interaction
- program upgrade
- program deploy to testnet
- program deploy to production
- account initialization on testnet
- fee payer usage on testnet
- SPL mint authority setup on testnet
- SPL CPI minting on testnet
- guardian package construction using non-local material
- bridge message submit rehearsal against live endpoints
- any command using real signing material
- any command using production accounts
- any command using production mint
- any command using production recipient accounts

## Required pre-command checks for future local-validator-only GO

Before any future local-validator-only execution, the command plan must prove:

1. Network boundary is local-only.
2. No testnet RPC is present.
3. No live RPC is present.
4. No production program id is used as execution target.
5. No production mint is used.
6. No production recipient account is used.
7. No signing material path is referenced.
8. No secret material is printed.
9. No submit command targets a live network.
10. No deploy command targets a live network.
11. No upgrade command targets a live network.
12. All fixture inputs are deterministic and local.
13. All expected outputs are deterministic and local.
14. Abort conditions are defined.
15. Cleanup conditions are defined.

## Required command groups for later design

A future command plan may be split into these groups:

### Group 1 — local source checks

Purpose:

Verify repo state, docs state, and local skeleton tests.

Allowed future examples:

- branch and status checks
- focused cargo tests
- secret pattern scans
- docs invariant scans

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 2 — local fixture generation

Purpose:

Generate deterministic local fixture files from the B6.32 schema and B6.33 skeleton.

Allowed future examples:

- generate manifest
- generate accounts
- generate instructions
- generate scenarios
- generate expected snapshots
- generate failure matrix
- generate safety report

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 3 — local validator startup

Purpose:

Start a local validator process only.

Allowed future examples:

- start local validator with disposable ledger
- ensure no live RPC endpoint
- ensure local-only process id tracking
- ensure cleanup trap

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 4 — local deployment fixture

Purpose:

Load or deploy a local-only program into the local validator.

Allowed future examples:

- local-only program load
- local-only program id verification
- local-only binary identity recording

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 5 — local account fixture setup

Purpose:

Set up local accounts only.

Allowed future examples:

- local gateway_config fixture account
- local guardian_set fixture account
- local mint_state fixture account
- local processed_event fixture account
- local SPL mint fixture
- local recipient token account fixture

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 6 — local success scenario

Purpose:

Run one local-only success scenario.

Required evidence:

- before snapshot
- instruction result
- after snapshot
- expected state delta
- expected logs
- cleanup result

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

### Group 7 — local failure matrix

Purpose:

Run local-only failure scenarios.

Required evidence:

- expected error
- no mutation on failure
- account byte snapshot equality
- expected logs
- cleanup result

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

## Required abort conditions for later execution

A future local-validator-only command plan must abort if:

- branch is not expected
- working tree is dirty
- testnet string appears in execution config
- live RPC appears in execution config
- production account appears in fixtures
- signing material path appears
- secret material appears
- program id boundary is ambiguous
- fixture safety report fails
- local validator fails to start cleanly
- local validator exposes non-local RPC
- local deployment target is ambiguous
- expected snapshot is missing
- mutation invariance fixture is missing
- any failure scenario mutates an account unexpectedly

## Required cleanup conditions for later execution

A future local-validator-only command plan must define cleanup for:

- local validator process
- disposable ledger
- generated fixture files
- temporary logs
- local deployment artifacts
- local account snapshots

Cleanup must not remove repo source files unless explicitly scoped.

## Evidence required before blocker H can close

This document does not close blocker H.

Required future evidence before closure:

- actual local-validator command plan
- explicit local-validator-only GO
- local-only fixture files
- local-only success scenario result
- local-only failure matrix result
- local-only mutation-invariance result
- safety report PASS
- cleanup report
- proof that no testnet was used
- proof that no live RPC was used
- proof that no real signing material was used

## Explicit non-closure

This checkpoint does not close blocker H.

It defines future command boundaries only.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only success and failure matrix command plan.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.

# Phase 41K.6 B6.37 — Rollback and recovery plan map

Status:

ROLLBACK_RECOVERY_PLAN_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps blocker G:

rollback and recovery plan.

It defines what must be known before any future local-validator execution, testnet initialization, program upgrade, submit rehearsal, or live route activation.

This is docs-only.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker G status

Blocker G:

rollback and recovery plan

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Why rollback is required

Any future move beyond local skeletons introduces execution risk.

The rollback and recovery plan must exist before:

- local-validator dry-run execution
- runtime handler enablement
- testnet program upgrade
- testnet account initialization
- SPL mint authority setup
- SPL CPI minting
- guardian package construction
- submit rehearsal
- live route activation

B6.37 does not approve any of these steps.

## Current known safety boundary

Current flags remain:

- live_runtime_handler_enabled: false
- live_route_enabled: false
- on_chain_state_write_enabled: false
- account_initialization_enabled: false
- spl_cpi_minting_enabled: false
- testnet_submit_enabled: false
- local_validator_execution_approved: false
- upgrade_go_approved: false
- state_init_go_approved: false
- submit_go_approved: false

Current decision remains:

NO-GO.

## Rollback scope categories

The rollback plan must cover the following categories:

1. Local-validator dry-run rollback.
2. Fixture generation rollback.
3. Runtime handler rollback.
4. Program upgrade rollback.
5. ProgramData hash mismatch recovery.
6. State initialization recovery.
7. SPL mint authority recovery.
8. Guardian set recovery.
9. Submit rehearsal recovery.
10. Live route activation recovery.

## Category 1 — local-validator dry-run rollback

Current status:

DESIGN_ONLY_NOT_EXECUTED

Required rollback boundary:

- stop local validator process
- remove disposable ledger
- remove generated fixture output if scoped
- preserve logs for review
- preserve failure artifacts for debugging
- confirm no testnet endpoint was used
- confirm no live signing material was used

Abort conditions:

- non-local RPC detected
- production account detected
- testnet endpoint detected
- key material path detected
- local validator fails to start cleanly
- fixture safety report fails
- mutation-invariance check fails

Recovery result required:

- local process stopped
- disposable state removed or archived
- repo source unchanged unless scoped
- no external state touched

## Category 2 — fixture generation rollback

Current status:

SKELETON_ONLY

Required rollback boundary:

- fixture output directory must be disposable
- generated files must be reproducible
- generated files must not contain live endpoints
- generated files must not contain signing material
- generated files must not contain production accounts

Abort conditions:

- safety report FAIL
- non-local fixture detected
- missing manifest
- missing expected snapshots
- missing mutation-invariance entries
- unsafe text pattern detected

Recovery result required:

- generated fixtures deleted or archived
- safety report preserved
- source files unchanged unless scoped

## Category 3 — runtime handler rollback

Current status:

NOT ENABLED

Required rollback boundary:

- handler feature gate must be explicit
- B1C7 guard status must be explicit
- live route must remain disabled unless separately approved
- build profile must be documented
- handler path must have a no-mutation failure matrix

Abort conditions:

- accidental live route
- handler enabled without feature gate
- mutation before validation
- CPI before validation
- replay mark before all preconditions pass
- missing failure matrix
- missing local-validator evidence

Recovery result required:

- return to guard-disabled or route-disabled handler boundary
- preserve local evidence
- no testnet state touched unless a later explicit GO has scoped it

## Category 4 — program upgrade rollback

Current status:

NOT APPROVED

Required rollback boundary:

- upgrade authority custody map must be complete
- expected ProgramData hash must be known
- pre-upgrade ProgramData hash must be recorded
- post-upgrade ProgramData hash must be verified
- abort conditions must be explicit
- recovery path must be explicit

Abort conditions:

- wrong branch
- dirty working tree
- build artifact mismatch
- expected hash missing
- upgrade authority ambiguity
- fee payer ambiguity
- network ambiguity
- post-upgrade hash mismatch
- post-upgrade read-only verification failure

Recovery result required:

- stop immediately on mismatch
- record observed ProgramData hash
- record program id
- record ProgramData account
- do not proceed to state initialization
- do not proceed to submit rehearsal
- prepare separate recovery decision

## Category 5 — ProgramData hash mismatch recovery

Current status:

NOT APPLICABLE_YET

Required recovery boundary:

- expected hash must be recorded before upgrade
- actual hash must be read after upgrade
- mismatch must block all further steps

Abort conditions:

- actual hash differs from expected
- ProgramData account differs from expected
- upgrade authority differs from expected
- program id differs from expected

Recovery result required:

- freeze all follow-up actions
- record mismatch report
- avoid state initialization
- avoid SPL setup
- avoid submit rehearsal
- decide whether recovery requires a new upgrade plan

## Category 6 — state initialization recovery

Current status:

NOT APPROVED

Required rollback boundary:

- state account list must be explicit
- PDA seeds must be explicit
- account sizes must be explicit
- rent requirements must be explicit
- initialization order must be explicit
- read-only verification must be explicit

Abort conditions:

- PDA mismatch
- owner mismatch
- discriminator mismatch
- account size mismatch
- rent mismatch
- duplicate initialized account
- wrong mint
- wrong authority
- wrong guardian set id
- wrong route id

Recovery result required:

- stop before submit rehearsal
- record initialized and non-initialized accounts
- avoid partial assumptions
- prepare separate recovery plan for any partial state

## Category 7 — SPL mint authority recovery

Current status:

NOT APPROVED

Required rollback boundary:

- SPL mint authority architecture must be finalized
- local-validator CPI evidence must exist
- authority handoff must be explicit
- authority verification must be read-only verifiable
- no production mint must be touched without explicit GO

Abort conditions:

- wrong mint
- wrong authority
- wrong PDA
- wrong bump
- wrong token program
- authority handoff mismatch
- unexpected mint supply change

Recovery result required:

- stop before submit rehearsal
- record mint authority state
- record mint supply
- avoid any gateway mint submit
- prepare separate recovery decision

## Category 8 — guardian set recovery

Current status:

NOT APPROVED

Required rollback boundary:

- guardian descriptor must be explicit
- guardian set id must be explicit
- threshold must be explicit
- guardian public keys must be explicit
- rotation policy must be explicit
- descriptor hash or integrity marker must be explicit

Abort conditions:

- wrong guardian set id
- wrong threshold
- wrong guardian count
- duplicate guardian
- unknown guardian
- invalid descriptor hash
- signature package mismatch

Recovery result required:

- stop before submit rehearsal
- record descriptor mismatch
- avoid live message authorization
- prepare corrected guardian descriptor

## Category 9 — submit rehearsal recovery

Current status:

NOT APPROVED

Required rollback boundary:

- submit rehearsal must have explicit GO
- dry-run evidence must exist
- fixture evidence must exist
- expected failure and success behavior must be known
- maximum risk boundary must be explicit

Abort conditions:

- wrong network
- wrong program id
- wrong recipient
- wrong mint
- wrong amount
- replay detected
- guardian mismatch
- finality mismatch
- unexpected error
- unexpected mutation

Recovery result required:

- stop further submits
- record transaction signature if any
- record logs
- record account snapshots
- do not retry blindly
- prepare separate recovery decision

## Category 10 — live route activation recovery

Current status:

NOT APPROVED

Required rollback boundary:

- live route activation must be separately approved
- all blockers A-H must be closed
- local-validator evidence must pass
- testnet evidence must pass
- route disable plan must exist

Abort conditions:

- any blocker still open
- any safety flag ambiguous
- any hash mismatch
- any account mismatch
- any guardian descriptor mismatch
- any no-mutation failure
- any unexpected live route behavior

Recovery result required:

- disable or keep disabled live route
- stop submits
- preserve evidence
- publish internal incident note if needed
- create separate recovery branch if code changes are needed

## Required rollback evidence checklist

Before blocker G can close, the following must exist:

- local-validator rollback plan
- fixture rollback plan
- handler rollback plan
- upgrade rollback plan
- ProgramData mismatch recovery plan
- state initialization recovery plan
- SPL mint authority recovery plan
- guardian set recovery plan
- submit rehearsal recovery plan
- live route activation recovery plan
- abort condition list
- cleanup condition list
- read-only verification commands
- evidence preservation plan
- no-secrets log policy

## Explicit non-closure

This checkpoint does not close blocker G.

It maps the rollback and recovery requirements only.

Current blocker G state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only upgrade authority custody map for blocker A.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

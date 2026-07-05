# Phase 41K.6 B6.41 — Local-validator-only GO form design

Status:

LOCAL_VALIDATOR_ONLY_GO_FORM_DESIGN_NOT_APPROVED

Current decision:

NO-GO

## Purpose

This document defines the future local-validator-only GO form.

It follows the B6.40 Strategy 2 closure-readiness checkpoint.

This is docs-only.

It does not approve local-validator execution.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current boundary

Current status:

LOCAL_VALIDATOR_ONLY_GO_FORM_DESIGN_NOT_APPROVED

Current decision:

NO-GO

The purpose of this document is to define what an explicit future local-validator-only GO must contain.

This checkpoint itself is not that GO.

## Why this form is required

A local-validator dry-run is safer than testnet, but it is still an execution step.

Before any future local-validator execution, the approval must be explicit and scoped.

Generic phrases are not enough.

Examples that are not enough:

- continue
- go ahead
- test it
- run it
- start validator
- let's try
- do local validator

A valid local-validator-only GO must specify the exact local scope.

## Required future GO fields

A future local-validator-only GO must include:

- phase id
- execution type
- local-only statement
- explicit no-testnet statement
- explicit no-live-RPC statement
- repo path
- branch
- expected source commit
- local program fixture id
- fixture output directory
- local validator ledger directory
- allowed command groups
- forbidden command groups
- maximum execution boundary
- abort conditions
- cleanup conditions
- evidence files to preserve
- statement that no production activation is included

## Required phase id

The future GO must name the phase exactly.

Expected phase id format:

Phase 41K.6 B6.x local-validator-only execution

The exact B6 substep must be included.

If the phase id is missing, execution is not approved.

## Required execution type

The future GO must say:

local-validator-only dry-run

It must not say:

- testnet dry-run
- live submit
- upgrade
- deployment
- production activation

If the execution type is ambiguous, execution is not approved.

## Required local-only statement

The future GO must include:

I approve local-validator-only execution.

It must also include:

No testnet action is approved.

No live RPC action is approved.

No production activation is approved.

## Required repo boundary

The future GO must include:

- repository path
- expected branch
- expected clean working tree
- expected commit hash or commit range
- expected focused test commands

If the repo boundary is missing, execution is not approved.

## Required fixture boundary

The future GO must include:

- fixture generator path
- fixture schema path
- fixture output directory
- deterministic fixture seed policy
- safety report path
- manifest path
- account fixture path
- instruction fixture path
- scenario fixture path
- snapshot fixture path
- failure matrix path

Fixture output must be local and disposable.

Fixture output must not contain:

- live RPC URLs
- production accounts
- production mint
- production guardian material
- private keys
- keypair paths
- seed phrases
- mnemonic material

## Required local validator boundary

The future GO must include:

- local validator command group
- local ledger directory
- local RPC endpoint if used
- process cleanup rule
- ledger cleanup rule
- timeout rule
- log path
- no-testnet proof requirement
- no-live-RPC proof requirement

The local validator must use a disposable local ledger.

## Required allowed command groups

The future GO may allow only explicitly listed local command groups.

Allowed future groups may include:

- local source checks
- local fixture generation
- local validator startup
- local-only program fixture load
- local account fixture setup
- local success scenario execution
- local failure matrix execution
- local snapshot comparison
- local log inspection
- local cleanup

If a command group is not listed, it is not approved.

## Required forbidden command groups

The future GO must explicitly forbid:

- testnet submit
- live RPC submit
- program upgrade
- program deploy to testnet
- program deploy to production
- account initialization on testnet
- SPL mint authority setup on testnet
- SPL CPI minting on testnet
- non-local guardian package construction
- production bridge message construction
- production route activation
- any command using real signing material
- any command using production accounts
- any command using production mint
- any command using production recipient accounts

## Required abort conditions

The future GO must require abort if:

- working tree is dirty
- branch differs from expected
- commit differs from expected
- fixture safety report fails
- testnet endpoint is detected
- live RPC endpoint is detected
- production account is detected
- production mint is detected
- keypair path is detected
- private material is detected
- submit command targets non-local network
- deploy command targets non-local network
- upgrade command appears
- local validator fails to start cleanly
- expected snapshot is missing
- mutation-invariance check fails
- cleanup fails

## Required cleanup conditions

The future GO must define cleanup for:

- local validator process
- disposable ledger
- generated fixture files
- temporary logs
- local deployment artifacts if any
- local snapshots

Cleanup must not remove source files unless explicitly scoped.

## Required evidence files

The future GO must list evidence to preserve:

- command log
- fixture safety report
- manifest
- generated fixture summary
- local validator startup log
- success scenario result
- failure matrix result
- mutation-invariance result
- sanitized logs
- cleanup report

## Required success criteria

The future GO must define success criteria:

- no testnet used
- no live RPC used
- no real signing material used
- fixture safety report PASS
- local validator starts cleanly
- success scenario passes
- failure matrix passes
- all failure cases preserve no-mutation invariant
- cleanup completes
- logs contain no forbidden material

## Required failure handling

The future GO must define failure handling:

- stop immediately
- preserve logs
- preserve safety report
- preserve snapshots
- do not retry blindly
- do not proceed to testnet
- create a separate recovery checkpoint if needed

## Explicit non-approval

This B6.41 checkpoint does not approve local-validator execution.

It only defines the future GO form.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is local fixture file emission skeleton planning.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

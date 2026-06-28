# XXXL Program ID and PDA Fixture Readiness Plan

Status: DRAFTED / BLOCKING.

This document defines the readiness plan for the `PLACEHOLDER_PROGRAM_ID` deployment blocker.

It does not remove the blocker.

It does not set a real Program ID.

It does not activate deployment.

It does not activate the live route.

It does not enable SPL CPI execution.

## Current blocker

Current blocker:

    PLACEHOLDER_PROGRAM_ID

Current meaning:

The runtime still exposes a placeholder Program ID boundary.

Current resolution guidance:

Set and review the real Program ID and regenerate all Program-ID-dependent PDA fixtures.

Current status:

    BLOCKED

## Goal

Prepare the exact process for replacing the placeholder Program ID boundary with a real Program ID boundary in a future reviewed stage.

The purpose of this plan is to prevent an unsafe or partial Program ID transition.

## Non-goals

This stage does not:

- choose a real Program ID
- generate production PDA fixtures
- change runtime deployability
- remove `PLACEHOLDER_PROGRAM_ID`
- enable live route activation
- enable SPL Token `mint_to` CPI
- modify mint authority behavior
- modify signer seeds
- deploy anything

## Why this matters

Program ID affects PDA derivation.

If the Program ID changes, every Program-ID-dependent PDA fixture must be regenerated and revalidated.

A partial Program ID migration could break:

- mint authority PDA assumptions
- gateway config PDA assumptions
- processed event PDA assumptions
- account contract assumptions
- SPL CPI authority assumptions
- Mollusk fixtures
- future deployment scripts

Therefore the Program ID transition must be isolated and reviewed.

## Required future inputs

Before a future Program ID stage can remove the blocker, the following inputs are required:

- selected real Program ID
- target network or environment label
- documented Program ID source
- PDA derivation inventory
- fixture regeneration process
- tests proving placeholder is absent from deployable paths
- tests proving PDA derivations match the selected Program ID
- review note confirming the Program ID boundary

## PDA derivation inventory

Future Program-ID-dependent fixtures must include every PDA used by the runtime or future CPI path.

At minimum, the inventory must review:

- gateway config PDA
- processed event PDA
- mint authority PDA
- any route/config PDA
- any proof-log PDA if introduced
- any guardian-set PDA if introduced
- any SPL mint authority PDA assumptions
- any account contract fixture that embeds Program ID assumptions

## Required evidence before removing blocker

The blocker may only be removed when the future checkpoint includes:

- real Program ID value
- reason for selected Program ID
- explicit statement that placeholder Program ID is no longer used in deployable paths
- regenerated PDA fixtures
- deterministic reproduction command for PDA fixtures
- test output proving fixture validity
- reviewer confirmation

## Required tests for future blocker removal

A future blocker-removal stage must prove:

- placeholder Program ID is still visible only in historical docs or explicit nondeployable status references
- deployable paths do not use the placeholder Program ID
- PDA fixtures are derived from the selected Program ID
- account validation still rejects invalid PDA accounts
- Mollusk negative account tests still pass
- predeploy gate does not pass unless all blockers are resolved

## Suggested future stage order

Recommended order:

1. Program ID inventory stage
2. PDA fixture derivation script or documented command stage
3. Program ID fixture regeneration stage
4. Program ID fixture verification stage
5. blocker-removal review stage

The blocker should not be removed before all required evidence exists.

## Interaction with other blockers

Removing `PLACEHOLDER_PROGRAM_ID` must not imply that any other blocker is resolved.

The following blockers must remain active unless separately resolved:

- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployment behavior was enabled.

No deployability predicate was changed.

No deployment blocker was removed.

The runtime remains scaffold-only and not deployable.

## Required hard checks

For future Program ID code/fixture stages:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Decision

The Program ID and PDA fixture readiness plan is a blocking preparation document.

The `PLACEHOLDER_PROGRAM_ID` blocker remains active.

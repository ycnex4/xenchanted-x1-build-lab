# XXXL Runtime Predeploy Readiness Checklist

Status: DRAFTED / BLOCKING.

This checklist defines the required gates before the XXXL SVM runtime can be considered deployable.

The current runtime remains:

    SCAFFOLD_ONLY_NOT_DEPLOYABLE

Current predeploy gate result:

    Blocked(report)

Current deploy allow value:

    false

## Rule

Deployment is allowed only when all of the following are true:

- `report.deployable == true`
- `report.blockers.is_empty() == true`
- live route activation has been intentionally enabled in a reviewed stage
- SPL Token mint_to CPI execution has been intentionally enabled in a reviewed stage
- Program ID and PDA fixtures are final
- guardian set and proof-log model are production-ready
- external review is complete

Until then, the runtime must remain blocked.

## Current blockers

### 1. PRODUCTION_PROGRAM_ID_UNSET

Status:

- `BLOCKED`

The previous blanket blocker `PLACEHOLDER_PROGRAM_ID` is retired for X1 testnet.

Current X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

X1 testnet Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Production remains blocked until:

- production Program ID is selected
- production PDA fixtures are regenerated
- production PDA fixtures are verified
- production authority model is recorded
- production release readiness is reviewed

This is network-aware.

X1 testnet deployment does not imply production identity readiness.

### 2. LIVE_ROUTE_DISABLED

Current meaning:

Live route activation from `process_instruction` remains disabled.

Required resolution:

Activate the live route only in a reviewed stage after all deployment blockers are resolved.

Required evidence before removal:

- deployment blockers resolved or intentionally superseded
- live route activation diff isolated in its own branch
- tests prove the previous disabled path remains impossible after activation conditions are checked
- negative tests prove invalid inputs still fail before mutation or CPI
- reviewer signs off on live route activation

### 3. SPL_CPI_EXECUTION_DISABLED

Current meaning:

SPL Token `mint_to` CPI execution remains disabled.

Required resolution:

Enable SPL Token `mint_to` CPI execution only after live route activation, PDA authority, account contract, and Mollusk coverage are complete.

Required evidence before removal:

- PDA authority validated against real Program ID
- account contract enforced and covered
- SPL mint and recipient token account validation covered
- Mollusk positive and negative CPI coverage completed
- no unauthorized signer or writable-account expansion
- no mint path bypasses gateway authorization assumptions

### 4. PRODUCTION_GUARDIAN_SET_UNSET

Current meaning:

The production guardian set is not configured or externally documented.

Required resolution:

Define, publish, and review the production guardian set, threshold, rotation policy, and key custody model.

Required evidence before removal:

- guardian keys defined
- threshold defined
- bootstrap vs production trust model disclosed
- key custody model documented
- rotation policy documented
- emergency replacement policy documented
- watcher/operator assumptions documented

### 5. PRODUCTION_PROOF_LOG_UNSET

Current meaning:

The production proof-log and public audit trail are not configured.

Required resolution:

Define the production proof-log format, retention policy, public audit trail, and operator publication flow.

Required evidence before removal:

- proof-log schema defined
- canonical event key included
- Ethereum burn evidence included
- X1 mint evidence included
- guardian approvals included or linkable
- retention policy documented
- public audit publication flow documented

### 6. EXTERNAL_REVIEW_INCOMPLETE

Current meaning:

External review is not complete for deployment activation.

Required resolution:

Complete external review of the live route, guardian policy, CPI path, account contract, replay protection, and deployment checklist.

Required evidence before removal:

- review scope defined
- reviewer notes archived
- open findings resolved or explicitly accepted
- live route reviewed
- CPI path reviewed
- account contract reviewed
- replay protection reviewed
- deployment checklist reviewed

## Non-negotiable safety requirements

Before any deployable runtime exists:

- no placeholder Program ID in deployable path
- no undocumented guardian set
- no undocumented proof-log
- no live route activation without isolated review
- no SPL CPI execution without isolated review
- no minting path without replay protection
- no deployability predicate set to true while blockers remain
- no hidden or implicit activation switch

## Required hard checks

Before a blocker-removal stage can be accepted, the following checks must pass:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Current decision

The runtime is not deployable.

The predeploy gate must remain blocked until this checklist is completed through separate reviewed stages.

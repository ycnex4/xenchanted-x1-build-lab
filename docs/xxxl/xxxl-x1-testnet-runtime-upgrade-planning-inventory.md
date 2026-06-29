# XXXL X1 Testnet Runtime Upgrade Planning Inventory

Status: Docs stage complete — all runtime blockers remain active
Branch: `stage-xxxl-x1-testnet-runtime-upgrade-planning-inventory`
Base: `e247831 Clean up X1 testnet blocker inventory summary`

## Purpose

This document records the runtime upgrade planning inventory after the first X1 testnet deployment and blocker model transition.

This is a docs-only planning stage.

It does not implement runtime code.

It does not execute an upgrade.

It does not submit a transaction.

It does not spend SOL.

It does not enable live gateway execution.

It does not enable SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

It does not remove any blocker.

## Current X1 testnet identity

Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Current upgrade authority:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Current X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

## Active blockers

The current active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Existing documentation inputs

This planning inventory is based on existing repository documentation:

- `docs/gateway/stage-2-0-gateway-runtime-planning-outline.md`
- `docs/gateway/stage-2-2-direct-mint-candidate-runtime-design.md`
- `docs/gateway/stage-2-3-claim-based-candidate-runtime-design.md`
- `docs/xxxl/xxxl-runtime-candidate-account-instruction-schema.md`
- `docs/xxxl/xxxl-runtime-candidate-transition-semantics.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/xxxl/xxxl-runtime-program-skeleton.md`
- `docs/xxxl/xxxl-x1-svm-program-skeleton.md`
- `docs/xxxl/xxxl-svm-runtime-decoder-handler-model.md`
- `docs/xxxl/xxxl-svm-runtime-port-readiness-package.md`
- `docs/xxxl/xxxl-x1-testnet-blocker-model-transition.md`

Forge / Stake boundary reference:

- `docs/x1-native/x1-forge-stake-dual-nominal-model.md`

This Forge / Stake document is referenced only to confirm that X1-native Forge / Stake economics are outside gateway runtime scope.

No Forge / Stake mechanic is included in this planning inventory.

No Forge / Stake mechanic may justify opening gateway mint execution.

## Runtime upgrade implementation boundary

A runtime upgrade implementation boundary is a docs-only gate that defines exactly what the first runtime implementation branch may and may not do.

It must define:

- allowed files / modules
- allowed account/state changes
- allowed instruction decoding changes
- required validation checks
- required tests
- forbidden live behavior
- forbidden SPL CPI behavior
- forbidden `invoke_signed`
- forbidden SPL Token `mint_to`
- external review requirements before any on-chain upgrade

The boundary is not a code upgrade.

The boundary is not an on-chain program upgrade.

The boundary is not blocker removal.

## Recommended future upgrade stages

1. Runtime upgrade implementation boundary.

   Define exactly what the first code branch may and may not implement.

   This stage is docs-only.

2. Account/state implementation plan.

   This is a docs-only planning stage.

   Map the existing account model into proposed Rust/SVM account structs and validation rules without changing code.

3. Instruction decode and validation plan.

   This is a docs-only planning stage.

   Map `CONSUME_GATEWAY_MINT` into proposed SVM instruction decoding and validation rules without changing code.

4. Replay / processed event implementation plan.

   This stage must define minimum replay requirements:

   - canonicalEventKey handling
   - duplicate canonicalEventKey rejection
   - processed mark atomicity
   - no result without processed mark
   - no processed mark without result
   - route replay rejection
   - coefficient version replay rejection
   - guardian set version replay rejection
   - pause/unpause replay rejection
   - upgrade replay rejection
   - source fork replay protection

5. Stage 1 authorization consumer mapping.

   Preserve the boundary that Stage 1 verifies and authorizes while XXXL runtime consumes only the approved authorization result.

6. Local runtime skeleton implementation branch.

   This may be a local code branch only.

   It must not be deployed on-chain.

   It must not execute a program upgrade.

   It must not enable live route execution.

   It must not enable SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

7. External review gate before any on-chain program upgrade.

   `EXTERNAL_REVIEW_INCOMPLETE` must remain active until this gate is completed.

   This gate must define completion criteria, including:

   - reviewer identity or reviewer role
   - review scope
   - required output artifacts
   - finding severity categories
   - finding resolution rules
   - final accept / blocked decision format

   This gate is mandatory and non-skippable before any on-chain runtime upgrade.

8. Live route gate design.

   Define the exact condition under which `LIVE_ROUTE_DISABLED` may later be removed.

   This stage does not remove the blocker.

9. SPL CPI / mint_to gate design.

   Define the exact condition under which `SPL_CPI_EXECUTION_DISABLED` may later be removed.

   This stage does not remove the blocker.

10. First guarded testnet on-chain program upgrade.

   This is the first possible on-chain code upgrade stage.

   It may occur only after completion and acceptance of Stages 1 through 9.

   It may occur only after the external review gate is completed.

   It must include explicit upgrade evidence.

   Explicit upgrade evidence means:

   - pre-upgrade Program ID
   - pre-upgrade ProgramData address
   - pre-upgrade upgrade authority
   - artifact path
   - artifact size
   - artifact SHA-256
   - upgrade command or procedure reference
   - upgrade transaction signature
   - upgrade slot
   - post-upgrade ProgramData address
   - post-upgrade upgrade authority
   - post-upgrade deployed data length
   - read-only post-upgrade verification
   - explicit statement that live route remains disabled
   - explicit statement that SPL CPI remains disabled

   It must still keep live route and SPL CPI disabled unless separate later blocker-removal stages explicitly authorize them.

11. Later live gateway activation stage.

   This is separate and must not be bundled with the first on-chain code upgrade.

## Persistent non-goals

The following must not occur at any stage until separately and explicitly authorized with evidence:

- live gateway minting
- removal of `LIVE_ROUTE_DISABLED`
- removal of `SPL_CPI_EXECUTION_DISABLED`
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian configuration
- production proof-log configuration
- authority freeze
- production readiness claim
- final immutability claim while upgrade authority exists
- merging Forge / Stake economics into gateway mint execution

## Required evidence before any on-chain code upgrade

Before any on-chain testnet program upgrade, the following must exist:

- external review checkpoint
- account/state validation plan
- instruction decoding plan
- replay / processed event plan
- negative test plan
- rollback behavior test plan
- Mollusk/SVM coverage plan
- upgrade evidence procedure
- explicit statement that live route remains disabled
- explicit statement that SPL CPI remains disabled
- explicit reviewer / validator for prerequisite sufficiency
- explicit go / no-go checkpoint before upgrade execution

## Inventory limits

This document does not claim that all listed inputs have been fully cross-reviewed for consistency.

This document does not claim implementation readiness.

This document does not claim deployment readiness.

This document only records the next safe planning order after X1 testnet scaffold deployment.

## Result

The next useful step is a runtime upgrade implementation boundary.

That boundary should define the first safe local implementation branch after testnet deployment while keeping the deployed program locked and non-live.

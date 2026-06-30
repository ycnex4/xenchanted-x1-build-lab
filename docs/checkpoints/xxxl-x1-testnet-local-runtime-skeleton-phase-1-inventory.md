# XXXL X1 Testnet Local Runtime Skeleton Phase 1 Inventory

Status: Docs-only inventory complete — all runtime blockers remain active
Branch: `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory`
Base: `a7fb50e Add X1 testnet local runtime skeleton implementation plan`

## Purpose

This checkpoint records Phase 1 inventory for the XXXL X1 testnet local runtime skeleton implementation plan.

This is a docs-only inventory stage.

It does not implement runtime code.

It does not edit runtime source files.

It does not edit tests.

It does not deploy or upgrade any program.

It does not submit transactions.

It does not spend SOL.

It does not enable live gateway execution.

It does not enable SPL CPI.

It does not add `invoke_signed`.

It does not add SPL Token `mint_to`.

Note: existing dormant `cpi.rs` helper functions containing `invoke_signed` and SPL Token `mint_to` were observed and are recorded in the Existing CPI Boundary Observation section.

Those helpers remain unreachable from the currently enabled executable entrypoint path.

It does not remove any blocker.

It does not claim production readiness.

It does not claim immutability while upgrade authority exists.

## Input Documents Read

- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-planning-inventory.md`

## Source File Inventory

Current `programs/xxxl-svm/src/**` files:

- `programs/xxxl-svm/src/account_contract.rs` records the 9-account `consume_gateway_mint` account contract, account indices, writable flags, signer requirements, and owner model categories.
- `programs/xxxl-svm/src/cpi.rs` records existing CPI planning and disabled execution-gate boundaries. It includes future-boundary helpers for SPL Token `mint_to` instruction construction and `invoke_signed`, but `spl_mint_to_cpi_execution_enabled()` returns `false` and the process-instruction route does not enable the CPI path.
- `programs/xxxl-svm/src/deployment_status.rs` records the scaffold-only deployment status, non-deployable gate, active deployment blocker reports, live-route-disabled status, and SPL-CPI-disabled status.
- `programs/xxxl-svm/src/entrypoint.rs` registers `processor::process_instruction` as the Solana entrypoint.
- `programs/xxxl-svm/src/error.rs` defines the current `XxxlError` custom error model.
- `programs/xxxl-svm/src/execution_plan.rs` defines the local atomic execution-plan model, processed-event mutation boundary, recipient-balance mutation boundary, and disabled live-route flags.
- `programs/xxxl-svm/src/instruction.rs` defines the `CONSUME_GATEWAY_MINT` instruction discriminator, version, fixed 208-byte layout, encoded account-meta count, account index fields, route/guardian/mint/event/recipient fields, amount, and source-chain weight.
- `programs/xxxl-svm/src/lib.rs` exports the runtime modules and still declares scaffold-only status and placeholder Program ID constants.
- `programs/xxxl-svm/src/pda.rs` defines the `gateway_mint_authority` PDA seeds, derivation inventory, fixture report model, and fixture verification helpers.
- `programs/xxxl-svm/src/processor.rs` defines the entrypoint handler, 9-account processor indices, disabled live-route constant, preparation/validation boundary, planning composition boundary, local state mutation composition boundary, and disabled SPL CPI gate boundary.
- `programs/xxxl-svm/src/program_id_status.rs` records Program ID readiness and placeholder boundary reporting.
- `programs/xxxl-svm/src/safety_invariants.rs` records safety lock, predeploy gate, live-route, SPL-CPI, deployment-blocker, unlock, and release-decision consistency checks.
- `programs/xxxl-svm/src/state.rs` defines local account layout views for mint state, gateway config, guardian set, processed event, and recipient balance, plus local processed-event and recipient-balance mutation helpers.
- `programs/xxxl-svm/src/validation.rs` defines account owner, rent exemption, SPL token mint, and recipient token account validation helpers.

Phase 1 made no changes to any source file.

## Test File Inventory

Current `programs/xxxl-svm/tests/**` files:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs` is the current Mollusk/SBF entrypoint coverage file for `consume_gateway_mint`.

Observed Mollusk entrypoint coverage categories include:

- malformed instruction rejection without live route
- discriminator/version/length strictness
- encoded account meta count and encoded account index rejection
- wrong account count and wrong account order rejection
- signer and writable/readonly account contract rejection
- program-owned account owner rejection
- account discriminator/truncation rejection
- rent-exemption rejection paths
- SPL token mint owner, authority, and initialized-state rejection
- recipient token account mint/owner/initialized-state rejection
- mint authority PDA and bump rejection
- valid scaffold entrypoint no-mutation behavior
- processed-event replay and processed-event relationship rejection
- recipient-balance owner and mint rejection
- no-mutation checks for selected rejection paths

No ignored tests were observed in `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`.

Phase 1 made no changes to any test file.

## Current Disabled-Route Behavior Summary

The current entrypoint path is:

`process_instruction` -> `process_consume_gateway_mint` -> `build_runtime_consume_gateway_mint_execution_plan_boundary`.

The route decodes and validates `CONSUME_GATEWAY_MINT`, obtains `Rent` and `Clock`, prepares the CPI boundary, and builds an execution plan.

`LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED` is `false`.

Execution plans produced by the boundary set:

- `live_route_activation_enabled: false`
- `mint_to_invocation_from_process_instruction_enabled: false`

For a valid scaffold entrypoint call, `process_consume_gateway_mint` logs that the execution plan was built and that live route execution is not activated, then returns without invoking SPL CPI.

The current live route does not mint.

The current live route does not submit CPI.

The current live route does not call `invoke_signed`.

The current live route does not call SPL Token `mint_to`.

The disabled SPL CPI gate boundary exists as a separate internal boundary and returns `CpiBoundaryNotReady` while `spl_mint_to_cpi_execution_enabled()` remains `false`.

## Current Error Model Summary

`XxxlError` maps to `ProgramError::Custom(error as u32)`.

Current custom error codes:

- `InvalidInstruction = 1`
- `InvalidAccountOwner = 2`
- `InvalidRentExemption = 3`
- `InvalidRecipientAta = 4`
- `InvalidPda = 5`
- `InvalidDiscriminator = 6`
- `InvalidVersion = 7`
- `CpiBoundaryNotReady = 8`

Current boundary usage includes:

- instruction length, account meta, index, relationship, zero amount, overflow, and replay failures returning `InvalidInstruction`
- wrong owner returning `InvalidAccountOwner`
- low-rent accounts returning `InvalidRentExemption`
- recipient token account / recipient balance relationship failures returning `InvalidRecipientAta`
- gateway mint authority PDA and mint authority mismatches returning `InvalidPda`
- wrong instruction or account discriminator returning `InvalidDiscriminator`
- wrong layout version returning `InvalidVersion`
- disabled CPI execution gate returning `CpiBoundaryNotReady`

## Current Account And Instruction Boundary Summary

`consume_gateway_mint` currently has a 9-account contract:

| Index | Account | Access | Signer | Owner model |
| --- | --- | --- | --- | --- |
| 0 | `mint_state` | readonly | not signer | program-owned |
| 1 | `gateway_config` | readonly | not signer | program-owned |
| 2 | `guardian_set` | readonly | not signer | program-owned |
| 3 | `processed_event` | writable | not signer | program-owned |
| 4 | `recipient_balance` | writable | not signer | program-owned |
| 5 | `spl_token_mint` | writable | not signer | SPL Token-owned |
| 6 | `recipient_token_account` | writable | not signer | SPL Token-owned |
| 7 | `mint_authority_pda` | readonly | not signer | PDA |
| 8 | `token_program` | readonly | not signer | SPL Token program |

The instruction boundary currently requires:

- exact `CONSUME_GATEWAY_MINT_INSTRUCTION_LEN = 208`
- exact 8-byte discriminator
- `INSTRUCTION_LAYOUT_VERSION = 1`
- `CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT = 9`
- encoded route account index `1`
- encoded guardian set account index `2`
- encoded mint state account index `0`
- encoded processed event account index `3`
- encoded recipient balance account index `4`
- route ID, guardian set ID, mint ID, canonical event key, recipient, amount, and source-chain weight fields

The preparation boundary currently validates:

- account count and encoded account meta count
- account contract flags
- token program ID equals `spl_token::id()`
- program-owned account owner equals current program ID
- rent exemption for program-owned and SPL token accounts
- account layout discriminator/version/length for local program accounts
- mint state target mint and gateway mint authority PDA relationship
- gateway config route, guardian set, target mint, and source-chain weight relationships
- guardian set ID relationship
- processed event not consumed and matching canonical event key, route, and recipient
- recipient balance owner and mint relationships
- initialized SPL mint and expected mint authority
- initialized recipient token account with expected owner and mint
- nonzero amount and amount within SPL Token `u64` range

## Existing CPI Boundary Observation

Phase 1 inventory found existing `cpi.rs` code containing `spl_token::instruction::mint_to` and `invoke_signed` inside future-boundary helper functions.

This stage did not add or edit those functions.

The enabled `process_instruction` live route still does not call `invoke_signed`.

The enabled `process_instruction` live route still does not call SPL Token `mint_to`.

Within the currently enabled executable entrypoint path, `invoke_signed` remains absent.

Within the currently enabled executable entrypoint path, SPL Token `mint_to` remains absent.

`spl_mint_to_cpi_execution_enabled()` remains `false`.

`SPL_CPI_EXECUTION_DISABLED` remains active.

Any future change that makes the existing CPI helper reachable from the live route requires a separate reviewed boundary and explicit blocker transition.

## Phase 2 Follow-up Requirements

Phase 2 account layout reconciliation must explicitly record:

- whether `cpi.rs` helper functions have any call sites outside the enabled `process_instruction` path
- whether any tests, benches, or utilities call the dormant CPI helpers directly
- current Mollusk test count and latest run result
- whether coefficient version replay rejection is currently covered
- whether guardian set version replay rejection is currently covered
- whether pause/unpause replay rejection is currently covered
- whether upgrade replay rejection is currently covered
- what `build_runtime_consume_gateway_mint_execution_plan_boundary` returns beyond disabled-route and disabled-CPI flags

Until those follow-ups are resolved or explicitly deferred, no on-chain upgrade may be planned from this inventory.

## Safety Confirmation

Phase 1 made no runtime code changes.

Phase 1 changed no tests.

Phase 1 did not deploy.

Phase 1 did not upgrade.

Phase 1 did not submit transactions.

Phase 1 did not spend SOL.

Phase 1 did not touch `.local-keys/**`.

Phase 1 did not touch keypair JSON files.

Phase 1 did not touch `.env`.

Phase 1 did not touch `target/deploy/**`.

Phase 1 did not add or commit `.so` artifacts.

Phase 1 did not add deployment scripts.

Phase 1 did not add upgrade scripts.

Phase 1 did not add CI/CD workflows.

The local runtime skeleton remains non-deployed, non-live, and unable to mint through the currently enabled executable entrypoint path.

Dormant CPI helper functions exist in `cpi.rs`, but they are not reachable from the enabled route.

The already-recorded X1 testnet scaffold status is not changed by this docs-only inventory.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Active Blocker Confirmation

The implementation-plan and upgrade-boundary documents keep these blockers active:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

The current source-level deployment report remains unchanged by Phase 1 and still records:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This inventory does not transition either naming model.

## Next Recommended Stage

Recommended next stage:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation`

That stage should reconcile the implementation plan's Phase 2 account layout objective with the account layout structures already present in `state.rs`, `account_contract.rs`, `processor.rs`, and the Mollusk coverage file before any additional runtime code is edited.

The next stage must remain non-deployed, non-live, unable to mint, and must keep `LIVE_ROUTE_DISABLED` and `SPL_CPI_EXECUTION_DISABLED` active unless a separate reviewed boundary explicitly authorizes otherwise.

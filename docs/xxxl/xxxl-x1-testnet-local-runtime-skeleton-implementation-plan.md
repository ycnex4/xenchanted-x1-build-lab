# XXXL X1 Testnet Local Runtime Skeleton Implementation Plan

Status: Docs-only implementation plan — all runtime blockers remain active
Branch: `stage-xxxl-x1-testnet-local-runtime-skeleton-implementation-plan`
Base: `fa6de99 Add X1 testnet runtime upgrade implementation boundary`

## Purpose

This document defines the plan for the first future local runtime skeleton implementation branch.

This is a docs-only planning stage.

It does not implement runtime code.

It does not execute an upgrade.

It does not submit a transaction.

It does not spend SOL.

It does not enable live gateway execution.

It does not enable SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

It does not remove any blocker.

It does not claim production readiness.

It does not claim immutability while upgrade authority exists.

## Current X1 testnet baseline

Current X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData address:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Gateway mint authority PDA seeds:

- `b"xxxl"`
- `b"gateway-mint-authority"`
- `b"v1"`

Gateway mint authority PDA bump:

- `252`

Current upgrade authority:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

## Active blockers

The following blockers remain active:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

This plan does not retire, rename, weaken, or remove any blocker.

## Input documents

This plan follows the boundary and planning documents already recorded:

- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-planning-inventory.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-runtime-candidate-account-instruction-schema.md`
- `docs/xxxl/xxxl-runtime-candidate-transition-semantics.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/xxxl/xxxl-runtime-program-skeleton.md`
- `docs/xxxl/xxxl-x1-svm-program-skeleton.md`
- `docs/xxxl/xxxl-svm-runtime-decoder-handler-model.md`
- `docs/xxxl/xxxl-svm-runtime-port-readiness-package.md`
- `docs/xxxl/xxxl-x1-testnet-blocker-model-transition.md`

## Future local implementation objective

The future local implementation branch may begin turning the runtime skeleton model into local Rust/SVM code.

The future branch must remain:

- local
- non-deployed
- non-live
- non-upgraded
- unable to mint
- unable to execute SPL CPI
- unable to call `invoke_signed`
- unable to call SPL Token `mint_to`

The objective is to improve local runtime structure and tests without changing deployed behavior.

## Planned local implementation phases

### Phase 1 — Existing runtime source inventory

Inspect current `programs/xxxl-svm` source and test structure.

Expected output:

- source file inventory
- test file inventory
- current disabled-route behavior summary
- current error model summary
- current account/instruction boundary summary

Phase 1 outputs must be recorded as checkpoint documents in `docs/checkpoints/**` before Phase 2 may begin.

No code changes are required by this phase.

### Phase 2 — Account layout skeleton

Define local Rust/SVM account layout structures for the model account kinds:

- Mint State
- Gateway Config
- Guardian Set
- Processed Event
- Recipient Balance

`Recipient Balance` represents a local model-level accounting structure only.

It is not an SPL token account.

It is not the recipient ATA.

It must not manage actual token balances.

Actual recipient token balance is managed by the recipient SPL token account / ATA, which is not in scope of this implementation branch.

Expected local implementation constraints:

- account discriminator checks
- account version checks
- account kind checks
- account owner checks
- length checks
- relationship checks

This phase must not add live route execution.

This phase must not add SPL CPI code.

### Phase 3 — Instruction decode skeleton

Define local decoding for `CONSUME_GATEWAY_MINT`.

Expected local implementation constraints:

- instruction discriminator check
- instruction version check
- exact byte-length or bounded-length validation
- canonical field order validation
- amount validation
- recipient field validation
- event-key field validation

This phase must not make the instruction executable as a live mint.

### Phase 4 — Validation and error model

Define local validation and error mapping.

Expected local implementation constraints:

- deterministic error variants
- no partial state mutation on validation failure
- route-disabled error
- SPL-CPI-disabled error
- replay error
- invalid account error
- invalid instruction error
- invalid relationship error

This phase must preserve clear rejection behavior for all disabled live paths.

### Phase 5 — Stage 1 authorization consumer modeling

Model the boundary between Stage 1 authorization and XXXL runtime consumption.

Expected local implementation constraints:

- Stage 1 verifies gateway message correctness.
- Stage 1 verifies guardian authorization.
- Stage 1 verifies source replay protection.
- XXXL runtime consumes only an approved authorization result.
- XXXL runtime performs local account, recipient, mint, route, and processed-event checks.
- XXXL runtime must not silently expand Stage 1 responsibility.
- XXXL runtime must not bypass Stage 1 authorization.
- This phase must not claim production authorization readiness.

This phase is local-model only.

### Phase 6 — Disabled processor control flow

Define local processor control flow for the future runtime skeleton.

The processor may decode and validate.

The processor must stop before live mint execution.

The processor must return a disabled-route or equivalent safety error before any mint path.

The processor must not write to any account before returning a disabled-route error.

This phase must not contain executable SPL CPI code.

This phase must not contain executable `invoke_signed`.

This phase must not contain executable SPL Token `mint_to`.

This phase must not contain flag-guarded SPL CPI code.

This phase must not contain feature-gated SPL CPI code.

This phase must not contain test-only SPL CPI code.

### Phase 7 — Replay and processed-event local model

Define local replay and processed-event validation.

Expected local implementation constraints:

- canonicalEventKey handling
- duplicate canonicalEventKey rejection
- processed event account relationship checks
- no result without processed mark
- no processed mark without result
- no recipient accounting update without processed mark
- no supply accounting update without processed mark
- no processed mark if validation fails
- route replay rejection
- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay rejection

While SPL CPI remains disabled, mint/supply accounting refers only to local model-level state.

Actual on-chain token supply must not be updated.

### Phase 8 — Local tests

Add or update local tests only.

Minimum expected test classes:

- valid disabled-route decode path
- invalid instruction discriminator
- invalid instruction version
- invalid account discriminator
- invalid account version
- wrong account owner
- wrong account kind
- wrong account order
- wrong Program ID relationship
- wrong mint account relationship
- wrong recipient relationship
- wrong processed event relationship
- duplicate processed event
- amount zero
- amount overflow
- route disabled
- SPL CPI disabled
- `invoke_signed` absent from executable code
- SPL Token `mint_to` absent from executable code
- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay rejection
- failed validation leaves state unchanged
- processed mark cannot advance on failure
- recipient accounting cannot advance on failure
- supply accounting cannot advance on failure

The future implementation branch must not proceed toward on-chain upgrade without a later explicit Mollusk/SVM coverage checkpoint.

Minimum Mollusk/SVM coverage checkpoint content must include:

- list of covered instruction paths
- list of covered account-validation paths
- list of covered replay paths
- list of covered disabled-route paths
- list of covered SPL-CPI-disabled paths
- list of negative tests
- rollback / no-state-change evidence
- explicit statement that no on-chain upgrade is authorized
- explicit statement that live route remains disabled
- explicit statement that SPL CPI remains disabled

## Forbidden implementation content

The future local implementation branch must not include:

- `invoke_signed` in executable runtime code
- SPL Token `mint_to` in executable runtime code
- SPL CPI calls in executable runtime code
- flag-guarded SPL CPI calls
- feature-gated SPL CPI calls
- test-only SPL CPI calls
- a reachable or pre-staged mint path
- deployment commands
- upgrade commands
- transaction submission
- SOL spending
- deployment CI/CD workflow
- upgrade CI/CD workflow
- production guardian keys
- production proof-log configuration
- production Program ID configuration
- authority freeze logic
- production readiness claims
- final immutability claims while upgrade authority exists

## Allowed file categories for the future implementation branch

A future local implementation branch may touch:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/**`
- `programs/xxxl-svm/Cargo.toml` only if needed for local tests
- `programs/xxxl-svm/Cargo.lock` only if needed for local dependency resolution
- `docs/xxxl/**`
- `docs/checkpoints/**`

Any change outside these categories requires a separate boundary update.

## Forbidden file categories

A future local implementation branch must not touch or commit:

- `.local-keys/**`
- keypair JSON files
- wallet files
- `.env`
- `target/deploy/**`
- generated `.so` artifacts
- deployment scripts that submit transactions
- upgrade scripts that submit transactions
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- production configuration files

## Non-deployable requirement

The future local implementation branch is non-deployable only if all of the following remain true:

- no deployment command is executed
- no upgrade command is executed
- no transaction is submitted
- no SOL is spent
- no deployment or upgrade CI/CD workflow is added
- no generated `.so` artifact is committed
- no `target/deploy` artifact is committed
- no keypair or wallet material is committed
- live route remains disabled
- SPL CPI remains disabled
- `invoke_signed` remains absent from executable runtime code
- SPL Token `mint_to` remains absent from executable runtime code

If any of these conditions fails, the future branch exits this plan boundary.

## Phase completion criteria

Before moving from one phase to the next, the branch must record:

- what changed
- which files changed
- which tests were added or updated
- which boundary constraints were checked
- which items were deferred
- why the branch remains non-deployable

Phase completion must be recorded in `docs/checkpoints/**`.

## Review plan for the future implementation branch

Before the future local implementation branch is merged, review must check:

- all blockers remain active
- live route remains disabled
- SPL CPI remains disabled
- `invoke_signed` remains absent from executable runtime code
- SPL Token `mint_to` remains absent from executable runtime code
- no transaction was submitted
- no SOL was spent
- no deployment or upgrade CI/CD workflow was added
- no keypair or wallet material was committed
- no production readiness claim was introduced
- no final immutability claim was introduced while upgrade authority exists
- all implemented safety boundaries have local tests

## Out of scope

This plan does not authorize:

- on-chain program upgrade
- live route activation
- SPL CPI implementation
- `invoke_signed`
- SPL Token `mint_to`
- production guardian configuration
- production proof-log configuration
- production Program ID configuration
- authority freeze
- production release
- public immutability claim

## Result

The next possible step after this docs-only plan is a future local runtime skeleton implementation branch.

That future branch must remain non-deployed, non-live, and unable to mint.

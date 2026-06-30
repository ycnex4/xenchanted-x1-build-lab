# XXXL X1 Testnet Runtime Upgrade Implementation Boundary

Status: Docs-only boundary — all runtime blockers remain active
Branch: `stage-xxxl-x1-testnet-runtime-upgrade-implementation-boundary`
Base: `1a0e2fa Add X1 testnet runtime upgrade planning inventory`

## Purpose

This document defines the implementation boundary for the first future local runtime implementation branch after the X1 testnet scaffold deployment.

This is a docs-only boundary stage.

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

The PDA seeds and bump are recorded here only to preserve consistency for future local implementation checks.

They are not secrets.

They do not authorize deployment, minting, or route execution.

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

This boundary does not retire, rename, weaken, or remove any blocker.

## Boundary definition

The first future local runtime implementation branch may only implement code that is consistent with this boundary.

That future branch must remain local and non-deployed.

It must not execute a testnet program upgrade.

It must not activate any live route.

It must not call SPL Token `mint_to`.

It must not call `invoke_signed`.

It must not perform SPL CPI.

It must not make the deployed X1 testnet program live.

It must not change production readiness status.

It must not freeze authority.

It must not imply final immutability.

## Allowed future local implementation scope

A future local implementation branch may work on:

- Rust/SVM account data structures
- account discriminator checks
- account version checks
- account owner checks
- account kind checks
- instruction discriminator checks
- instruction version checks
- `CONSUME_GATEWAY_MINT` decoding
- input length checks
- amount range checks
- recipient account relationship checks
- mint account relationship checks
- processed event account relationship checks
- local replay-state validation logic
- local error types
- local processor control flow
- local tests
- local fixtures
- docs for implemented local boundaries

The allowed work is limited to local implementation and local validation.

## Forbidden future local implementation scope

A future local implementation branch must not:

- deploy to X1 testnet
- execute `solana program deploy`
- execute `solana program upgrade`
- submit any transaction
- spend SOL
- modify or expose keypair material
- modify `.local-keys`
- commit keypair files
- commit `.env` files
- commit generated `.so` artifacts
- commit `target/deploy`
- change the selected X1 testnet Program ID
- change the selected gateway mint authority PDA seeds
- change the selected gateway mint authority PDA bump without a dedicated PDA transition stage
- remove `LIVE_ROUTE_DISABLED`
- remove `SPL_CPI_EXECUTION_DISABLED`
- add a live route
- activate route execution
- add production guardian keys
- add production proof-log configuration
- add production Program ID
- claim production readiness
- claim final immutability

## SPL CPI boundary

The future local implementation branch must keep SPL CPI disabled.

It may define types, comments, or placeholder boundaries describing the future SPL Token mint path.

It must not execute SPL Token CPI.

It must not call SPL Token `mint_to`.

It must not introduce `invoke_signed` execution.

It must not include a reachable code path that mints tokens.

Disabled route scaffolding must not contain `invoke_signed`, SPL CPI calls, or SPL Token `mint_to` calls, even in flag-guarded, feature-gated, test-only, or currently unreachable paths.

SPL CPI code may not exist in the future local implementation branch in any executable form until a separate explicitly authorized SPL CPI stage.

Any future SPL CPI implementation requires a separate stage after:

- account contract evidence
- recipient token account validation
- SPL mint account validation
- mint authority PDA signer-seed evidence
- negative test coverage
- rollback behavior tests
- external review
- explicit blocker transition for `SPL_CPI_EXECUTION_DISABLED`

## Live route boundary

The future local implementation branch must keep live gateway route execution disabled.

It may implement validation scaffolding for a disabled route.

It must not activate a real route.

It must not accept a real gateway mint as executable.

It must not produce a successful live mint result.

Any future live route activation requires a separate stage after:

- route policy review
- guardian policy review
- finality policy review
- replay policy review
- external review
- explicit blocker transition for `LIVE_ROUTE_DISABLED`

## Stage 1 authorization consumer boundary

The future local implementation branch must preserve the existing trust boundary:

- Stage 1 verifies gateway message correctness.
- Stage 1 verifies guardian authorization.
- Stage 1 verifies source replay protection.
- XXXL runtime consumes only an approved authorization result.
- XXXL runtime performs local account, recipient, mint, route, and processed-event checks.
- XXXL runtime must not silently expand Stage 1 responsibility.
- XXXL runtime must not bypass Stage 1 authorization.

A local implementation may model the consumer boundary, but it must not claim production authorization readiness.

## Replay and processed-event boundary

The future local implementation branch must treat replay protection as a core safety boundary.

At minimum, local logic and tests must cover:

- canonicalEventKey handling
- duplicate canonicalEventKey rejection
- processed event account derivation or relationship checks
- no result without processed mark
- no processed mark without result
- no recipient balance update without processed mark
- no supply update without processed mark
- no processed mark if validation fails
- route replay rejection
- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay protection model

If any of these are deferred, the branch must remain non-deployable and the deferral must be recorded.

## Atomicity boundary

The future local implementation branch must preserve the model-level atomicity rule:

Successful execution means all required state transitions occur together.

Failure means no required state transition occurs.

The required future transition set is:

- local processed event marked consumed
- recipient accounting updated
- mint/supply accounting updated

While SPL CPI remains disabled, `mint/supply accounting` refers to local model-level state only.

Actual on-chain token supply must not be updated until SPL CPI is explicitly enabled in a separate authorized stage.

While SPL CPI remains disabled, token minting must remain unreachable.

A local implementation may model the transition flow, but it must not create a reachable mint path.

## Test boundary

The future local implementation branch must include tests for every implemented safety boundary.

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
- `invoke_signed` unreachable
- `mint_to` unreachable
- failed validation leaves state unchanged
- source fork replay rejection
- processed mark cannot advance on failure
- recipient accounting cannot advance on failure
- supply accounting cannot advance on failure

The branch must not proceed toward on-chain upgrade without a later explicit Mollusk/SVM coverage checkpoint.

## Files and modules boundary

A future local implementation branch may touch runtime source and test files only if the changes remain inside the disabled, non-live boundary.

Allowed categories:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/**`
- `programs/xxxl-svm/Cargo.toml` only if needed for local tests
- `programs/xxxl-svm/Cargo.lock` only if needed for local dependency resolution
- `docs/xxxl/**`
- `docs/checkpoints/**`

Forbidden categories:

- `.local-keys/**`
- `target/deploy/**`
- generated `.so` artifacts
- wallet files
- keypair JSON files
- `.env`
- deployment scripts that submit transactions
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- production configuration files

Any future change outside the allowed categories requires a separate boundary update.

CI/CD changes require separate review.

Any CI/CD workflow that can deploy, upgrade, submit a transaction, or spend SOL is forbidden in the future local implementation branch.

## Review boundary

The future local implementation branch must be reviewed before any on-chain upgrade.

The external review gate remains mandatory and non-skippable.

The review must check:

- live route remains disabled
- SPL CPI remains disabled
- `invoke_signed` remains unreachable
- SPL Token `mint_to` remains unreachable
- no transaction was submitted
- no SOL was spent
- no blocker was removed without explicit blocker-transition evidence
- no production readiness claim was introduced
- no final immutability claim was introduced while upgrade authority exists

## Non-deployable branch definition

A future local implementation branch is considered non-deployable only if all of the following remain true:

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
- `invoke_signed` remains absent from executable code
- SPL Token `mint_to` remains absent from executable code

If any of these conditions fails, the branch must be treated as outside this boundary.

## Minimum upgrade evidence procedure

A later on-chain upgrade stage must define and provide explicit upgrade evidence.

Minimum upgrade evidence must include:

- pre-upgrade Program ID
- pre-upgrade ProgramData address
- pre-upgrade upgrade authority
- pre-upgrade deployed data length
- artifact path
- artifact size
- artifact SHA-256
- upgrade command or procedure reference
- upgrade transaction signature
- upgrade slot
- post-upgrade Program ID
- post-upgrade ProgramData address
- post-upgrade upgrade authority
- post-upgrade deployed data length
- read-only post-upgrade verification
- explicit statement that live route remains disabled
- explicit statement that SPL CPI remains disabled
- explicit statement that `invoke_signed` remains absent from executable code
- explicit statement that SPL Token `mint_to` remains absent from executable code

This boundary does not authorize that upgrade.

## On-chain upgrade boundary

This document does not authorize any on-chain program upgrade.

The first guarded testnet on-chain program upgrade remains a later separate stage.

That later stage requires:

- completion and acceptance of prerequisite planning stages
- external review completion
- upgrade evidence procedure
- explicit go / no-go checkpoint
- explicit statement that live route remains disabled
- explicit statement that SPL CPI remains disabled

## Result

The next possible implementation step is a local runtime skeleton implementation branch inside this boundary.

That future branch must remain non-deployed, non-live, and unable to mint.

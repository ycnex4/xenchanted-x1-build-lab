# XXXL Deployment Roadmap Boundary

Status: COMPLETED.

This document defines the ordered roadmap from the current locked XXXL SVM runtime scaffold toward future deployment and gateway activation.

It is a documentation-only boundary.

No runtime code is changed by this stage.

## Purpose

The goal is to convert the current deployment blockers into an explicit ordered roadmap.

The current reviewed baseline is:

- `213b8014b0f69e32a43e8f9f7a5be2ebd7174dca`

At this baseline, the XXXL SVM runtime safety package is closed for the locked scaffold boundary.

The runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

The release decision remains:

- release allowed: `false`
- release blocked: `true`
- primary blocker code: `RUNTIME_SAFETY_LOCK_ACTIVE`

This roadmap does not unlock the runtime.

This roadmap does not approve deployment.

This roadmap does not approve live route activation, SPL CPI execution, `invoke_signed`, or SPL Token `mint_to`.

## Current deployment blockers

The current deployment blocker set contains eight blockers:

1. `PLACEHOLDER_PROGRAM_ID`
2. `LIVE_ROUTE_DISABLED`
3. `SPL_CPI_EXECUTION_DISABLED`
4. `ACCOUNT_CONTRACT_UNREVIEWED`
5. `MOLLUSK_COVERAGE_INCOMPLETE`
6. `PRODUCTION_GUARDIAN_SET_UNSET`
7. `PRODUCTION_PROOF_LOG_UNSET`
8. `EXTERNAL_REVIEW_INCOMPLETE`

This roadmap describes how these blockers should be addressed in future stages.

A blocker must not be removed merely because work started.

A blocker can only be removed or replaced after the required evidence, tests, and review for that blocker are complete.

## Roadmap principle

Future progress must happen in small reviewed boundaries.

Each boundary should open at most one new risk surface.

Dangerous runtime changes require separate future review.

Dangerous runtime changes include:

- live route activation
- runtime account writes in a production path
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- real Program ID selection
- production PDA fixture regeneration
- deployment blocker removal
- deployability predicate changes

## Phase 1: Account contract review

Primary blocker addressed:

- `ACCOUNT_CONTRACT_UNREVIEWED`

Goal:

Define and review the complete runtime account contract before any production-path mutation is enabled.

Required outputs:

- account inventory
- writable account set
- read-only account set
- PDA account list
- caller-supplied account list
- signer requirements
- owner requirements
- account substitution protections
- account index mapping
- failure behavior for wrong accounts

Accounts to classify include:

- runtime config PDA
- route config PDA
- guardian set PDA
- processed burn registry PDA
- proof or audit log PDA
- XXXL mint account
- mint authority PDA
- recipient token account or ATA
- SPL Token program
- system program, if required
- rent sysvar, if required

Required tests:

- wrong account count
- wrong writable flag
- required writable account passed readonly
- readonly account passed writable
- wrong owner
- wrong PDA
- wrong mint account
- wrong recipient token account
- unexpected signer
- missing required account

Exit condition:

The account contract is documented, tested, and externally reviewed.

The blocker must remain active until this evidence exists.

## Phase 2: Canonical event and replay implementation planning

Primary risks addressed:

- replay
- wrong canonical event key
- processed registry drift
- mint without processed mark
- processed mark without mint

Goal:

Prepare implementation-level rules for canonical event verification and processed-burn registry mutation.

Required outputs:

- exact canonical event key encoding reference
- recompute-and-compare rule
- replay check ordering
- processed registry key derivation
- processed registry account layout
- no-state-change-on-failure matrix
- duplicate canonical event key rejection rule

Required invariants:

- `canonicalEventKey` is recomputed before replay check
- processed registry key is exactly `canonicalEventKey`
- one `canonicalEventKey` can produce at most one successful XXXL mint
- failed verification must not mark processed
- failed mint must not mark processed
- processed mark must not happen without corresponding successful mint

Exit condition:

Processed registry implementation plan is reviewed before runtime mutation code is enabled.

## Phase 3: Program ID and PDA finalization planning

Primary blocker addressed later:

- `PLACEHOLDER_PROGRAM_ID`

Goal:

Prepare the transition from placeholder Program ID to real Program ID without enabling deployment or live execution.

Required outputs:

- real Program ID selection plan
- PDA seed inventory
- production PDA fixture regeneration plan
- fixture verification plan
- migration rule for placeholder-derived fixtures
- documentation separating scaffold fixtures from production fixtures

Required tests:

- PDA derivation changes with Program ID
- production PDA fixture matches derivation
- wrong bump rejected
- wrong PDA rejected
- wrong seed rejected
- placeholder fixture not accepted as production fixture

Exit condition:

Program ID and PDA plan is reviewed.

The blocker must remain active until real Program ID selection and production PDA fixtures are reviewed in a separate future boundary.

## Phase 4: SPL CPI threat model and Mollusk coverage

Primary blockers addressed:

- `SPL_CPI_EXECUTION_DISABLED`
- `MOLLUSK_COVERAGE_INCOMPLETE`

Goal:

Model and test the future SPL CPI mint path before `invoke_signed` or `mint_to` can become reachable.

Threats to cover:

- wrong token program
- wrong mint account
- wrong mint authority PDA
- wrong signer seeds
- wrong bump
- wrong recipient token account
- wrong recipient owner
- wrong recipient token mint
- wrong amount
- zero amount
- failed SPL CPI
- replay before CPI
- CPI before quorum
- CPI before message verification
- CPI before recipient binding
- CPI before amount binding
- account substitution
- processed mark without mint
- mint without processed mark

Required tests:

- unit tests for CPI planning boundary
- SBF / Mollusk tests for account substitution
- failed CPI no-state-change tests
- wrong token program tests
- wrong mint account tests
- wrong authority PDA tests
- wrong recipient token account tests
- replay rejection tests
- gate-disabled tests proving CPI is unreachable

Exit condition:

Mollusk coverage is complete and externally reviewed.

SPL CPI execution remains disabled until a separate future unlock review.

## Phase 5: Guardian set and proof log policy

Primary blockers addressed:

- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`

Goal:

Define production off-chain gateway security assumptions before route activation.

Required guardian policy outputs:

- guardian list
- threshold
- key custody model
- signer rotation model
- emergency replacement model
- guardian failure policy
- duplicate signer handling
- unknown signer handling
- signature domain separation

Required proof log outputs:

- public proof format
- retained fields
- canonical event key publication
- source burn reference
- guardian approval publication
- relayer submission record
- audit trail retention policy
- monitoring hooks
- incident response path

Exit condition:

Guardian and proof log policy are documented and externally reviewed.

No production route activation is allowed before this phase is complete.

## Phase 6: Testnet deployment with route still locked

Primary blocker addressed later:

- `PLACEHOLDER_PROGRAM_ID`

Goal:

Deploy runtime artifacts to testnet while keeping live execution blocked.

Allowed outcomes:

- real Program ID may be selected after review
- PDA fixtures may be regenerated after review
- program may be deployed to testnet
- runtime must remain not production-active
- live route must remain disabled
- SPL CPI may remain disabled or gated
- no production Ethereum burn may mint XXXL

Required evidence:

- deployed Program ID
- deployed XXXL mint
- verified PDA fixtures
- verified account contract
- blocked live route proof
- blocked SPL CPI proof
- no-mint safety proof

Exit condition:

Testnet deployment is verified as still locked.

This is not production activation.

## Phase 7: Controlled test route

Primary blockers addressed later:

- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `EXTERNAL_REVIEW_INCOMPLETE`

Goal:

Test the end-to-end gateway path under controlled test conditions before production route activation.

Required constraints:

- test route only
- test guardian set or explicitly marked test configuration
- test source event or test source route
- small bounded amount
- public proof log
- replay test
- failed submission test
- duplicate relayer test
- wrong recipient test
- wrong amount test
- wrong canonical event key test

Exit condition:

Controlled test route passes and is externally reviewed.

Production route remains disabled until a separate review.

## Phase 8: Production readiness review

Primary blockers addressed later:

- all remaining deployment blockers

Goal:

Review whether runtime and gateway infrastructure are ready for production activation.

Required evidence:

- all account contract reviews complete
- all PDA fixtures production-verified
- all canonical event key tests complete
- processed registry replay tests complete
- SPL CPI / `invoke_signed` / `mint_to` tests complete
- Mollusk coverage complete
- guardian policy complete
- proof log policy complete
- monitoring and incident response complete
- testnet route evidence complete
- no unresolved safety findings

Exit condition:

Only after this phase may a future boundary consider changing release or deployability predicates.

## Phase 9: Production route activation

Goal:

Activate production gateway only after explicit final review.

This phase may include dangerous changes, but only after separate approval.

Dangerous changes may include:

- removing or replacing deployment blockers
- enabling live route
- enabling SPL CPI
- enabling `invoke_signed`
- enabling SPL Token `mint_to`
- allowing production route messages
- changing release predicates

Exit condition:

Production route activation is explicitly reviewed, documented, and accepted.

## Gateway infrastructure track

The SVM runtime is only one side of the gateway.

The gateway infrastructure track must proceed in parallel and includes:

- Ethereum watcher
- source event normalizer
- finality checker
- canonical event key builder
- guardian signing pipeline
- relayer
- relayer replay guard
- public proof log
- X1 submitter
- monitoring and alerts
- incident response
- operational runbook

The runtime must not trust relayer-provided fields without verification.

The gateway infrastructure must not bypass runtime replay protection.

## Non-goals

This roadmap does not implement runtime mutation.

This roadmap does not enable runtime account writes.

This roadmap does not enable live route execution.

This roadmap does not enable SPL CPI execution.

This roadmap does not enable `invoke_signed`.

This roadmap does not enable SPL Token `mint_to`.

This roadmap does not enable XXXL minting.

This roadmap does not select a real Program ID.

This roadmap does not regenerate production PDA fixtures.

This roadmap does not remove deployment blockers.

This roadmap does not change deployability predicates.

## Decision

The XXXL deployment roadmap boundary is accepted.

The current runtime remains scaffold-only, locked, unreleasable, and not deployable.

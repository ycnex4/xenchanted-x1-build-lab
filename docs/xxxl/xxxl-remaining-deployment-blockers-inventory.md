# XXXL Remaining Deployment Blockers Inventory

## 1. Purpose

This document records the remaining XXXL SVM deployment blockers after the
account-contract blocker transition and the Mollusk coverage gap-analysis
boundary.

This is an inventory boundary only.

It does not remove any blocker.

It does not change Rust runtime code.

It does not enable deployment, live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.

## 2. Current Baseline

Current baseline:

- account-contract review evidence has been accepted for the current locked scaffold boundary
- `ACCOUNT_CONTRACT_UNREVIEWED` has been transitioned out of active blockers
- `MOLLUSK_COVERAGE_INCOMPLETE` remains active
- Mollusk/SVM coverage requirements have been documented
- runtime remains scaffold-only
- runtime remains locked
- runtime remains unreleasable
- runtime remains not deployable

## 3. Remaining Active Blockers

The remaining active deployment blockers are:

1. `PLACEHOLDER_PROGRAM_ID`
2. `LIVE_ROUTE_DISABLED`
3. `SPL_CPI_EXECUTION_DISABLED`
4. `MOLLUSK_COVERAGE_INCOMPLETE`
5. `PRODUCTION_GUARDIAN_SET_UNSET`
6. `PRODUCTION_PROOF_LOG_UNSET`
7. `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed by this inventory.

## 4. Blocker: PLACEHOLDER_PROGRAM_ID

### Meaning

The runtime still uses a placeholder / non-production Program ID boundary.

### Why It Remains Active

A production Program ID has not been selected, reviewed, locked, and connected
to production PDA derivation evidence.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must define:

- production Program ID selection criteria
- Program ID immutability assumptions
- PDA derivation impact
- fixture regeneration policy
- proof that deployment status remains blocked by other blockers after transition
- review evidence that changing this blocker does not activate live execution

### Must Not Happen Implicitly

Transitioning this blocker must not automatically enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- release readiness
- deployability

## 5. Blocker: LIVE_ROUTE_DISABLED

### Meaning

`process_instruction` does not activate the live mint route.

### Why It Remains Active

The live route must stay disabled until all account, CPI, guardian, proof-log,
Mollusk, external-review, and release-gate requirements are satisfied.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must prove:

- account contract enforcement is production-ready
- all required Mollusk/SVM coverage exists
- replay and atomicity behavior are covered in runtime-like tests
- guardian verification is production-ready
- proof logging is production-ready
- SPL CPI execution is either still disabled or separately reviewed
- release safety gates explicitly allow the transition

### Must Not Happen Implicitly

This blocker must not be removed as a side effect of another blocker transition.

## 6. Blocker: SPL_CPI_EXECUTION_DISABLED

### Meaning

SPL Token CPI execution remains disabled.

The runtime may plan or model SPL Token `mint_to`, but it must not execute live
SPL CPI in the current locked scaffold state.

### Why It Remains Active

SPL CPI execution is a high-risk boundary because it connects account validation,
PDA signer authority, replay protection, state mutation, and token minting.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must prove:

- `invoke_signed` authority is safe and scoped
- SPL Token `mint_to` account order and authority are correct
- wrong token program is rejected
- wrong mint is rejected
- wrong recipient token account is rejected
- wrong PDA or bump is rejected
- failed CPI cannot leave partial state
- replay remains impossible
- close/reinitialize race concerns are addressed
- Mollusk/SVM coverage exists for live-like CPI paths

### Must Not Happen Implicitly

This blocker must not be removed by Program ID work, account-contract work, or
Mollusk planning alone.

## 7. Blocker: MOLLUSK_COVERAGE_INCOMPLETE

### Meaning

Required Mollusk/SVM runtime coverage has not yet been implemented and accepted.

### Why It Remains Active

Rust unit tests provide useful lower-level evidence, but they are not equivalent
to Mollusk/SVM runtime coverage.

### Future Requirements Before Transition

The separate Mollusk implementation stage must cover the required areas
documented in:

- `docs/xxxl/xxxl-mollusk-coverage-gap-analysis.md`
- `docs/checkpoints/xxxl-mollusk-coverage-gap-analysis.md`

At minimum, future coverage must address:

- account ordering
- duplicate account keys / aliasing
- owner and layout validation
- SPL Token mint/account validation
- PDA validation
- disabled execution gates
- replay and atomicity
- instruction bytes
- rent and lifecycle checks

### Must Not Happen Implicitly

This blocker must not be removed by documenting coverage requirements alone.

## 8. Blocker: PRODUCTION_GUARDIAN_SET_UNSET

### Meaning

Production guardian set configuration is not selected, reviewed, or locked.

### Why It Remains Active

Current guardian-related evidence is suitable for scaffold/model/test
boundaries, not production operation.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must define:

- production guardian identities or key policy
- threshold policy
- guardian count bounds
- guardian set rotation policy
- emergency replacement policy
- quorum validation behavior
- invalid guardian rejection behavior
- proof that guardian set changes cannot bypass replay, route, amount, or mint checks

### Must Not Happen Implicitly

This blocker must not be removed by generic external review or deployment
readiness wording.

## 9. Blocker: PRODUCTION_PROOF_LOG_UNSET

### Meaning

Production proof-log policy and evidence retention are not selected, reviewed,
or locked.

### Why It Remains Active

A bridge/gateway runtime needs durable proof records for auditability,
replay investigation, incident response, and user-facing verification.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must define:

- proof log schema
- retention policy
- event key indexing
- guardian approval indexing
- source-chain evidence indexing
- failure/rejection logging policy
- privacy and redaction assumptions
- audit reconstruction flow

### Must Not Happen Implicitly

This blocker must not be removed by runtime tests alone.

## 10. Blocker: EXTERNAL_REVIEW_INCOMPLETE

### Meaning

The runtime has not received sufficient external review for production readiness.

### Why It Remains Active

Internal review, self-audit, and AI-assisted review are useful, but they are not
a replacement for independent external review before deployment or live route
activation.

### Future Requirements Before Transition

Before this blocker can be transitioned, a separate reviewed stage must define:

- review scope
- reviewed commit hash
- reviewed files
- reviewer identity or review standard
- findings
- remediation status
- unresolved risks
- explicit non-authorization boundaries

### Must Not Happen Implicitly

This blocker must be transitioned only after review evidence is recorded.

## 11. Recommended Future Order

A conservative future order is:

1. Mollusk/SVM coverage implementation boundary
2. Mollusk coverage review and blocker-transition boundary
3. Program ID production-readiness boundary
4. Guardian set production policy boundary
5. Proof log production policy boundary
6. SPL CPI threat model and execution boundary
7. External review package
8. Live route activation boundary

This order is not deployment approval.

It is only a planning inventory.

## 12. Current Safety Statement

After this inventory:

- no blocker is removed
- active blocker count remains unchanged
- runtime remains scaffold-only
- runtime remains locked
- runtime remains unreleasable
- runtime remains not deployable
- live route execution remains disabled
- SPL CPI execution remains disabled
- `invoke_signed` remains unreachable from the live route
- SPL Token `mint_to` remains disabled from live route execution

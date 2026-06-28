# XXXL Account Contract Review Closure Boundary

## 1. Purpose

This document records the closure of the XXXL SVM `consume_gateway_mint` account-contract review boundary.

This is a narrow review closure.

It accepts the account-contract evidence for the current scaffold boundary only.

It does not approve production deployment, runtime unlock, live route execution, SPL CPI execution, `invoke_signed`, or SPL Token `mint_to`.

It does not remove or weaken `ACCOUNT_CONTRACT_UNREVIEWED`.

If `ACCOUNT_CONTRACT_UNREVIEWED` is ever removed, that must happen only in a later, separate, explicitly reviewed blocker-transition stage.

## 2. Base Checkpoint

Closure is based on the review package merged at:

- `b2b6d6030e4ebb91f110a741194621238acf3f97`
- `Merge XXXL account contract review assessments`

Relevant assessment artifacts:

- `docs/reviews/xxxl-account-contract-review-assessment-codex.md`
- `docs/reviews/xxxl-account-contract-review-assessment-theo.md`
- `docs/reviews/xxxl-account-contract-review-assessment-claude.md`
- `docs/reviews/xxxl-account-contract-review-assessment-synthesis.md`

Relevant evidence artifacts:

- `docs/xxxl/xxxl-account-contract-review-evidence-package.md`
- `docs/xxxl/xxxl-account-contract-review-boundary.md`
- `docs/checkpoints/xxxl-account-contract-review-evidence-package.md`
- `docs/checkpoints/xxxl-account-contract-test-gap-closure.md`

## 3. Closure Scope

This closure applies only to the account-contract evidence for the current locked scaffold.

It covers:

- the 9-account `consume_gateway_mint` account list
- account indexes
- writable / readonly policy
- no-external-signer policy
- structural account-contract checks
- processor-boundary substitution tests
- lower-level instruction/account/CPI planning coverage claims
- evidence mapping from substitution threats to tests
- known limitations and future production-path concerns

It does not cover:

- production deployment
- production Program ID
- live route activation
- SPL Token CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set configuration
- production proof log
- Mollusk/SBF live-execution coverage
- runtime release readiness
- removing any deployment blocker

## 4. Account Contract Accepted for This Boundary

The reviewed account contract for `consume_gateway_mint` is:

| Index | Account | Access | Signer |
| --- | --- | --- | --- |
| 0 | `mint_state` | readonly | not signer |
| 1 | `gateway_config` | readonly | not signer |
| 2 | `guardian_set` | readonly | not signer |
| 3 | `processed_event` | writable | not signer |
| 4 | `recipient_balance` | writable | not signer |
| 5 | `spl_token_mint` | writable | not signer |
| 6 | `recipient_token_account` | writable | not signer |
| 7 | `mint_authority_pda` | readonly | not signer |
| 8 | `token_program` | readonly | not signer |

External signer accounts are not required and are not accepted for this account contract.

## 5. Enforcement Model Clarification

The closure accepts the account-contract evidence only with the following enforcement model.

`account_contract.rs` is classification and structural contract evidence.

It is not the only enforcement layer.

The reviewed defense model is layered:

1. `instruction.rs` enforces rigid account index and account meta structure.
2. `account_contract.rs` enforces the structural account matrix, including writable and signer expectations.
3. `processor.rs` and validation helpers enforce semantic account binding.
4. `cpi.rs` and CPI planning tests preserve the locked planning-only boundary.
5. `deployment_status.rs` and `safety_invariants.rs` keep runtime release blocked.

Future work must not weaken processor validation by assuming that `account_contract.rs` alone enforces all semantic constraints.

## 6. Evidence Accepted

The review assessments agree that the evidence is sufficient for this boundary.

Accepted evidence includes:

- clear 9-account contract documentation
- consistency between docs, instruction constants, account contract, and processor constants
- writable / readonly checks
- no-external-signer checks
- account count and meta count coverage
- account index mapping coverage
- owner model classification coverage
- processor-boundary negative tests for high-priority substitution threats
- replay protection evidence for the locked scaffold boundary
- SPL Token amount compatibility checks, including zero amount and `u128` to `u64` overflow
- PDA key and bump validation for `mint_authority_pda`
- SPL Token program binding evidence
- safety framing that avoids deployability or runtime-unlock overclaims

## 7. Accepted Reviewer Verdict

The combined reviewer verdict is:

- proceed to account contract review closure boundary
- do not run another evidence/test-gap stage first
- keep runtime locked
- keep all deployment blockers active
- record known future concerns explicitly
- do not treat this closure as production readiness

## 8. Required Production Requirements Recorded by Closure

The following requirements are not blockers for this account-contract scaffold closure, but they are mandatory before any future production live execution.

### 8.1 Program-owned account PDA derivation

Current program-owned accounts are bound by field/content validation, not by explicit PDA derivation constraints.

This applies to:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `processed_event`
- `recipient_balance`

This is accepted for the current scaffold review boundary.

It is not sufficient for production deployment.

Production execution requires explicit PDA derivation constraints for these program-owned accounts.

This is a mandatory production requirement, not an optional improvement.

### 8.2 Guardian set live-path validation

The current scaffold validates `guardian_set` binding primarily through `guardian_set_id`.

Future live execution must validate guardian quorum semantics, including:

- quorum threshold constraints
- guardian count constraints
- guardian set consistency with the active route/config
- any future production guardian-set invariants required by the live authorization path

This is a future live-path requirement.

It is not a current scaffold blocker.

## 9. Known Future Concerns for Production Path

The following concerns were identified by reviewers and are explicitly preserved for future stages.

They do not block this account-contract review closure.

### 9.1 Duplicate account keys / account deduplication

Future live-execution work should consider explicit account deduplication or tests for same-pubkey accounts across multiple indices.

Important examples include:

- `processed_event`
- `recipient_balance`
- writable SPL Token accounts
- `mint_authority_pda` semantic separation from other accounts

Duplicate mutable accounts can cause borrow conflicts or test-to-mainnet behavior mismatch.

### 9.2 SPL Token close / reinitialization race

Future SPL CPI execution must account for close/reinitialization race patterns where an SPL Token account passes validation and is later closed or changed by another instruction before CPI execution.

This is not exploitable in the current scaffold because SPL CPI execution remains disabled.

### 9.3 Rent exemption timing

Future live mutation work should revisit rent exemption timing for writable accounts.

Preparation-time rent checks alone may not be sufficient as a complete production-path argument.

### 9.4 PDA semantic separation

Future hardening may add explicit defense-in-depth assertions that `mint_authority_pda` is not:

- the program id
- another account in the 9-account list
- a semantically unrelated PDA accepted by coincidence

Current PDA key and bump validation is sufficient for this scaffold boundary, but production hardening should revisit semantic separation.

### 9.5 Instruction padding / reserved bytes

Future instruction layout documentation should clarify unused, reserved, or future-expansion bytes.

The production path should decide whether such bytes:

- must be zero
- are intentionally ignored
- are included in canonical audit/indexing hashes
- are reserved for future versioned expansion

### 9.6 Writable account swap coverage

Future tests may add direct processor-boundary coverage for swapping two writable accounts, such as:

- `processed_event`
- `recipient_balance`

This is optional for this closure but useful as future hardening.

### 9.7 Dual-source account index invariant

The instruction carries explicit account index bytes while the processor also has hardcoded constants.

Current tests verify consistency.

Future refactors must preserve this invariant and must not introduce drift between instruction layout and processor account lookup.

## 10. Blocker State After Closure

This closure does not remove any blocker.

The following remain active:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `ACCOUNT_CONTRACT_UNREVIEWED`
- `MOLLUSK_COVERAGE_INCOMPLETE`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Even after this account-contract review closure is recorded, `ACCOUNT_CONTRACT_UNREVIEWED` remains active until a later separate blocker-transition stage explicitly decides otherwise.

## 11. Safety Non-Changes

This closure does not change Rust runtime code.

It does not change:

- `programs/xxxl-svm/src/deployment_status.rs`
- `programs/xxxl-svm/src/safety_invariants.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/account_contract.rs`
- any live route flag
- any SPL CPI flag
- any Program ID
- any PDA fixture
- any deployment predicate
- any blocker semantics

## 12. Final Closure Statement

The XXXL SVM `consume_gateway_mint` account-contract evidence is accepted for the current locked scaffold boundary.

The account-contract review boundary is closed as evidence-complete.

This closure is not deployment approval.

This closure is not production readiness.

This closure is not runtime unlock approval.

This closure is not SPL CPI approval.

This closure is not authorization to call `invoke_signed`.

This closure is not authorization to execute SPL Token `mint_to`.

`ACCOUNT_CONTRACT_UNREVIEWED` remains active until a later separate transition stage explicitly removes it after review.

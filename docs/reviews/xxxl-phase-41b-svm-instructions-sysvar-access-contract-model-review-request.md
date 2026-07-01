# XXXL Phase 41B SVM Instructions Sysvar Access Contract Model Review Request

Status: External review request.

Audience:

- Audit demon
- Theo

## Context

Phase 41A was accepted.

Phase 41B is model-only.

It materializes the Phase 41A review notes into Rust:

- 20 requirements
- 18 rejection cases
- full ownership of rejection cases
- four Phase 40 orphan rejection cases closed
- all safety flags remain false

## Review Scope

Please review Phase 41B only.

Primary files:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_access_contract_model.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/xxxl/xxxl-phase-41b-svm-instructions-sysvar-access-contract-model.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41b-svm-instructions-sysvar-access-contract-model.md`
- `docs/reviews/xxxl-phase-41b-svm-instructions-sysvar-access-contract-model-review-request.md`
- `docs/checkpoints/current-design-checkpoint.md`

## Core Boundary To Review

Phase 41B must preserve:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != runtime sysvar read
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Questions For Reviewers

1. Did Phase 41B remain model-only?

2. Did Phase 41B avoid parsing raw Instructions sysvar account data?

3. Did Phase 41B avoid parsing `AccountInfo`?

4. Did Phase 41B avoid calling or enabling `load_instruction`?

5. Did Phase 41B avoid selecting a concrete runtime API?

6. Did Phase 41B avoid deriving current instruction identity from runtime
   context?

7. Did Phase 41B avoid Ed25519 signature verification?

8. Did Phase 41B avoid accepting cryptographic proof or verification evidence?

9. Did Phase 41B avoid quorum, authorization, replay, CPI, and mint execution?

10. Did Phase 41B correctly declare 20 requirements?

11. Did Phase 41B correctly declare 18 rejection cases?

12. Does every rejection case have an owning requirement?

13. Are the four Phase 40 orphan rejection cases correctly closed?

14. Does the terminology preserve the distinction between SVM Ed25519 program
    instruction candidate and guardian signature verification?

15. Is Phase 41C allowed to start, and if so, what is the minimum safe boundary?

## Requested Verdict Format

Please answer with one of:

- ACCEPT
- ACCEPT WITH NOTES
- REQUEST CHANGES
- BLOCK

Please include:

- required fixes, if any
- blocking risks, if any
- optional notes, if any
- whether Phase 41C may start
- minimum safe Phase 41C boundary
- what must remain prohibited in Phase 41C

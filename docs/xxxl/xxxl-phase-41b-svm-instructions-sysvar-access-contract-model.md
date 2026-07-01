# XXXL Phase 41B SVM Instructions Sysvar Access Contract Model

Status: Rust model-only.

## Purpose

Phase 41B follows the accepted Phase 41A docs-only plan.

It materializes the Phase 41A review requirements into a tiny Rust model.

The model closes the docs-to-code drift identified during Phase 41A review:

- Phase 40D/40F had 16 future requirements.
- Phase 41A added 4 owning requirements for orphan rejection cases.
- Phase 41B declares 20 requirements in Rust.
- Phase 41B maps all 18 rejection cases to owning requirements.

Phase 41B does not implement real runtime integration.

## Files

Added:

- `programs/xxxl-svm/src/verifier/instructions_sysvar_access_contract_model.rs`
- `docs/xxxl/xxxl-phase-41b-svm-instructions-sysvar-access-contract-model.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41b-svm-instructions-sysvar-access-contract-model.md`
- `docs/reviews/xxxl-phase-41b-svm-instructions-sysvar-access-contract-model-review-request.md`

Changed:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `docs/checkpoints/current-design-checkpoint.md`

## Model Scope

Phase 41B models:

- Instructions sysvar presence/readability requirement ownership
- current instruction identity requirement ownership
- prior Ed25519 instruction lookup requirement ownership
- prior Ed25519 instruction strict ordering requirement ownership
- guardian evidence uniqueness requirement ownership
- single candidate resolution requirement ownership
- full rejection ownership for all 18 rejection cases
- safety flags proving no runtime integration is implemented

## Preserved Boundary

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

Phase 41B adds a model of requirement ownership.

It does not add runtime sysvar read.

## Orphan Rejection Cases Closed In Rust

Phase 41B closes the four Phase 40 orphan rejection cases at model level:

| Owning requirement | Rejection case |
| --- | --- |
| `InstructionsSysvarReadable` | `UnreadableInstructionsSysvar` |
| `PriorEd25519InstructionOrdering` | `Ed25519InstructionAfterCurrentInstruction` |
| `GuardianEvidenceUniqueness` | `DuplicateGuardianEvidence` |
| `SingleCandidateResolution` | `AmbiguousCandidateEvidence` |

## Requirement Count

Phase 41B declares 20 requirements:

- 16 inherited planning requirements from the Phase 40 requirement surface
- 4 added requirements from Phase 41A review closure

## Rejection Count

Phase 41B declares 18 rejection cases.

Every rejection case has at least one owning requirement.

`DeterministicFailureReason` remains a meta-requirement and intentionally has no
single primary rejection case.

## Terminology Guard

Phase 41B keeps the terminology distinction:

- SVM Ed25519 program instruction candidate
- guardian signature verification
- cryptographic proof acceptance

Locating or modeling a future SVM Ed25519 program instruction is not the same as
performing guardian signature verification.

It is not proof.

It is not quorum.

It is not authorization.

It is not execution.

## Explicit Non-Goals

Phase 41B does not parse raw Instructions sysvar account data.

Phase 41B does not parse `AccountInfo`.

Phase 41B does not call `load_instruction`.

Phase 41B does not select a concrete runtime API.

Phase 41B does not derive current instruction identity from runtime context.

Phase 41B does not verify Ed25519 signatures.

Phase 41B does not accept cryptographic signature proof.

Phase 41B does not accept verification evidence.

Phase 41B does not count quorum.

Phase 41B does not authorize minting.

Phase 41B does not add a runtime instruction handler.

Phase 41B does not add CPI.

Phase 41B does not enable `invoke_signed`.

Phase 41B does not enable SPL Token `mint_to`.

Phase 41B does not add replay writes.

Phase 41B does not mark processed events.

Phase 41B does not mutate runtime/account state.

Phase 41B does not unlock live route execution.

Phase 41B does not remove deployment blockers.

Phase 41B does not select a production Program ID.

Phase 41B does not claim production readiness.

Phase 41B does not claim final immutability while upgrade authority exists.

Phase 41B does not build SBF artifacts.

Phase 41B does not touch `target/deploy`.

Phase 41B does not read or modify keypair files.

Phase 41B does not read or modify `.env`.

Phase 41B does not inspect `.local-keys`.

Phase 41B does not run deploy commands.

Phase 41B does not run network commands.

Phase 41B does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41B.

## Recommended Next Stage

Phase 41B must be reviewed before Phase 41C.

Phase 41C must not start without a dedicated review agreement.

If accepted, Phase 41C may begin only as a separately reviewed raw Instructions
sysvar implementation phase.

No proof, quorum, authorization, replay consumption, or mint execution should be
combined with Phase 41C.

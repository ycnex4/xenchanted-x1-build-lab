# XXXL Phase 40G Ed25519 Verification Evidence Series Closure

Status: Docs-only closure checkpoint.

## Purpose

Phase 40G closes the Phase 40 Ed25519 verification evidence preparation series.

It records what was completed from Phase 40A through Phase 40F and defines the
risk boundary before any real raw Instructions sysvar integration is attempted.

This is a checkpoint phase.

It does not add Rust code.

It does not modify Rust source files.

It does not parse raw Instructions sysvar account data.

It does not parse `AccountInfo`.

It does not call `load_instruction`.

It does not verify Ed25519 signatures.

It does not accept cryptographic proof.

It does not count quorum.

It does not authorize minting.

It does not enable execution.

## Closed Series

Phase 40A:

- documented the verification evidence boundary
- clarified that located/parsed candidate evidence is not proof, quorum,
  authorization, or execution

Phase 40B:

- added a tiny Rust model for Ed25519 verification evidence boundary
- proved that located/parsed evidence remains non-authorizing

Phase 40C:

- documented the future SVM Ed25519 verification evidence integration design
- described future requirements without implementation

Phase 40D:

- added a typed Rust design surface for future integration requirements and
  rejection cases

Phase 40E:

- added a Rust prior-instruction ordering model
- modeled the future rule that an Ed25519 verification instruction must appear
  before the XXXL instruction that consumes it

Phase 40F:

- added a Rust coverage matrix mapping future requirements to primary rejection
  cases
- kept all proof, quorum, authorization, and execution surfaces disabled

## Control Point

The Phase 40 series is a safe control point because the project now has:

- explicit boundary documentation
- typed non-authorizing evidence models
- future integration requirements
- future rejection cases
- prior-instruction ordering model
- requirement-to-error coverage matrix
- preserved runtime safety blockers

The project has not crossed into live verification or execution.

## Boundary Preserved

The preserved rule remains:

~~~text
located candidate evidence
  != parsed evidence
  != prior-instruction ordering
  != requirement coverage
  != verification evidence
  != quorum
  != authorization
  != execution
~~~

## Risk Boundary Before Phase 41

Phase 41 must not begin as implementation unless externally reviewed.

The next dangerous boundary is real runtime interaction with the SVM
Instructions sysvar.

The following should require dedicated review before implementation:

- raw Instructions sysvar account parsing
- `AccountInfo` integration
- current instruction identity derivation
- prior Ed25519 instruction lookup from real transaction instructions
- Solana/SVM `load_instruction` or equivalent use
- Ed25519 program id validation against real instructions
- proving that the Ed25519 verification instruction actually executed before
  XXXL instruction consumption
- accepting verification evidence
- composing verification evidence into quorum
- composing quorum into authorization
- composing authorization into replay protection
- composing authorization into SPL mint execution

## Explicit Non-Goals

Phase 40G does not add Rust code.

Phase 40G does not modify Rust source files.

Phase 40G does not modify TypeScript source files.

Phase 40G does not modify test files.

Phase 40G does not modify Cargo files.

Phase 40G does not modify package files.

Phase 40G does not parse raw Instructions sysvar account data.

Phase 40G does not parse `AccountInfo`.

Phase 40G does not call `load_instruction`.

Phase 40G does not verify Ed25519 signatures.

Phase 40G does not accept cryptographic signature proof.

Phase 40G does not accept verification evidence.

Phase 40G does not count quorum.

Phase 40G does not authorize minting.

Phase 40G does not add a runtime instruction handler.

Phase 40G does not add CPI.

Phase 40G does not enable `invoke_signed`.

Phase 40G does not enable SPL Token `mint_to`.

Phase 40G does not add replay writes.

Phase 40G does not mark processed events.

Phase 40G does not mutate runtime/account state.

Phase 40G does not unlock live route execution.

Phase 40G does not remove deployment blockers.

Phase 40G does not select a production Program ID.

Phase 40G does not claim production readiness.

Phase 40G does not claim final immutability while upgrade authority exists.

Phase 40G does not build SBF artifacts.

Phase 40G does not touch `target/deploy`.

Phase 40G does not read or modify keypair files.

Phase 40G does not read or modify `.env`.

Phase 40G does not inspect `.local-keys`.

Phase 40G does not run deploy commands.

Phase 40G does not run network commands.

Phase 40G does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 40G.

## Review Request

Phase 40G should be reviewed before Phase 41 implementation starts.

Reviewers should focus on:

- whether Phase 40A through Phase 40F preserved the boundary correctly
- whether any module accidentally implies proof, quorum, authorization, or
  execution
- whether the prior-instruction ordering model is conceptually correct
- whether the future integration requirements are sufficient
- whether the coverage matrix misses important rejection cases
- whether Phase 41 should start as docs-only, model-only, or implementation

## Recommended Next Stage

Phase 41 should be opened only after review.

Recommended Phase 41 shape:

- Phase 41A: docs-only reviewed runtime integration plan
- Phase 41B: tiny model for real Instructions sysvar access contract
- Phase 41C: implementation only after audit agreement

No raw Instructions sysvar integration should be merged without dedicated review.

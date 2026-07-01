# XXXL Phase 41C4 Descriptor Series Closure And Runtime Wiring Boundary

Status: Docs-only closure.

## Purpose

Phase 41C4 closes the Phase 41C descriptor/model boundary series.

It also defines the boundary for the future Phase 41D runtime-wiring series.

Phase 41C4 does not add runtime code.

Phase 41C4 does not add Rust code.

Phase 41C4 does not modify tests.

Phase 41C4 does not begin real runtime wiring.

## Phase 41C Series Closure

The Phase 41C descriptor series is complete.

Closed phases:

| Phase | Scope | Main merge |
| --- | --- | --- |
| 41C0 | real Instructions sysvar implementation boundary docs | `1fc9fe1` |
| 41C0A | 41C1 sysvar access boundary clarification | `d76c024` |
| 41C1 | Instructions sysvar presence/readability boundary | `bc04dd6` |
| 41C2 | current instruction identity descriptor boundary | `47eda81` |
| 41C3 | prior Ed25519 lookup + strict ordering descriptor boundary | `a533530` |
| 41C3A | edge-case semantics clarification | `0226900` |

The Phase 41C descriptor series established:

- presence/readability boundary
- current-instruction identity descriptor boundary
- prior Ed25519 lookup and strict ordering over descriptors
- edge-case semantics for same/later matching descriptors
- descriptor input contract for future runtime-wiring
- Phase 41B taxonomy as authoritative

## Important Closure Statement

Phase 41C did not perform real runtime wiring.

Phase 41C did not parse real Solana `AccountInfo`.

Phase 41C did not parse real Instructions sysvar account data.

Phase 41C did not call `load_instruction`.

Phase 41C did not call `load_instruction_at`.

Phase 41C did not call `load_instruction_at_checked`.

Phase 41C did not read concrete transaction instruction contents from runtime.

Phase 41C did not verify Ed25519 signatures.

Phase 41C did not accept cryptographic proof.

Phase 41C did not accept verification evidence.

Phase 41C did not count quorum.

Phase 41C did not authorize minting.

Phase 41C did not write replay state.

Phase 41C did not mutate runtime/account state.

Phase 41C did not perform CPI.

Phase 41C did not call `invoke_signed`.

Phase 41C did not call SPL Token `mint_to`.

Phase 41C did not unlock the live route.

## Preserved Boundary

The preserved boundary remains:

~~~text
descriptor/model boundary
  != real runtime sysvar read
  != real AccountInfo parsing
  != real load_instruction_at call
  != concrete instruction content read from runtime
  != Ed25519 signature verification
  != verification evidence acceptance
  != cryptographic proof
  != quorum
  != authorization
  != replay consumption
  != execution
~~~

## 41C Descriptor Artifacts

The descriptor/model artifacts now available for later use are:

- Phase 41C1:
  - `MissingInstructionsSysvar`
  - `UnreadableInstructionsSysvar`
  - `PresentAndReadable`

- Phase 41C2:
  - `MissingCurrentInstructionIdentity`
  - `InconsistentCurrentInstructionIdentity`
  - `CurrentInstructionIdentityBound`

- Phase 41C3:
  - `PriorEd25519InstructionNotFound`
  - `WrongEd25519ProgramId`
  - `MalformedStructuralCandidate`
  - `DuplicateGuardianEvidence`
  - `Ed25519InstructionNotBeforeCurrentInstruction`
  - `AmbiguousCandidateEvidence`
  - `PriorEd25519InstructionLocatedAndOrdered`

These are non-authorizing descriptor states.

A successful descriptor state is not proof.

A successful descriptor state is not verification evidence.

A successful descriptor state is not quorum.

A successful descriptor state is not authorization.

A successful descriptor state is not replay consumption.

A successful descriptor state is not execution.

## Future Phase 41D Boundary

The future Phase 41D series is the first real runtime-wiring boundary.

Phase 41D must be treated as panic-safety-critical.

Phase 41D must be split into separate reviewed read layers.

Recommended split:

| Phase | Minimum scope |
| --- | --- |
| 41D0 | docs-only runtime-wiring plan and safety checklist |
| 41D1 | real Instructions sysvar presence/readability from runtime `AccountInfo` |
| 41D2 | real current-instruction identity population |
| 41D3 | real prior-instruction enumeration and prefiltering into descriptors |

Each 41D subphase must be reviewed before merge.

Each 41D subphase must introduce only one real read layer.

## 41D0 Minimum Boundary

41D0 should be docs-only.

41D0 should define:

- exact runtime API/helper to be used
- exact `AccountInfo` inputs
- borrow failure behavior
- missing sysvar behavior
- malformed sysvar behavior
- deterministic error mapping
- no-panic requirements
- forbidden operation list
- per-flag transition plan

41D0 must not add runtime code.

## 41D1 Minimum Boundary

41D1 may introduce only:

- real Instructions sysvar presence check
- real Instructions sysvar readability check
- panic-safe `AccountInfo` borrow handling
- deterministic mapping into:
  - `MissingInstructionsSysvar`
  - `UnreadableInstructionsSysvar`
  - `PresentAndReadable`

41D1 must not call `load_instruction_at`.

41D1 must not read concrete instruction contents.

41D1 must not derive current instruction identity.

41D1 must not locate prior Ed25519 instructions.

41D1 must not accept verification evidence.

## 41D2 Minimum Boundary

41D2 may introduce only:

- real current-instruction identity population
- deterministic mapping into Phase 41C2 descriptor states

41D2 must not perform prior Ed25519 lookup.

41D2 must not accept verification evidence.

41D2 must not verify signatures.

41D2 must not authorize minting.

## 41D3 Minimum Boundary

41D3 may introduce only:

- real prior-instruction enumeration
- panic-safe `load_instruction_at_checked` or equivalent checked helper
- prefiltering unrelated instructions before descriptor construction
- construction of Phase 41C3 candidate descriptors
- descriptor-level prior lookup and strict ordering

41D3 must preserve the Phase 41C3A input contract:

- Phase 41C3 receives candidate descriptors, not all raw transaction instructions.
- unrelated non-Ed25519 transaction instructions must not be forwarded into 41C3 as candidate descriptors.
- `WrongEd25519ProgramId` means an evidence-candidate descriptor has the wrong program id.

41D3 must not accept verification evidence.

41D3 must not verify Ed25519 signatures.

41D3 must not count quorum.

41D3 must not authorize minting.

## Operations Forbidden Throughout 41D

Until a later separately reviewed phase, 41D must not:

- perform Ed25519 cryptographic verification
- set `cryptographic_signature_proof_accepted: true`
- set `verification_evidence_accepted: true`
- count guardian quorum
- authorize minting
- write replay state
- mark processed events
- mutate runtime/account state
- add CPI
- call `invoke_signed`
- call SPL Token `mint_to`
- add a runtime execution handler
- unlock live route execution
- select a production Program ID
- remove deployment blockers
- claim production readiness
- claim final immutability while upgrade authority exists
- build SBF artifacts
- touch `target/deploy`
- read or modify keypair files
- read or modify `.env`
- inspect `.local-keys`
- run deploy commands
- run network commands
- spend SOL

## Active Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C4.

## Review Gate

Phase 41C4 must be reviewed before opening Phase 41D0.

Phase 41D0 should be docs-only.

No real runtime wiring should begin before 41D0 is reviewed.

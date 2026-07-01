# XXXL Phase 41A SVM Instructions Sysvar Runtime Integration Plan

Status: Docs-only reviewed runtime integration plan.

## Purpose

Phase 41A opens the Phase 41 series after the Phase 40A-40G control point.

Both independent reviews of Phase 40 returned:

- `ACCEPT WITH NOTES`
- required fixes: none
- blocking risks: none
- Phase 41A must be docs-only

Phase 41A defines the safe plan for future SVM Instructions sysvar runtime
integration.

It does not implement runtime integration.

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

## Review Inputs

The Phase 40 review consensus was:

- Phase 40A-40G correctly preserve the boundary.
- Phase 40 has no blocking risks.
- Phase 40 has no required fixes.
- Phase 41 must not start as implementation.
- Phase 41A must be docs-only.
- Four orphan rejection cases must receive explicit owning requirements before
  implementation.

## Preserved Boundary

The preserved boundary remains:

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

Phase 41A adds planning for `runtime sysvar read`, but does not implement it.

## Safe Read-Only Runtime Integration Contract

Future runtime integration must be read-only at the evidence layer.

The future runtime integration contract must only produce candidate runtime
evidence about transaction instruction structure.

It must not produce authorization.

It must not mutate account state.

It must not mark processed events.

It must not execute CPI.

It must not mint tokens.

It must not unlock live route execution.

The contract must prove only structural facts:

- the Instructions sysvar account is present
- the Instructions sysvar account is readable
- the current XXXL instruction identity is derived from runtime context
- prior instructions can be inspected before the current instruction
- a prior Ed25519 instruction candidate can be located
- candidate ambiguity is rejected
- duplicate guardian evidence is rejected
- ordering violations are rejected

Even after this contract succeeds, the output is still not proof, quorum,
authorization, or execution.

## Current Instruction Identity Derivation

Future implementation must not trust a caller-supplied current instruction index
as an authorization input.

Current instruction identity must be derived from runtime context.

The future plan must define:

- how the current instruction index is derived
- how the current instruction program id is checked
- how the current instruction payload or discriminator is bound
- how mismatch between derived identity and expected XXXL route is rejected

Phase 41A does not choose or call a concrete runtime API.

That choice is deferred until a reviewed implementation phase.

## Prior Ed25519 Instruction Lookup

Future implementation must inspect only instructions strictly before the current
XXXL instruction.

The valid ordering rule remains:

~~~text
matched_ed25519_instruction_index < current_instruction_index
~~~

The following must be rejected:

- no Ed25519 instruction found
- Ed25519 instruction equal to current instruction
- Ed25519 instruction after current instruction
- ambiguous matching candidate evidence
- duplicate guardian evidence for the same payload
- malformed Ed25519 instruction data
- wrong Ed25519 program id
- wrong guardian public key
- wrong message hash

The prior lookup layer still does not verify Ed25519 signatures itself.

It only confirms that the transaction contains a structurally valid prior
Ed25519 verification instruction candidate that can later be used as
verification evidence.

## Owning Requirements For Four Orphan Rejection Cases

Phase 40F intentionally mapped every Phase 40D requirement to a primary
rejection case.

The reviews identified four rejection cases that remained declared but did not
have an explicit owning requirement in the Phase 40F primary coverage matrix.

Phase 41A assigns ownership before implementation.

| Owning requirement | Rejection case | Reason |
| --- | --- | --- |
| `InstructionsSysvarReadable` | `UnreadableInstructionsSysvar` | The Instructions sysvar must be present and readable, not merely present. |
| `PriorEd25519InstructionOrdering` | `Ed25519InstructionAfterCurrentInstruction` | A valid Ed25519 candidate must strictly precede the current XXXL instruction. |
| `GuardianEvidenceUniqueness` | `DuplicateGuardianEvidence` | One guardian must not contribute duplicate evidence for the same payload. |
| `SingleCandidateResolution` | `AmbiguousCandidateEvidence` | Runtime evidence must resolve to one unambiguous candidate set. |

These requirements are added as Phase 41A planning requirements.

They are not implemented by Phase 41A.

## Full Future Rejection Ownership Surface

Future implementation should cover all Phase 40D rejection cases with explicit
owners:

| Rejection case | Owning requirement |
| --- | --- |
| `MissingInstructionsSysvar` | `InstructionsSysvarRuntimeRead` |
| `UnreadableInstructionsSysvar` | `InstructionsSysvarReadable` |
| `MissingCurrentInstructionIdentity` | `CurrentInstructionIdentity` |
| `Ed25519InstructionNotFound` | `PriorEd25519Instruction` |
| `Ed25519InstructionAfterCurrentInstruction` | `PriorEd25519InstructionOrdering` |
| `WrongEd25519ProgramId` | `Ed25519ProgramIdMatch` |
| `MalformedEd25519InstructionData` | `Phase37LayoutConstraints` and `Phase38InstructionDataParsing` |
| `UnsupportedOffsetLayout` | `SupportedOffsetLayout` |
| `PublicKeyMismatch` | `GuardianPublicKeyInActiveSet` |
| `MessageHashMismatch` | `Phase34PayloadHashMatch` |
| `GuardianSetMismatch` | `GuardianSetIdMatch` |
| `DuplicateGuardianEvidence` | `GuardianEvidenceUniqueness` |
| `AmbiguousCandidateEvidence` | `SingleCandidateResolution` |
| `ExpiredEvidence` | `ExpirationOrFinalityBinding` |
| `WrongRoute` | `RouteBinding` |
| `WrongTargetMint` | `TargetMintBinding` |
| `WrongRecipient` | `RecipientBinding` |
| `WrongAmount` | `AmountBinding` |

## Future Layering Order

Phase 41 must preserve narrow reviewed steps.

The safe order is:

~~~text
41A docs-only runtime integration plan
  -> 41B model-only sysvar access contract
  -> 41C reviewed raw Instructions sysvar implementation
  -> later verification evidence acceptance
  -> later quorum composition
  -> later authorization composition
  -> later replay protection composition
  -> later SPL mint execution
~~~

No single phase should combine these layers.

## Explicit Non-Goals

Phase 41A does not add Rust code.

Phase 41A does not modify Rust source files.

Phase 41A does not modify TypeScript source files.

Phase 41A does not modify test files.

Phase 41A does not modify Cargo files.

Phase 41A does not modify package files.

Phase 41A does not parse raw Instructions sysvar account data.

Phase 41A does not parse `AccountInfo`.

Phase 41A does not call `load_instruction`.

Phase 41A does not verify Ed25519 signatures.

Phase 41A does not accept cryptographic signature proof.

Phase 41A does not accept verification evidence.

Phase 41A does not count quorum.

Phase 41A does not authorize minting.

Phase 41A does not add a runtime instruction handler.

Phase 41A does not add CPI.

Phase 41A does not enable `invoke_signed`.

Phase 41A does not enable SPL Token `mint_to`.

Phase 41A does not add replay writes.

Phase 41A does not mark processed events.

Phase 41A does not mutate runtime/account state.

Phase 41A does not unlock live route execution.

Phase 41A does not remove deployment blockers.

Phase 41A does not select a production Program ID.

Phase 41A does not claim production readiness.

Phase 41A does not claim final immutability while upgrade authority exists.

Phase 41A does not build SBF artifacts.

Phase 41A does not touch `target/deploy`.

Phase 41A does not read or modify keypair files.

Phase 41A does not read or modify `.env`.

Phase 41A does not inspect `.local-keys`.

Phase 41A does not run deploy commands.

Phase 41A does not run network commands.

Phase 41A does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41A.

## Required Review Before Phase 41B

Phase 41A must be reviewed before Phase 41B starts.

Reviewers should confirm:

- the four orphan rejection cases now have owning requirements
- the runtime sysvar access contract is still read-only
- current instruction identity is not caller-trusted
- prior Ed25519 lookup remains structural
- evidence remains separate from proof, quorum, authorization, and execution
- no blocker is weakened
- Phase 41B should be model-only, not implementation

## Recommended Next Stage

If Phase 41A is accepted, Phase 41B should be model-only.

Phase 41B should add a tiny typed model for:

- Instructions sysvar presence/readability contract
- current instruction identity contract
- prior instruction lookup contract
- orphan rejection ownership surface

Phase 41B should still not parse real raw Instructions sysvar account data.

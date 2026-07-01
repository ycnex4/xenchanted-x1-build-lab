# XXXL Phase 41C3A Edge Case Semantics Clarification

Status: Narrow clarification.

## Purpose

Phase 41C3A follows the Phase 41C3 reviews.

Phase 41C3 was accepted by Theo and the audit demon.

The audit demon raised two non-blocking edge-case notes:

1. a valid prior match plus a same-index or later matching candidate currently
   returns `PriorEd25519InstructionLocatedAndOrdered`
2. unrelated non-Ed25519 instructions must not be passed into the 41C3 descriptor
   layer as if they were evidence candidates

Phase 41C3A pins the intended descriptor-layer behavior and clarifies the future
runtime-wiring input contract.

## No Logic Change

Phase 41C3A does not change the lookup algorithm.

Phase 41C3A adds tests and documentation to pin existing behavior.

## Clarified Descriptor Input Contract

Phase 41C3 does not consume raw transaction instructions.

Phase 41C3 does not consume all instructions from the real Instructions sysvar.

Phase 41C3 consumes already-separated candidate descriptors.

A candidate descriptor means:

- the upstream layer intentionally classified this item as relevant to the
  Ed25519 evidence lookup boundary
- unrelated transaction instructions must not be forwarded into Phase 41C3 as
  candidate descriptors
- real runtime-wiring must prefilter or classify real instructions before
  constructing Phase 41C3 descriptors

Therefore:

- an empty descriptor list means no evidence candidates were provided
- an empty descriptor list maps to `PriorEd25519InstructionNotFound`
- an unrelated non-Ed25519 transaction instruction should normally be omitted
  before Phase 41C3
- `WrongEd25519ProgramId` means an evidence-candidate descriptor has the wrong
  program id, not that an arbitrary unrelated transaction instruction exists

## Same Or Later Matching Candidate Semantics

The strict prior-ordering requirement is:

~~~text
candidate.instruction_index < current_instruction_index
~~~

Only strictly-prior matching descriptors count as valid prior evidence
candidates.

A same-index or later matching descriptor is not accepted as prior evidence.

In the current descriptor-layer semantics:

- if there is no valid strictly-prior match, a same-index or later matching
  descriptor maps to `Ed25519InstructionNotBeforeCurrentInstruction`
- if there is exactly one valid strictly-prior match, the same-index or later
  descriptor does not invalidate that prior match
- ambiguity is defined as more than one strictly-prior matching descriptor

This behavior is pinned by test.

Future real runtime-wiring may add a stricter anomaly layer if the real
transaction context shows that a same-index or later fully-matching Ed25519
instruction should be rejected globally. That must be a separate reviewed
decision.

## Rejection Priority Clarification

The descriptor-layer rejection priority remains:

1. duplicate guardian evidence
2. malformed structural candidate
3. ambiguous strictly-prior matching evidence
4. exactly one located and ordered strictly-prior match
5. same-index or later matching evidence when no valid prior match exists
6. wrong program id evidence-candidate descriptor
7. not found

This is a safety-first descriptor-layer ordering.

## Real Runtime Wiring Remains Deferred

Phase 41C3A does not introduce real runtime wiring.

Phase 41C3A does not parse `AccountInfo`.

Phase 41C3A does not parse real Instructions sysvar account data.

Phase 41C3A does not call `load_instruction`.

Phase 41C3A does not call `load_instruction_at`.

Phase 41C3A does not call `load_instruction_at_checked`.

Real runtime-wiring remains deferred to a future separately reviewed phase.

That future phase must be panic-safety-critical and must decide how real
instructions are classified into candidate descriptors before Phase 41C3 is
called.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41C3A.

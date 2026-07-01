# XXXL Phase 41D0 Runtime Wiring Plan And Safety Checklist

Status: Docs-only runtime-wiring plan.

## Purpose

Phase 41D0 opens the Phase 41D runtime-wiring series.

Phase 41D0 does not add runtime code.

Phase 41D0 does not add Rust code.

Phase 41D0 does not modify tests.

Phase 41D0 does not parse real `AccountInfo`.

Phase 41D0 does not parse real Instructions sysvar account data.

Phase 41D0 does not call `load_instruction`.

Phase 41D0 does not call `load_instruction_at`.

Phase 41D0 does not call `load_instruction_at_checked`.

Phase 41D0 is a safety plan for later phases.

## Background

Phase 41C closed the descriptor/model boundary series.

Phase 41C produced non-authorizing descriptor boundaries for:

- Instructions sysvar presence/readability
- current instruction identity
- prior Ed25519 lookup and strict ordering
- edge-case semantics for candidate descriptors

Phase 41D is the first series where real runtime data may enter those
descriptor boundaries.

Therefore Phase 41D is panic-safety-critical and trust-boundary-critical.

## Phase 41D Split

Phase 41D must be split into separate reviewed read layers.

| Phase | Scope | Risk |
| --- | --- | --- |
| 41D0 | docs-only runtime-wiring plan and safety checklist | planning only |
| 41D1 | real Instructions sysvar presence/readability from runtime `AccountInfo` | panic-safety-critical |
| 41D2 | real current-instruction identity population | trust-boundary-critical |
| 41D3 | real prior-instruction enumeration, prefiltering, and descriptor construction | trust-boundary-critical |

Each phase must be reviewed before merge.

Each real runtime-wiring phase must introduce only one real read layer.

No 41D phase may include proof acceptance, quorum, authorization, replay writes,
CPI, mint execution, or live route unlock.

## 41D1 Boundary

41D1 may introduce only the first real runtime read boundary:

- identify the Instructions sysvar `AccountInfo`
- check that it is supplied
- check that it is readable
- handle borrow failure without panic
- map the result into Phase 41C1 descriptor states:
  - `MissingInstructionsSysvar`
  - `UnreadableInstructionsSysvar`
  - `PresentAndReadable`

41D1 must not:

- call `load_instruction`
- call `load_instruction_at`
- call `load_instruction_at_checked`
- parse concrete instruction contents
- derive current instruction identity
- enumerate prior instructions
- construct Phase 41C3 candidate descriptors
- accept verification evidence
- verify Ed25519 signatures
- count quorum
- authorize minting
- write replay state
- mutate runtime/account state

## 41D2 Boundary

41D2 may introduce only real current-instruction identity population.

41D2 may map real runtime context into Phase 41C2 descriptor states:

- `MissingCurrentInstructionIdentity`
- `InconsistentCurrentInstructionIdentity`
- `CurrentInstructionIdentityBound`

41D2 must not:

- enumerate prior instructions
- construct Phase 41C3 candidate descriptors
- accept verification evidence
- verify Ed25519 signatures
- count quorum
- authorize minting
- write replay state
- mutate runtime/account state

## 41D3 Boundary

41D3 may introduce only:

- real prior-instruction enumeration
- panic-safe checked instruction loading
- prefiltering unrelated instructions
- construction of Phase 41C3 candidate descriptors
- descriptor-level prior lookup and strict ordering

41D3 must preserve the Phase 41C3A pre-filter contract:

- Phase 41C3 receives candidate descriptors, not all raw transaction instructions.
- unrelated non-Ed25519 transaction instructions must not be forwarded into
  Phase 41C3 as candidate descriptors.
- `WrongEd25519ProgramId` means an evidence-candidate descriptor has the wrong
  program id, not that an arbitrary unrelated transaction instruction exists.
- an empty descriptor list means no evidence candidates were supplied and maps
  to `PriorEd25519InstructionNotFound`.

41D3 must explicitly decide the same/later anomaly question:

- if there is one valid strictly-prior matching Ed25519 instruction and also
  one same-index or later fully-matching Ed25519 instruction, should the real
  runtime layer:
  - accept the strictly-prior match and ignore the same/later match, matching
    Phase 41C3 descriptor semantics; or
  - reject the transaction globally as anomalous real runtime evidence?

This decision must be made explicitly in 41D3 or in a reviewed pre-41D3 plan.

It must not be inherited accidentally from descriptor-layer behavior.

41D3 must not:

- accept verification evidence
- verify Ed25519 signatures
- count quorum
- authorize minting
- write replay state
- mutate runtime/account state
- perform CPI
- call `invoke_signed`
- call SPL Token `mint_to`
- unlock the live route

## Runtime API / Helper Plan

41D0 does not select final implementation code.

Later phases must explicitly document the chosen runtime APIs or helpers before
using them.

The plan is:

- 41D1:
  - use runtime `AccountInfo` only for Instructions sysvar presence/readability
  - no instruction content loading

- 41D2:
  - derive current-instruction identity from reviewed runtime context only
  - no prior-instruction enumeration

- 41D3:
  - use a checked instruction-loading helper such as
    `load_instruction_at_checked` or a reviewed equivalent
  - no unchecked index access
  - no unchecked sysvar parsing
  - no panic on malformed sysvar contents

## Panic-Safety Requirements

Every real runtime read phase must satisfy:

- no `unwrap`
- no `expect`
- no `panic!`
- no `unsafe`
- no unchecked indexing
- no unchecked sysvar read
- no unchecked instruction index
- no out-of-bounds read
- no borrow-failure panic
- no overflow-dependent logic
- deterministic failure mapping
- no state mutation on failure

Borrow failure must map to deterministic rejection.

Malformed sysvar data must map to deterministic rejection.

Missing sysvar account must map to deterministic rejection.

Out-of-range instruction index must map to deterministic rejection.

No real runtime read may silently fall through to authorization.

## Deterministic Error Mapping Requirements

41D1 must map runtime read outcomes into Phase 41C1 states.

41D2 must map runtime identity outcomes into Phase 41C2 states.

41D3 must map real prior-instruction classification outcomes into Phase 41C3
states.

A mapping failure must be a rejection, not a panic.

A missing input must be a rejection, not a panic.

A malformed input must be a rejection, not a panic.

An ambiguous input must be a rejection, not a panic.

## Per-Flag Transition Plan

Phase 41D must flip flags only when the corresponding reviewed boundary is
actually implemented.

Expected plan:

| Flag | 41D0 | 41D1 | 41D2 | 41D3 |
| --- | --- | --- | --- | --- |
| `account_info_parser_implemented` | false | may become true only for presence/readability | true if already flipped | true if already flipped |
| `raw_instructions_sysvar_parser_implemented` | false | false | false | may become true only for reviewed enumeration/classification |
| `load_instruction_called` | false | false | false | may become true only for checked prior enumeration |
| `current_instruction_identity_derived_from_runtime` | false | false | may become true | true if already flipped |
| `locates_prior_ed25519_instruction_from_runtime` | false | false | false | may become true |
| `cryptographic_signature_proof_accepted` | false | false | false | false |
| `verification_evidence_accepted` | false | false | false | false |
| `quorum_counting_enabled` | false | false | false | false |
| `authorization_enabled` | false | false | false | false |
| `replay_write_enabled` | false | false | false | false |
| `account_mutation_enabled` | false | false | false | false |
| `cpi_enabled` | false | false | false | false |
| `invoke_signed_enabled` | false | false | false | false |
| `spl_token_mint_to_enabled` | false | false | false | false |
| `live_route_enabled` | false | false | false | false |

No trust-sensitive flag may become true in 41D.

## Forbidden Operations Throughout 41D

Throughout Phase 41D, the following remain forbidden:

- Ed25519 cryptographic verification
- cryptographic proof acceptance
- verification evidence acceptance
- guardian quorum counting
- authorization
- replay writes
- processed event marking
- runtime/account mutation
- CPI
- `invoke_signed`
- SPL Token `mint_to`
- runtime execution handler
- live route unlock
- production Program ID selection
- deployment blocker removal
- production readiness claim
- final immutability claim while upgrade authority exists
- SBF build
- deploy artifact changes
- keypair file reads or writes
- `.env` reads or writes
- `.local-keys` inspection
- deploy commands
- network commands
- SOL spending

## 41D Review Gates

41D0 must be reviewed before 41D1.

41D1 must be reviewed before 41D2.

41D2 must be reviewed before 41D3.

41D3 must be reviewed before any later proof/evidence phase.

No 41D phase may be merged without external review.

## Active Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 41D0.

## Phase 41D0 Closure Criteria

Phase 41D0 may be closed only if reviewers confirm:

- the plan is docs-only
- the 41D split is safe
- each 41D subphase introduces only one real read layer
- 41D1 is limited to presence/readability
- 41D2 is limited to current-instruction identity
- 41D3 is limited to prior enumeration and descriptor construction
- the 41C3A pre-filter contract is preserved
- the same/later anomaly decision is explicitly carried forward
- proof, quorum, authorization, replay, CPI, mint execution, and live route
  remain forbidden

# XXXL Atomic Mutation Planning Notes Boundary

Status: COMPLETED.

This document records planning notes for future XXXL SVM atomic mutation work.

It is a documentation-only boundary.

No runtime code is changed by this stage.

## Purpose

The goal is to define the safety constraints that must be preserved before any future runtime implementation introduces real account writes, SPL CPI execution, or minting.

This document does not activate mutation.

This document does not implement SPL CPI.

This document does not enable minting.

This document exists so that future implementation work starts from explicit safety constraints rather than from ad hoc runtime edits.

## Current runtime state

The current runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

Current release decision:

- release allowed: `false`
- release blocked: `true`
- primary blocker code: `RUNTIME_SAFETY_LOCK_ACTIVE`

## Atomicity requirement

Future XXXL mint execution must be atomic at the transaction level.

The required future atomic operation is:

- verify message
- verify route
- verify source chain
- verify source token
- verify finality or finality-bound condition
- verify recipient binding
- verify amount binding
- verify guardian quorum
- verify burn has not already been processed
- execute XXXL mint
- mark the canonical burn event as processed
- write any required proof or audit record

The future runtime must not allow a successful mint without the processed-burn mark.

The future runtime must not allow a processed-burn mark without the corresponding successful mint.

The future runtime must not split mint and processed-burn mark into separate user-visible completion paths.

## Check-before-mark requirement

The processed-burn registry must preserve the Stage 1 model rule:

- check first
- mutate only after all checks pass
- mark processed only as part of the successful mint path

The registry must not be marked if any of the following fails:

- message verification
- route verification
- source chain verification
- source token verification
- finality check
- recipient check
- amount check
- guardian quorum check
- replay check
- mint account check
- mint authority check
- token program check
- SPL CPI execution

## No-state-change-on-failure requirement

Every failure path must leave runtime state unchanged.

This includes failures from:

- malformed instruction data
- wrong schema version
- wrong route id
- wrong source chain id
- wrong source token
- wrong canonical event key
- wrong source burn transaction hash
- wrong source burn event index
- wrong source block data
- wrong recipient hash
- empty or invalid recipient
- wrong burned amount
- wrong XXXL mint amount
- expired deadline or insufficient finality
- missing guardian approval
- unknown guardian
- duplicated guardian approval
- invalid guardian signature
- insufficient quorum
- already processed canonical burn event
- wrong XXXL mint account
- wrong mint authority PDA
- wrong token program
- wrong recipient token account
- failed SPL CPI

The future implementation must make these failure paths testable.

## Future account write boundary

A future implementation must explicitly classify all accounts as read-only or writable before runtime mutation is enabled.

Expected writable accounts may include:

- processed burn registry PDA
- proof or audit log PDA
- XXXL mint account
- recipient token account

Expected read-only or constrained accounts may include:

- runtime config PDA
- route config PDA
- guardian set PDA
- source chain config PDA
- SPL Token program
- system program, if required
- rent sysvar, if required

No account should become writable without a documented reason.

No account should be trusted only because it is passed by the caller.

## Future SPL CPI boundary

Future SPL CPI execution must remain unreachable until all checks pass.

The future SPL CPI mint path must verify:

- expected token program
- expected XXXL mint account
- expected mint authority PDA
- expected signer seeds
- expected recipient token account
- expected mint amount
- expected mint decimals assumptions, if any
- expected owner or authority constraints for recipient token account, if applicable

SPL CPI must not be callable as a standalone route.

SPL CPI must not be callable before replay protection is resolved.

SPL CPI must not be callable before quorum is verified.

SPL CPI must not be callable before message and recipient binding are verified.

## Mint authority PDA requirement

The future mint authority PDA must be derived from explicit documented seeds.

The derivation must be deterministic.

The derivation must be tested.

The derivation must not depend on caller-controlled ambiguous data.

The signer seeds used by `invoke_signed` must match the reviewed PDA derivation exactly.

No placeholder Program ID PDA fixture should be treated as production PDA data.

## Processed-burn registry requirement

The processed-burn registry key must be tied to the canonical burn event.

The canonical burn event identity must remain stable across relayers.

The registry must prevent replay even if the same message is submitted by another relayer.

The registry must prevent replay even if guardian approvals are reordered.

The registry must prevent replay even if duplicate guardian signatures are included.

## Proof or audit log requirement

If a future proof or audit log is written, it must not create a second source of truth that can contradict the processed-burn registry.

The processed-burn registry remains the replay-protection authority.

Any proof or audit log should be derived from the same verified message fields.

A failed proof or audit log write must not leave minting in a partially completed state.

## Ordering requirement

The intended future logical order is:

1. parse instruction data
2. load and validate config accounts
3. verify route and source constraints
4. verify message hash and canonical fields
5. verify recipient binding
6. verify amount binding
7. verify finality or finality-bound condition
8. verify guardian approvals and quorum
9. verify processed-burn registry is empty or unprocessed
10. verify SPL mint accounts and mint authority
11. execute SPL CPI mint
12. mark canonical burn event as processed
13. write proof or audit log, if required

Implementation may reorder internal read-only checks for efficiency.

Implementation must not move state mutation before required verification.

## Review requirements before implementation

Before runtime mutation is implemented, the following must be reviewed:

- account list
- writable account set
- PDA seeds
- signer seeds
- token program constraints
- mint account constraints
- recipient token account constraints
- processed-burn registry key derivation
- proof or audit log model
- exact failure behavior
- exact atomicity behavior
- tests for no-state-change-on-failure
- tests for replay prevention
- tests for CPI account substitution
- tests for wrong mint authority
- tests for wrong token program
- tests for failed SPL CPI

## Non-goals

This document does not implement atomic mutation.

This document does not enable runtime account writes.

This document does not enable live route execution.

This document does not enable SPL CPI execution.

This document does not enable `invoke_signed`.

This document does not enable SPL Token `mint_to`.

This document does not enable XXXL minting.

This document does not select a real Program ID.

This document does not regenerate production PDA fixtures.

This document does not remove deployment blockers.

This document does not change deployability predicates.

## Decision

The XXXL atomic mutation planning notes boundary is accepted.

The current runtime remains scaffold-only, locked, unreleasable, and not deployable.

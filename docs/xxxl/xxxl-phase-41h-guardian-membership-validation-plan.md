# XXXL Phase 41H — Guardian Membership Validation Plan

Date: 2026-07-03

## Status

Docs-only planning checkpoint.

No runtime code is introduced.

No `.rs` file is changed.

No verification logic is changed.

No guardian quorum authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41G.3 focused audit accepted:

`f910152 Merge XXXL phase 41G payload binding focused audit`

## Purpose

Phase 41H plans the next narrow trust boundary after payload hash binding:

verified signer public key → guardian set membership.

The goal is not to authorize execution.

The goal is not to count final quorum authorization.

The goal is not to mint.

The goal is only to define how a signer that has already been cryptographically verified by the SVM Ed25519 path can be checked against a configured guardian set.

## Current Architecture Position

Accepted chain so far:

1. Phase 41F: native SVM Ed25519 verification boundary.
2. Phase 41G.2: payload hash binding boundary.
3. Phase 41G.3: payload binding negative matrix and focused audit.

Next planned chain:

4. Phase 41H: guardian membership validation.
5. Future Phase 41I: quorum counting / threshold authorization.
6. Future Phase 41J+: replay protection, state mutation, CPI mint, and live route wiring.

## Existing Phase 35 Structural Verifier

The repository already contains a structural guardian quorum verifier.

Existing structural verifier capabilities:

- guardian set is not empty;
- threshold is not zero;
- threshold does not exceed guardian count;
- duplicate guardian public keys are rejected;
- empty approvals are rejected;
- guardian set ID mismatch is rejected;
- unknown guardian is rejected;
- duplicate approval is rejected;
- insufficient quorum is rejected.

Important limitation:

The existing structural verifier operates on structural `GuardianApprovalClaim` data.

It does not by itself prove that the approval came from a signer verified by SVM Ed25519.

It does not by itself bind a verified Ed25519 public key to guardian membership.

It does not accept cryptographic proof.

It does not authorize execution.

## Why 41H Must Be Separate From Quorum

Guardian membership and quorum authorization are separate trust steps.

41H should answer only:

“Is this SVM-verified signer public key a member of the configured guardian set?”

41H must not answer:

“Is quorum reached?”

41H must not answer:

“Is execution authorized?”

Quorum counting requires multiple accepted guardian membership results and belongs to a later high-risk gate.

## 41H Boundary Goal

Planned future boundary:

`establish_guardian_membership_validation(...)`

Provisional conceptual inputs:

- accepted Phase 41F native Ed25519 verification result;
- accepted Phase 41G payload hash binding result;
- verified signer public key bytes from the Ed25519 instruction evidence path;
- configured guardian set reference;
- expected guardian set ID.

Provisional conceptual output:

- narrow `GuardianMembershipValidated` marker/status;
- matched guardian public key;
- matched guardian index if needed for later quorum de-duplication;
- guardian set ID;
- no authorization marker;
- no quorum marker;
- no replay marker;
- no mutation marker;
- no mint marker.

## Source Of Signer Public Key

41H must not trust a caller-provided guardian approval claim.

The guardian public key must come from the verified Ed25519 evidence path.

If the accepted Phase 41F result does not directly expose a trusted signer public key, then 41H must depend on a separately reviewed extraction/binding boundary before membership validation is implemented.

Required rule:

caller-provided guardian public key is not trusted.

Allowed source:

verified Ed25519 instruction public key bytes extracted from the already accepted SVM verification evidence path.

## Required Preconditions

41H membership validation must require:

1. Phase 41F native Ed25519 verification established.
2. Phase 41G payload hash binding established.
3. verified signer public key bytes available from a reviewed extraction path.
4. guardian set ID is available from configured guardian set data.
5. guardian set is structurally valid.
6. signer public key is present in the guardian set.

## Rejected Preconditions

41H must reject:

- Phase 41F not established;
- Phase 41G payload hash binding not established;
- missing verified signer public key;
- caller-provided signer public key without verified extraction;
- empty guardian set;
- threshold zero;
- threshold greater than guardian count;
- duplicate guardian public key in guardian set;
- guardian set ID mismatch;
- verified signer not in guardian set.

## Relationship To Existing Phase 35

41H may reuse existing Phase 35 structural types or logic only if reuse does not overclaim trust.

Allowed reuse:

- `GuardianPublicKey`;
- `GuardianSetRef`;
- duplicate guardian public key checks;
- threshold sanity checks;
- unknown guardian rejection logic.

Forbidden reuse interpretation:

- do not treat structural quorum as cryptographic quorum;
- do not treat `GuardianApprovalClaim` as trusted proof;
- do not treat `quorum_reached` from structural data as execution authorization;
- do not treat Phase 35 success as sufficient for minting.

## Membership Does Not Mean Quorum

A successful 41H membership validation may mean only:

- one SVM-verified signer public key belongs to the configured guardian set.

It must not mean:

- enough guardians approved;
- quorum threshold reached;
- all approvals are unique;
- authorization exists;
- replay is safe;
- minting is allowed.

## Membership Does Not Mean Proof Acceptance

41H membership validation must not set:

- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `quorum_counting_enabled`;
- `authorization_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

41H may introduce a narrow local marker such as:

- `guardian_membership_validated: true`.

But that local marker must not be conflated with proof/evidence/quorum/auth/execution.

## Expected 41H Report Taxonomy

A future 41H report may say true for:

- guardian membership boundary exists;
- guardian set structural sanity checks exist;
- verified signer public key membership check exists;
- unknown guardian rejected;
- duplicate guardian public key rejected;
- guardian set ID mismatch rejected.

It must keep false for:

- quorum authorization;
- source burn proof acceptance;
- verification evidence acceptance;
- replay safety;
- mutation;
- CPI;
- mint;
- handler;
- live route.

## Expected 41H Error Model

A future 41H implementation should be fail-closed.

Expected error kinds:

- Phase41FNotEstablished;
- PayloadHashBindingNotEstablished;
- VerifiedSignerPublicKeyMissing;
- CallerProvidedGuardianClaimRejected;
- EmptyGuardianSet;
- InvalidThresholdZero;
- ThresholdExceedsGuardianSet;
- DuplicateGuardianPublicKey;
- GuardianSetIdMismatch;
- VerifiedSignerNotGuardian.

A failure must not produce a partial membership marker.

## Expected 41H Tests

A future implementation plan should require tests for:

- valid verified signer is accepted as guardian member;
- Phase 41F not established rejected;
- Phase 41G payload binding not established rejected;
- missing verified signer public key rejected;
- caller-provided guardian key not trusted;
- empty guardian set rejected;
- threshold zero rejected;
- threshold greater than guardian set rejected;
- duplicate guardian public key rejected;
- guardian set ID mismatch rejected;
- verified signer not in guardian set rejected;
- success does not set quorum/auth/replay/mutation/CPI/mint/live flags;
- failure paths are fail-closed.

## Explicitly Out Of Scope For 41H

41H must not implement:

- quorum counting;
- threshold authorization;
- multi-guardian approval aggregation;
- replay protection;
- processed event marking;
- account mutation;
- CPI;
- SPL Token mint;
- handler wiring;
- live route.

Those remain later reviewed boundaries.

## High-Risk Forward Chain

After 41H, the high-risk chain should be reviewed in separate gates:

- Phase 41I: quorum counting / threshold authorization plan;
- Phase 41J: replay protection plan;
- Phase 41K: account mutation and processed-event marking plan;
- Phase 41L: CPI mint execution plan;
- Phase 41M: handler/live route integration plan.

No later phase should be bundled into 41H.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by 41H planning.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Forward Risk Reminder

The live-wiring Model A precondition from Phase 41F.2 remains a future high-risk audit item.

41H does not close handler integration risk.

41H only plans guardian membership validation.

## Review Questions

External review should answer:

1. Is 41H correctly scoped as guardian membership validation, not quorum authorization?
2. Is the separation from existing Phase 35 structural quorum verifier correct?
3. Is it correct that caller-provided `GuardianApprovalClaim` must not be trusted?
4. Is verified signer public key extraction/binding correctly identified as a precondition?
5. Are Phase 41F and Phase 41G prerequisites correct?
6. Are accepted and rejected preconditions complete?
7. Is the trust taxonomy correct?
8. Are false flags correctly preserved?
9. Is the error model fail-closed?
10. Are expected tests sufficient?
11. Is quorum/authorization correctly deferred to a later phase?
12. Can 41H proceed to implementation planning after acceptance?

## Next Gate

After external acceptance of this plan, create the 41H plan acceptance record.

Then create a separate 41H implementation plan.

No `.rs` implementation should begin until the implementation plan is separately reviewed and accepted.

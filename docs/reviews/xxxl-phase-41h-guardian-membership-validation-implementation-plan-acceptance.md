# XXXL Phase 41H — Guardian Membership Validation Implementation Plan Acceptance

Date: 2026-07-03

## Accepted Main

`4a2b962 Merge XXXL phase 41H guardian membership implementation plan`

## Parent Gate

`0fa2220 Merge XXXL phase 41H guardian membership validation plan acceptance`

## Plan

`docs/xxxl/xxxl-phase-41h-guardian-membership-validation-implementation-plan.md`

## Checkpoint

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-guardian-membership-validation-implementation-plan.md`

## Review Request

`docs/reviews/xxxl-phase-41h-guardian-membership-validation-implementation-plan-review-request.md`

## Final Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Blocking risks: none.

Phase 41H guardian membership validation implementation planning is accepted.

A narrow `.rs` implementation may begin after this acceptance record.

The implementation must stay inside the accepted boundary.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- Guardian-set provenance closed: yes
- Authoritative guardian set wrapper sufficient: yes
- Caller-supplied guardian set rejection sufficient: yes
- Signer provenance bound to 41F.1/41F.2: yes
- Phase 41G prerequisite acceptable: yes
- Required checks complete: yes
- Required rejections complete: yes
- Error kinds sufficient: yes
- Phase 35 reuse constrained: yes
- Trust taxonomy preserved: yes
- False flags preserved: yes
- Tests sufficient: yes
- Forbidden operations excluded: yes
- `.rs` implementation may begin after acceptance: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Guardian-set provenance closed: yes
- Authoritative guardian set wrapper sufficient for this phase: yes, with constructor constraint note
- Signer provenance binding sufficient: yes
- Caller-supplied guardian set rejection sufficient: yes
- Required checks complete: yes, with guardian_set_id payload linkage note
- Required rejections complete: yes
- Error kinds sufficient: yes
- Tests sufficient: yes
- Phase 35 reuse constrained: yes
- Trust taxonomy preserved: yes
- False flags preserved: yes
- Forbidden operations excluded: yes
- `.rs` implementation may begin after acceptance: yes

## Accepted Implementation Boundary

The accepted 41H implementation boundary is:

`verified_signer_public_key ∈ authoritative_guardian_set`

This means one SVM-verified signer public key is checked for membership in one authoritative guardian set.

It does not mean quorum.

It does not mean authorization.

It does not mean replay safety.

It does not mean mutation.

It does not mean CPI.

It does not mean mint.

It does not mean handler or live route enablement.

## Accepted Signer Provenance

The signer operand must be derived from:

- Phase 41F.1 extracted Ed25519 public key bytes;
- Phase 41F.2 native SVM Ed25519 verification result.

Required binding:

- Phase 41F.2 status is `NativeEd25519VerificationEstablished`;
- Phase 41F.2 verified public key range equals Phase 41F.1 extracted public key range;
- Phase 41F.2 matched instruction index equals Phase 41F.1 matched instruction index;
- Phase 41F.2 instruction data length equals Phase 41F.1 instruction data length.

Caller-provided `GuardianApprovalClaim` must not be trusted.

Caller-provided signer public key must not be trusted.

## Accepted Guardian-Set Provenance

The guardian-set operand must come through an authoritative provenance wrapper or equivalent.

Planned type name:

`AuthoritativeGuardianSetRef`

The wrapper must represent:

- guardian set ID;
- threshold;
- guardian public keys;
- source marker: program-controlled / on-chain;
- caller instruction data marker: false.

The wrapper must not represent arbitrary caller instruction data as authoritative.

## Mandatory `.rs` Note 1 — Wrapper Constructor Constraint

Audit Demon noted that `AuthoritativeGuardianSetRef` is a provenance marker, not an enforcement mechanism by itself.

Therefore, the `.rs` implementation must not expose an unrestricted public constructor that allows arbitrary caller-controlled data to be marked as authoritative.

Required code-design rule:

- wrapper fields should not be publicly forgeable;
- production constructor must be constrained;
- unauthenticated caller data must not be able to construct `AuthoritativeGuardianSetRef`;
- if a test-only constructor is needed, it must be clearly test-only or fixture-only;
- the future authenticated guardian-set account-loading boundary must be the real production source of this wrapper.

The implementation must avoid this anti-pattern:

`AuthoritativeGuardianSetRef { source = ProgramControlled, caller_data = false, guardians = caller_supplied_keys }`

when the input is actually caller-controlled.

## Mandatory `.rs` Note 2 — Payload Guardian Set ID Linkage

Audit Demon noted that guardian set ID must be linked to the signed payload, not only to a locally expected configured ID.

Membership should be checked against the guardian set named by the signed payload.

Required code-design rule:

- the 41G-bound signed payload guardian set ID must equal the authoritative guardian set ID;
- the implementation must not pair a signer from one guardian set with a payload declaring another guardian set;
- this matters for guardian rotation safety.

Required check, or explicitly named future gate with justification:

`payload.guardian_set_id == authoritative_guardian_set.guardian_set_id`

The preferred direction for 41H `.rs` is to make this binding explicit now.

If the current payload decoder/result does not expose guardian_set_id yet, the `.rs` phase must either:

- add a narrow accepted way to pass the 41G-bound payload guardian set ID into 41H; or
- stop and create a separate named gate before implementation proceeds further.

## Accepted Error Direction

The future implementation must include semantic errors equivalent to:

- `Phase41FNotEstablished`
- `Phase41FExtractionNotEstablished`
- `VerifiedSignerPublicKeyMissing`
- `VerifiedSignerRangeMissing`
- `VerifiedSignerRangeMismatch`
- `MatchedInstructionIndexMismatch`
- `InstructionDataLengthMismatch`
- `PayloadHashBindingNotEstablished`
- `UnauthenticatedGuardianSet`
- `CallerSuppliedGuardianSetRejected`
- `EmptyGuardianSet`
- `InvalidThresholdZero`
- `ThresholdExceedsGuardianSet`
- `DuplicateGuardianPublicKey`
- `GuardianSetIdMismatch`
- `VerifiedSignerNotGuardian`

Additional required or expected error from Demon note 2:

- `PayloadGuardianSetIdMismatch`

Naming may be adjusted, but semantic coverage is mandatory.

## Accepted Test Direction

The future implementation must include tests for the accepted implementation-plan cases.

Additional mandatory tests from acceptance notes:

1. unrestricted caller construction of authoritative guardian set is impossible or rejected;
2. caller-supplied guardian set cannot be marked authoritative and pass;
3. signed payload guardian set ID mismatch is rejected;
4. a signer from one guardian set cannot be paired with a payload declaring another guardian set;
5. success still does not set quorum/auth/replay/mutation/CPI/mint/handler/live flags.

## Accepted Phase 35 Reuse Constraint

Allowed:

- structural concepts;
- `GuardianPublicKey`;
- duplicate guardian key check idea;
- threshold sanity idea;
- unknown guardian rejection idea.

Forbidden:

- `GuardianApprovalClaim` as proof;
- structural quorum as cryptographic quorum;
- `quorum_reached` as authorization;
- Phase 35 success as mint-sufficient;
- caller-supplied guardian set becoming authoritative.

## False Flags Preserved

41H implementation must keep false:

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

A local marker such as `guardian_membership_validated` may be true only on success.

## Forbidden Operations

41H implementation must not introduce:

- handler;
- `process_instruction`;
- runtime account loading;
- `AccountInfo`;
- account deserialization from runtime accounts;
- sysvar loading;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- replay write;
- processed event marking;
- account mutation;
- quorum counting;
- authorization;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Begin narrow Phase 41H `.rs` implementation:

`programs/xxxl-svm/src/verifier/guardian_membership_validation_boundary.rs`

Also update:

`programs/xxxl-svm/src/verifier/mod.rs`

No other runtime surface should be touched unless a blocker appears.

The code review must focus especially on:

1. constrained wrapper construction;
2. payload guardian set ID linkage;
3. preservation of false downstream flags;
4. no quorum/auth/replay/mutation/CPI/mint/handler/live route.

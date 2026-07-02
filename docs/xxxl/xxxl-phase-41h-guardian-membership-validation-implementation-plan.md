# XXXL Phase 41H — Guardian Membership Validation Implementation Plan

Date: 2026-07-03

## Status

Docs-only implementation plan.

No runtime code is introduced by this plan.

No `.rs` file is changed by this plan.

No guardian quorum authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41H guardian membership validation plan acceptance:

`0fa2220 Merge XXXL phase 41H guardian membership validation plan acceptance`

## Purpose

This implementation plan defines the narrow code boundary for Phase 41H.

The future code boundary should validate:

`verified_signer_public_key ∈ authoritative_guardian_set`

This is guardian membership validation only.

It is not quorum counting.

It is not authorization.

It is not replay protection.

It is not mint execution.

## Mandatory Demon Note Closed Here

Demon identified a provenance asymmetry in the accepted 41H plan.

Membership validation has two operands:

1. `verified_signer_public_key`
2. `guardian_set`

Both operands require trusted provenance.

This implementation plan closes that note:

- signer public key must come from the reviewed Ed25519 evidence/extraction path;
- guardian set must come from an authoritative program-controlled / on-chain source;
- neither operand may be trusted from caller instruction data.

## Existing Sources

### Signer Public Key Source

Existing Phase 41F.1 extraction result exposes:

`public_key_bytes: &[u8; ED25519_PUBLIC_KEY_LEN]`

This is the only acceptable signer public key byte source for 41H.

### Native Ed25519 Verification Source

Existing Phase 41F.2 result establishes native SVM Ed25519 verification through:

`NativeEd25519VerificationEstablished`

and provides verified ranges.

41H must require Phase 41F.2 success and must bind the Phase 41F.1 extracted public key range to the Phase 41F.2 verified public key range.

### Payload Binding Source

Existing Phase 41G.2 result establishes payload hash binding.

41H must require Phase 41G.2 success.

### Guardian Set Source

41H must not accept a raw caller-provided guardian set.

41H must require an explicit authoritative guardian set wrapper.

The wrapper is a local provenance marker for this boundary.

It must represent a guardian set loaded from, or standing in for, a program-controlled / on-chain source.

It must record that the following are not caller instruction data:

- guardian set ID;
- threshold;
- guardian public key list.

## Planned New Module

Future implementation should add:

`programs/xxxl-svm/src/verifier/guardian_membership_validation_boundary.rs`

And export it from:

`programs/xxxl-svm/src/verifier/mod.rs`

No other `.rs` files should change except module export, unless reviewers explicitly accept a narrower alternative.

## Planned Function

Future implementation should introduce:

`establish_guardian_membership_validation(...)`

Conceptual signature:

`establish_guardian_membership_validation(phase_41f_result, phase_41g_result, extraction_result, authoritative_guardian_set) -> Result<GuardianMembershipValidated, GuardianMembershipValidationError>`

The exact Rust types may be adjusted during code implementation, but the boundary must preserve the following input provenance rules.

## Planned Inputs

### Required Signer Inputs

- `Phase41F_2Ed25519SignatureVerificationResult`
- `Phase41F_1CheckedByteExtractionResult`
- `PayloadHashBindingEstablished`

The signer public key must be derived from:

`extraction_result.extracted_slices.public_key_bytes`

41H must not accept:

- `GuardianApprovalClaim` as proof;
- caller-provided signer public key;
- signer public key supplied in instruction data outside the reviewed Ed25519 evidence path.

### Required Guardian Set Input

Planned wrapper:

`AuthoritativeGuardianSetRef`

Required semantic fields:

- guardian set ID;
- threshold;
- guardian public keys;
- source marker: program-controlled / on-chain;
- caller instruction data marker: false.

41H must not accept:

- raw caller-provided `GuardianSetRef`;
- guardian set ID from caller instruction data;
- threshold from caller instruction data;
- guardian public key list from caller instruction data;
- any wrapper that marks the guardian set as unauthenticated or caller-supplied.

## Planned Output

Future success type:

`GuardianMembershipValidated`

Allowed fields:

- status marker;
- matched guardian public key;
- matched guardian index;
- guardian set ID;
- local marker `guardian_membership_validated: true`.

Forbidden fields:

- quorum reached;
- authorization marker;
- replay marker;
- mutation marker;
- CPI marker;
- mint marker;
- handler marker;
- live route marker.

## Required Checks

The implementation must perform checks in this order or an equivalent fail-closed order:

1. Phase 41F.2 status is `NativeEd25519VerificationEstablished`.
2. Phase 41F.2 `establishes_native_ed25519_verification == true`.
3. Phase 41F.1 extraction status is `CheckedEd25519ByteSlicesExtracted`.
4. Phase 41F.1 extracted slices exist.
5. Phase 41F.2 verified ranges exist.
6. Phase 41F.2 verified public key range equals Phase 41F.1 extracted public key range.
7. Phase 41F.2 matched instruction index matches Phase 41F.1 matched instruction index.
8. Phase 41F.2 instruction data length matches Phase 41F.1 instruction data length.
9. Phase 41G.2 payload hash binding is established.
10. Guardian set wrapper source is authoritative/program-controlled/on-chain.
11. Guardian set wrapper is not caller instruction data.
12. Guardian set is not empty.
13. Threshold is not zero.
14. Threshold does not exceed guardian count.
15. Guardian set has no duplicate guardian public keys.
16. Guardian set ID matches expected configured guardian set ID.
17. Verified signer public key is present in the guardian set.

## Required Rejections

The implementation must reject:

- Phase 41F not established;
- Phase 41F established flag false;
- Phase 41F.1 extraction not established;
- extracted signer public key missing;
- verified ranges missing;
- extracted public key range mismatch;
- matched instruction index mismatch;
- instruction data length mismatch;
- Phase 41G payload hash binding not established;
- unauthenticated guardian set;
- caller-supplied guardian set;
- caller-supplied guardian set ID;
- caller-supplied threshold;
- caller-supplied guardian public key list;
- empty guardian set;
- threshold zero;
- threshold greater than guardian count;
- duplicate guardian public key;
- guardian set ID mismatch;
- verified signer not in guardian set.

## Required Error Kinds

Future error enum must include equivalents of:

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

Naming may be adjusted, but semantic coverage is mandatory.

## Relationship To Phase 35 Structural Verifier

The implementation may reuse structural concepts from Phase 35.

Allowed reuse:

- `GuardianPublicKey`;
- duplicate guardian public key detection idea;
- threshold sanity check idea;
- unknown guardian rejection idea.

Forbidden reuse:

- do not accept `GuardianApprovalClaim` as proof;
- do not accept raw structural quorum result as authorization;
- do not treat `quorum_reached` as execution authorization;
- do not treat Phase 35 success as sufficient for minting;
- do not allow caller-supplied guardian set data to become authoritative.

## Trust Taxonomy

Successful 41H membership validation may mean only:

- one SVM-verified signer public key is a member of the authoritative guardian set.

It must not mean:

- cryptographic proof accepted;
- verification evidence accepted;
- quorum reached;
- authorization exists;
- replay is safe;
- account mutation is allowed;
- CPI is allowed;
- mint is allowed;
- handler is enabled;
- live route is enabled.

## Required False Flags

The implementation must keep false:

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

## Panic / Allocation Requirements

Production implementation must use:

- no `unwrap`;
- no `panic`;
- no unchecked indexing;
- no unchecked slicing;
- no attacker-sized copied allocation.

The signer public key is fixed 32 bytes.

The guardian set list may be iterated by borrowed slice.

Any fixed-size public key copy into `GuardianPublicKey` is acceptable.

## Required Tests

Future implementation must include tests for:

1. valid verified signer accepted as guardian member;
2. Phase 41F not established rejected;
3. Phase 41F establishes flag false rejected;
4. Phase 41F.1 extraction not established rejected;
5. missing extracted signer public key rejected;
6. verified range missing rejected;
7. public key range mismatch rejected;
8. matched instruction index mismatch rejected;
9. instruction data length mismatch rejected;
10. Phase 41G payload hash binding not established rejected;
11. unauthenticated guardian set rejected;
12. caller-supplied guardian set rejected;
13. caller-supplied guardian set ID rejected;
14. caller-supplied threshold rejected;
15. caller-supplied guardian public key list rejected;
16. empty guardian set rejected;
17. threshold zero rejected;
18. threshold greater than guardian count rejected;
19. duplicate guardian public key rejected;
20. guardian set ID mismatch rejected;
21. verified signer not in guardian set rejected;
22. success does not set quorum/auth/replay/mutation/CPI/mint/live flags;
23. failure produces no partial membership marker.

## Forbidden Operations

Future implementation must not introduce:

- handler;
- `process_instruction`;
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

Runtime account loading for the real guardian set source must remain a later reviewed boundary unless explicitly accepted.

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

## Out Of Scope

This implementation plan does not implement or authorize:

- production guardian set account loading;
- quorum counting;
- threshold authorization;
- replay protection;
- processed event marking;
- account mutation;
- CPI mint;
- handler wiring;
- live route.

## Review Questions

External review should answer:

1. Does the implementation plan correctly close guardian-set provenance?
2. Is `AuthoritativeGuardianSetRef` or equivalent source wrapper sufficient?
3. Is caller-supplied guardian set rejection explicit enough?
4. Is signer public key provenance correctly bound to Phase 41F.1 and Phase 41F.2?
5. Is Phase 41G payload hash binding prerequisite correct?
6. Are required checks complete?
7. Are required rejections complete?
8. Are error kinds sufficient?
9. Is Phase 35 reuse constrained correctly?
10. Is trust taxonomy preserved?
11. Are false flags preserved?
12. Are tests sufficient?
13. Are forbidden operations sufficiently excluded?
14. Can `.rs` implementation begin after acceptance?

## Next Gate

After external acceptance of this implementation plan, a narrow `.rs` implementation may begin.

The implementation must be limited to the accepted boundary and must not introduce runtime account loading, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.

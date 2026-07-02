# XXXL Phase 41H — Guardian Membership Validation Plan Acceptance

Date: 2026-07-03

## Accepted Main

`930c940 Merge XXXL phase 41H guardian membership validation plan`

## Parent Gate

`f910152 Merge XXXL phase 41G payload binding focused audit`

## Plan

`docs/xxxl/xxxl-phase-41h-guardian-membership-validation-plan.md`

## Checkpoint

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-guardian-membership-validation-plan.md`

## Review Request

`docs/reviews/xxxl-phase-41h-guardian-membership-validation-plan-review-request.md`

## Final Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Blocking risks: none at the plan stage.

Phase 41H guardian membership validation planning is accepted.

Implementation planning may begin, but the guardian-set provenance note below must be closed before any `.rs` implementation begins.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- 41H scoped as membership only: yes
- Quorum/auth deferred correctly: yes
- 41H may proceed to implementation planning after acceptance: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none at the plan stage
- Blocking risks: none at the plan stage
- Scope drift: no
- Membership-only scope preserved: yes
- Phase 35 separation acceptable: yes
- Caller-provided `GuardianApprovalClaim` distrust sufficient: yes
- Verified signer public key precondition sufficient: yes
- Phase 41F prerequisite acceptable: yes
- Phase 41G prerequisite acceptable: yes
- Trust taxonomy acceptable: yes
- False flags preserved: yes
- Error model fail-closed: yes
- Quorum/auth deferred correctly: yes
- 41H may proceed to implementation planning after acceptance: yes

## Accepted Scope

41H is accepted as guardian membership validation planning only.

41H asks:

“Is this SVM-verified signer public key a member of the configured guardian set?”

41H does not ask:

- is quorum reached;
- is execution authorized;
- is replay safe;
- should state mutate;
- should CPI mint execute;
- should handler/live route be enabled.

## Accepted Trust Chain

Accepted prerequisites for future 41H implementation planning:

1. Phase 41F native SVM Ed25519 verification established.
2. Phase 41G payload hash binding established.
3. verified signer public key is available from a reviewed extraction/binding path.
4. guardian set is available from an authoritative source.
5. guardian set ID matches expected configured value.
6. guardian set is structurally valid.
7. verified signer public key is a member of that guardian set.

## Mandatory Downstream Note — Guardian Set Provenance

Demon identified an important asymmetry in operand provenance.

Membership validation is:

`verified_pubkey ∈ guardian_set`

Both operands must have trusted provenance.

The accepted plan strongly specifies provenance for `verified_pubkey`:

- it must come from the verified Ed25519 evidence path;
- it must not be caller-provided.

The plan was less explicit about provenance for `guardian_set`.

The 41H implementation plan must close this before code.

Mandatory implementation-plan rule:

- guardian set;
- guardian set ID;
- threshold;
- guardian public keys;

must come from an authoritative program-controlled / on-chain source.

They must never be trusted from caller instruction data.

## Mandatory Downstream Rejection Case

The 41H implementation plan must add an explicit rejected precondition:

- unauthenticated guardian set rejected;
- caller-supplied guardian set rejected;
- caller-supplied guardian set ID rejected;
- caller-supplied threshold rejected;
- caller-supplied guardian public key list rejected.

## Mandatory Downstream Error Kind

The 41H implementation plan must add an error kind equivalent to:

`UnauthenticatedGuardianSet`

or:

`CallerSuppliedGuardianSetRejected`

The exact name may be chosen during implementation planning, but the semantic case is mandatory.

## Mandatory Downstream Test

The 41H implementation plan must require a test equivalent to:

`caller_supplied_guardian_set_is_rejected`

The test must show that an attacker cannot submit a guardian set containing their own verified signer key and thereby pass membership validation.

## Accepted Phase 35 Separation

Existing Phase 35 structural guardian quorum verifier may inform structural checks.

Allowed reuse:

- `GuardianPublicKey`;
- `GuardianSetRef`;
- duplicate guardian public key checks;
- threshold sanity checks;
- unknown guardian rejection logic.

Forbidden interpretation:

- structural quorum is not cryptographic quorum;
- `GuardianApprovalClaim` is not trusted proof;
- `quorum_reached` is not authorization;
- Phase 35 success is not sufficient for minting.

## Accepted Caller-Provided Claim Rule

Caller-provided `GuardianApprovalClaim` must not be trusted.

A caller can fabricate a claim.

Only a public key extracted from the reviewed and accepted Ed25519 evidence path may be treated as the signer operand for membership validation.

## Accepted Membership Taxonomy

Successful 41H membership validation may mean only:

- one SVM-verified signer public key belongs to the authoritative configured guardian set.

It must not mean:

- quorum reached;
- enough guardians approved;
- proof accepted;
- verification evidence accepted;
- authorization exists;
- replay is safe;
- mutation is allowed;
- CPI is allowed;
- mint is allowed;
- handler/live route is enabled.

## False Flags Preserved

41H must keep the following false:

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

A future local marker such as `guardian_membership_validated` may be allowed only if it is not conflated with proof/evidence/quorum/auth/execution.

## Accepted Error Model Direction

The future 41H implementation plan should remain fail-closed.

Expected error classes include:

- Phase41FNotEstablished;
- PayloadHashBindingNotEstablished;
- VerifiedSignerPublicKeyMissing;
- CallerProvidedGuardianClaimRejected;
- UnauthenticatedGuardianSet / CallerSuppliedGuardianSetRejected;
- EmptyGuardianSet;
- InvalidThresholdZero;
- ThresholdExceedsGuardianSet;
- DuplicateGuardianPublicKey;
- GuardianSetIdMismatch;
- VerifiedSignerNotGuardian.

No failure path may produce a partial membership marker.

## Accepted Test Direction

The future 41H implementation plan should require tests for:

- valid verified signer is accepted as guardian member;
- Phase 41F not established rejected;
- Phase 41G payload binding not established rejected;
- missing verified signer public key rejected;
- caller-provided guardian key not trusted;
- caller-supplied guardian set rejected;
- empty guardian set rejected;
- threshold zero rejected;
- threshold greater than guardian set rejected;
- duplicate guardian public key rejected;
- guardian set ID mismatch rejected;
- verified signer not in guardian set rejected;
- success does not set quorum/auth/replay/mutation/CPI/mint/live flags;
- failure paths are fail-closed.

## Still Out Of Scope

41H still must not implement:

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

Those remain later reviewed gates.

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

## Forward Risk Reminder

The live-wiring Model A precondition from Phase 41F.2 remains a future high-risk audit item.

41H does not close handler integration risk.

41H only plans guardian membership validation.

## Next Gate

Create a separate Phase 41H guardian membership validation implementation plan.

The implementation plan must explicitly close guardian-set provenance before code.

No `.rs` implementation may begin until that implementation plan is reviewed and accepted.

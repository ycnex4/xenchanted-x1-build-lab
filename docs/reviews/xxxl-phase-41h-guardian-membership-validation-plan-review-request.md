# XXXL Phase 41H Review Request — Guardian Membership Validation Plan

Date: 2026-07-03

## Current Main

`f910152 Merge XXXL phase 41G payload binding focused audit`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41h-guardian-membership-validation-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-guardian-membership-validation-plan.md`

## Scope

Docs-only plan.

No `.rs` changes.

No runtime code.

No quorum authorization.

No replay write.

No account mutation.

No CPI.

No mint.

No handler.

No live route.

## Purpose

Review the plan for Phase 41H guardian membership validation.

41H should connect:

- Phase 41F native SVM Ed25519 verification;
- Phase 41G payload hash binding;
- verified signer public key;
- configured guardian set membership.

41H must not authorize execution.

## Requested Review

Please check:

1. Is 41H correctly scoped as guardian membership validation, not quorum authorization?
2. Is the separation from existing Phase 35 structural quorum verifier correct?
3. Is it correct that caller-provided GuardianApprovalClaim must not be trusted?
4. Is verified signer public key extraction/binding correctly identified as a precondition?
5. Are Phase 41F and Phase 41G prerequisites correct?
6. Are accepted and rejected preconditions complete?
7. Is the trust taxonomy correct?
8. Are false flags correctly preserved?
9. Is the error model fail-closed?
10. Are expected tests sufficient?
11. Is quorum/authorization correctly deferred to a later phase?
12. Can 41H proceed to implementation planning after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Scope drift: yes/no
- 41H scoped as membership only: yes/no
- Separation from Phase 35 structural verifier acceptable: yes/no
- Caller-provided GuardianApprovalClaim distrust correct: yes/no
- Verified signer public key precondition correct: yes/no
- Phase 41F prerequisite correct: yes/no
- Phase 41G prerequisite correct: yes/no
- Trust taxonomy acceptable: yes/no
- False flags preserved: yes/no
- Error model fail-closed: yes/no
- Expected tests sufficient: yes/no
- Quorum/auth deferred correctly: yes/no
- 41H may proceed to implementation planning after acceptance: yes/no

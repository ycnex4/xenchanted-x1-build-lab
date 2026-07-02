# XXXL Phase 41H Review Request — Guardian Membership Validation Implementation Plan

Date: 2026-07-03

## Current Main

`0fa2220 Merge XXXL phase 41H guardian membership validation plan acceptance`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41h-guardian-membership-validation-implementation-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41h-guardian-membership-validation-implementation-plan.md`

## Scope

Docs-only implementation plan.

No `.rs` changes yet.

No runtime code yet.

No quorum authorization.

No replay write.

No account mutation.

No CPI.

No mint.

No handler.

No live route.

## Purpose

Review the implementation plan for the narrow Phase 41H boundary:

`verified_signer_public_key ∈ authoritative_guardian_set`

The plan must close Demon’s guardian-set provenance note before code.

## Requested Review

Please check:

1. Does the implementation plan correctly close guardian-set provenance?
2. Is AuthoritativeGuardianSetRef or equivalent source wrapper sufficient?
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
14. Can .rs implementation begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Scope drift: yes/no
- Guardian-set provenance closed: yes/no
- Authoritative guardian set wrapper sufficient: yes/no
- Caller-supplied guardian set rejection sufficient: yes/no
- Signer provenance bound to 41F.1/41F.2: yes/no
- Phase 41G prerequisite acceptable: yes/no
- Required checks complete: yes/no
- Required rejections complete: yes/no
- Error kinds sufficient: yes/no
- Phase 35 reuse constrained: yes/no
- Trust taxonomy preserved: yes/no
- False flags preserved: yes/no
- Tests sufficient: yes/no
- Forbidden operations excluded: yes/no
- .rs implementation may begin after acceptance: yes/no

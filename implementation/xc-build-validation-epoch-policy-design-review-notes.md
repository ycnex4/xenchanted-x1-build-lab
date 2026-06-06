# XC Build validation epoch policy design review

This document reviews the XC Build validation epoch policy design milestone.

Reviewed branch:

    xc-build-validation-epoch-policy-design-review

Reviewed design milestone:

    xc-build-validation-epoch-policy-design

Reviewed files:

- implementation/xc-build-validation-epoch-policy-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build validation epoch policy design is accepted.

The design correctly separates historical contribution from current active validity.

The design correctly treats Core redeem proof as historical contribution.

The design correctly states that `history_bld` is historical and non-decreasing.

The design correctly uses XNTD lock / relock as the active validity layer.

The design correctly states that `currentEpoch` should not invalidate historical Core redeem history.

The design correctly removes Forge from MVP Build validity.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-validation-epoch-policy-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Forge scope review

Forge is correctly out of scope for MVP Build validity.

The design states:

- Forge participation is out of scope for MVP Build validity
- Forge is not required by this epoch policy
- Forge should not be used as an implicit Build activation requirement
- Forge should not be used as an implicit epoch validation requirement
- Forge should not be mixed into this milestone

This resolves the previous ambiguity and prevents old Forge-based assumptions from leaking into the MVP rule path.

## Historical contribution review

The design correctly defines Core redeem proof as historical.

A valid Core redeem proof should remain valid even if current XC epoch changes later.

The design correctly says Core redeem should not be rejected merely because:

- currentEpoch changed
- currentBaseNominal changed
- currentXenBurnAmount changed
- nextHalvingTs moved

This preserves honest historical participation.

## history_bld review

The design correctly states that `history_bld` is historical and non-decreasing.

Epoch policy may affect whether a Build is currently active, but it should not erase historical contribution.

This matches the intended Build model.

## Active validity review

The design correctly defines XNTD lock / relock as the active validity layer.

A Build may have historical contribution but still require current XNTD lock / relock compliance to be active.

The design correctly distinguishes:

- historical layer
- active validity layer

This avoids accidental invalidation of history while preserving current commitment requirements.

## Current epoch review

The design correctly says `currentEpoch` should not be used to reject historical Core redeem proof.

The design allows currentEpoch to be used later for focused active-status checks such as:

- whether a Build lock is current
- whether relock is needed
- what currentBaseNominal applies to new lock/relock requirements
- what current active validity means

This is the correct separation.

## Existing XNTD lock validation review

The design correctly keeps the existing XNTD lock validation path intact.

It does not replace:

- observedRequiredXntdLock propagation
- authoritative XC epoch minimum source
- registrar/app lock validation
- relock validation

Future enforcement should reuse the existing lock validation chain where possible.

## Boundary review

The future implementation should not add anything until a focused runtime milestone is designed.

This design does not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- global Build validity enforcement
- Forge requirements
- currentEpoch enforcement in code
- requiredXntdLockMinimum enforcement in code
- BLD transfer/sale rule changes

## Grep review

The review grep found architecture and boundary terms inside design/checkpoint text only.

That is expected.

No runtime files were added or changed by this design milestone.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC Build validation epoch policy design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-active-validity-rule-design

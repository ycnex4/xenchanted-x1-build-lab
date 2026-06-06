# XC Build validation epoch policy completion checkpoint

This document closes the XC Build validation epoch policy milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build validation epoch policy milestone completed the full progression:

1. epoch policy design
2. epoch policy design review
3. merge to main

## Current main status

Latest completed main milestone:

    main -> 11619b7 Merge branch 'xc-build-validation-epoch-policy-design-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Safe refusal without env remained active:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Completed documents

Design document:

    implementation/xc-build-validation-epoch-policy-design.md

Review document:

    implementation/xc-build-validation-epoch-policy-design-review-notes.md

Checkpoint update:

    docs/checkpoints/current-design-checkpoint.md

## Accepted MVP epoch policy

The accepted MVP epoch policy is:

    historical contribution remains historical
    active validity may require current XNTD lock/relock compliance
    currentEpoch should not invalidate Core redeem history
    Forge participation is out of scope for MVP Build validity

## Historical contribution layer

Core redeem proof is historical.

A valid Core redeem proof should remain valid even if current XC epoch changes later.

Core redeem contributes to historical Build value.

`history_bld` is historical and non-decreasing.

Epoch policy may affect active status, but it must not erase historical contribution.

## Active validity layer

XNTD lock / relock are the active validity layer.

A Build may have historical contribution but still require current XNTD lock / relock compliance to be active.

Relock is the normal path for refreshing active validity across epoch changes.

## Current epoch policy

`currentEpoch` should not be used to reject historical Core redeem proof.

`currentEpoch` may be used later to decide:

- whether a Build lock is current
- whether relock is needed
- what currentBaseNominal applies to new lock/relock requirements
- what current active validity means

Any such checks must be introduced only through focused future enforcement milestones.

## Forge scope

Forge participation is out of scope for MVP Build validity.

Forge is not required by this epoch policy.

Forge should not be used as an implicit Build activation requirement.

Forge should not be used as an implicit epoch validation requirement.

If Forge participation is ever reintroduced, it must be handled in a separate future design milestone.

## Boundary

This milestone does not add:

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

## Completed protocol-context position

The protocol-context pipeline remains:

    XcProtocolParams
    -> deriveCurrentXcBuildRequirements()
    -> XcBuildValidationContext
    -> optional xcBuildValidationContext in appSubmitProof()

Epoch policy now defines how this context should be interpreted before enforcement is added.

## Recommended next milestone

Recommended next design milestone:

    xc-build-active-validity-rule-design

Purpose:

- define the active Build validity rule
- decide how XNTD lock/relock state maps to active/inactive status
- preserve historical contribution even when active status changes
- keep Forge out of MVP validity
- prepare a focused implementation milestone after design review

## Decision

The XC Build validation epoch policy milestone is complete.

Next step should be design-only:

    xc-build-active-validity-rule-design

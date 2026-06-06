# XC Build active validity rule completion checkpoint

This document closes the XC Build active validity rule milestone.

This checkpoint is documentation-only.

No runtime code is changed in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build active validity rule milestone completed the full progression:

1. active validity rule design
2. active validity rule design review
3. merge to main

## Current main status

Latest completed main milestone:

    main -> b3e812f Merge branch 'xc-build-active-validity-rule-design-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 40 test files, 317 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Completed documents

Design document:

    implementation/xc-build-active-validity-rule-design.md

Review document:

    implementation/xc-build-active-validity-rule-design-review-notes.md

Checkpoint update:

    docs/checkpoints/current-design-checkpoint.md

## Accepted MVP active validity rule

The accepted MVP active validity rule is:

    Active status is an optional current-commitment signal.
    It is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

## Meaning of active status

Active status means the Build currently shows XNTD commitment.

For MVP, active status is based on XNTD lock / relock state.

Active status may be useful for:

- showing current commitment status
- filtering active participants
- optional eligibility signals
- future project-owned features
- external X1 project interpretation if they choose to use it

Active status is a signal, not a universal punishment rule.

## Meaning of inactive status

Inactive Build still keeps historical contribution.

Inactive Build does not mean:

- history is invalid
- Build is deleted
- Core redeem proof is rejected
- history_bld is reduced
- available_bld is automatically reduced
- external projects must ignore the Build

Inactive means:

    historical contribution exists, but current commitment is not active or not current.

## External project policy

External X1 projects may choose whether to use active status.

They may:

- ignore active status
- use only history_bld
- display active/inactive as context
- give bonuses to active Builds
- require active status for their own specific feature

The XC Build protocol should expose the signal, not force policy on every external project.

## XNTD lock / relock role

XNTD lock / relock are the basis of active status.

Lock shows initial current commitment.

Relock shows refreshed commitment across epoch changes.

If epoch conditions change, active status may require relock under the new accepted requirement.

This affects only the active signal and must not erase historical contribution.

## Forge scope

Forge participation is out of scope for MVP active validity.

Forge is not an active status requirement.

Forge is not an implicit Build activation requirement.

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
- active status enforcement
- external project policy
- inactive Build history erasure
- Forge requirements
- unlock mechanics
- BLD transfer/sale rule changes

## Recommended next milestone

Recommended next design milestone:

    xc-build-active-status-model-design

Purpose:

- define the exact active status model shape
- decide status values and reason codes
- decide how lockedXntd, requiredXntdLock, lockEpoch, and current context map to active/inactive
- keep the model read-only and non-mutating
- prepare a focused implementation milestone after review

## Decision

The XC Build active validity rule milestone is complete.

Next step should be design-only:

    xc-build-active-status-model-design

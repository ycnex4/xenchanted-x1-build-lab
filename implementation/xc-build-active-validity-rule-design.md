# XC Build active validity rule design

This document defines the active validity rule for XC Build.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The accepted MVP epoch policy separates:

    historical contribution

from:

    current active validity

Historical contribution remains historical.

Current active validity is an optional current-commitment signal based on XNTD lock / relock state.

## Design goal

Define what active Build means without treating inactive Build as invalid history.

The goal is to make active status useful, but not punitive.

## Core principle

Active status is not universal punishment.

Active status is a current-commitment signal.

Inactive Build still keeps historical contribution.

External X1 projects may choose whether to use active status.

They may:

- ignore active status
- use only history_bld
- require active status for their own feature
- give a bonus to active Builds
- display active status as an informational signal

This project should expose the signal, not force every project to treat inactive Builds as useless.

## Historical layer

The historical layer includes:

- Core redeem history
- history_bld
- available_bld mechanics
- origin_bld logic
- historical participation record

Inactive status must not erase this layer.

Inactive status must not set history_bld to zero.

Inactive status must not invalidate Core redeem proof.

Inactive status must not make the Build fake.

## Active layer

The active layer represents current commitment.

For MVP, active status should be based on XNTD lock / relock state.

A Build may be considered active when:

- it has historical contribution
- it has an accepted XNTD lock or relock state
- the lock/relock satisfies the accepted required lock rule for its epoch/current policy
- the active status is not stale under the selected epoch policy

The exact runtime computation should be implemented later in a focused milestone.

## Inactive meaning

Inactive means:

    historical contribution exists, but current commitment is not active or not current.

Inactive does not mean:

- history is invalid
- Build is deleted
- Core redeem proof is rejected
- history_bld is reduced
- available_bld is automatically reduced
- external projects must ignore the Build

## Why active status exists

Active status is useful because it shows current XNTD commitment.

For this project, XNTD lock is useful because it supports XC / XNTD economic alignment.

For external projects, active status is optional context.

The status gives them an extra signal, but does not force policy on them.

## XNTD lock / relock role

XNTD lock / relock are the basis of active status.

Lock shows initial current commitment.

Relock shows refreshed commitment across epoch changes.

If epoch conditions change, active status may require relock under the new accepted requirement.

This should not erase history.

It only affects the current active signal.

## Epoch relationship

currentEpoch should not invalidate historical Core redeem.

currentEpoch may help decide whether active status is current.

The policy should avoid accidental historical invalidation.

The safe model is:

    history persists
    active signal may expire or require refresh

## External project interpretation

External X1 projects are not required to use active status.

Possible interpretations:

1. Ignore active status

    Use history_bld or other historical metrics only.

2. Informational display

    Show active/inactive as context, without changing access.

3. Bonus model

    Give additional benefits to active Builds.

4. Requirement model

    Require active status for a specific feature.

This choice belongs to each project.

## Project-owned usage

This project may use active status for its own features.

Examples:

- showing current commitment status
- filtering active participants
- future access or reward features
- optional eligibility signals
- analytics around XNTD lock commitment

These are project-level choices and should not redefine historical validity.

## MVP wording

Recommended wording:

    Active Build means the Build has current XNTD commitment.

    Inactive Build keeps its historical contribution but does not currently show active XNTD commitment.

Avoid wording like:

    Inactive Build is invalid.

    Inactive Build loses its history.

    Inactive Build cannot be used by other projects.

    Inactive Build is punished.

## Non-goals

This milestone does not:

- add runtime code
- add tests
- add dependencies
- execute real RPC
- change appSubmitProof behavior
- change watcher behavior
- change registrar behavior
- change proof payload behavior
- enforce active status
- define external project policy
- erase inactive Build history
- introduce Forge requirements
- add unlock mechanics
- change BLD transfer/sale rules

## Recommended future implementation direction

A future implementation may expose an active status helper.

Possible name:

    isBuildActive()

or:

    getBuildActiveStatus()

The helper should return status without mutating Build history.

Possible output shape:

    {
      isActive,
      reason,
      requiredXntdLock,
      lockedXntd,
      lockEpoch
    }

This is only a future direction, not part of this milestone.

## Recommended future milestones

After this design is reviewed, recommended future milestones are:

1. active validity design review

    xc-build-active-validity-rule-design-review

2. active status model implementation

    xc-build-active-status-model

3. active status app/service integration design

    xc-build-active-status-app-integration-design

4. final MVP Build validation rule design

    xc-build-validation-mvp-rule-design

Forge is intentionally not included in this sequence.

## Decision

MVP active validity rule:

    Active status is an optional current-commitment signal.
    It is based on XNTD lock / relock state.
    Inactive Build keeps historical contribution.
    External X1 projects may choose whether to use active status.
    Forge participation is out of scope for MVP active validity.

Next step after this design:

    xc-build-active-validity-rule-design-review

# XC Build active validity rule design review

This document reviews the XC Build active validity rule design milestone.

Reviewed branch:

    xc-build-active-validity-rule-design-review

Reviewed design milestone:

    xc-build-active-validity-rule-design

Reviewed files:

- implementation/xc-build-active-validity-rule-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build active validity rule design is accepted.

The design correctly defines active status as an optional current-commitment signal.

The design correctly avoids treating inactive Build as invalid history.

The design correctly states that inactive Build keeps historical contribution.

The design correctly states that inactive Build does not lose `history_bld`.

The design correctly states that inactive Build does not invalidate Core redeem proof.

The design correctly allows external X1 projects to choose whether to use active status.

The design correctly keeps Forge out of MVP active validity.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-active-validity-rule-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Active status review

The design correctly defines active status as:

    optional current-commitment signal

This means active status is useful context, but not universal punishment.

It may be used by this project or external X1 projects, but it does not redefine historical validity.

## Inactive Build review

The design correctly states that inactive Build still keeps historical contribution.

Inactive Build does not mean:

- history is invalid
- Build is deleted
- Core redeem proof is rejected
- history_bld is reduced
- available_bld is automatically reduced
- external projects must ignore the Build

This preserves the intended separation between history and current commitment.

## External project policy review

The design correctly avoids forcing policy on external X1 projects.

External projects may:

- ignore active status
- use only history_bld
- display active/inactive as context
- give bonuses to active Builds
- require active status for their own specific feature

This keeps active status as a signal, not a universal access rule.

## XNTD lock / relock review

The design correctly uses XNTD lock / relock as the basis of active status.

Lock shows initial current commitment.

Relock shows refreshed commitment across epoch changes.

If epoch conditions change, active status may require relock under the new accepted requirement.

This affects only the active signal and must not erase historical contribution.

## Forge scope review

Forge is correctly out of scope for MVP active validity.

The design does not reintroduce Forge participation as an activation requirement.

Forge should remain outside the MVP validity path unless explicitly reintroduced in a separate future design milestone.

## Boundary review

The design does not add:

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

## Review decision

The XC Build active validity rule design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-active-status-model-design

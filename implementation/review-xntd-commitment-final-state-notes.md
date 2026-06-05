# Review XNTD Commitment Final State Notes

## Branch

review-xntd-commitment-final-state

## Purpose

This milestone synchronizes current review-facing documentation after the XNTD commitment replay and ordering guard runtime milestones.

The goal is to remove outdated wording that described XNTD lock / relock as having only registrar-level replay protection.

## Scope

Documentation-only.

Updated documents:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-event-identity.md

No runtime code changed.

## Updated state

XNTD lock / relock now has three protection layers:

1. processedMessages
   - protects against replay of the same registrar messageId

2. usedXntdCommitmentEvents
   - protects against replay of the same source event under a different messageId

3. monotonic lockEpoch ordering guard
   - protects against stale-but-unique source events that are not replay events but could regress commitment state

## README update

The MVP assumptions list now reflects:

- XNTD lock / relock source-event replay protection
- XNTD lock / relock monotonic lockEpoch ordering guard

It no longer states that XNTD lock / relock has registrar-level replay protection only.

## Assumptions update

The XNTD lock / relock replay protection section now describes the implemented runtime model:

- registrar-level replay protection through processed message IDs
- source-event replay protection through XntdCommitmentEventKey / usedXntdCommitmentEvents
- monotonic lockEpoch ordering guard

The remaining MVP boundary is no longer missing per-event replay protection.

The remaining production consideration is whether to replace or strengthen lockEpoch ordering with a stricter ordering source.

## Event identity document update

The XNTD lock / relock event identity document now records that the model has been implemented.

It was updated from a design-only document into a design-and-implementation reference.

It now lists the implemented runtime path:

- XntdCommitmentEventKey
- usedXntdCommitmentEvents
- snapshot persistence
- registrar integration
- proof canonicalEventKey usage
- CLI summary visibility
- monotonic lockEpoch ordering guard

## Historical checkpoint note

Older entries in docs/checkpoints/current-design-checkpoint.md were not rewritten.

Reason:

- checkpoint entries are historical records of repository state at the time of each milestone
- later checkpoint sections already document the implemented replay and ordering guard milestones

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

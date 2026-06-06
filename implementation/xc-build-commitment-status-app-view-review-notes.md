# XC Build commitment status app view review

This document reviews the XC Build commitment status app view runtime implementation milestone.

Reviewed branch:

    xc-build-commitment-status-app-view-review

Reviewed implementation milestone:

    xc-build-commitment-status-app-view

Reviewed files:

- src/app/build-view.ts
- tests/app-build-view.test.ts
- src/index.ts

## Review summary

The XC Build commitment status app view implementation is accepted.

The implementation adds a read-only `appGetBuildView()` helper.

The helper returns Build state plus commitment status:

    AppBuildView {
      build
      commitmentStatus
    }

The implementation exports the helper and related types through `src/index.ts`.

The implementation adds focused unit tests for COMMITTED, UNCOMMITTED, UNKNOWN, current requirement, and non-mutating behavior.

## Diff review

Diff from pre-implementation baseline to current HEAD shows only:

- src/app/build-view.ts
- src/index.ts
- tests/app-build-view.test.ts

No other runtime files changed.

No appSubmitProof code changed.

No watcher files changed.

No registrar files changed.

No proof payload files changed.

No ethereum/RPC files changed.

No script changed.

No dependency changed.

## App view review

The implementation adds:

- AppBuildView
- AppGetBuildViewInput
- appGetBuildView()

The helper accepts:

- build
- optional currentEpoch
- optional currentRequiredXntdLock
- optional requireCurrentEpoch

The helper returns:

- build
- commitmentStatus

The helper delegates commitment interpretation to:

    getBuildCommitmentStatus()

## exactOptionalPropertyTypes review

The implementation handles optional fields correctly under `exactOptionalPropertyTypes`.

It does not pass explicit `undefined` values.

It uses conditional object spreads so optional fields are included only when present.

## Behavior review

The tests verify that appGetBuildView():

- returns COMMITTED commitment status when Build has sufficient XNTD commitment
- returns UNCOMMITTED commitment status when Build has history but no XNTD commitment
- returns UNKNOWN when strict current context is required but missing
- uses provided current requirement for commitment status
- does not mutate Build state

## Non-mutating review

The implementation is read-only and non-mutating.

The test suite verifies that calling appGetBuildView() does not mutate:

- historyBld
- availableBld
- originBld
- lockedXntd
- requiredXntdLock
- lockEpoch

## Boundary review

The implementation does not change:

- appSubmitProof behavior
- watcher behavior
- registrar behavior
- proof payload behavior
- ethereum/RPC code
- scripts
- dependencies
- CLI commands
- BLD transfer/sale rules
- Forge requirements
- unlock mechanics
- Build actor scope

## Commitment status boundary

The helper exposes commitmentStatus as app/service view context.

It does not enforce commitmentStatus.

It does not reject historical proofs.

It does not treat UNCOMMITTED as invalid Build history.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 42 test files, 328 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Review decision

The XC Build commitment status app view implementation is accepted.

No implementation changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-commitment-status-app-view-completion-checkpoint

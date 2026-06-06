# XC Build commitment status rename checkpoint

This document closes the rename from active status to commitment status.

This checkpoint is documentation-only.

No runtime code is changed in this checkpoint.

No dependencies are changed in this checkpoint.

No real RPC is executed in this checkpoint.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed rename

The Build status terminology was renamed from:

    active status

to:

    commitment status

Reason:

The previous active / inactive wording created the wrong effect.

It could imply:

    active = valid / full / correct
    inactive = invalid / disabled / punished

That is not the intended model.

The intended model is:

    Build history remains valid.
    Commitment status only describes current XNTD commitment.

## Current main status

Latest completed main milestone:

    main -> a64c313 Merge branch 'xc-build-commitment-status-rename'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 41 test files, 323 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Runtime rename

Renamed runtime file:

    src/model/build-active-status.ts
    -> src/model/build-commitment-status.ts

Renamed test file:

    tests/build-active-status.test.ts
    -> tests/build-commitment-status.test.ts

Updated export:

    export * from "./model/build-commitment-status.js";

## Type and helper rename

Renamed:

    BuildActiveStatusValue
    -> BuildCommitmentStatusValue

    BuildActiveStatusReason
    -> BuildCommitmentStatusReason

    BuildActiveStatus
    -> BuildCommitmentStatus

    GetBuildActiveStatusInput
    -> GetBuildCommitmentStatusInput

    getBuildActiveStatus()
    -> getBuildCommitmentStatus()

## Status value rename

Renamed:

    ACTIVE
    -> COMMITTED

    INACTIVE
    -> UNCOMMITTED

Kept:

    UNKNOWN

## Reason value rename

Renamed:

    ACTIVE_LOCK_CURRENT
    -> COMMITMENT_CURRENT

    INACTIVE_NO_HISTORY
    -> NO_HISTORY

    INACTIVE_NO_LOCK
    -> NO_COMMITMENT

    INACTIVE_LOCK_BELOW_REQUIRED
    -> COMMITMENT_BELOW_REQUIRED

    INACTIVE_RELOCK_REQUIRED
    -> RECOMMITMENT_REQUIRED

Kept:

    UNKNOWN_NO_CURRENT_CONTEXT

## Meaning

Commitment status means:

    current XNTD commitment signal

It does not mean:

    Build validity

It does not mean:

    historical contribution validity

It does not mean:

    user punishment

It does not erase:

- historyBld
- availableBld
- originBld
- Core redeem history
- Build history

## Current interpretation

COMMITTED means:

    the Build currently has sufficient XNTD commitment under the supplied/current requirement context.

UNCOMMITTED means:

    the Build has no XNTD commitment, insufficient XNTD commitment, or no history.

UNKNOWN means:

    the caller requested strict current context but did not provide enough context.

## Boundary preserved

The rename did not change behavior.

The rename did not change:

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

## Important follow-up

The previous branch:

    xc-build-active-status-app-integration-design

used old active-status terminology and should not be merged.

Future app/service design should use:

    commitmentStatus

not:

    activeStatus

Future helper direction should be:

    appGetBuildView()

with:

    commitmentStatus

## Recommended next milestone

Recommended next design milestone:

    xc-build-commitment-status-app-integration-design

Purpose:

- expose commitment status as optional current XNTD commitment context
- avoid global enforcement
- avoid rejecting historical proofs because commitment status is UNCOMMITTED
- keep appSubmitProof, watcher, registrar, and proof payload behavior unchanged
- keep external X1 project usage optional

## Decision

The active-status terminology is retired for runtime/app direction.

Use commitment-status terminology going forward.

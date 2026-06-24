# Build State history and identity cleanup checkpoint

## Branch

`build-state-history-identity-cleanup`

## Code commit

`d5f9123 Refine Build state history and identity model`

## Purpose

This checkpoint records the cleanup that separates durable public Build history from live spendable balances and separates Build Identity from protocol accounting.

## Main decisions implemented

### 1. Build State stores durable history only

Build State now stores:

- `historyBld`
- `originBld`
- `historyXbp`
- stable XNTD lock facts
- X1 fee checkpoint facts
- optional Build Identity display metadata

Build State no longer stores:

- `availableBld`
- `earnedXbp`
- `availableXbp`

### 2. XBP model simplified

Old model:

    earnedXbp
    availableXbp

Current model:

    historyXbp

`historyXbp` is historical and non-decreasing.

### 3. Genesis Origin changed from one-time claim to tier upgrade

Genesis Origin is now upgrade-to-cap based on `historyBld`.

Tiers:

    historyBld >= 1     -> originBld = 11
    historyBld >= 11    -> originBld = 22
    historyBld >= 121   -> originBld = 55
    historyBld >= 1111  -> originBld = 121

Repeated upgrades only move `originBld` to the next eligible cap.

Genesis Origin does not create or expose a public spendable Build balance.

### 4. Relock no longer depends on Build.availableBld

The old rule:

    availableBld >= historyBld

was removed from Build State.

Future actual BLD availability checks belong to an external BLD asset / ledger / escrow layer at operation time.

### 5. Public Build commitment status no longer exposes UNKNOWN

Public commitment status is now derived from stored lock facts only.

Allowed statuses:

- `COMMITTED`
- `UNCOMMITTED`

Allowed reasons:

- `NO_HISTORY`
- `NO_COMMITMENT`
- `COMMITMENT_BELOW_REQUIRED`
- `COMMITMENT_CURRENT`

Live external context issues are operation-level validation or infrastructure concerns, not public Build state.

### 6. Build Identity added

Build Identity fields:

- `buildName`
- `logoUri`
- `metadataUpdatedAt`

Owner-only update flow added:

- `updateBuildIdentity`

Build Identity has no effect on:

- `historyBld`
- `originBld`
- `historyXbp`
- XNTD lock facts
- X1 fee contribution
- replay protection

## Validation

At the time of the code checkpoint:

    npm run typecheck
    npm test

passed with:

    57 test files passed
    394 tests passed

## Current authoritative docs

The current active model is documented in:

- `README.md`
- `docs/build/build-v1-spec.md`
- `docs/build/build-state-history-identity-model.md`

Older design notes may still contain historical terms and should be read as superseded unless updated to reference this checkpoint.

## Follow-up boundary clarification

Spendable / transferable BLD is a separate token.

Build State and Build view do not display, mirror, or cache BLD token balance.

A UI may show wallet token balances elsewhere, but that is outside Build State and outside Build Identity.

Relock must not read `Build.availableBld` because that field does not exist. Any future operation requiring spendable BLD availability must check the BLD token layer directly at operation time.

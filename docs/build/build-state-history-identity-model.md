# Build State history and identity model

## Status

This document is the current authoritative summary for the Build State cleanup introduced on branch `build-state-history-identity-cleanup`.

It supersedes earlier internal Build balance wording in older design notes.

## Core formula

    Build State = durable public protocol history
    Build Identity = owner-controlled name/logo presentation
    Build Actor = future active layer, not current MVP

## Build State stores

Build State stores durable facts:

- `history_bld`
- `origin_bld`
- `history_xbp`
- `locked_xntd`
- `required_xntd_lock`
- `lock_epoch`
- `xc_commitment_active`
- `x1_fee_contribution`
- `x1_tx_count`
- `x1_fee_counted_until_slot`
- `last_fee_update_at`
- replay protection state in the relevant registries

## Build State does not store

Build State does not store live spendable balances:

- no public spendable BLD balance
- no public spendable XBP balance
- no live wallet balance
- no live token escrow balance

Spendable or transferable BLD belongs to a separate future BLD asset / ledger / escrow layer.

## history_bld

`history_bld` is historical BLD derived from verified redeemed Core history.

It is non-decreasing.

It is not reduced by transfer, sale, burn, use, or future spendable BLD mechanics.

## origin_bld

`origin_bld` is Genesis Origin BLD.

It is not earned history.

It is an upgrade-to-cap value derived from `history_bld`.

Tiers:

    history_bld >= 1     -> origin_bld cap = 11
    history_bld >= 11    -> origin_bld cap = 22
    history_bld >= 121   -> origin_bld cap = 55
    history_bld >= 1111  -> origin_bld cap = 121

Upgrade rule:

    eligible_origin_bld = tier(history_bld)
    delta_origin_bld = eligible_origin_bld - current_origin_bld
    origin_bld = eligible_origin_bld

If the delta is zero or negative, there is no upgrade.

Genesis Origin does not mint a public spendable balance inside Build State.

## history_xbp

`history_xbp` is historical XEN Burn Power derived from verified global `XEN.burn(user, amount)` calls.

It is non-decreasing.

Build State v1 does not expose a public spendable XBP balance.

## XNTD commitment status

Public commitment status is derived from stored facts only.

Allowed public statuses:

- `COMMITTED`
- `UNCOMMITTED`

Allowed public reasons:

- `NO_HISTORY`
- `NO_COMMITMENT`
- `COMMITMENT_BELOW_REQUIRED`
- `COMMITMENT_CURRENT`

`UNKNOWN` is not public Build state.

Missing current RPC / epoch / external context belongs to operation-level validation or infrastructure handling.

## Relock boundary

Relock must not read a public BuildState spendable balance.

Future relock rules that require actual BLD availability must check the external BLD asset / ledger / escrow layer at operation time.

Build State only records stable facts.

## Build Identity

Build Identity fields:

- `buildName`
- `logoUri`
- `metadataUpdatedAt`

Rules:

- optional
- owner-controlled
- names are not globally unique
- logo is URI-based
- no raw logo file is stored in Build State
- no effect on protocol accounting
- no effect on BLD / XBP / XNTD / fee contribution / replay protection

Build Identity is not Build Actor.

Build Actor remains a future layer.

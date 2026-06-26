# XXXL Genesis Phase Public Explanation

## Short version

XXXL is the canonical X1-native token for the xEnchanted / X1 path.

In the Genesis Phase, XXXL is minted only through verified Ethereum XNTD gateway events.

There is:

- no manual mint
- no premine
- no founder allocation
- no hidden emission
- no Build requirement for basic XNTD transfer to X1

The gateway path is:

    Ethereum XNTD event
      -> verified gateway authorization
      -> XXXL mint on X1

## Why Genesis Phase starts gateway-only

The first goal is to create a clean canonical X1-native token layer without introducing unfinished X1-native emission mechanics too early.

Gateway-only launch keeps the initial model simple:

- every minted XXXL must correspond to a verified Ethereum XNTD event
- every accepted event can be consumed only once
- total supply follows accepted gateway mints
- there is no discretionary supply control

This gives participants a usable X1-native token layer while the larger X1-native protocol mechanics are still being designed.

## What temporary upgradeability means

The XXXL program may be temporarily upgradeable during staged protocol finalization.

This does not mean admin mint.

This does not mean discretionary supply control.

This does not mean founder allocation.

Temporary upgradeability exists only because the final X1-native protocol emission mechanics are not complete yet.

The intended future direction is deterministic Core / Forge / Stake-like mechanics on X1.

After those mechanics are designed, tested, reviewed, explained publicly, and implemented, the upgrade authority must be removed or frozen.

## What upgradeability must not allow

Temporary upgradeability must not be used to:

- mint arbitrary XXXL
- bypass gateway authorization
- rewrite balances
- clear processed gateway events
- create a premine
- create a founder allocation
- create hidden emission
- create arbitrary supply control

Future upgrades may only add deterministic user-action protocol mechanics.

## Why Build is separate

XXXL and Build are different layers.

XXXL is transferable token state.

Build is non-transferable history / identity / contribution state.

A user can transfer XNTD to X1 and receive XXXL without activating Build.

Build activation remains a separate profile/history operation.

Build does not take rights from current XXXL balance.

This matters because history and current balance are not the same thing.

A user may transfer, sell, hold, or use XXXL, but Build should still be based on confirmed historical actions, not only on the balance visible at one moment.

## Why xDex listing can come before full Build launch

XXXL can become useful before the full Build system is live.

An xDex listing can let participants use the canonical X1-native token layer without waiting for the complete Build launch.

This does not conflict with Build because:

- XXXL is the token layer
- Build is the history / identity layer
- Build activation is separate
- Build recognition should be based on confirmed historical actions

## What users should understand

During Genesis Phase:

- XXXL is initially gateway-only
- XNTD transfer to X1 does not require Build
- trading XXXL does not activate Build
- Build remains separate
- final X1-native emission mechanics are not live yet
- temporary upgradeability is staged protocol finalization, not admin control
- the final goal is deterministic protocol mechanics and then freeze

## What this is not

Genesis Phase is not:

- a premine
- a founder allocation
- an admin mint system
- a hidden emission system
- a promise of guaranteed price
- a promise of guaranteed liquidity
- a promise of guaranteed rewards
- a promise of guaranteed Build allocation

## Core public formula

XXXL is the canonical X1-native token, initially minted only through verified Ethereum XNTD gateway events.

Build is a separate non-transferable history / identity / contribution layer.

Upgradeable now means staged protocol finalization, not admin supply control.

Final goal: deterministic X1-native Core / Forge / Stake-like mechanics, then freeze.

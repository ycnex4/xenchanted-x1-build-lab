# X1-native Forge / Stake dual nominal model

This document defines an early design direction for future X1-native Forge-like and Stake-like mechanics.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

## Context

The XNTD-to-XXXL gateway brings external XNTD energy into X1.

The gateway direction is:

    source-chain XNTD burn -> X1-native XXXL mint

XXXL is not the same token as Ethereum XNTD.

XXXL is an X1-native token with different origin and future utility.

The first gateway route is expected to be Ethereum-first.

Future source chains may be added later with reduced source-chain conversion weights.

Over time, external inflow from Ethereum and other chains may decline.

Therefore, X1 should not depend only on gateway inflow forever.

X1 needs its own native economic mechanics.

## High-level purpose

The purpose of X1-native Forge / Stake mechanics is to create a long-term X1 economic cycle around XXXL.

The gateway brings XXXL into X1.

X1-native Forge-like mechanics may transform liquid XXXL into long-term protocol positions.

X1-native Stake-like mechanics may allow those positions to earn slow passive yield.

Future Build mechanics may later record participation, history, and state.

Conceptual chain:

    XNTD burn on Ethereum
    -> XXXL mint on X1
    -> XXXL used in X1 Forge-like mechanics
    -> X1 Forged position created
    -> position may participate in X1 Stake-like mechanics
    -> Build layer may later record participation / history / state

## Not a direct copy of Ethereum XC

X1-native Forge and Stake should not be assumed to be direct copies of Ethereum xEnchanted Crypto Forge and Stake.

Ethereum XC remains the source of the idea.

X1-native mechanics should continue the logic under X1 conditions.

They may differ in:

- asset origin
- token name
- mint source
- position model
- stake reward curve
- redeem rules
- duration rules
- interaction with Build
- interaction with future X1-native utility

## Core economic principle

Forge is valuable because it removes liquid supply and converts it into a long-term position.

A Forge-like action should not simply be a short-term path to recover the burned or committed amount.

The economic principle is:

    Forge removes liquid XXXL from circulation.
    Stake gives the resulting position productive value.
    Stake yield must not quickly neutralize the Forge burn / commit cost.

If stake rewards quickly return what was burned or committed, Forge becomes only delayed emission.

That would weaken the supply discipline of the model.

## Dual nominal model

X1-native positions may use two nominal values:

    mainNominal
    stakeNominal

These values have different meanings.

They must not be mixed.

## mainNominal

mainNominal is the conservative base value of the position.

It may represent:

- redeem value
- base position value
- conservative accounting weight
- long-term position size

For position evolution, mainNominal may grow by summing parent main nominals:

    child.mainNominal = parentA.mainNominal + parentB.mainNominal

This avoids aggressive redeem-value expansion.

The goal is to let the position develop without creating excessive redeem pressure.

## stakeNominal

stakeNominal is the productive staking value of the position.

It is used only for stake reward calculations.

stakeNominal may grow slightly faster than mainNominal through a soft level coefficient or level bonus.

It should make position development feel meaningful.

It should not grow as aggressively as Ethereum Core enchant nominal growth.

The Ethereum Core-style `*3` growth is too strong for this X1-native Forge / Stake purpose.

Possible direction:

    stakeNominal = mainNominal + softStakePremium

Where:

    softStakePremium = mainNominal * levelStakeBonusBps / 10000

Example conceptual level bonuses:

    L1: +0%
    L2: +2%
    L3: +4%
    L4: +6%
    L5: +8%

These numbers are examples only.

The exact curve is not defined in this document.

## Redeem rule

If a redeem function exists, redeem must use mainNominal, not stakeNominal.

Possible direction:

    redeemAmount = mainNominal

or:

    redeemAmount = mainNominal * redeemBps / 10000

The exact redeem rule is not defined here.

Important rule:

    stakeNominal must not be redeemable value.

stakeNominal is staking power only.

## Stake reward rule

Stake-like rewards may use stakeNominal as the reward base.

Conceptual direction:

    reward = stakeNominal * aprBps * duration / timeBase / 10000

The exact APR, duration, decay, cap, and reward formula are not defined in this document.

Important rule:

    stake rewards should be slow enough that they do not quickly offset the Forge burn / commit cost.

## Position development

X1-native positions should feel alive and developable.

A user should be able to improve a position and feel that it becomes more productive.

However, development should not create uncontrolled redeem value.

This is why the model separates:

    redeem value -> mainNominal
    staking power -> stakeNominal

## Relationship to XXXL

XXXL is the X1-native fuel.

Possible flow:

    user holds XXXL
    user uses XXXL in Forge-like action
    liquid XXXL is burned or committed
    X1-native position is created
    position has mainNominal and stakeNominal
    position may later be staked
    stake rewards may mint XXXL slowly

The exact burn or commit mechanism is not defined here.

## Burn vs commit

This document does not decide whether X1 Forge-like mechanics should use:

- permanent XXXL burn
- time-locked XXXL commit
- partial burn and partial lock
- another X1-native mechanism

This must be a separate design decision.

The key principle remains:

    liquid XXXL should be transformed into a long-term position.

## Frontend role

The existing xEnchanted frontend may be used as the interface for this future X1-native Forge / Stake model.

The frontend should show:

- XXXL balance
- amount used for Forge-like action
- expected mainNominal
- expected stakeNominal
- expected staking power
- possible redeem value
- warnings that stakeNominal is not redeem value
- position level / development state
- future stake preview

The frontend must not be the source of truth.

The X1 program / contract must be the source of truth for:

- formulas
- limits
- APR
- duration rules
- nominal calculations
- redeem calculations
- stake reward calculations

Frontend previews are only UX helpers.

Final calculations must be enforced by X1-side rules.

## UX warning

The UI should clearly explain:

    Main nominal controls redeem value.
    Stake nominal controls staking reward power.
    Stake nominal is not redeem value.

This prevents users from assuming that a higher stakeNominal means a higher redeem amount.

## Relationship to Build

Build is not required for the first X1-native Forge / Stake model.

Build may later record:

- participation
- X1 Forge-like actions
- X1 Stake-like actions
- long-term position history
- project / user / actor context

Do not block X1 Forge / Stake design on Build actor.

Build remains a future memory / state layer.

## Relationship to the gateway

The gateway is the entrance.

XXXL is the X1-native fuel.

X1-native Forge / Stake mechanics are the economic engine.

Build is the future memory / state layer.

Conceptual framing:

    Gateway brings energy into X1.
    XXXL carries that energy.
    X1 Forge transforms liquid XXXL into long-term positions.
    X1 Stake gives those positions slow productive value.
    Build records the history later.

## Non-goals

This document does not define:

- final formulas
- final APR
- final redeem rate
- final level bonus curve
- final burn / commit rule
- X1 program implementation
- frontend implementation
- Build actor integration
- gateway implementation
- production deployment

## Decision

The future X1-native Forge / Stake model should explore a dual nominal design.

The core idea is:

    mainNominal controls redeem / conservative value.
    stakeNominal controls staking reward power.

stakeNominal may grow softly with level.

redeem must not use stakeNominal.

stake rewards must not quickly neutralize the XXXL burned or committed through Forge-like actions.

The existing xEnchanted frontend can later serve as the UX interface for this model, but X1-side rules must remain the source of truth.

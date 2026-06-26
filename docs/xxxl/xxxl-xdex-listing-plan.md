# XXXL xDex Listing Plan

## Purpose

This document defines the initial xDex listing plan for XXXL.

The goal is to allow participants to use the canonical X1-native token layer without waiting for the full Build launch or final X1-native emission mechanics.

## Strategic boundary

XXXL can become the first live X1-native token layer before full Build launch.

This does not conflict with Build because:

- XXXL is transferable token state
- Build is separate non-transferable history / identity / contribution state
- Build uses confirmed historical actions, not current XXXL balance
- XNTD transfer to X1 does not require Build activation
- Build activation remains a separate profile/history operation

## Listing prerequisite

xDex listing should only happen after Genesis Phase deployment readiness is satisfied.

Required before listing:

- canonical XXXL token is deployed
- gateway-only mint path is live or ready
- no manual mint path exists
- no premine exists
- no founder allocation exists
- no hidden emission path exists
- public upgradeability covenant is documented
- public Genesis Phase explanation is available
- liquidity / market risk explanation is available

## Listing model

Initial listing should present XXXL as:

    canonical X1-native token initially minted only through verified Ethereum XNTD gateway events

Public wording should prefer:

- XNTD transfer to X1
- Gateway
- transport path
- canonical X1-native token

Public wording should avoid making these the main public terms too early:

- XXXL internal mechanics not yet finalized
- BLD
- XBP
- unreleased Build internals
- speculative final X1-native emission details

## User expectations

Participants must understand:

- XXXL Genesis Phase is gateway-only
- receiving XXXL does not require Build
- trading XXXL does not activate Build
- Build is separate and history-based
- future Build recognition may depend on confirmed historical actions, not only current token balance
- final X1-native emission mechanics are not live yet
- upgradeability is temporary staged protocol finalization, not admin supply control

## Explicit Genesis Phase disclosure

Before any xDex listing, the public listing text must include this disclosure:

    XXXL is in Genesis Phase.
    Supply increases only through verified Ethereum XNTD gateway events.
    Future X1-native emission mechanics are not yet active.
    Build layer is not yet launched.
    Build does not confer rights to XXXL holders during Genesis Phase.
    Trading XXXL does not activate Build.
    Listing does not guarantee price, liquidity, rewards, Build allocation, or final emission schedule.

This disclosure must be visible before or alongside any listing announcement.

## Market risk notes

Initial xDex listing must not imply:

- guaranteed price
- guaranteed liquidity
- guaranteed Build allocation
- guaranteed future rewards
- guaranteed final emission schedule
- hidden founder market support

The listing should be framed as early utility / transferability for the canonical X1-native token, not as a promise of financial return.

## Operational plan

Possible order:

1. finish XXXL Program v1 deterministic model and runtime mapping
2. complete deployment readiness checklist
3. prepare public Genesis Phase explanation
4. prepare xDex listing explanation
5. deploy canonical XXXL
6. verify gateway-only mint constraints
7. perform small operational test
8. prepare initial liquidity plan if needed
9. announce xDex listing with clear Genesis Phase caveats

## Non-goals

This document does not implement:

- xDex smart contracts
- listing transaction scripts
- liquidity management
- price policy
- market-making policy
- frontend trading UI

# XXXL Multichain Low-Weight Route Policy

## Purpose

This document records the multichain route policy for future XC deployments on non-Ethereum chains.

This is important before the runtime skeleton stage because XXXL runtime must not be hardcoded as Ethereum-only.

At the same time, non-Ethereum routes must not become equal economic peers of Ethereum.

## Core principle

XXXL is the canonical X1-native aggregation token for approved XC source-chain XNTD burns.

Ethereum route is the primary full-weight route.

Non-Ethereum routes are low-weight historical access routes.

Their purpose is inclusion, not equal supply power.

## Why non-Ethereum routes are low-weight

Current observed XEN conditions outside Ethereum are extremely weak.

The user-provided market snapshot shows Avalanche aXEN liquidity around $169.98, while Ethereum XEN liquidity is around $66.78K.

That implies Avalanche liquidity is roughly 0.25% of Ethereum liquidity.

Therefore even 1% route weight can be too generous under current conditions.

## Route weights

The route-weighted mint formula is:

    xxxlMintAmount = burnedSourceAmount * sourceChainWeightBps / 10000

Ethereum route:

    sourceChainWeightBps = 10000

Avalanche route:

    hard max = 25 bps
    conservative initial candidate = 5-10 bps

Other non-Ethereum routes:

    must be <= configured Avalanche route weight
    default status should be candidate or inactive
    must require explicit route policy approval

## Avalanche policy

Avalanche is treated as the strongest non-Ethereum candidate route, but it is still not an economic peer of Ethereum.

Policy:

- Avalanche hard max weight: 25 bps
- Avalanche conservative initial range: 5-10 bps
- Any increase above 25 bps requires a new explicit route policy review
- 100 bps is not allowed under current market conditions
- 500 bps is not allowed under current market conditions

This replaces the earlier rough idea that Avalanche could be up to 5%.

Given the observed liquidity/activity, 5% would be far too high.

## Non-Ethereum caps

Weight alone is not enough.

Every non-Ethereum route must define caps:

- per-event mint cap
- daily route mint cap
- epoch route mint cap
- global non-Ethereum supply share cap

This gives two layers of protection:

    reduced route weight + route caps

## Runtime implication

The runtime should support multiple approved route ids, but initial active deployment may remain Ethereum-only.

This means:

- do not hardcode Ethereum-only assumptions into runtime skeleton
- keep routeId as part of instruction data
- route policy decides sourceChainWeightBps
- Stage 1 authorization must bind sourceChainWeightBps and xxxlMintAmount
- runtime consumes the already authorized xxxlMintAmount
- supply invariant aggregates consumed mint amounts across all approved routes

## Genesis supply invariant

The Genesis invariant becomes:

    XXXL total supply = sum(consumed gateway mint amounts across all approved routes)

This is not:

    XXXL total supply = sum(raw burned XNTD across all chains)

The route weight is applied before XXXL mint amount is authorized.

## Build boundary

Multichain XXXL balances do not grant hidden Build supply rights.

Build remains separate.

Build uses confirmed contribution/history rules, not current token balance and not arbitrary multichain XXXL balance.

## Non-goals

This policy does not activate Avalanche route now.

It does not deploy XC on Avalanche.

It does not define final source token contract addresses.

It does not change current Ethereum primary route.

It does not make non-Ethereum XNTD equal to Ethereum XNTD.

It only records the low-weight route policy required before runtime skeleton design.

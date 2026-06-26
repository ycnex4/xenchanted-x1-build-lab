# XXXL Program v1 Deployment Readiness

## Purpose

This document defines the deployment readiness boundary for XXXL Program v1.

This is not a deployment script.

It is a checklist for deciding when the canonical XXXL Program v1 is ready to move from deterministic model / runtime mapping into a real X1 deployment plan.

## Current stage

Current XXXL Program v1 status:

- design boundary exists
- Stage 1 gateway authorization consumer exists
- Genesis supply invariant hardening exists
- X1 runtime mapping exists
- no production runtime code yet
- no deployment scripts yet
- no live X1 program yet
- no live token mint yet
- no xDex listing yet

## Genesis Phase deployment scope

The first deployable XXXL scope is Genesis Phase only.

Allowed:

- canonical XXXL token on X1
- gateway-only mint path
- verified Ethereum XNTD source events
- Stage 1 gateway authorization
- consumed canonical event replay protection
- recipient balance update
- total supply update
- public documentation of temporary upgradeability covenant

Not allowed:

- manual mint
- premine
- founder allocation
- hidden emission
- Build-gated mint
- Build-derived supply rights
- current-balance-derived supply rights
- arbitrary admin supply control
- X1-native emission before deterministic mechanics are designed and tested

## Required before deployment

Before live deployment, the project must have:

- final route configuration
- final source XNTD token address
- final target X1 network id
- final XXXL mint identifier
- final guardian set policy
- final quorum threshold
- final finality policy
- final processed-event storage design
- final authority model
- final upgradeability disclosure
- final freeze / authority removal plan
- deployment dry-run checklist
- rollback / incident response policy
- public explanation draft

## Runtime requirements

The deployed runtime must preserve:

- check-before-mark replay rule
- atomic mint + consumed-event mark
- no state mutation on failed authorization
- no state mutation on replay
- no state mutation on wrong route
- no state mutation on wrong mint token
- no state mutation on wrong source chain
- no state mutation on wrong source token
- no state mutation on invalid quorum
- no manual supply increase
- no Build dependency for gateway mint

## Authority requirements

Temporary upgradeability is allowed only for staged protocol finalization.

Upgrade authority must not be able to:

- mint XXXL directly
- bypass gateway authorization
- rewrite balances
- clear processed event history
- create premine
- create founder allocation
- create hidden emission
- create arbitrary supply control

Allowed future upgrades may only add deterministic user-action protocol mechanics.

After planned X1-native emission mechanics are complete, upgrade authority must be removed / frozen.

## Public disclosure requirement

Deployment communication must explain upfront:

- XXXL Genesis Phase starts gateway-only
- XXXL is temporarily upgradeable only because final X1-native protocol emission is not complete yet
- temporary upgradeability is not admin mint authority
- temporary upgradeability is not discretionary supply control
- no premine, no founder allocation, no hidden emission
- future emission must be deterministic Core / Forge / Stake-like mechanics
- final goal is freeze / removal of upgrade authority

## Validation requirements

Before deployment readiness can be considered complete:

- TypeScript typecheck must pass
- all deterministic model tests must pass
- build must pass
- gateway vector tests must pass
- Stage 1 authorization tests must pass
- XXXL consumer tests must pass
- Genesis supply invariant tests must pass
- runtime mapping must be reviewed
- deployment checklist must be reviewed

## Non-goals

This document does not deploy XXXL.

This document does not define:

- production addresses
- private keys
- live guardian keys
- RPC URLs
- xDex contracts
- frontend release timing
- final X1-native emission mechanics

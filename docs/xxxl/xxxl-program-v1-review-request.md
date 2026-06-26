# XXXL Program v1 Review Request

## Purpose

This document is a focused review request for XXXL Program v1.

It is intended for architecture review before moving from model / documentation work toward production X1 runtime planning.

## Review scope

Please review whether the current XXXL Program v1 Genesis Phase design is clean, bounded, and internally consistent.

The intended Genesis Phase model is:

    Ethereum XNTD gateway event
      -> Stage 1 gateway verification / authorization
      -> XXXL consumer
      -> XXXL mint on X1
      -> consumed event mark

## Current baseline

Current validation baseline:

- TypeScript typecheck: passing
- Tests: 68 files / 458 tests passing
- Build: passing

Latest main checkpoint:

- `0c127f7 Merge XXXL Genesis Phase public explanation`

## Primary review entrypoint

Start here:

- `docs/xxxl/xxxl-program-v1-review-summary.md`

Supporting documents:

- `docs/xxxl/xxxl-program-v1-design-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/xxxl/xxxl-genesis-supply-invariant.md`
- `docs/xxxl/xxxl-program-v1-x1-runtime-mapping.md`
- `docs/xxxl/xxxl-program-v1-deployment-readiness.md`
- `docs/xxxl/xxxl-xdex-listing-plan.md`
- `docs/xxxl/xxxl-genesis-phase-public-explanation.md`

Implementation / tests:

- `src/xxxl/program-v1.ts`
- `src/xxxl/stage-1-gateway-consumer.ts`
- `src/xxxl/genesis-supply-invariant.ts`
- `tests/xxxl/program-v1.test.ts`
- `tests/xxxl/stage-1-gateway-consumer.test.ts`
- `tests/xxxl/genesis-supply-invariant.test.ts`

## Core design summary

XXXL is the canonical X1-native token for the xEnchanted / X1 path.

During Genesis Phase:

- XXXL is gateway-only
- mint requires successful Stage 1 gateway authorization
- no manual mint
- no premine
- no founder allocation
- no hidden emission
- no Build dependency for gateway mint
- no Build-derived supply rights
- no current-balance-derived supply rights

Main invariant:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

## Build separation

XXXL and Build are intentionally separate.

XXXL:

- transferable token state
- gateway mint target
- can launch before full Build

Build:

- non-transferable history / identity / contribution state
- uses confirmed historical actions
- does not derive rights from current XXXL balance

This separation is important because XNTD transfer to X1 should not require Build activation.

## Upgradeability covenant

Temporary upgradeability may exist only for staged protocol finalization.

It must not be interpreted as:

- admin mint authority
- discretionary supply control
- founder allocation authority
- hidden emission authority
- permission to rewrite balances
- permission to bypass gateway authorization

Future upgrades may only add deterministic user-action protocol mechanics.

After planned X1-native emission mechanics are complete, upgrade authority must be removed / frozen.

## Questions for review

### 1. Boundary clarity

Is the boundary between these layers clean enough?

    Stage 1 gateway verification / authorization
    XXXL gateway consumer
    XXXL supply invariant
    future X1 runtime mapping
    Build identity/history layer

Does anything mix concerns that should remain separate?

### 2. Gateway-only Genesis Phase

Is the Genesis Phase rule sufficiently strict?

    supply can increase only through Stage 1 authorized gateway mints consumed exactly once

Are there missing failure cases or replay cases?

### 3. Stage 1 dependency

Is it correct that XXXL does not re-verify every Stage 1 field itself, but consumes a successful Stage 1 authorization result and adds local replay / supply protection?

Should any Stage 1 fields be duplicated at the XXXL consumer boundary for extra defense?

### 4. Runtime atomicity

Is the runtime mapping strong enough?

Required atomic effect:

    success = balance update + supply update + consumed event mark
    failure = no balance update + no supply update + no consumed event mark

Are there any account-level write-order risks that should be documented now?

### 5. Upgradeability covenant

Is the temporary upgradeability explanation acceptable?

The current claim is:

    upgradeable now = staged protocol finalization, not admin supply control

Is the covenant specific enough to avoid ambiguity?

### 6. Build separation

Is the separation between XXXL and Build clear enough?

Important rule:

    XNTD transfer to X1 does not require Build activation

Does this create any hidden inconsistency with future Build history recognition?

### 7. xDex timing

Is it architecturally safe to allow xDex listing before full Build launch, given that Build does not depend on current XXXL balance?

What risk disclosures should be added before public listing?

### 8. Missing production-readiness items

What is missing before moving toward a production X1 runtime candidate?

Possible missing items:

- concrete account layout
- concrete instruction schema
- final route config
- guardian rotation policy
- finality policy
- deployment dry run
- authority freeze procedure
- incident response checklist

## Desired review output

Useful review output would be:

- approved / not approved boundary assessment
- list of blockers
- list of non-blocking refinements
- missing invariants
- missing runtime failure cases
- suggested wording changes for public explanation
- recommendation for next implementation stage

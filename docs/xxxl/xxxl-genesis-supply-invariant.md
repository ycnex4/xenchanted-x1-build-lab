# XXXL Genesis Supply Invariant

## Purpose

This document hardens the XXXL Genesis Phase supply rule.

In the Genesis Phase, XXXL supply may increase only through accepted gateway mints derived from successful Stage 1 gateway authorization.

The rule is:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

## Forbidden supply paths

The Genesis Phase must not include:

- manual mint
- premine
- founder allocation
- admin mint
- hidden emission
- balance rewrite
- Build-gated supply rights
- current-balance-derived supply rights
- X1-native mint outside the planned future deterministic protocol mechanics

## Accepted supply path

The only accepted Genesis Phase supply increase is:

    verified Ethereum XNTD gateway event
      -> successful Stage 1 mint authorization
      -> consumed once by XXXL
      -> XXXL supply increases by the authorized amount

The consumed canonical event key must be marked locally by the XXXL program model.

## Rejected transition rule

If a gateway authorization fails, a replay is detected, a manual mint is attempted, or any other unauthorized transition is attempted:

- supply must not increase
- processed gateway event state must not advance
- the failed transition must not create future mint rights

## Runtime consequence

The future X1 runtime must preserve this invariant atomically.

For every accepted mint:

- the gateway authorization must be valid
- the canonical event key must not have been consumed
- XXXL supply must increase by exactly the authorized amount
- the canonical event key must be marked consumed

For every rejected mint:

- supply must remain unchanged
- replay state must remain unchanged

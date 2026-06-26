# XXXL Stage 1 Gateway Authorization Consumer

## Purpose

This document records the next Stage XXXL Program v1 boundary:

XXXL must consume Stage 1 gateway mint authorization instead of accepting an unrelated local mint object.

The intended flow is:

    Stage 1 gateway verification
      -> guardian quorum authorization
      -> processed burn replay check
      -> authorized Stage 1 mint result
      -> XXXL program consumer
      -> XXXL supply update and local processed event mark

## Boundary

The XXXL program consumer does not replace Stage 1 verification.

Stage 1 remains responsible for:

- canonical gateway message validation
- route binding
- source chain binding
- source token binding
- X1 recipient hash binding
- burned amount and XXXL mint amount binding
- domain separator binding
- message hash binding
- guardian quorum verification
- processed burn replay protection

The XXXL consumer is responsible for:

- refusing failed Stage 1 authorization results
- refusing Stage 1 results that did not mark the source event processed
- refusing local XXXL replay for the same canonical event key
- increasing XXXL supply only by the Stage 1 authorized mint amount
- keeping the Genesis Phase supply invariant

## Invariant

During the gateway-only Genesis Phase:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

A Stage 1 authorization result is consumable only if:

- authorization.ok is true
- authorization.authorized is true
- authorization.markedProcessed is true
- the XXXL program has not already consumed the canonical event key
- the authorized mint amount is greater than zero

## Atomicity note

This TypeScript model separates the Stage 1 authorization object and the XXXL consumer state for review clarity.

The future X1 runtime must preserve the same effect atomically:

- verify / authorize gateway message
- check replay
- mint XXXL
- mark canonical event key processed

If minting fails, replay state must not be advanced.

If replay state cannot be advanced, minting must not happen.

## Non-goals

This stage does not add:

- live X1 runtime code
- production account layout
- deployment scripts
- xDex listing code
- frontend changes
- new emission paths
- manual mint paths

# XXXL SVM Serialized Runtime Vectors

## Purpose

This document records deterministic serialized runtime vectors for XXXL Program v1.

This stage connects:

- production runtime byte layouts
- X1/SVM program skeleton
- account meta expectations
- SPL Token `mint_to` CPI boundary

It is still not live deployable runtime code.

## Serialized vectors

Defined vectors:

- serialized Mint State account
- serialized Gateway Config account
- serialized Guardian Set account
- serialized Processed Event account
- serialized Recipient Balance account
- serialized `consume_gateway_mint` instruction

Each vector records:

- layout kind
- byte length
- canonical hex
- selected field probes
- field offsets
- field sizes
- field hex slices

## Bundle boundary

The serialized runtime bundle also records:

- `consume_gateway_mint` handler
- SPL Token Program ID
- gateway mint authority PDA model
- canonical account meta roles
- CPI prepared flag
- CPI atomic-with-parent-transaction flag

## Why this exists

The production byte layout stage defines offsets and sizes.

The X1/SVM skeleton defines handler and account boundaries.

This stage adds deterministic serialized bytes so the next runtime stage can test:

    bytes -> decode/validate -> handler skeleton -> expected boundary result

## Non-goals

This stage does not deploy.

It does not submit transactions.

It does not derive real on-chain PDAs.

It does not call SPL Token.

It does not activate Avalanche.

It does not replace live runtime tests.

It only provides deterministic serialized runtime vectors for implementation-facing work.

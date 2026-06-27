# XXXL Production Runtime Byte Layout

## Purpose

This document records the first production-oriented byte layout layer for XXXL runtime accounts and instructions.

This stage follows Theo approval of the model-layer runtime preparation package.

It is still not live X1/SVM code.

It defines exact fixed binary layouts that a live runtime implementation can later map into real account serialization and instruction decoding.

## Encoding

Encoding:

- `FIXED_BINARY_LE_V1`

Global rules:

- first field is always an 8-byte discriminator
- second field is a little-endian `version: u16`
- layouts are fixed-size
- integer fields are little-endian
- `u128` fields are 16-byte aligned
- total layout size is 8-byte aligned
- reserved padding is explicit

## Account layouts

Defined account layouts:

- Mint State account: 176 bytes
- Gateway Config account: 256 bytes
- Guardian Set account: 320 bytes
- Processed Event account: 144 bytes
- Recipient Balance account: 144 bytes

## Instruction layouts

Defined instruction layouts:

- Consume Gateway Mint instruction: 208 bytes

The instruction layout includes:

- 8-byte discriminator
- version
- account meta count
- account indices
- route id
- guardian set id
- mint id
- canonical event key
- recipient
- amount
- source chain weight bps
- reserved padding

## Mint authority PDA

The Mint State layout includes a fixed field for:

- gateway mint authority PDA

It also separates:

- program upgrade authority
- SPL Token mint authority

This preserves the authority-surface separation approved in the previous review.

## Route-aware fields

Gateway Config keeps the route policy data explicit:

- route id
- source chain id
- source token
- target mint
- guardian set id
- finality rule id
- source chain weight bps
- per-event cap
- daily cap
- epoch cap

This keeps the runtime route-aware and not Ethereum-only.

## Replay protection

Processed Event layout stores:

- consumed flag
- canonical event key
- route id
- recipient
- consumed amount
- consumed slot

## Non-goals

This stage does not implement live X1/SVM program code.

It does not derive real PDAs.

It does not call SPL Token.

It does not submit transactions.

It does not activate Avalanche.

It does not replace the model-layer execution vectors.

It only defines deterministic byte layouts for the next runtime implementation steps.

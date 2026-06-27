# XXXL SVM Runtime Decoder Handler Model

## Purpose

This document records the SVM runtime decoder/handler model for XXXL Program v1.

This stage connects:

- production byte layouts
- serialized runtime vectors
- X1/SVM program skeleton
- handler input construction
- CPI boundary preparation

It verifies the deterministic path:

    bytes -> decode/validate -> handler input -> skeleton execution boundary

## What is decoded

The model decodes:

- Mint State account bytes
- Gateway Config account bytes
- Guardian Set account bytes
- Processed Event account bytes
- Recipient Balance account bytes
- `consume_gateway_mint` instruction bytes

For each decoded layout it validates:

- byte length
- discriminator
- version
- canonical bytes
- field ranges
- decoded field values

## Handler model

The handler model builds a `consume_gateway_mint` input from decoded bytes.

It requires:

- all five runtime account layouts
- one `consume_gateway_mint` instruction layout
- valid decoded bytes
- valid serialized vector bundle

Only then does it execute the X1/SVM skeleton boundary.

## CPI rule

The CPI boundary is prepared only for valid decoded input.

If account bytes or instruction bytes are corrupted, the model rejects before CPI preparation.

## Rejection coverage

The model rejects:

- wrong byte length
- discriminator mismatch
- version mismatch
- canonical byte corruption
- missing required account
- missing instruction
- wrong instruction kind
- invalid decoded account/instruction bytes

## Non-goals

This stage does not deploy.

It does not submit transactions.

It does not derive real on-chain PDAs.

It does not call SPL Token.

It does not activate Avalanche.

It does not replace live runtime tests.

It is still a deterministic implementation-facing model.

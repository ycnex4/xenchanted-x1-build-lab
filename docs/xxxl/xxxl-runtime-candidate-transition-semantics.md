# XXXL Runtime Candidate Transition Semantics

## Purpose

This document defines the first deterministic transition semantics for the XXXL runtime candidate.

This is still planning / model work.

It is not production X1 runtime code.

## Canonical transition

The runtime candidate transition models:

    CONSUME_GATEWAY_MINT

Inputs:

- runtime instruction schema
- Stage 1 to XXXL authorization contract

The transition is accepted only if:

- instruction schema is valid
- Stage 1 authorization contract is successful
- Stage 1 marked the source event processed
- amount is greater than zero
- authorization canonical event key matches instruction canonical event key
- authorization amount matches instruction amount
- processed event account is not already consumed

## Success effect

On success, the candidate transition updates:

- Mint State total supply
- Recipient Balance balance
- Processed Event consumed flag

The success effect is:

    mint.totalSupply += amount
    recipientBalance.balance += amount
    processedEvent.consumed = true

## Failure effect

On failure:

- Mint State is unchanged
- Recipient Balance is unchanged
- Processed Event is unchanged

This preserves the approved atomicity rule:

    success = balance update + supply update + consumed event mark
    failure = no balance update + no supply update + no consumed event mark

## Boundary with Stage 1

The runtime transition does not re-verify all Stage 1 message fields.

It consumes the formal Stage 1 authorization contract:

- authorizationOk
- authorized
- markedProcessed
- canonicalEventKey
- amount

This keeps Stage 1 as the verifier layer and XXXL runtime as the execution layer.

## Runtime meaning

In future production X1 runtime, this model should map to one atomic transaction.

The runtime must not allow partial state:

- no supply update without consumed event mark
- no consumed event mark without supply update
- no balance update without supply update
- no replay state mutation on failure

## Non-goals

This stage does not implement:

- production X1 program code
- CPI calls
- deployment scripts
- production PDAs
- live route config
- live guardian keys
- RPC usage

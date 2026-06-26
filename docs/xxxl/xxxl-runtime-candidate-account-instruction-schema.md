# XXXL Runtime Candidate Account and Instruction Schema

## Purpose

This document defines the first concrete runtime candidate schema for XXXL Program v1.

This is still planning / model work.

It is not production X1 runtime code.

## Review origin

Theo review identified concrete account layout and concrete instruction schema as blockers for production runtime.

This stage addresses those blockers at candidate-schema level.

## Runtime account kinds

The runtime candidate separates five account kinds:

- Mint State
- Gateway Configuration
- Guardian Set
- Processed Event
- Recipient Balance

## Mint State account

Purpose:

- stores canonical XXXL token state

Fields:

- account kind
- version
- mint id
- decimals
- total supply
- authority mode
- upgrade authority status

Genesis Phase authority mode:

    GATEWAY_ONLY

## Gateway Configuration account

Purpose:

- stores route-level gateway configuration

Fields:

- account kind
- version
- route id
- source chain id
- source token
- target mint token
- target X1 network id
- target mint core id
- guardian set id
- quorum threshold
- finality rule id
- status

Genesis Phase required status:

    ACTIVE

## Guardian Set account

Purpose:

- stores guardian public keys and quorum configuration

Fields:

- account kind
- version
- guardian set id
- guardian public keys
- quorum threshold
- status

Genesis Phase required status:

    ACTIVE

## Processed Event account

Purpose:

- stores consumed canonical event information

Fields:

- account kind
- version
- canonical event key
- route id
- consumed flag
- consumed amount
- recipient

## Recipient Balance account

Purpose:

- stores holder balance for XXXL

Fields:

- account kind
- version
- mint id
- owner
- balance

## Canonical instruction

The canonical Genesis Phase runtime instruction is:

    CONSUME_GATEWAY_MINT

Instruction data:

- instruction id
- route id
- guardian set id
- mint id
- canonical event key
- recipient
- amount

Required accounts:

- Mint State
- Gateway Configuration
- Guardian Set
- Processed Event
- Recipient Balance

## Schema validation

The candidate schema validates that:

- all account kinds match the expected layout
- mint authority mode is gateway-only
- route is active
- guardian set is active
- gateway config points to the guardian set
- gateway quorum threshold matches guardian set quorum threshold
- guardian set is not empty
- quorum threshold is valid
- recipient balance belongs to the XXXL mint
- processed event belongs to the route
- instruction route matches route account
- instruction guardian set matches guardian set account
- instruction mint id matches mint state
- instruction event key matches processed event account
- instruction recipient matches recipient balance and processed event
- instruction amount matches processed event amount

## Runtime write set

The consume gateway mint instruction writes:

- Mint State
- Processed Event
- Recipient Balance

The instruction reads:

- Gateway Configuration
- Guardian Set

The final runtime may also read sysvars / clock / rent / token program accounts depending on actual X1 implementation.

## Atomicity

The schema keeps the previously approved atomicity rule:

    success = balance update + supply update + consumed event mark
    failure = no balance update + no supply update + no consumed event mark

## Non-goals

This stage does not implement:

- live X1 program code
- CPI calls
- deployment scripts
- production PDAs
- live route config
- live guardian keys
- RPC usage

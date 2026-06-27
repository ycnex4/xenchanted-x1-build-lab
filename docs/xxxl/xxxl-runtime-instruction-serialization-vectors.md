# XXXL Runtime Instruction Serialization Vectors

## Purpose

This document defines the deterministic instruction serialization vector layer for XXXL Program v1 runtime planning.

It follows the runtime account serialization vector stage and focuses on the canonical runtime instruction:

    CONSUME_GATEWAY_MINT

This is still TypeScript model code.

It is not deployed X1 runtime code.

It does not use RPC.

It does not require secrets.

## Instruction covered

The serialization layer covers:

- CONSUME_GATEWAY_MINT

Each instruction layout has:

- instruction kind
- layout version
- encoding
- discriminator
- deterministic account meta order
- canonical field order
- field type annotations
- deterministic canonical JSON vector

## Encoding boundary

The selected candidate encoding is:

    CANONICAL_BINARY_V1

This stage does not define final byte-level encoding yet.

It defines the stable instruction layout boundary and deterministic JSON vector representation that later byte-level vectors must preserve.

## Account meta order

SVM programs receive accounts in a deterministic order.

The candidate account meta order for CONSUME_GATEWAY_MINT is:

1. MINT_STATE
2. GATEWAY_CONFIG
3. GUARDIAN_SET
4. PROCESSED_EVENT
5. RECIPIENT_BALANCE
6. SPL_TOKEN_MINT
7. RECIPIENT_TOKEN_ACCOUNT
8. MINT_AUTHORITY_PDA
9. TOKEN_PROGRAM

## Writable and signer boundary

Writable accounts:

- MINT_STATE
- PROCESSED_EVENT
- RECIPIENT_BALANCE
- SPL_TOKEN_MINT
- RECIPIENT_TOKEN_ACCOUNT

Read-only accounts:

- GATEWAY_CONFIG
- GUARDIAN_SET
- MINT_AUTHORITY_PDA
- TOKEN_PROGRAM

The mint authority PDA is not a parent instruction signer.

It is a CPI signer used by the XXXL runtime program when invoking SPL Token `mint_to`.

The token program account is read-only and does not sign.

## Field order

The canonical field order is:

1. instruction
2. version
3. routeId
4. guardianSetId
5. mintId
6. canonicalEventKey
7. recipient
8. amount

## Big integer rule

Runtime bigint values are represented in canonical JSON vectors as decimal strings.

For CONSUME_GATEWAY_MINT this applies to:

- amount

## Vector id

The vector set defines:

- XXXL_RUNTIME_CONSUME_GATEWAY_MINT_INSTRUCTION_V1

## Validation coverage

The tests verify:

- one canonical layout exists for CONSUME_GATEWAY_MINT
- the layout uses canonical binary v1
- account metas are in deterministic SVM order
- instruction and version are first serialized fields
- writable accounts and CPI signer boundary are explicit
- deterministic instruction vector is valid
- amount serializes as a decimal string
- canonical JSON is derived from field order
- missing layouts are rejected
- duplicate layouts are rejected
- wrong account meta order is rejected
- missing mint authority CPI signer is rejected
- wrong canonical JSON is rejected

## Boundary with future stages

This stage prepares for:

- final byte-level instruction vectors
- runtime program skeleton
- SPL Token CPI planning
- dry-run fixtures from the candidate policy package

The next runtime stage should use the account and instruction vectors as the stable boundary for the X1 runtime program skeleton.

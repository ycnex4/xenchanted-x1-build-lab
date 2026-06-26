# XXXL Runtime Account Serialization Vectors

## Purpose

This document defines the first deterministic account serialization vector layer for XXXL Program v1 runtime planning.

It moves the runtime track from a general serialization boundary into explicit account layout and vector fixtures.

This is still TypeScript model code.

It is not deployed X1 runtime code.

It does not use RPC.

It does not require secrets.

## Account kinds covered

The serialization layer covers all candidate runtime account kinds:

- MINT_STATE
- GATEWAY_CONFIG
- GUARDIAN_SET
- PROCESSED_EVENT
- RECIPIENT_BALANCE

Each layout has:

- account kind
- layout version
- encoding
- discriminator
- canonical field order
- field type annotations

## Encoding boundary

The selected candidate encoding is:

    CANONICAL_BINARY_V1

This stage does not define final byte-level encoding yet.

It defines the stable layout boundary and deterministic JSON vector representation that later byte-level vectors must preserve.

## Field order rule

Every account layout starts with:

1. kind
2. version

After that, fields follow the explicit account-specific order.

This makes the account kind discriminator and version visible before account-specific data.

## Big integer rule

Runtime bigint values are represented in canonical JSON vectors as decimal strings.

Examples:

- sourceChainId
- totalSupply
- consumedAmount
- balance

This avoids JavaScript BigInt JSON ambiguity and creates stable fixtures.

## Layouts

### MINT_STATE

Field order:

1. kind
2. version
3. mintId
4. decimals
5. totalSupply
6. authorityMode
7. upgradeAuthorityStatus

### GATEWAY_CONFIG

Field order:

1. kind
2. version
3. routeId
4. sourceChainId
5. sourceToken
6. targetMintToken
7. targetX1NetworkId
8. targetMintCoreId
9. guardianSetId
10. quorumThreshold
11. finalityRuleId
12. status

### GUARDIAN_SET

Field order:

1. kind
2. version
3. guardianSetId
4. guardianPublicKeys
5. quorumThreshold
6. status

### PROCESSED_EVENT

Field order:

1. kind
2. version
3. canonicalEventKey
4. routeId
5. consumed
6. consumedAmount
7. recipient

### RECIPIENT_BALANCE

Field order:

1. kind
2. version
3. mintId
4. owner
5. balance

## Vector ids

The vector set defines:

- XXXL_RUNTIME_MINT_STATE_ACCOUNT_V1
- XXXL_RUNTIME_GATEWAY_CONFIG_ACCOUNT_V1
- XXXL_RUNTIME_GUARDIAN_SET_ACCOUNT_V1
- XXXL_RUNTIME_PROCESSED_EVENT_ACCOUNT_V1
- XXXL_RUNTIME_RECIPIENT_BALANCE_ACCOUNT_V1

## Validation coverage

The tests verify:

- all mandatory account kinds have layouts
- all layouts use canonical binary v1
- kind and version are first
- field order matches the expected field order table
- deterministic vectors are valid
- bigint values serialize as decimal strings
- missing layouts are rejected
- duplicate layouts are rejected
- wrong field order is rejected
- duplicate fields are rejected
- wrong canonical JSON is rejected

## Boundary with future stages

This stage prepares for:

- instruction serialization vectors
- final byte-level account vectors
- runtime program skeleton
- dry-run fixtures from policy package

Instruction serialization is intentionally not added here.

It should be the next runtime stage.

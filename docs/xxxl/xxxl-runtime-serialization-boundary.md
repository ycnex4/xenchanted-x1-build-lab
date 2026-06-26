# XXXL Runtime Serialization Boundary

## Purpose

This document starts the runtime implementation track for XXXL Program v1.

It defines the serialization and authority boundary before writing the live runtime skeleton.

This stage is still model/planning code.

It is not a deployed X1 program.

It does not use RPC.

It does not require secrets.

## Why this stage exists

Theo approved the production-readiness candidate package and identified five non-blocking runtime-stage gaps:

1. CPI atomicity note.
2. Mint authority PDA.
3. Upgrade authority vs mint authority distinction.
4. Runtime supply audit function.
5. Guardian signature verification boundary.

This stage captures those gaps as an explicit runtime serialization and authority boundary.

## Serialization boundary

The candidate runtime must serialize these account kinds:

- MINT_STATE
- GATEWAY_CONFIG
- GUARDIAN_SET
- PROCESSED_EVENT
- RECIPIENT_BALANCE

The candidate runtime must serialize this instruction:

- CONSUME_GATEWAY_MINT

The current boundary selects:

    CANONICAL_BINARY_V1

for both account and instruction serialization.

This is a planning boundary, not yet a final byte layout.

## Mint authority PDA decision

The runtime boundary defines the mint authority as:

    GATEWAY_MINT_AUTHORITY_PDA

Canonical candidate seeds:

    xxxl
    gateway-mint-authority
    v1

This PDA is the authority that signs the SPL Token `mint_to` CPI.

The purpose is to avoid using a discretionary keypair for XXXL supply issuance.

## CPI atomicity note

The runtime plan must explicitly state:

    CPI into SPL Token is atomic with the parent SVM transaction.

This matters because a successful gateway mint must not partially update state.

The intended success transition remains atomic:

1. increase XXXL mint supply
2. mint tokens to recipient through SPL Token CPI
3. mark the processed event as consumed
4. preserve the Genesis supply invariant

If any step fails, the parent transaction must fail and no partial state transition should remain.

## Program upgrade authority vs SPL Token mint authority

The runtime must treat these as distinct authority surfaces:

- program upgrade authority
- SPL Token mint authority

The authority freeze procedure must cover both distinctly.

Freezing only program upgrade authority is not enough if mint authority remains discretionary.

Freezing only mint authority is not enough if program upgrade authority can later reintroduce hidden supply paths.

## Guardian signature verification boundary

XXXL runtime does not re-verify guardian signatures.

Guardian signature verification belongs to Stage 1.

XXXL runtime consumes a Stage 1 authorization result and checks that the result matches the runtime route / guardian / event / recipient / amount boundary.

This prevents duplicated cryptographic responsibility across layers.

## Supply audit function shape

The runtime skeleton should include or plan a read-only audit function:

    auditGenesisSupplyInvariant

The audit shape verifies:

    mintState.totalSupply == splTokenMint.supply == sum(consumed gateway event amounts)

Inputs:

- mintState.totalSupply
- splTokenMint.supply
- processedEvents[].consumed
- processedEvents[].consumedAmount

This function is for runtime safety, monitoring, and public verification.

## Deterministic vector plan

The next runtime stages should produce deterministic vectors for:

- Mint State account
- Gateway Config account
- Guardian Set account
- Processed Event account
- Recipient Balance account
- CONSUME_GATEWAY_MINT instruction

These vectors should become fixtures for production serialization tests.

## Non-goals

This stage does not implement:

- final production account byte layout
- final production instruction byte layout
- live X1 runtime program
- SPL Token CPI code
- deployment scripts
- watcher / relayer runtime
- live guardian signatures
- RPC integration

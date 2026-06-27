# XXXL Runtime Program Skeleton

## Purpose

This document records the first route-aware runtime program skeleton for XXXL Program v1.

The skeleton is still a TypeScript model. It is not a live X1/SVM program and does not connect to RPC.

Its purpose is to connect the previous runtime preparation layers into one executable runtime path.

## Existing layers consumed

The skeleton consumes these already-defined layers:

- runtime candidate account/instruction schema
- runtime transition semantics
- account serialization boundary
- instruction serialization vectors
- Stage 1 authorization consumer contract
- multichain low-weight route policy

## Main instruction

The skeleton models:

    CONSUME_GATEWAY_MINT

Execution path:

1. load accounts
2. validate instruction serialization boundary
3. validate optional route policy
4. consume Stage 1 authorization result
5. simulate SPL Token `mint_to` CPI boundary
6. mark processed event
7. update mint state mirror
8. update recipient balance mirror
9. audit Genesis supply invariant

## Route-aware design

The skeleton must not hardcode Ethereum-only assumptions.

Ethereum remains the primary full-weight route.

Non-Ethereum routes, such as Avalanche, may be supported only through explicit low-weight route policy.

This means the runtime path uses `routeId` from instruction data and gateway config.

Initial active deployment may still be Ethereum-only.

The skeleton simply prevents runtime design from becoming structurally Ethereum-only.

## Guardian signature boundary

The runtime skeleton does not verify guardian signatures.

Guardian signature verification remains in Stage 1.

The runtime consumes the Stage 1 authorization result only:

    STAGE_1_AUTHORIZATION_RESULT_ONLY

This preserves the boundary:

    Stage 1 verifies and authorizes.
    XXXL runtime consumes the authorized result.

## SPL Token CPI boundary

The skeleton models SPL Token `mint_to` as a CPI step.

The CPI step is atomic with the parent transaction.

The mint authority is a PDA used as CPI signer.

The parent instruction does not require the mint authority PDA to be an external signer.

Roles:

- SPL Token mint account: writable
- recipient token account: writable
- mint authority PDA: CPI signer
- token program: read-only

## Atomicity model

If any preflight validation fails, no transition is executed.

If the transition rejects, the original accounts are preserved.

If the CPI/supply audit boundary would fail, the skeleton result is rejected and accounts are not committed.

The intended runtime model is:

    one transaction
    one instruction
    CPI atomic with parent transaction
    no partial state commit

## Supply audit

The skeleton audits the accepted mint amount:

    expectedTotalSupplyAfter = totalSupplyBefore + acceptedMintAmount
    expectedRecipientBalanceAfter = recipientBalanceBefore + acceptedMintAmount

The processed event must be consumed and must record the accepted mint amount.

## Non-goals

This stage does not implement a live X1 program.

It does not serialize real account bytes.

It does not derive real PDAs.

It does not submit transactions.

It does not connect to RPC.

It does not activate Avalanche route.

It does not verify guardian signatures inside runtime.

It does not change XXXL supply policy.

## Status

This is the first route-aware runtime skeleton before real X1/SVM implementation.

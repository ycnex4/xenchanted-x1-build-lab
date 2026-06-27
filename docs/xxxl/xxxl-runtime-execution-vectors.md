# XXXL Runtime Execution Vectors

## Purpose

This document records deterministic execution vectors for the XXXL runtime program skeleton.

The vectors turn the route-aware skeleton behavior into stable test cases before runtime dry-run fixtures.

## Vector scope

The vectors cover:

- valid Ethereum primary full-weight gateway mint execution
- valid Avalanche low-weight route-aware execution
- invalid route policy rejection
- missing route rejection
- Stage 1 authorization rejection
- replay rejection
- event key mismatch rejection
- instruction serialization boundary rejection

## Core boundary

Runtime execution vectors do not verify guardian signatures.

They consume the Stage 1 authorization result.

This preserves the boundary:

    Stage 1 verifies signatures and authorizes.
    XXXL runtime consumes the authorized result.

## Route-aware behavior

The vectors confirm that runtime execution is not hardcoded as Ethereum-only.

Ethereum remains the primary full-weight route.

Avalanche can be represented only through explicit low-weight route policy.

The Avalanche execution vector uses an already authorized XXXL mint amount.

The weight calculation remains upstream in route policy / Stage 1 authorization.

## CPI boundary

Successful vectors include a modeled SPL Token `mint_to` CPI step.

The CPI step is atomic with the parent transaction.

The mint authority PDA is the CPI signer.

Failed vectors skip CPI.

## Atomicity and rollback

Rejected vectors preserve original account state.

Preflight failures reject before transition execution.

Transition failures reject without committing CPI.

## Canonical JSON

Each vector includes canonical JSON.

BigInt values are serialized as decimal strings.

This gives stable execution references for the next dry-run fixture stage.

## Non-goals

This stage does not implement live X1/SVM code.

It does not derive real PDAs.

It does not serialize actual account bytes.

It does not submit transactions.

It does not activate Avalanche route.

It does not change supply policy.

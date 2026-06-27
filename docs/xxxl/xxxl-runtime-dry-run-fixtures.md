# XXXL Runtime Dry-Run Fixtures

## Purpose

This document records the first runtime dry-run fixture package for XXXL Program v1.

The fixtures sit after runtime execution vectors and before real runtime dry-run/deployment fixtures.

They are still TypeScript/model-layer fixtures.

## What the fixtures do

A dry-run fixture:

1. selects deterministic execution vectors
2. validates the execution vector set
3. reruns the runtime program skeleton for each selected scenario
4. compares runtime skeleton output with the stored execution vector
5. produces a dry-run report

## Fixture groups

The default fixture set contains:

- all execution vectors
- successful routes
- preflight rejections
- transition rejections

## Successful routes

The successful route fixture includes:

- Ethereum primary full-weight gateway mint
- Avalanche low-weight route-aware gateway mint

This confirms the runtime path is route-aware and not hardcoded as Ethereum-only.

## Rejection fixtures

Preflight rejections include:

- invalid route policy
- missing route
- invalid instruction serialization boundary

Transition rejections include:

- Stage 1 authorization rejection
- replay rejection
- event-key mismatch rejection

Rejected vectors are successful dry-run outcomes when the rejection matches expectation.

## CPI boundary

Successful dry-run reports confirm that CPI is not skipped.

Rejected dry-run reports confirm that CPI is skipped.

This keeps the intended parent-transaction atomicity boundary explicit.

## Non-goals

This stage does not implement live X1/SVM code.

It does not connect to RPC.

It does not derive real PDAs.

It does not serialize real account bytes.

It does not submit transactions.

It does not activate Avalanche route.

It does not change supply policy.

## Status

This is a deterministic model-layer dry-run fixture package before real runtime dry-run fixtures.

# XXXL Runtime Route, Guardian, and Finality Policy Candidate

## Purpose

This document defines the first candidate policy layer between the runtime schema and live deployment readiness.

This is still planning / model work.

It is not production X1 runtime code.

## Why this stage exists

Theo review listed these production-readiness items as missing or partial:

- final route config
- guardian rotation policy
- finality policy
- authority freeze procedure

This stage defines a deterministic candidate policy for route, guardian, and finality configuration.

## Route policy

The Genesis Phase route must define:

- route id
- source chain id
- source token
- target mint token
- target X1 network id
- target mint core id
- guardian set id
- quorum threshold
- finality rule id
- route status

Required Genesis Phase values:

- route id: canonical XXXL gateway route
- source chain id: Ethereum mainnet
- target mint token: XXXL
- status: ACTIVE

The source token must be fixed before production deployment.

## Guardian policy

The guardian policy defines:

- guardian set id
- guardian public keys
- quorum threshold
- rotation mode
- rotation timelock
- emergency freeze threshold
- status

The candidate requires:

- non-empty guardian set
- no duplicate guardian keys
- valid quorum threshold
- active guardian set
- rotation timelock of at least 7 days
- emergency freeze threshold at least equal to quorum threshold

## Finality policy

The finality policy defines:

- finality rule id
- source chain id
- finality kind
- minimum confirmations
- status

Supported candidate finality kinds:

- Ethereum finalized
- Ethereum safe with confirmations

For Ethereum finalized mode:

    minConfirmations = 0

For safe-with-confirmations mode:

    minConfirmations >= 64

## Deployment meaning

This policy is the bridge between:

- abstract runtime account / instruction schema
- production route configuration

It does not yet choose live guardian keys or live addresses.

Those must be fixed in a later deployment-readiness stage.

## Account derivation

The policy can build:

- Gateway Configuration account candidate
- Guardian Set account candidate

These accounts feed the runtime candidate schema.

## Non-goals

This stage does not implement:

- live X1 program code
- live guardian keys
- live source token address
- production PDA derivation
- deployment scripts
- RPC usage

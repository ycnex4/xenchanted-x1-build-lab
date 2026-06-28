# Bootstrap Guardian Policy

Status: DRAFT POLICY.

This document defines the acceptable early-stage guardian model for the XXXL gateway.

## Purpose

The gateway may need a working guardian set before independent validators or external operators are ready to participate.

A bootstrap guardian set is acceptable only if it is named honestly, limited, and publicly auditable.

## Correct name

The correct term is:

    operator-controlled bootstrap guardian set

This means the initial guardian keys may be controlled by the project operator during early launch.

## Incorrect claim

The bootstrap set must not be described as:

    decentralized guardian network

unless the guardians are actually operated independently.

## Security meaning

Five guardian keys on one server do not provide five independent security domains.

They provide a threshold-signature workflow controlled by one operator environment.

This may be acceptable for bootstrap, but it must not be misrepresented.

## Minimum bootstrap requirements

If bootstrap guardians are used, the setup should satisfy the following:

- separate key files
- no private keys in logs
- no private keys in git
- no secrets printed in terminal output
- encrypted storage where practical
- separate relayer key
- clear guardian set id
- public guardian public keys
- public threshold
- amount caps
- proof bundle for every mint
- monitoring of all accepted and rejected mint attempts
- planned migration to independent guardians

## Mint rule

Bootstrap guardians may sign only canonical XNTD burn-backed XXXL mint messages.

They must not sign arbitrary mint messages.

Every signed mint must bind:

- route id
- source chain id
- source token
- source burn transaction
- source event index
- canonical event key
- X1 recipient
- burned amount
- mint amount
- mint token
- finality/deadline field
- message nonce

## Public proof bundle

Every accepted mint should be publishable as a proof bundle containing:

- Ethereum burn tx hash
- event index
- source block number
- source block hash
- canonical event key
- burned amount
- XXXL mint amount
- X1 recipient
- guardian set id
- guardian signatures
- X1 transaction id

The goal is that observers do not need to trust the operator's word.

They can check the burn-to-mint link.

## Caps

Bootstrap mode should have blast-radius limits.

Suggested caps:

- per-mint cap
- daily mint cap
- manual review threshold
- temporary high-value delay
- public cap disclosure

Caps are a safety layer, not a correctness layer.

## Migration path

The bootstrap guardian set should be replaced over time with independent guardians.

Suggested phases:

1. operator-controlled bootstrap set
2. operator keys split across independent infrastructure
3. first external guardian added
4. external guardian majority
5. bootstrap set deprecated
6. independent guardian set becomes default

## Reputation rule

The project should never ask the community to trust hidden minting behavior.

The gateway must make every mint explainable, auditable, and burn-backed.

## Decision

Bootstrap guardians are acceptable only as a temporary, disclosed, capped, auditable launch mode.

The long-term route is independent guardian operation.

# XXXL Live Route Activation Checklist

Status: DRAFT POLICY.

This document defines the conditions that must be satisfied before the XXXL gateway route may be activated for live minting.

The current runtime state is intentionally non-live.

Live route activation is a separate milestone.

## Current safety posture

The runtime has boundaries for:

- instruction decode
- guarded account validation
- execution plan construction
- CPI planning
- local state mutation composition
- disabled SPL CPI gate integration

The live `process_instruction` path must remain disabled until all activation gates below are satisfied.

## Non-negotiable rule

No live XXXL mint may be executed unless it is backed by a canonical XNTD burn proof.

Every live mint must be publicly explainable by:

- Ethereum source burn transaction
- burn event index
- source block number
- source block hash
- canonical event key
- burned amount
- recipient binding
- route id
- guardian approval set
- X1 mint transaction

## Activation gates

### Gate 1: Runtime account contract

Before activation, the account contract must be frozen and documented:

- exact account order
- writable accounts
- readonly accounts
- signer requirements
- PDA requirements
- owner requirements
- rent-exemption requirements
- SPL mint requirements
- recipient token account requirements

Any account-order change after this gate requires a new review.

### Gate 2: Mint authority model

Before activation, the XXXL SPL mint authority must be proven to be the gateway mint authority PDA.

The mint authority must not be an operator wallet.

The runtime must reject wrong PDA and wrong bump.

### Gate 3: Replay protection

Before activation, replay protection must be proven at the runtime account level.

A canonical event key may be consumed only once.

Replay attempts must not mint XXXL.

Replay attempts must not mutate recipient balance.

Replay attempts must not change SPL mint supply.

### Gate 4: Atomicity

Before activation, the runtime must prove the intended atomic order.

The accepted live order must be explicitly documented before activation.

No partial state is allowed.

If SPL CPI fails, the route must not leave a misleading processed-event state.

If local state mutation fails, SPL CPI must not execute.

### Gate 5: SPL CPI execution proof

Before activation, the guarded SPL CPI path must be proven in a controlled test-only environment.

This proof must show:

- correct PDA signer seeds
- correct SPL Token program id
- correct mint account
- correct recipient token account
- correct mint amount
- SPL mint supply changes only on accepted mint
- recipient token account balance changes only on accepted mint
- invalid cases do not mint

### Gate 6: Guardian approval model

Before activation, guardian approval must be documented and test-covered.

The policy must define:

- guardian set id
- threshold
- guardian public keys
- message hash
- signature domain
- route id binding
- recipient binding
- amount binding
- deadline/finality binding
- replay binding

### Gate 7: Bootstrap guardian disclosure

If the initial guardian set is operator-controlled, the gateway must disclose that status.

The correct wording is:

    operator-controlled bootstrap guardian set

The incorrect wording is:

    decentralized guardian network

until guardians are independently operated.

### Gate 8: Public proof log

Before activation, every accepted mint must be capable of being published as a proof bundle.

The proof bundle must include enough data for an outside observer to verify that XXXL minting corresponds to XNTD burn history.

### Gate 9: Caps and blast-radius limits

Before activation, bootstrap mode must have limits.

Examples:

- per-mint cap
- daily cap
- manual review threshold
- delayed high-value mint threshold
- public cap policy

Caps are not a replacement for correctness.

Caps limit damage if infrastructure or keys are compromised.

### Gate 10: Monitoring and incident response

Before activation, monitoring must exist for:

- accepted mints
- failed submits
- replay attempts
- wrong route attempts
- wrong recipient attempts
- guardian disagreement
- amount mismatch
- unusually large mint attempts

Incident response must be defined before live minting.

### Gate 11: External review

Before activation, the live route must receive a final review pass.

The review must cover:

- runtime account validation
- PDA model
- SPL CPI path
- replay protection
- guardian approval model
- proof bundle
- bootstrap guardian disclosure
- caps
- monitoring
- activation/deactivation process

## Explicit non-goals before activation

Before the activation milestone, the project must not claim:

- independent guardian decentralization
- live bridge finality
- permissionless production bridge operation
- production-grade external guardian network

unless those are actually true.

## Decision

Live route activation is blocked until this checklist is satisfied.

The next implementation stages may continue proving isolated boundaries, but they must not enable live `process_instruction` minting until activation is explicitly approved.

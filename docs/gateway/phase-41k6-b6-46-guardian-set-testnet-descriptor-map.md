# Phase 41K.6 B6.46 — Guardian set testnet descriptor map

Status:

GUARDIAN_SET_TESTNET_DESCRIPTOR_MAP_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document maps blocker F:

guardian set testnet descriptor.

It defines the descriptor requirements for a future testnet guardian set.

This is docs-only.

It does not create a guardian descriptor.

It does not construct guardian packages.

It does not sign messages.

It does not run a local validator.

It does not build, deploy, upgrade, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, or perform SPL CPI minting.

## Current blocker F status

Blocker F:

guardian set testnet descriptor

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Descriptor principle

The guardian set descriptor must be explicit, deterministic, auditable, and separated from signing material.

The descriptor may contain public verification data.

The descriptor must not contain signing material.

The descriptor must not contain keypair paths.

The descriptor must not contain mnemonic material.

The descriptor must not approve submit flow by itself.

## Required descriptor fields

A future guardian set descriptor must include:

- descriptor_version
- descriptor_status
- network_scope
- route_id
- source_chain_id
- mint_token
- guardian_set_id
- threshold
- guardian_count
- guardian_public_keys
- public_key_encoding
- signature_algorithm
- message_hash_algorithm
- canonical_message_schema_version
- descriptor_created_at
- descriptor_source_commit
- descriptor_integrity_hash
- activation_status
- activation_block_or_slot_if_any
- expiration_policy_if_any
- rotation_policy
- emergency_disable_policy
- no_signing_material_statement

## Required network boundary

The future descriptor must explicitly state its network scope.

Allowed future values must be explicit.

For this blocker, the intended future scope is:

testnet only

The descriptor must not be valid for production unless a separate production descriptor is created and approved.

## Required guardian set id boundary

The future descriptor must define:

- guardian_set_id
- whether guardian_set_id is new or existing
- whether it replaces a prior set
- whether it is active or pending
- whether old sets remain valid
- replay and rotation implications

If guardian_set_id is ambiguous, blocker F cannot close.

## Required threshold boundary

The future descriptor must define:

- guardian_count
- threshold
- minimum valid approvals
- duplicate guardian handling
- unknown guardian handling
- inactive guardian handling
- threshold failure behavior

The threshold must be less than or equal to guardian_count.

The threshold must be greater than zero.

## Required public key boundary

The future descriptor must define public keys only.

Required rules:

- public keys must be unique
- public keys must be encoded consistently
- public keys must be sorted or order-defined
- public keys must not be empty
- public keys must not be placeholders
- public keys must not be private keys
- public keys must not include keypair paths
- public keys must not include mnemonic data

## Required signature boundary

The future descriptor must define:

- signature algorithm
- public key encoding
- signature encoding
- message hash algorithm
- signed payload boundary
- canonical message field order
- guardian approval format
- duplicate signature handling
- invalid signature handling
- unknown signer handling

The current intended algorithm remains:

ed25519

The descriptor must not include private signing material.

## Required descriptor integrity boundary

The future descriptor must have an integrity marker.

Required fields:

- descriptor_integrity_hash_algorithm
- descriptor_integrity_hash_value
- descriptor_source_commit
- descriptor_file_path
- descriptor_review_status

Preferred hash algorithm:

SHA-256

If descriptor integrity is missing, blocker F cannot close.

## Required runtime relationship

The descriptor must map to runtime state.

Required mapping:

- descriptor guardian_set_id maps to guardian_set account
- descriptor threshold maps to guardian_set threshold
- descriptor public keys map to guardian_set public keys
- descriptor status maps to active or inactive runtime state
- descriptor route scope maps to gateway_config route constraints
- descriptor message schema maps to verifier expectations

If descriptor and runtime state diverge, all submit flow must stop.

## Required negative cases

Blocker F closure requires coverage or explicit planned coverage for:

- threshold zero
- threshold greater than guardian count
- empty guardian list
- duplicate guardian public key
- malformed public key
- placeholder public key
- unknown guardian approval
- duplicate guardian approval
- invalid signature
- signature over wrong message
- signature over wrong route
- signature over wrong recipient
- signature over wrong amount
- wrong guardian_set_id
- inactive guardian_set
- descriptor hash mismatch
- descriptor route mismatch
- descriptor network mismatch

## Required local-validator evidence

Before blocker F can close as execution-ready, local-validator evidence should show:

- local guardian_set fixture exists
- guardian_set descriptor maps to local guardian_set fixture
- threshold success path works
- threshold failure path fails
- duplicate guardian approval fails
- unknown guardian approval fails
- invalid signature package fails
- wrong message signature fails
- failure cases preserve no-mutation invariant

This checkpoint does not run local validator.

## Required testnet readiness evidence

Before any testnet guardian set initialization or submit rehearsal, evidence must include:

- scoped written GO
- descriptor file path
- descriptor integrity hash
- guardian_set_id
- threshold
- guardian public keys
- network scope
- route scope
- expected guardian_set PDA or account
- initialization command if applicable
- read-only verification command
- abort conditions
- recovery policy

No testnet guardian action is approved by this checkpoint.

## Forbidden until scoped GO

The following remain forbidden:

- creating a testnet guardian set
- initializing guardian_set account on testnet
- constructing non-local guardian packages
- signing gateway messages
- submitting gateway mint transactions
- enabling live route
- using production guardian material
- using keypair paths
- using private signing material

## Relationship to other blockers

Blocker F depends on or interacts with:

- blocker C: handler boundary must not bypass validation
- blocker D: state initialization must define guardian_set account layout
- blocker E: minting must not happen without valid guardian authorization
- blocker G: rollback and recovery must cover descriptor mismatch
- blocker H: local-validator dry-run must prove guardian success and failure paths

Current state:

These blockers are not closed for execution readiness.

## Explicit non-closure

This checkpoint does not close blocker F.

It maps guardian set descriptor requirements only.

Current blocker F state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a local-only guardian descriptor skeleton plan.

No guardian descriptor is created by this checkpoint.

No guardian package is constructed by this checkpoint.

No local-validator execution is approved by this checkpoint.

No testnet action is approved by this checkpoint.

Current decision remains:

NO-GO.

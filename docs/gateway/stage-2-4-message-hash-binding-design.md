# Stage 2.4 Message Hash Binding Design

This document defines the Stage 2.4 message_hash binding model for the X1-side gateway runtime.

## Purpose

Guardian signatures must not authorize an opaque message_hash only.

They must authorize a message_hash that is deterministically bound to the exact submitted gateway context.

This prevents a relayer from reusing a valid guardian signature for a different canonical_event_key, recipient, amount, route, guardian_set_version, or deadline/finality context.

## Problem

The current Stage 2 guardian signature prototype verifies that guardians signed the provided message_hash.

However, without on-chain binding, the program does not yet prove that message_hash corresponds to the same values passed into submit_mint_approval.

A relayer must not be able to take a signature for:

- canonical_event_key A
- recipient Alice
- amount 100

and submit it with:

- canonical_event_key B
- recipient relayer
- amount 100

## Binding model

The signed message_hash should be derived from an ordered canonical context.

Stage 2.4 canonical context fields:

1. message_type
2. route_id
3. source_chain_id
4. source_token
5. canonical_event_key
6. x1_recipient
7. minted_amount
8. guardian_set_version
9. deadline_or_finality_block
10. message_nonce

## Canonical construction

The reference construction is:

    message_hash = keccak256(
        message_type_hash ||
        route_id_hash ||
        source_chain_id_u64_be ||
        source_token_hash ||
        canonical_event_key ||
        x1_recipient ||
        minted_amount_u64_be ||
        guardian_set_version_u64_be ||
        deadline_or_finality_block_u64_be ||
        message_nonce
    )

Where:

- message_type_hash = keccak256("XEC_STAGE2_DIRECT_MINT_APPROVAL_V1")
- route_id_hash = keccak256(route_id string)
- source_token_hash = keccak256(source token identifier)
- canonical_event_key is bytes32
- x1_recipient is bytes32
- message_nonce is bytes32
- all numeric fields use fixed-width u64 big-endian encoding

## Required properties

The derived message_hash must change if any of these fields change:

- canonical_event_key
- x1_recipient
- minted_amount
- route_id
- source_chain_id
- source_token
- guardian_set_version
- deadline_or_finality_block
- message_nonce
- message_type

## Runtime requirement

Before token mint CPI is added, the runtime program must derive expected_message_hash from the submitted context and require:

    expected_message_hash == message_hash

Only after this check should guardian signature verification be considered bound to the submitted context.

## Guardian signing rule

Guardians must sign only the derived message_hash.

They must not sign arbitrary opaque hashes.

A guardian approval is valid only for the exact context used to derive the message_hash.

## Stage 2.4 prototype boundary

This stage defines and locally tests the deterministic binding model.

It does not yet prove:

- live X1 testnet execution
- on-chain hash syscall availability
- token mint CPI
- production gateway readiness

## Production blocker status

Message hash binding is a production blocker.

The gateway must not mint real XXXL through token mint CPI until message_hash binding is implemented and tested in the runtime program.

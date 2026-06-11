# Stage 2.4 Runtime Message Hash Binding Evidence

This document records local runtime prototype evidence for Stage 2.4 message_hash binding.

## Scope

This is local runtime implementation and test evidence.

It is not live X1 testnet execution evidence.

It is not production gateway readiness.

It does not prove token mint CPI.

It does not prove final XXXL mint integration.

## Prototype repository

Runtime prototype repository:

- ~/xenchanted-x1-lab/hello-x1

Prototype branch:

- stage-2-guardian-signature-verification

Relevant local commits:

- d8bd927 Bind guardian message hash to mint context
- bec701b Update gateway test to sign bound message hash
- 7207e0f Fix gateway test bound hash arguments

## Background

Stage 2.4 design defined that guardian signatures must not authorize an opaque message_hash only.

The message_hash must be deterministically bound to the submitted gateway context.

This closes the relayer-reuse issue identified during Theo review, where a signature for one canonical event could otherwise be reused with a different canonical_event_key, recipient, or amount.

## Runtime implementation

The runtime prototype now derives expected_message_hash inside submit_mint_approval.

The handler now accepts:

- canonical_event_key
- recipient
- minted_amount
- deadline_or_finality_block
- message_nonce
- message_hash
- approved_guardians

The runtime derives expected_message_hash from:

- message_type
- route_id
- source_chain_id
- source_token
- canonical_event_key
- recipient
- minted_amount
- guardian_set_version
- deadline_or_finality_block
- message_nonce

Then it requires:

    expected_message_hash == message_hash

Only after that check does guardian Ed25519 signature verification proceed.

## Hash implementation

The runtime prototype uses:

- solana-keccak-hasher = 3.1.0

This provides keccak hashv support through the Solana/X1 syscall path.

The runtime build successfully compiled with this dependency.

## Gateway test update

The TypeScript gateway test was updated to derive the same context-bound message_hash before creating Ed25519 verification instructions.

The submitMintApproval test call now passes the full bound context:

- canonicalEventKey
- recipient
- mintedAmount
- deadlineOrFinalityBlock
- messageNonce
- messageHash
- approvedGuardians

The older 5-argument submitMintApproval shape was removed from the gateway test.

## Local test evidence

Local binding tests:

    cargo test -p hello-x1 binding_

Result:

    7 passed
    0 failed

The binding tests verify that the derived message_hash changes when the following fields change:

- canonical_event_key
- recipient
- minted_amount
- guardian_set_version
- deadline_or_finality_block
- message_nonce

They also verify stable derivation for the same context.

Local parser tests:

    cargo test -p hello-x1 parser_

Result:

    7 passed
    0 failed

The parser tests verify Ed25519 instruction layout handling for:

- correct guardian pubkey and message_hash acceptance
- wrong guardian pubkey rejection
- wrong message_hash rejection
- non-32-byte message rejection
- truncated instruction data rejection
- multi-signature Ed25519 instruction data rejection
- non-current instruction index rejection

Build:

    anchor build

Result:

    passed

## Known non-blocking warnings

The runtime build still reports existing warnings:

- ambiguous glob re-exports from existing instruction module exports
- AccountInfo deprecation warning for instructions_sysvar

These are not blockers for the Stage 2.4 runtime binding evidence.

## What this proves

This proves locally that the runtime prototype no longer treats message_hash as an opaque unbound value.

The runtime now derives message_hash from submitted mint context and rejects mismatched hashes before guardian signature verification.

This directly addresses the main production blocker identified by Theo at the prototype level.

## What this does not prove

This does not yet prove:

- live X1 testnet execution
- X1 testnet deploy of this refined implementation
- live instructions sysvar behavior with context-bound Ed25519 messages
- token mint CPI
- production gateway readiness

## Current conclusion

Stage 2.4 message_hash binding has moved from design/reference-test evidence into local runtime prototype implementation.

The next evidence target is live X1 testnet execution of the refined guardian signature flow:

- context-bound message_hash
- Ed25519 instruction scanning
- non-Ed25519 instruction interleaving
- ProcessedBurnEntry creation
- replay rejection
- wrong context rejection

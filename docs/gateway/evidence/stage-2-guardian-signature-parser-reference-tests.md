# Stage 2 Guardian Signature Parser Reference Tests

This document records local parser-level reference test evidence for Stage 2 guardian signature verification.

## Scope

This is local reference test evidence.

It is not live X1 testnet execution evidence.

It is not production gateway readiness.

It does not prove token mint CPI.

It does not prove final XXXL mint integration.

## Prototype repository

Runtime prototype repository:

- ~/xenchanted-x1-lab/hello-x1

Prototype branch:

- stage-2-guardian-signature-verification

Local commits:

- ccdc41c Add guardian signature verification prototype
- 104bdd7 Add guardian signature parser reference tests

## What was tested

Reference tests were added directly around the Ed25519 instruction parser used by submit_mint_approval.

The parser checks the Ed25519 instruction data layout used by the Solana / X1 Ed25519 verification instruction.

The parser verifies:

- signature count
- signature offset bounds
- public key offset bounds
- message offset bounds
- message size
- guardian public key match
- message_hash match
- truncated instruction rejection

## Test command

The local parser reference tests were run with:

    cargo test -p hello-x1 parser_

## Test result

The parser reference test result:

    running 5 tests
    parser_accepts_expected_guardian_pubkey_and_message_hash ... ok
    parser_rejects_wrong_guardian_pubkey ... ok
    parser_rejects_wrong_message_hash ... ok
    parser_rejects_non_32_byte_message ... ok
    parser_rejects_truncated_instruction_data ... ok

    test result: ok. 5 passed; 0 failed

## Build result

anchor build completed successfully after adding the parser reference tests.

Known non-blocking warnings remained:

- ambiguous glob re-exports from existing instruction module exports
- AccountInfo deprecation warning for instructions_sysvar

## What this proves

This proves that the local parser logic correctly accepts and rejects the intended Ed25519 instruction data layouts.

It strengthens the guardian signature verification prototype beyond compile-only evidence.

## What this does not prove

This does not yet prove:

- live X1 testnet execution
- successful X1 program deploy with this implementation
- live instructions sysvar behavior on X1
- real transaction execution through the public X1 testnet RPC
- token mint CPI
- production gateway readiness

## Current conclusion

Stage 2 guardian signature verification now has local parser reference test evidence.

The next evidence target remains live X1 testnet execution once the X1 testnet RPC / deploy path is stable.

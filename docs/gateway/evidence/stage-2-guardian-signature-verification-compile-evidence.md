# Stage 2 Guardian Signature Verification Compile Evidence

This document records the first compile-level implementation evidence for Stage 2 guardian signature verification in the X1 runtime prototype.

## Scope

This is compile-level implementation evidence.

It is not live X1 testnet execution evidence.

It is not production gateway readiness.

It does not prove final XXXL mint integration.

It does not prove token mint CPI.

It does not prove production watcher or relayer correctness.

## Prototype repository

Runtime prototype repository:

- ~/xenchanted-x1-lab/hello-x1

Prototype branch:

- stage-2-guardian-signature-verification

Local commit:

- ccdc41c Add guardian signature verification prototype

## What changed

The runtime prototype now extends submit_mint_approval beyond plain guardian key membership checking.

The instruction now accepts:

- canonical_event_key
- recipient
- minted_amount
- message_hash
- approved_guardians

The instruction now requires the instructions sysvar account and checks prior Ed25519 verification instructions in the same transaction.

## X1 modular crate compatibility

The X1 / Anchor toolchain does not expose the old monolithic solana-program import path for this feature.

The implementation uses modular Solana crates instead:

- solana-instructions-sysvar = 3.0.1
- solana-sdk-ids = 3.1.0

These provide:

- load_current_index_checked
- load_instruction_at_checked
- instructions sysvar ID
- ed25519_program ID

## Verification model

For each approved guardian, submit_mint_approval expects a preceding Ed25519 verification instruction.

The runtime checks:

1. enough prior Ed25519 instructions exist
2. each prior instruction belongs to the Ed25519 program
3. each Ed25519 instruction contains one signature
4. public key offset points to the expected guardian public key
5. message offset points to the expected message_hash
6. message length is exactly 32 bytes
7. guardian public key belongs to the active GuardianSet
8. duplicate guardian approvals are rejected
9. unknown guardian approvals are rejected

## Tests updated

The runtime test file was updated to build transactions with real Ed25519 verification instructions before submit_mint_approval.

The intended test coverage is:

- valid guardian signatures create ProcessedBurnEntry
- replay with the same canonical_event_key is rejected
- missing Ed25519 verification instruction is rejected
- unknown guardian with valid Ed25519 signatures is rejected
- failed invalid path does not create ProcessedBurnEntry

## Compile result

anchor build completed successfully after the implementation.

Known non-blocking warnings remained:

- ambiguous glob re-exports from existing instruction module exports
- AccountInfo deprecation warning for instructions_sysvar

These warnings do not block the compile-level evidence.

## Live X1 testnet status

Live X1 testnet deploy and runtime tests were attempted but not completed because the public X1 testnet RPC / deploy path became unreliable.

Observed infrastructure symptoms included:

- 429 Too Many Requests
- Blockhash expired retries
- deploy waiting for next block for an extended time
- Max retries exceeded during program data writes

Because of that, this document intentionally does not claim successful live X1 testnet execution.

## Current conclusion

Stage 2 guardian signature verification has moved from design-only planning into a compile-level runtime prototype.

The next evidence target is live X1 testnet execution once RPC / deploy path is stable.

Until that live test passes, this implementation should be treated as a prototype, not production gateway evidence.

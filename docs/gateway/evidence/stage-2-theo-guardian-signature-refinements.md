# Stage 2 Theo Guardian Signature Refinements

This document records the Stage 2 guardian signature verification refinements applied after Theo review.

## Scope

This is local implementation and reference-test evidence.

It is not live X1 testnet execution evidence.

It is not production gateway readiness.

It does not prove token mint CPI.

It does not prove final XXXL mint integration.

## Prototype repository

Runtime prototype repository:

- ~/xenchanted-x1-lab/hello-x1

Prototype branch:

- stage-2-guardian-signature-verification

Local commits included in this refinement stage:

- ccdc41c Add guardian signature verification prototype
- 104bdd7 Add guardian signature parser reference tests
- 6adbeb0 Scan prior Ed25519 instructions for guardian approvals
- f898350 Limit guardian parser helper to tests

## Theo review conclusion

Theo confirmed that Ed25519 verification instruction plus instructions sysvar inspection is the correct SVM runtime direction.

However, Theo identified two important issues:

1. The initial implementation used "immediately preceding N instructions".
2. The message_hash is still not bound on-chain to canonical_event_key, recipient, minted_amount, route, and guardian_set_version.

Theo recommended scanning prior transaction instructions instead of assuming the N immediately preceding instructions are Ed25519 verification instructions.

Theo also identified message hash binding as a production blocker before real token mint CPI.

## Refinement applied: scanning prior instructions

The runtime prototype now scans all prior transaction instructions before submit_mint_approval.

For each approved guardian, the program:

1. scans instructions from index 0 to current_index - 1
2. skips non-Ed25519 instructions
3. parses Ed25519 instruction data
4. checks that each Ed25519 instruction has exactly one signature
5. checks that instruction indexes point to the current Ed25519 instruction data
6. checks that the embedded guardian public key matches the expected guardian
7. checks that the embedded message matches the expected message_hash
8. requires a matching signature for every approved guardian

This replaces the earlier "immediately preceding N instructions" assumption.

## Duplicate approvals

The runtime still rejects duplicate approved_guardians before scanning.

This prevents counting the same guardian more than once.

## Test update for interleaving

The live transaction test was updated to include a non-Ed25519 instruction between Ed25519 verification instructions and submit_mint_approval.

The prepared transaction shape is now:

1. Ed25519 instruction for guardian A
2. SystemProgram transfer
3. Ed25519 instruction for guardian B
4. submit_mint_approval

This is intended to prove that scanning works when non-Ed25519 instructions are interleaved.

This still requires live X1 testnet execution once the public X1 testnet RPC / deploy path is stable.

## Parser reference tests expanded

Parser reference tests were expanded from 5 to 7.

Current parser test result:

- cargo test -p hello-x1 parser_
- 7 passed
- 0 failed

The parser tests now cover:

- correct guardian pubkey and message_hash acceptance
- wrong guardian pubkey rejection
- wrong message_hash rejection
- non-32-byte message rejection
- truncated instruction data rejection
- multi-signature Ed25519 instruction data rejection
- non-current instruction index rejection

## Build result

anchor build completed successfully after applying the scanning refinement.

Known non-blocking warnings remain:

- ambiguous glob re-exports from existing instruction module exports
- AccountInfo deprecation warning for instructions_sysvar

The previous dead_code warning for the test-only parser helper was removed by gating the helper with cfg(test).

## Message hash binding status

Theo identified message_hash binding as a structural issue.

Current prototype verifies that guardians signed the provided message_hash.

However, the on-chain program does not yet derive or verify that message_hash is bound to:

- canonical_event_key
- recipient
- minted_amount
- route_id
- guardian_set_version
- deadline or finality context

This is acceptable as a local prototype / MVP assumption only if guardians are trusted to verify the full off-chain context before signing.

It is not acceptable as production gateway logic before token mint CPI.

Before production minting is enabled, message_hash must be bound to the exact submitted context.

Possible future direction:

- on-chain hash derivation if the X1 runtime exposes a practical keccak or hash syscall
- canonical message hash construction including canonical_event_key, recipient, minted_amount, route_id, guardian_set_version, and deadline/finality fields
- require signed_message_hash == derived_expected_hash

## Current conclusion

Theo's first refinement was applied:

- guardian signature verification now scans prior instructions instead of requiring immediately preceding instructions.

Local parser evidence was strengthened:

- 7 parser tests passed
- anchor build passed

The next technical blocker before production direction is message_hash binding.

The next evidence target remains live X1 testnet execution once the X1 testnet RPC / deploy path is stable.

# Stage 2.4 Runtime Binding X1 Testnet Evidence

This document records live X1 testnet evidence for the refined Stage 2.4 guardian signature flow with context-bound message_hash.

## Scope

This is live X1 testnet runtime evidence.

It is not production gateway readiness.

It does not prove token mint CPI.

It does not prove final XXXL mint integration.

## Runtime prototype repository

Runtime prototype repository:

- ~/xenchanted-x1-lab/hello-x1

Prototype branch:

- stage-2-guardian-signature-verification

Relevant local runtime commits:

- d8bd927 Bind guardian message hash to mint context
- bec701b Update gateway test to sign bound message hash
- 7207e0f Fix gateway test bound hash arguments

## Program

X1 testnet program id:

- 9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984

ProgramData address:

- 32XqEK3cV1gySnS4gWAhEcTMfGtmNUcQrjdNkk4FVFWn

Upgrade authority:

- DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Last deployed slot after refined deploy:

- 164855038

Program data length:

- 238288 bytes
- 0x3a2d0

Deploy result:

- DEPLOY_EXIT=0

## Runtime flow tested

The live X1 testnet gateway test covered the refined Stage 2.4 flow:

- context-bound message_hash
- Ed25519 verification instructions
- scanning prior transaction instructions
- non-Ed25519 instruction interleaving
- ProcessedBurnEntry creation
- replay rejection
- missing guardian signature rejection
- unknown guardian rejection even with valid Ed25519 signatures

## Message hash binding

The runtime no longer treats message_hash as an opaque unbound value.

submit_mint_approval derives expected_message_hash from the submitted context:

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

The runtime requires:

    expected_message_hash == message_hash

Only after this check does guardian Ed25519 signature verification proceed.

## Ed25519 scanning

The runtime scans prior transaction instructions instead of relying on immediately preceding instructions.

The live test includes an interleaved non-Ed25519 instruction:

1. Ed25519 instruction for guardian A
2. SystemProgram transfer
3. Ed25519 instruction for guardian B
4. submit_mint_approval

This confirms the scanning path works when non-Ed25519 instructions are interleaved.

## Live test command result

Live X1 testnet test result:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, and rejects replay
      ✔ rejects missing guardian signature instruction
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    3 passing

## Known non-blocking warning

The Node test runner printed a MODULE_TYPELESS_PACKAGE_JSON warning.

This did not block execution and all live tests passed.

## What this proves

This proves on live X1 testnet that the refined Stage 2.4 runtime flow works:

- deploy succeeded
- context-bound message_hash is accepted when correct
- guardian Ed25519 signatures are verified through transaction instructions
- prior-instruction scanning works with non-Ed25519 interleaving
- ProcessedBurnEntry is created on valid approval
- replay is rejected
- missing signature path rejects
- unknown guardian path rejects

## What this does not prove

This does not yet prove:

- token mint CPI
- final XXXL mint integration
- production gateway readiness
- production guardian operations
- production watcher/relayer correctness

## Current conclusion

Stage 2.4 has live X1 testnet evidence for the refined guardian signature approval path with context-bound message_hash.

The next major implementation step can move toward token mint CPI planning, but production readiness still requires separate evidence for mint authority, token account wiring, failure cases, and operational guardian procedures.

# Phase 41K.6 B6.10 — State account and instruction design skeleton

## Purpose

This document records the Strategy 2 state account and instruction design skeleton after B6.9 runtime upgrade target design.

This is a local planning artifact.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not approve program upgrade.

This document does not approve account initialization.

This document does not approve SPL mint setup.

This document does not approve guardian package construction.

This document does not approve submit rehearsal.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

B6.9 runtime upgrade target design is merged on main:

1ca73a2 Merge phase 41K.6 B6.9 runtime upgrade target design

Current decision remains:

NO-GO.

## Design scope

B6.10 defines the minimum local design skeleton for:

- gateway_config account
- guardian_set account
- mint_state account
- processed_event account
- initialization instruction tags
- ConsumeGatewayMint execution instruction tag
- account order requirements
- idempotency behavior
- local test requirements

This document does not implement the runtime.

## State account skeleton

### gateway_config

Purpose:

Global gateway route configuration for the XXXL mint gateway.

Required fields:

- discriminator
- version
- bump
- route_id
- source_chain_id
- source_token
- mint_token
- gateway_mint_authority_pda
- guardian_set_id
- is_active
- created_at_slot
- reserved

PDA seed family:

- b"xxxl"
- b"gateway-config"
- b"v1"

Required checks:

- account owner must be program
- PDA must match program id and seeds
- initialized flag or discriminator must reject double init
- route_id must match canonical route
- source token must match configured route
- mint token must match configured route

### guardian_set

Purpose:

Authoritative guardian set for testnet gateway message verification.

Required fields:

- discriminator
- version
- bump
- guardian_set_id
- threshold
- guardian_count
- guardian_pubkeys
- status
- created_at_slot
- deprecated_at_slot
- reserved

PDA seed family:

- b"xxxl"
- b"guardian-set"
- guardian_set_id

Required checks:

- account owner must be program
- PDA must match program id and seeds
- threshold must satisfy 1 <= threshold <= guardian_count
- guardian_count must be nonzero
- guardian pubkeys must be unique
- guardian status must be active for ConsumeGatewayMint
- double init must return AlreadyInitialized

No private guardian keys may be stored.

### mint_state

Purpose:

Program-owned mint state metadata for the XXXL SPL mint.

Required fields:

- discriminator
- version
- bump
- mint
- mint_authority_pda
- mint_authority_bump
- token_program
- decimals
- total_minted
- is_active
- created_at_slot
- reserved

PDA seed family:

- b"xxxl"
- b"mint-state"
- mint

Required checks:

- account owner must be program
- PDA must match program id and seeds
- mint authority must be gateway mint authority PDA or approved program-controlled PDA
- token program must be SPL Token program
- decimals must match design spec
- double init must return AlreadyInitialized

### processed_event

Purpose:

Replay protection for a canonical Ethereum burn event consumed by the X1 gateway.

Required fields:

- discriminator
- version
- bump
- canonical_event_key
- message_hash
- source_burn_tx_hash
- source_burn_event_index
- recipient_hash
- minted_amount
- processed_at_slot
- reserved

PDA seed family:

- b"xxxl"
- b"processed-event"
- canonical_event_key

Required checks:

- account owner must be program
- PDA must match program id and seeds
- account must not already be initialized before mint
- processed mark must be atomic with mint
- duplicate submit must reject before mint

## Instruction skeleton

### InitializeGatewayConfig

Purpose:

Create or initialize gateway_config.

Required accounts:

1. payer
2. gateway_config
3. system_program
4. rent_sysvar

Required behavior:

- verify PDA
- verify account owner or create path
- write discriminator and version
- write route data
- write guardian_set_id
- set is_active
- reject double init

### InitializeGuardianSet

Purpose:

Create or initialize guardian_set.

Required accounts:

1. payer
2. guardian_set
3. system_program
4. rent_sysvar

Required behavior:

- verify PDA
- validate threshold
- validate guardian_count
- reject duplicate guardian pubkeys
- set status active
- reject double init

### InitializeMintState

Purpose:

Create or initialize mint_state.

Required accounts:

1. payer
2. mint_state
3. mint
4. gateway_mint_authority_pda
5. token_program
6. system_program
7. rent_sysvar

Required behavior:

- verify PDA
- verify mint authority PDA
- verify SPL Token program
- record mint and decimals
- set is_active
- reject double init

### ConsumeGatewayMint

Purpose:

Consume a verified gateway mint package and mint XXXL through SPL Token CPI.

Required accounts:

1. payer_or_relayer
2. gateway_config
3. guardian_set
4. mint_state
5. processed_event
6. mint
7. recipient_token_account
8. gateway_mint_authority_pda
9. token_program
10. system_program
11. rent_sysvar

Required behavior:

- decode message
- verify route
- verify source chain
- verify source token
- verify mint token
- verify x1 recipient hash
- verify guardian set active
- verify guardian quorum
- verify processed_event PDA
- reject replay before mint
- verify mint_state active
- verify mint authority PDA
- call SPL Token mint_to CPI
- mark processed_event atomically with mint
- return deterministic errors

## Instruction tag reservation

Proposed local instruction tags:

- 0: InitializeGatewayConfig
- 1: InitializeGuardianSet
- 2: InitializeMintState
- 3: ConsumeGatewayMint

Tags are local design skeleton values until implemented and tested.

## Error skeleton

Required deterministic errors:

- InvalidInstructionTag
- InvalidInstructionData
- InvalidAccountCount
- InvalidAccountOrder
- InvalidPda
- InvalidOwner
- AlreadyInitialized
- GatewayConfigInactive
- GuardianSetInactive
- InvalidGuardianThreshold
- DuplicateGuardian
- GuardianQuorumNotMet
- UnknownGuardian
- SignatureMismatch
- ReplayDetected
- InvalidMint
- InvalidMintAuthority
- InvalidTokenProgram
- MintStateInactive
- CpiMintFailed

## Local tests required

Before any artifact hash or upgrade GO, tests must prove:

- instruction tags decode correctly
- invalid instruction tag rejected
- account order enforced
- gateway_config PDA derivation stable
- guardian_set PDA derivation stable
- mint_state PDA derivation stable
- processed_event PDA derivation stable
- each init succeeds once
- each init rejects double initialization
- guardian threshold bounds enforced
- duplicate guardians rejected
- ConsumeGatewayMint handler path exists
- replay rejected before mint
- wrong SPL Token program rejected
- wrong mint authority PDA rejected
- B1C7 compile_error guard remains protected

## Blocker status impact

This document advances planning for:

- blocker C: B1C7 handler presence verification
- blocker D: state initialization instruction design
- blocker E: SPL mint authority architecture
- blocker F: guardian set testnet descriptor

But no blocker is closed by this document.

## Recommended next boundary

The next boundary should be:

B6.11 — Local Rust state and instruction skeleton.

B6.11 may add local Rust types, constants, PDA seed definitions, instruction tags, and tests.

B6.11 must remain local-only.

B6.11 must not sign, submit, spend SOL, deploy, upgrade, or initialize testnet accounts.

## Current decision

Current decision:

NO-GO.

This B6.10 state and instruction design skeleton does not authorize live action.

## B6.11 local Rust state and instruction skeleton

B6.11 local Rust state and instruction skeleton is implemented in:

programs/xxxl-svm/src/state_instruction_skeleton.rs

The module is local-only and explicitly marked:

LOCAL_ONLY_NOT_DEPLOYABLE

It does not approve deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, or submit rehearsal.

Current decision remains:

NO-GO.

## B6.12 local instruction codec skeleton

B6.12 local instruction codec skeleton is implemented in:

programs/xxxl-svm/src/instruction_codec_skeleton.rs

The module is local-only and explicitly marked:

LOCAL_ONLY_NOT_DEPLOYABLE

It defines reserved instruction tag decoding and encoding only.

It does not implement live runtime handlers.

It does not approve deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, or submit rehearsal.

Current decision remains:

NO-GO.

## B6.13 local instruction payload skeleton

B6.13 local instruction payload skeleton is implemented in:

programs/xxxl-svm/src/instruction_payload_skeleton.rs

The module is local-only and explicitly marked:

LOCAL_ONLY_NOT_DEPLOYABLE

It defines local payload boundaries for initialization and consume instruction planning.

It does not implement live runtime handlers.

It does not approve deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, or submit rehearsal.

Current decision remains:

NO-GO.

## B6.14 local typed instruction skeleton

B6.14 local typed instruction skeleton is implemented in:

programs/xxxl-svm/src/typed_instruction_skeleton.rs

The module is local-only and explicitly marked:

LOCAL_ONLY_NOT_DEPLOYABLE

It combines reserved instruction tags with local payload skeletons into typed instruction encode/decode planning.

It does not implement live runtime handlers.

It does not approve deploy, upgrade, signing, submit, SOL spend, account initialization, SPL mint setup, guardian package construction, or submit rehearsal.

Current decision remains:

NO-GO.

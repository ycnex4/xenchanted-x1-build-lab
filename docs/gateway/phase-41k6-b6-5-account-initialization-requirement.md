# Phase 41K.6 B6.5 — Account initialization requirement

## Purpose

This document records the implication of the B6.5 read-only program-owned account discovery.

The discovery observed zero program-owned accounts for the documented X1 testnet program.

Therefore, B6.5 cannot promote existing on-chain program-owned accounts into a submit rehearsal packet.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

Read-only program-owned account discovery is merged on main:

657f571 Merge phase 41K.6 B6.5 read-only program-owned account discovery

Current decision remains:

NO-GO.

## Discovery result

The read-only discovery recorded:

- total_program_owned_accounts_observed: 0
- mint_state candidate_count_by_length: 0
- gateway_config candidate_count_by_length: 0
- guardian_set candidate_count_by_length: 0
- processed_event candidate_count_by_length: 0
- recipient_balance candidate_count_by_length: 0

## Implication

The following required B6.5 values remain unavailable from existing program-owned on-chain state:

- actual mint_state account
- actual gateway_config account
- actual guardian_set account
- actual recipient_balance account
- actual processed_event account for a real event

The following values therefore also remain unavailable as verified runtime relationships:

- actual target_spl_mint from mint_state
- actual mint_authority_pda relationship from mint_state
- actual route_id from gateway_config
- actual source_chain_id from gateway_config
- actual source_chain_weight_bps from gateway_config
- actual guardian_set_id from gateway_config and guardian_set
- actual guardian threshold from guardian_set
- actual guardian public keys from guardian_set
- actual recipient owner from recipient_balance
- actual recipient mint from recipient_balance
- actual replay state from processed_event

## Requirement before any later submit rehearsal

Before any later B6.5 submit rehearsal can be considered, the missing state must be resolved by one of the following explicit paths:

1. Existing state account discovery from a different documented program id or deployment, if such a deployment is later identified.

2. A separate explicit testnet state-initialization boundary.

3. A separate explicit program upgrade plus state-initialization boundary, if the current program cannot initialize the required accounts.

4. A decision to abandon this deployed baseline for B6.5 and prepare a different testnet baseline.

No path above is approved by this document.

## Required future initialization boundary contents

Any future testnet state-initialization boundary must explicitly define:

- exact program id
- exact instruction or procedure used to initialize each account
- exact fee payer public address
- exact SOL spend approval
- exact signing approval
- exact submit approval
- exact accounts to be created
- exact rent requirements
- exact owner relationships
- exact PDA derivations
- exact abort conditions
- exact post-initialization read-only verification
- explicit statement that production activation remains excluded
- explicit statement that B1C7 gate handling remains separately controlled

## Values still unknown

The following B6.5 values remain unknown:

- actual target_spl_mint
- actual mint_decimals
- actual mint_supply_before
- actual route_id
- actual gateway_config account
- actual guardian_set_id
- actual guardian_set_account
- actual guardian_count
- actual guardian_threshold
- actual guardian_public_keys_source
- actual recipient_owner
- actual recipient_token_account
- actual processed_event_pda
- actual canonical_event_key
- actual source_token
- actual source_chain_weight_bps
- actual amount
- actual payload_v2_hash
- actual quorum_package_hash
- actual relayer_submission_package_hash
- actual fee_payer_public_address
- actual testnet_submit_rehearsal_approved_by

## Current decision

Current decision:

NO-GO.

This account initialization requirement does not authorize live action.

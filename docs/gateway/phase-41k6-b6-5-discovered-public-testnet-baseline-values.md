# Phase 41K.6 B6.5 — Discovered public testnet baseline values

## Purpose

This document records public testnet baseline values discovered in the repository after the B6.5 redacted testnet value packet.

This document does not open B6.5 submit rehearsal.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

This document performs no RPC.

This document performs no live verification.

This document records repository-discovered public values only.

## Current main checkpoint

B6.5 redacted testnet value packet is merged on main:

ca5070a Merge phase 41K.6 B6.5 redacted testnet value packet

Current decision remains:

NO-GO.

## Repository sources inspected

Public baseline values were found in existing repository documentation and source files:

- docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md
- docs/xxxl/xxxl-x1-testnet-runtime-upgrade-planning-inventory.md
- docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md
- programs/xxxl-svm/src/lib.rs
- programs/xxxl-svm/src/program_id_status.rs
- programs/xxxl-svm/src/deployment_status.rs
- programs/xxxl-svm/src/pda.rs
- tests/phase41k6_b6_no_send_dry_run_package_rehearsal.test.ts

## Discovered public X1 testnet baseline

- network_name: X1 testnet
- documented_testnet_status: X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED
- documented_testnet_program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- documented_program_data_address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- documented_gateway_mint_authority_pda: BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
- documented_gateway_mint_authority_pda_bump: 252
- documented_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

These values are public baseline values already present in repository documentation.

They are not secrets.

They do not authorize signing.

They do not authorize submission.

They do not authorize SOL spend.

They do not authorize deploy.

They do not authorize gate removal.

## Discovered public PDA derivation baseline

Gateway mint authority PDA seeds:

- seed_0: xxxl
- seed_1: gateway-mint-authority
- seed_2: v1

PDA derivation depends on the program id.

The documented PDA and bump are tied to the documented testnet program id.

Any future program id change requires regenerating PDA-dependent fixtures and rechecking the packet.

## Discovered token program baseline

- token_program_id: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

This is the standard SPL Token program id recorded in source as XXXL_TOKEN_PROGRAM_ID.

## Source-level runtime readiness status

Current source-level runtime status remains blocked:

- source_runtime_status: SCAFFOLD_ONLY_NOT_DEPLOYABLE
- source_deployable: false
- source_program_id_boundary: PLACEHOLDER_PROGRAM_ID_BOUNDARY
- configured_source_placeholder_program_id: XXXLProgram111111111111111111111111111111111

This means the repository contains a documented X1 testnet deployed baseline, while the current source-level runtime readiness model still records a scaffold-only, non-deployable boundary.

This distinction is intentional and must not be collapsed into a GO decision.

## Source-level active deployment blockers

Current source-level deployment blockers include:

- PLACEHOLDER_PROGRAM_ID
- LIVE_ROUTE_DISABLED
- SPL_CPI_EXECUTION_DISABLED
- PRODUCTION_GUARDIAN_SET_UNSET
- PRODUCTION_PROOF_LOG_UNSET
- EXTERNAL_REVIEW_INCOMPLETE

These blockers remain active.

This document does not remove, weaken, rename, or retire any blocker.

## Document-level active blockers

Existing X1 testnet planning docs record active blockers:

- PRODUCTION_PROGRAM_ID_UNSET
- LIVE_ROUTE_DISABLED
- SPL_CPI_EXECUTION_DISABLED
- PRODUCTION_GUARDIAN_SET_UNSET
- PRODUCTION_PROOF_LOG_UNSET
- EXTERNAL_REVIEW_INCOMPLETE

These blockers remain active.

## Dry-run fixture values found but not promoted to real testnet values

B6.3 dry-run package rehearsal contains fixture values:

- processed_event fixture: 0xb2 repeated
- route_id fixture: 0x41 repeated
- mint fixture: 0x51 repeated
- recipient_token_account fixture: 0x61 repeated
- guardian_set_id fixture: 0xc7 repeated
- amount fixture: 1234567890
- payload_v2_hash fixture: 0x56a318440e188d864052b8518f41deb7e4f998a975e3b6e19ca63815535ec77d
- guardian public key fixtures: 0xa1 repeated, 0xa2 repeated, 0xa3 repeated
- threshold fixture: 2

These are no-send dry-run fixtures.

They are not real testnet submit values.

They must not be copied into a GO packet as live values without a separate explicit decision and verification.

## Values still unknown for future GO

The following remain unknown:

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
- actual processed_event_pda for a real event
- actual canonical_event_key for a real event
- actual source_token
- actual source_chain_weight_bps for a real route
- actual amount for a real submit rehearsal
- actual payload_v2_hash for a GO package
- actual quorum_package_hash
- actual relayer_submission_package_hash
- actual fee_payer_public_address
- actual testnet_submit_rehearsal_approved_by

## Safe packet update rule

The redacted testnet value packet may record discovered public baseline values.

The redacted packet must keep all submit-dependent values unknown or null until they are chosen, verified, and approved through a later explicit written decision.

## Current decision

Current decision:

NO-GO.

This discovery does not authorize live action.


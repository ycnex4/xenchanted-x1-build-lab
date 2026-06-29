# XXXL X1 Testnet Program ID PDA Off-Chain Dry Run

Status: Completed
Branch: `stage-xxxl-x1-testnet-program-id-pda-offchain-dry-run`
Base: `50b4c29 Add XXXL program identity authority procedure`

## Purpose

This document records the off-chain dry run for deriving and verifying the gateway mint authority PDA from an X1 testnet Program ID candidate.

This dry run is off-chain only.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Public Inputs

Public Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

This is a public address only.

The local keypair file used to derive this public address is intentionally not committed.

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

The path is ignored through local git exclude:

- `.git/info/exclude`

The keypair contents must not be printed, copied, committed, pasted into chat, or included in documentation.

## Candidate Safety Checks

The Program ID candidate was checked against forbidden values.

It is not:

- `XXXLProgram111111111111111111111111111111111`
- `11111111111111111111111111111111`
- `BPFLoaderUpgradeab1e11111111111111111111111`
- SPL Token Program ID

This does not yet make the candidate a deployed Program ID.

This does not yet make the candidate production-ready.

This only proves that the candidate can be used for deterministic off-chain PDA derivation.

## PDA Derivation Result

PDA name:

- `gateway_mint_authority`

Seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Input Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Derived gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Derived bump:

- `252`

## Test Coverage Added

A new ignored off-chain dry-run test was added to:

- `programs/xxxl-svm/src/pda.rs`

Test name:

- `x1_testnet_program_id_candidate_pda_dry_run`

The test is ignored by default because it requires:

- `XXXL_TESTNET_PROGRAM_ID_CANDIDATE`

The test is explicitly off-chain only.

It does not use RPC.

It does not deploy.

It does not spend SOL.

## Default Test Result

Default library test suite result:

- `201 passed`
- `0 failed`
- `1 ignored`

The new candidate dry-run test is ignored in the default suite and does not block normal testing.

## Candidate Dry-Run Test Result

Candidate-specific ignored test result:

- `1 passed`
- `0 failed`

The test printed:

- `XXXL_TESTNET_PROGRAM_ID_CANDIDATE=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- `GATEWAY_MINT_AUTHORITY_PDA=BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- `GATEWAY_MINT_AUTHORITY_BUMP=252`
- `OFFCHAIN_ONLY=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `SOL_SPENT=false`

## Verification Coverage

The dry-run test verifies:

- candidate is present
- candidate is a valid SVM/Solana pubkey
- candidate is not the placeholder Program ID
- candidate is not System Program ID
- candidate is not BPF loader fixture value
- candidate is not SPL Token Program ID
- `gateway_mint_authority` PDA is derived from the candidate
- bump is derived from the candidate
- generated fixture verifies successfully
- wrong Program ID is rejected
- wrong PDA is rejected
- wrong bump is rejected

## Safety Boundaries Preserved

This stage does not enable:

- deployment
- runtime release
- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure

This stage does not remove:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Relationship To Future On-Chain Work

This dry run is not an on-chain deployment.

The Program ID candidate is not yet recorded as deployed.

The PDA fixture is not yet a production fixture.

Before any on-chain deployment or blocker transition, future stages must still record:

1. X1 testnet Program ID selection record
2. X1 testnet deployment or deployment-readiness evidence
3. production/testnet PDA fixture regeneration record
4. production/testnet PDA fixture verification record
5. Program ID readiness model update
6. placeholder Program ID blocker transition

## Result

The X1 testnet Program ID candidate PDA path was tested off-chain.

The gateway mint authority PDA was derived.

The bump was derived.

The candidate-specific PDA fixture was verified.

Wrong Program ID, wrong PDA, and wrong bump were rejected.

No RPC was used.

No program was deployed.

No SOL was spent.

No secret material was recorded.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

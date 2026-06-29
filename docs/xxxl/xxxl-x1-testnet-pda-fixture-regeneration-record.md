# XXXL X1 Testnet PDA Fixture Regeneration Record

Status: Completed
Branch: `stage-xxxl-x1-testnet-pda-fixture-regeneration-record`
Base: `659d37c Add X1 testnet Program ID selection record`

## Purpose

This document records the regenerated PDA fixture for the selected X1 testnet Program ID candidate.

This is a testnet PDA fixture regeneration record only.

This is not a mainnet fixture record.

This is not a deployment record.

This is not a production readiness record.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Selected X1 Testnet Program ID

Selected public X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Selection record:

- `docs/xxxl/xxxl-x1-testnet-program-id-selection-record.md`

The Program ID is public.

The local keypair contents are not recorded.

The local keypair is not committed.

## PDA Fixture Regeneration Inputs

PDA name:

- `gateway_mint_authority`

PDA derivation kind:

- `GatewayMintAuthority`

Seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Program ID input:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

## Regenerated PDA Fixture

Regenerated gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Regenerated bump:

- `252`

Fixture report fields:

- `kind = GatewayMintAuthority`
- `name = gateway_mint_authority`
- `program_id = D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- `pda = BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- `bump = 252`

## Regeneration Method

The fixture was regenerated through the ignored off-chain dry-run test:

- `x1_testnet_program_id_candidate_pda_dry_run`

Environment variable used:

- `XXXL_TESTNET_PROGRAM_ID_CANDIDATE=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

The test uses deterministic PDA derivation.

The test does not use RPC.

The test does not deploy.

The test does not spend SOL.

## Regeneration Output

The regeneration output confirmed:

- `XXXL_TESTNET_PROGRAM_ID_CANDIDATE=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- `GATEWAY_MINT_AUTHORITY_PDA=BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- `GATEWAY_MINT_AUTHORITY_BUMP=252`
- `OFFCHAIN_ONLY=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `SOL_SPENT=false`

Test result:

- `1 passed`
- `0 failed`

## Verification Boundaries

This regeneration record confirms that the PDA fixture was derived from the selected X1 testnet Program ID.

It does not yet prove on-chain deployment.

It does not yet prove that the program account exists on X1 testnet.

It does not yet prove that the deployed program address equals the selected Program ID.

It does not yet prove that the SPL Token mint authority has been transferred to the PDA.

It does not yet remove the placeholder Program ID blocker.

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

## Required Future Work

Future stages must still record:

1. X1 testnet PDA fixture verification record
2. X1 testnet deployment or deployment-readiness evidence
3. Program ID readiness model update
4. placeholder Program ID blocker transition

The blocker transition must happen only after the required evidence exists.

## Result

The X1 testnet `gateway_mint_authority` PDA fixture was regenerated from the selected Program ID.

The regenerated PDA is recorded.

The regenerated bump is recorded.

No RPC was used.

No program was deployed.

No SOL was spent.

No secret material was recorded.

No blocker is removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

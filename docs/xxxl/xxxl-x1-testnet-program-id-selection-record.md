# XXXL X1 Testnet Program ID Selection Record

Status: Completed
Branch: `stage-xxxl-x1-testnet-program-id-selection-record`
Base: `14e7039 Add X1 testnet PDA off-chain dry run`

## Purpose

This document records the selected public X1 testnet Program ID candidate for the XXXL SVM runtime.

This is a testnet selection record only.

This is not a mainnet Program ID record.

This is not a deployment record.

This is not a production readiness record.

No RPC was used in this stage.

No program was deployed in this stage.

No transaction was submitted in this stage.

No SOL was spent in this stage.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Selected Public X1 Testnet Program ID

Selected X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

This value is a public address only.

The local keypair that corresponds to this public address is not committed.

The local keypair contents are not recorded.

The local keypair must not be printed, copied, pasted into chat, committed, or included in documentation.

## Local Keypair Handling

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

Ignored through:

- `.git/info/exclude`

This local path is used only to preserve the testnet Program ID candidate locally.

The repository records only the public Program ID string.

## Selection Scope

This selection applies to:

- X1 testnet only

This selection does not apply to:

- mainnet
- production release
- other networks
- immutable release
- external review closure

A future mainnet Program ID requires a separate selection record.

## Forbidden Value Checks

The selected public Program ID candidate is not:

- `XXXLProgram111111111111111111111111111111111`
- `11111111111111111111111111111111`
- `BPFLoaderUpgradeab1e11111111111111111111111`
- SPL Token Program ID
- System Program ID

This confirms that the selected X1 testnet Program ID candidate is not the known placeholder or local fixture value.

## PDA Dry-Run Evidence

The selected Program ID candidate was used in the off-chain PDA dry-run.

Input Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Derived gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Derived bump:

- `252`

Dry-run flags:

- `OFFCHAIN_ONLY=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `SOL_SPENT=false`

## PDA Meaning

The derived PDA is:

- `gateway_mint_authority`

Its intended role is to be the SPL Token mint authority for gateway-backed XXXL minting.

This does not enable minting yet.

This does not enable live route execution.

This does not enable SPL CPI execution.

This only records the deterministic PDA result for the selected X1 testnet Program ID candidate.

## Test Evidence

The previous off-chain dry-run added an ignored candidate-specific test:

- `x1_testnet_program_id_candidate_pda_dry_run`

The dry-run verified:

- candidate is a valid pubkey
- candidate is not placeholder
- candidate is not local fixture
- candidate is not SPL Token Program ID
- PDA is derived from candidate
- bump is derived from candidate
- generated fixture verifies
- wrong Program ID is rejected
- wrong PDA is rejected
- wrong bump is rejected

Default library test result from dry-run stage:

- `201 passed`
- `0 failed`
- `1 ignored`

Candidate-specific dry-run result:

- `1 passed`
- `0 failed`

## Deployment Status

This selection record does not mean the program is deployed.

Current deployment status remains:

- scaffold-only
- locked
- unreleasable
- not deployable

The selected public Program ID candidate must still go through future deployment or deployment-readiness evidence before it can be treated as an on-chain deployed program identity.

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`PLACEHOLDER_PROGRAM_ID` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Required Future Work

Future stages must still record:

1. X1 testnet deployment or deployment-readiness evidence
2. X1 testnet PDA fixture regeneration record
3. X1 testnet PDA fixture verification record
4. Program ID readiness model update
5. placeholder Program ID blocker transition

The blocker transition must happen only after the required evidence exists.

## Non-Goals

This record does not enable:

- deployment
- runtime release
- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure
- mainnet selection
- mainnet release

## Result

The X1 testnet public Program ID candidate is selected and recorded.

The matching gateway mint authority PDA dry-run evidence is recorded.

No private key material is recorded.

No keypair contents are recorded.

No RPC was used.

No program was deployed.

No SOL was spent.

No blocker is removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

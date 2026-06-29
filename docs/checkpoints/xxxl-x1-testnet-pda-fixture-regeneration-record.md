# XXXL X1 Testnet PDA Fixture Regeneration Record Checkpoint

Status: Completed
Branch: `stage-xxxl-x1-testnet-pda-fixture-regeneration-record`
Base: `659d37c Add X1 testnet Program ID selection record`

## Summary

This checkpoint records the regenerated X1 testnet PDA fixture for the selected public Program ID candidate.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material is recorded.

## Files Changed

Expected changed files:

- `docs/xxxl/xxxl-x1-testnet-pda-fixture-regeneration-record.md`
- `docs/checkpoints/xxxl-x1-testnet-pda-fixture-regeneration-record.md`
- `docs/checkpoints/current-design-checkpoint.md`

No Rust source changes are expected.

No Cargo changes are expected.

No keypair files are expected.

## Regenerated Fixture

Selected X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

PDA name:

- `gateway_mint_authority`

Regenerated PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Regenerated bump:

- `252`

## Regeneration Evidence

The fixture was regenerated through the ignored off-chain dry-run test:

- `x1_testnet_program_id_candidate_pda_dry_run`

Dry-run flags:

- `OFFCHAIN_ONLY=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `SOL_SPENT=false`

Test result:

- `1 passed`
- `0 failed`

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

## Result

The X1 testnet PDA fixture was regenerated and recorded.

No RPC was used.

No program was deployed.

No SOL was spent.

No secret material was recorded.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

# XXXL X1 Testnet Program ID Selection Record Checkpoint

Status: Completed
Branch: `stage-xxxl-x1-testnet-program-id-selection-record`
Base: `14e7039 Add X1 testnet PDA off-chain dry run`

## Summary

This checkpoint records the selected public X1 testnet Program ID candidate for the XXXL SVM runtime.

This is a testnet selection checkpoint only.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material is recorded.

## Files Changed

Expected changed files:

- `docs/xxxl/xxxl-x1-testnet-program-id-selection-record.md`
- `docs/checkpoints/xxxl-x1-testnet-program-id-selection-record.md`
- `docs/checkpoints/current-design-checkpoint.md`

No Rust source changes are expected.

No Cargo changes are expected.

No keypair files are expected.

## Selected Public Program ID

X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

The local keypair is ignored through `.git/info/exclude`.

The keypair contents are not recorded.

## PDA Dry-Run Evidence

Derived gateway mint authority PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Derived bump:

- `252`

Dry-run flags:

- `OFFCHAIN_ONLY=true`
- `RPC_USED=false`
- `DEPLOYED=false`
- `SOL_SPENT=false`

## Scope

This selection applies to:

- X1 testnet only

This selection does not apply to:

- mainnet
- production release
- immutable release
- external review closure

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

The public X1 testnet Program ID candidate is selected and recorded.

No private key material is recorded.

No keypair contents are recorded.

No RPC was used.

No program was deployed.

No SOL was spent.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

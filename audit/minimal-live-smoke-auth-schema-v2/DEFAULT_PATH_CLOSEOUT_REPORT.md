# Default Path Closeout Report

Generated UTC: 2026-08-16T15:41:54Z

## Scope

This report closes the canonical_asset_id refactor default-path verification.

No D2/B1C7 dangerous-gated mint execution is included in this closeout.

## Current repository state

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head: `6da71d76fde8f0da7ead25781b4c3c125fa2d77d`
- Audited source checkpoint: `240e3e89100893939339ee5cc1476298e1ea4571`

## Review status

- Source semantic review: PASS
- Daemon follow-up review: PASS
- Constructed layout sanity-check: PASS
- Theo final default-path status: PASS

## Testnet default artifact upgrade

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- Upgrade signature: `417K1VNG2DKLkj5UFeNWPdet25KC3DHW6DKjNXV4wZm46fxrnnqzHooL1Gbn3WsCZwnaatCETFuBGRBxN2dvgXrG`
- Last deployed slot after upgrade: `181033350`
- Candidate artifact sha256: `035e9d4ba25b455e80ff8b791d69c6aff2a91c3ca22bc6955d9b6b080605a1e9`
- Candidate artifact size: `82056`
- Deployed dump sha256 after upgrade: `abcb64fd18c57c6204ce8cd577176d2210c14e411047c35e4b08b625bc041185`
- Deployed dump size after upgrade: `186376`
- Artifact integrity: candidate matches deployed dump prefix; deployed tail is zero padding only.
- Rollback artifact sha256: `8e8410070d31f0a50cdb851d40896dd8e6656e28e840ad95fa4fda0b79eb7e97`
- Rollback artifact size: `186376`

## Pre-upgrade smoke attempt

- Tx signature: `4yMBMVpVgXvoGvaoJT11wtV7AKwp4VmAuTouoNnQLg5ox74vUMiiVEsPvnPLQoX2dxAq7zuvEtb9kmWSfbTSqWmt`
- Result: landed safely, but failed with `Custom 1 / InvalidInstruction`
- Interpretation: deployed artifact was older/pre-refactor.
- State mutation: none.

## Post-upgrade default smoke

- Tx signature: `3Uoo62L9r8H1Y1GiZr9ktP4p2oJ3bb18v6mMCURYPV6oe7YQAGReaQb1nyyfGvesBZ6iYAr2z19YivVZzSTTWhFF`
- Slot: `181033864`
- Result: `InstructionError [3, Custom 8]`
- Expected failure: `CpiBoundaryNotReady`
- Failure code matches expected: true
- Option1 success: true

## Mutation check

Post-upgrade default smoke confirmed:

- `changed_accounts=[]`
- `processed_event_exists: false -> false`
- `spl_mint_supply: 0 -> 0`
- `recipient_ata_amount: 0 -> 0`
- `mint_state_total_supply: 0 -> 0`
- `recipient_balance_amount: 0 -> 0`

## Final default-path status

canonical_asset_id refactor is smoke-ready for the default path.

The default build now reaches the intended safety gate:

`CpiBoundaryNotReady before mutation`

## Next gate

D2/B1C7 dangerous-gated mint execution is not started.

It requires separate explicit approval before any build, deploy, upgrade, or live mint execution.

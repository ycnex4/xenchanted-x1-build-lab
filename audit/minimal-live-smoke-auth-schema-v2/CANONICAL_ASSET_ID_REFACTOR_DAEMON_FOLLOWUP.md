# Canonical Asset ID Refactor — Daemon Follow-up

This follow-up addresses the daemon NEEDS_CHANGES result.

## Changes in this follow-up

1. The legacy/test-only apply_atomic_state_mutations_fixture now accepts target_mint_pubkey explicitly.
2. The fixture now checks RecipientBalance.mint against target_mint_pubkey, not canonical_asset_id.
3. The fixture now credits recipient balance using target_mint_pubkey, not canonical_asset_id.
4. A test was added proving canonical_asset_id can be distinct from target_mint_pubkey in this fixture path.
5. A raw diff bundle for the seven named test files from primary commit 705d957 was added for daemon review.

## Files added for daemon

audit/minimal-live-smoke-auth-schema-v2/CANONICAL_ASSET_ID_REFACTOR_705D957_TEST_DIFFS_FOR_DAEMON.patch

## Review request

Please re-review:

- the follow-up commit that contains this file,
- programs/xxxl-svm/src/execution_plan.rs,
- audit/minimal-live-smoke-auth-schema-v2/CANONICAL_ASSET_ID_REFACTOR_705D957_TEST_DIFFS_FOR_DAEMON.patch.

Requested verdict:

PASS / FAIL / NEEDS_CHANGES

Please specifically confirm:

1. The legacy fixture divergence is resolved.
2. The added distinct canonical_asset_id vs target_mint_pubkey fixture test is valid.
3. The seven test-file diffs from 705d957 do not weaken negative/hostile coverage.
4. It is now safe to share the refactor with Theo.
5. Whether a new live smoke can be attempted, or whether another source-only review step is still required.

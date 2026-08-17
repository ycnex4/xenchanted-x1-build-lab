# Gate U3 Testnet Upgrade Only

Date UTC: 2026-08-17T11:04:05Z

## Scope

Explicitly approved testnet program upgrade only.

No provisioning, guardian_set initialization, gateway_config initialization, guarded mint, replay, rollback, second mint, production deployment, or other transaction was authorized.

## Safety Flags

- provisioning_executed: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- production_deployment_executed: `false`
- raw_signatures_committed: `false`
- signed_transaction_committed: `false`

## Repository

- branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- head_sha: `30b0154b38f0acfeef826e275721e921a3e406f2`
- status_before_count: `0`

## Program

- program_id: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- programdata: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- upgrade_authority_pubkey: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

## Candidate Artifact

- candidate_path: `/mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm/target/sbpf-solana-solana/release/deps/xxxl_svm.so`
- candidate_sha256: `ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`
- candidate_size: `201160`
- expected_sha256: `ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`
- expected_size: `201160`
- wrong_target_deploy_artifact_sha256: `402a94a23feff7b87af25b9e83137b33bc1359bd7c495618e9c11f1222686bd4`
- wrong_target_deploy_artifact_size: `161672`

## Rollback Baseline

- rollback_baseline_path: `/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate-u3-testnet-upgrade-only-20260817T110354Z/results/deployed-before-u3-upgrade.so`
- rollback_baseline_sha256: `6e7e1c7b82cf9394129a20f3fee81d653cf501a2b50dcd5ba0dd7dff4fd6d509`
- rollback_baseline_size: `186376`

## Execution

- precheck_result: `PASS`
- upgrade_code: `0`
- upgrade_signature: `2WnXWuRNAxCtZTbw9LLPEfFawqi9DJyC5tqUttT6DtghKngcCKDvSTbNbkR6LLdEyRYbHWr36CgZydYXesNy7Gni`
- confirm_code: `0`
- dump_after_code: `0`
- deployed_after_sha256: `ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`
- deployed_after_size: `201160`
- u3_result: `PASS_U3_TESTNET_UPGRADE_ONLY_EXECUTED`

## Decision Boundary

U3 does not approve provisioning or mint validation.

Next allowed step after U3 PASS is U5 read-only verification.

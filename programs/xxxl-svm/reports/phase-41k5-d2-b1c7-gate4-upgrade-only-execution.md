# Phase 41K.5 D2 / B1C7 Gate 4 Upgrade-Only Execution

Date UTC: 2026-08-16T18:36:01Z

## Scope

This report records Gate 4 upgrade-only execution for the D2/B1C7 testnet artifact.

This report does not authorize or execute live guarded mint, replay, rollback, or production deployment.

## Explicit Gate 3 Approval

Approval phrase recorded:

```text
I approve the D2/B1C7 testnet upgrade transaction using artifact sha256 e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766.
```

## Safety Flags

- transactions_executed: `true`
- deploy_executed: `false`
- upgrade_executed: `true`
- live_broadcast: `true`
- guarded_mint_executed: `false`
- rollback_executed: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before Gate 4: `b175078a8fc1a4ef0bcc05733d4c690c1da64969`
- Git status before Gate 4: `0` changed files

## Upgrade Target

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- ProgramData: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- Expected upgrade authority: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- Observed upgrade authority signer: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- authority_signer_match_code: `0`

## Candidate Artifact

- expected candidate sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- observed candidate sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- expected candidate size: `161664`
- observed candidate size: `161664`
- candidate_compare_code: `0`

## Pre-Upgrade Baseline

- expected pre-upgrade deployed sha256: `abcb64fd18c57c6204ce8cd577176d2210c14e411047c35e4b08b625bc041185`
- observed pre-upgrade deployed sha256: `abcb64fd18c57c6204ce8cd577176d2210c14e411047c35e4b08b625bc041185`
- expected pre-upgrade deployed size: `186376`
- observed pre-upgrade deployed size: `186376`
- pre_deployed_baseline_match_code: `0`
- pre_already_candidate_code: `1`
- pre_programdata_match_code: `0`
- pre_authority_match_code: `0`
- pre_rpc_readonly_code: `0`

Rollback artifact candidate:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate4-upgrade-only-execution-20260816T183547Z/results/deployed-current-before-upgrade.so
```

## Upgrade Transaction

- precheck_pass_code: `0`
- deploy_code: `0`
- transaction_signature: `2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY`

Key upgrade log lines:

```text
Program Id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
Signature: 2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY
```

## Post-Upgrade Verification

- post_program_show_code: `0`
- post_program_account_code: `0`
- post_programdata_account_code: `0`
- post_program_dump_code: `0`
- post_programdata_match_code: `0`
- post_authority_match_code: `0`
- post_deployed_sha256: `6e7e1c7b82cf9394129a20f3fee81d653cf501a2b50dcd5ba0dd7dff4fd6d509`
- post_deployed_size: `186376`
- post_candidate_prefix_match: `true`
- post_candidate_tail_zero: `true`
- post_candidate_tail_size: `24712`
- post_compare_code: `0`

## Gate 4 Decision

Gate 4 upgrade-only execution result: `PASS`

## Non-Claims

This report does not claim:

- live guarded mint approval;
- live guarded mint execution;
- replay tested against deployed program;
- production readiness;
- rollback execution.

## Next Gate

If Gate 4 is PASS, the next allowed step is Gate 5 post-upgrade read-only verification.

Live guarded mint remains blocked until a separate Gate 6 package and explicit approval.

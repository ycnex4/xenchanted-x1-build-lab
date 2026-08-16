# Phase 41K.5 D2 / B1C7 Gate 5 Post-Upgrade Read-Only Verification

Date UTC: 2026-08-16T18:38:39Z

## Scope

This is Gate 5 post-upgrade read-only verification after the D2/B1C7 Gate 4 upgrade-only transaction.

No live guarded mint, replay, rollback, activation, or production deployment is authorized or executed by this verification.

## Safety Flags

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- rollback_executed: `false`
- read_only_rpc_checks_executed: `true`

## Gate 4 Reference

- Gate 4 commit: `38febb2dc7965094e4c01e5453deed53df48f2cf`
- Gate 4 upgrade transaction: `2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before Gate 5: `38febb2dc7965094e4c01e5453deed53df48f2cf`
- Git status before Gate 5: `0` changed files

## Candidate Artifact Rebuild

- build_sbf_code: `0`
- expected candidate sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- observed candidate sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- expected candidate size: `161664`
- observed candidate size: `161664`
- candidate_compare_code: `0`

## Program Metadata

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- ProgramData: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- Expected upgrade authority: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- program_show_code: `0`
- program_account_code: `0`
- programdata_account_code: `0`
- program_dump_code: `0`
- programdata_match_code: `0`
- authority_match_code: `0`

## Deployed Program Byte Verification

- deployed sha256 after Gate 5 read: `6e7e1c7b82cf9394129a20f3fee81d653cf501a2b50dcd5ba0dd7dff4fd6d509`
- deployed size after Gate 5 read: `186376`
- candidate prefix match: `true`
- zero tail: `true`
- tail size: `24712`
- post_compare_code: `0`

## Target SPL Mint Verification

- target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- expected token program owner: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- observed token program owner: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- observed mint data length: `82`
- expected mint authority: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- observed mint authority option: `1`
- observed mint authority: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- expected supply: `0`
- observed supply: `0`
- expected decimals: `9`
- observed decimals: `9`
- observed initialized flag: `1`
- observed freeze authority option: `0`
- observed freeze authority: `None`
- mint_account_code: `0`
- mint_parse_code: `0`
- mint_check_code: `0`

## Hygiene Checks

- git_diff_check_code: `0`
- forbidden_worktree_count: `0`

## Gate 5 Decision

Gate 5 post-upgrade read-only verification result: `PASS`

## Non-Claims

This report does not claim:

- live guarded mint approval;
- live guarded mint execution;
- replay tested against deployed program;
- production readiness;
- rollback execution.

## Next Gate

If Gate 5 is PASS, the next possible step is a separate Gate 6 live guarded mint planning package.

Gate 6 still requires separate planning, separate review, and a separate explicit approval phrase before any live guarded mint transaction.

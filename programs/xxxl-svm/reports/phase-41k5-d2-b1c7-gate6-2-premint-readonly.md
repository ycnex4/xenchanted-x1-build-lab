# Phase 41K.5 D2 / B1C7 Gate 6.2 Pre-Mint Read-Only Verification

Date UTC: 2026-08-16T21:02:24Z

## Scope

This report records Gate 6.2 pre-mint read-only verification for the finalized D2/B1C7 live guarded mint payload.

This report does **not** authorize or execute live guarded mint, replay, rollback, deploy, upgrade, activation, or production deployment.

## Safety Flags

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- read_only_rpc_checks_executed: `true`
- raw_signatures_committed: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before Gate 6.2: `7c4ee53d8c7fd4ceacd3349ebc56fb64b6684684`
- Git status before Gate 6.2: `0` changed files

## Gate References

- Gate 6.1 payload commit: `e90c9679554cfe9e0d4d11615ce3cc8863f8af10`
- Gate 6.1 signature collection commit: `7c4ee53d8c7fd4ceacd3349ebc56fb64b6684684`

## Payload Reference

- canonical_event_key: `eb1988a359d5ef099d1f478f352d6c7c25647d9414eb8644fecd3436bc54cdad`
- authorization_payload_hash_signed: `419adcde33de8ec1e51f0d9668f5a2a2f1945ba7739198bc5e572c4bebb73f2e`
- recipient wallet: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- recipient ATA: `9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k`
- amount base units: `1`
- amount human: `0.000000001`
- target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- processed_event PDA: `FM92jtmo2YAvnVsEQtHL8aKMYxwFJTPaaSsZYg1rwspQ`
- recipient_balance PDA: `5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB`

## Program Byte / Metadata Checks

- build_sbf_code: `0`
- candidate sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- candidate size: `161664`
- candidate_compare_code: `0`
- program_show_code: `0`
- program_account_code: `0`
- programdata_account_code: `0`
- program_dump_code: `0`
- programdata_match_code: `0`
- authority_match_code: `0`
- deployed sha256: `6e7e1c7b82cf9394129a20f3fee81d653cf501a2b50dcd5ba0dd7dff4fd6d509`
- deployed size: `186376`
- candidate prefix match: `true`
- zero tail: `true`
- tail size: `24712`
- post_compare_code: `0`

## Pre-Mint State Checks

Target SPL mint:

- mint_account_code: `0`
- mint_parse_code: `0`
- mint_check_code: `0`
- mint_supply: `0`
- mint_authority: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Recipient ATA:

- ata_account_code: `0`
- ata_parse_code: `0`
- ata_check_code: `0`
- ata_balance: `0`

Recipient balance PDA:

- recipient_balance_account_code: `0`
- recipient_balance_parse_code: `0`
- recipient_balance_check_code: `0`
- recipient_balance_balance: `0`

Processed event PDA:

- processed_event_account_code: `1`
- processed_event_parse_code: `0`
- processed_event_check_code: `0`
- processed_event_status: `UNPROCESSED_ACCOUNT_MISSING`

## Signature Bundle Checks

- signature_bundle_exists: `true`
- signature_bundle_path:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-1-signature-collection-pure-python-20260816T205254Z/results/gate6_1_guardian_signature_bundle.execution-material.local.json
```

- expected signature_bundle_sha256: `bb6849577ed880552553881a73883d48679b698f7d5a4bfaa49b46f2a7de3962`
- observed signature_bundle_sha256: `bb6849577ed880552553881a73883d48679b698f7d5a4bfaa49b46f2a7de3962`
- signature_bundle_check_code: `0`
- signature_count: `3`
- unique_signer_count: `3`
- quorum_met: `true`
- signer_pubkeys: `7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf,GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp,6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih`

## Hygiene Checks

- git_diff_check_code: `0`
- forbidden_worktree_count: `0`

## Gate 6.2 Decision

Gate 6.2 pre-mint read-only verification result: `PASS_PREMINT_READONLY_READY_FOR_EXPLICIT_GO`

## Non-Claims

This report does not claim:

- live guarded mint approval;
- live guarded mint execution;
- replay approval;
- replay execution;
- production source event proof;
- production readiness;
- rollback execution.

## Next Gate

If Gate 6.2 is PASS, the next possible step is Gate 6.3 explicit live mint GO.

Gate 6.3 requires a separate human approval phrase before any live guarded mint transaction may be broadcast.

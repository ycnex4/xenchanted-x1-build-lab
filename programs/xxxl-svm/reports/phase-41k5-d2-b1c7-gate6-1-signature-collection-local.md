# Phase 41K.5 D2 / B1C7 Gate 6.1 Local Signature Collection

Date UTC: 2026-08-16T20:52:58Z

## Scope

This report records local guardian signature collection for the finalized Gate 6.1 payload.

This report does **not** authorize or execute live guarded mint, replay, rollback, deploy, upgrade, activation, or production deployment.

Raw guardian signatures are intentionally not committed to git. They remain local execution material.

## Safety Flags

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- guardian_signatures_generated: `true`
- raw_signatures_committed: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before signature collection: `e90c9679554cfe9e0d4d11615ce3cc8863f8af10`
- Git status before signature collection: `0` changed files

## Payload Reference

- Gate 6.1 payload commit: `e90c9679554cfe9e0d4d11615ce3cc8863f8af10`
- canonical_event_key: `eb1988a359d5ef099d1f478f352d6c7c25647d9414eb8644fecd3436bc54cdad`
- authorization_payload_hash_signed: `419adcde33de8ec1e51f0d9668f5a2a2f1945ba7739198bc5e572c4bebb73f2e`
- recipient wallet: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- recipient ATA: `9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k`
- amount base units: `1`
- target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- processed_event PDA: `FM92jtmo2YAvnVsEQtHL8aKMYxwFJTPaaSsZYg1rwspQ`
- recipient_balance PDA: `5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB`

## Local Signature Bundle

- local signature bundle path:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-1-signature-collection-pure-python-20260816T205254Z/results/gate6_1_guardian_signature_bundle.execution-material.local.json
```

- signature_bundle_sha256: `bb6849577ed880552553881a73883d48679b698f7d5a4bfaa49b46f2a7de3962`
- signature_count: `3`
- unique_signer_count: `3`
- quorum_required: `3`
- quorum_met: `true`
- signer_pubkeys: `7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf,GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp,6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih`

## Hygiene Checks

- sign_code: `0`
- signer_implementation: `python_stdlib_ed25519_rfc8032`
- git_diff_check_code: `0`
- forbidden_worktree_count: `0`

## Gate 6.1 Signature Collection Decision

Gate 6.1 local signature collection result: `PASS_SIGNATURES_COLLECTED_LOCAL_ONLY`

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

If signature collection is PASS, the next allowed step is Gate 6.2 pre-mint read-only verification.

Gate 6.3 explicit live mint GO remains separate and is not implied by this report.

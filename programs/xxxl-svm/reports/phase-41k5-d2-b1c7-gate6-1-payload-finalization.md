# Phase 41K.5 D2 / B1C7 Gate 6.1 Payload Finalization / Signing Package

Date UTC: 2026-08-16T20:35:50Z

## Scope

This report finalizes the event payload and signing package for the first D2/B1C7 live guarded mint validation on X1 testnet.

This report does **not** authorize or execute live guarded mint, replay, rollback, deploy, upgrade, activation, or production deployment.

Guardian signatures are not generated in this report. This report produces the deterministic authorization payload hash that guardians must sign in a later step.

## Safety Flags

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- read_only_rpc_checks_executed: `true`
- guardian_signatures_generated: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before Gate 6.1: `e35df1166cb8d50f2dc533056c16f44d5d63d917`
- Git status before Gate 6.1: `0` changed files

## Prior Gate References

- Gate 4 commit: `38febb2dc7965094e4c01e5453deed53df48f2cf`
- Gate 4 upgrade transaction: `2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY`
- Gate 5 commit: `3c95fd24e91bb91afb7927b297a8958b2a5be08a`
- Gate 6 planning commit: `e35df1166cb8d50f2dc533056c16f44d5d63d917`

## Finalized Mint Intent

- source event kind: `synthetic_testnet_validation_event`
- synthetic source event id:

```text
synthetic-x1-testnet-d2-b1c7-first-live-mint-v1|nonce=20260816T203532Z|gate4_tx=2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY|recipient=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc|amount_base_units=1|target_mint=g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM
```

- recipient wallet: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- recipient ATA: `9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k`
- amount base units: `1`
- amount human: `0.000000001`
- target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- canonical_event_key: `eb1988a359d5ef099d1f478f352d6c7c25647d9414eb8644fecd3436bc54cdad`

## Route / Asset / Guardian Parameters

- route_id: `d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c`
- canonical_asset_id: `479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458`
- source_chain_id: `1`
- source_chain_weight_bps: `10000`
- guardian_set_id: `4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83`
- guardian quorum: `3`

## Derived PDA / Account Package

Derived payload JSON:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-1-payload-finalization-rust-retry-20260816T203532Z/results/gate6_1_derived_payload.json
```

- processed_event PDA: `FM92jtmo2YAvnVsEQtHL8aKMYxwFJTPaaSsZYg1rwspQ`
- recipient_balance PDA: `5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB`
- gateway mint authority: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

## Instruction Data

- instruction data length: `208`
- instruction data hex:

```text
f2f4a868bb89fe5202000b0102000304d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458eb1988a359d5ef099d1f478f352d6c7c25647d9414eb8644fecd3436bc54cdadb9215bc8d66cdfbbdda0af91cf8f711ff6beae17334bfa552834bde19cf06ce90100000000000000000000000000000010270100000000000000000000000000
```

## B1C Authorization Signing Package

Guardians must sign exactly this 32-byte message:

- authorization payload domain: `consume_gateway_mint_authorization_v2`
- authorization payload hash / signed message:

```text
419adcde33de8ec1e51f0d9668f5a2a2f1945ba7739198bc5e572c4bebb73f2e
```

Signature status:

- guardian_signatures_required: `3`
- guardian_signatures_provided: `0`
- guardian_signature_status: `PENDING_NOT_GENERATED_IN_GATE_6_1`

## Pre-State Read-Only Checks

- RPC URL: `https://rpc.testnet.x1.xyz`
- recipient_ata_account_code: `0`
- recipient_ata_parse_code: `0`
- recipient_ata_check_code: `0`
- recipient_ata_pre_balance: `0`
- recipient_balance_account_code: `0`
- recipient_balance_parse_code: `0`
- recipient_balance_check_code: `0`
- recipient_balance_pre_balance: `0`
- processed_event_account_code: `1`
- processed_event_parse_code: `0`
- processed_event_pre_status: `UNPROCESSED_ACCOUNT_MISSING`
- processed_event_check_code: `0`
- readonly_rpc_code: `0`

Expected post-state after a later approved live mint:

- recipient ATA balance increases by exactly `1`
- recipient_balance increases by exactly `1`
- target SPL mint supply increases by exactly `1`
- processed_event becomes marked/consumed for canonical_event_key `eb1988a359d5ef099d1f478f352d6c7c25647d9414eb8644fecd3436bc54cdad`

## Hygiene Checks

- solana_version_code: `0`
- solana_config_code: `0`
- rust_derive_code: `0`
- git_diff_check_code: `0`
- forbidden_worktree_count: `0`

## Gate 6.1 Decision

Gate 6.1 payload finalization result: `PASS_PAYLOAD_FINALIZED_SIGNATURES_PENDING`

## Non-Claims

This report does not claim:

- guardian signatures collected;
- live guarded mint approval;
- live guarded mint execution;
- replay approval;
- replay execution;
- production source event proof;
- production readiness;
- rollback execution.

## Next Gate

If Gate 6.1 is PASS, the next allowed step is review of this finalized payload/signing package.

The next disallowed steps without separate approval:

- live guarded mint;
- replay transaction;
- rollback;
- deploy;
- upgrade;
- any live transaction.

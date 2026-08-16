# Phase 41K.5 D2 / B1C7 Testnet Upgrade Planning Package

Date UTC: 2026-08-16T18:20:23Z

## Scope

This is an internal planning package for a possible D2/B1C7 testnet upgrade and subsequent live guarded mint validation.

This document is **not** an approval to execute any live action.

No transaction, deploy, upgrade, live mint, activation, or rollback is authorized by this package.

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before planning package: `78d1d6d7593ed3adafc97539bdfe205e618caa7c`
- Fixture alignment commit: `1389981cd8ffafba754540a94a3558df576efccb`
- Closeout report commit: `78d1d6d7593ed3adafc97539bdfe205e618caa7c`
- Closeout report path: `programs/xxxl-svm/reports/phase-41k5-d2-b1c7-closeout.md`
- Git status before planning package: `0` changed files

## Existing Local Readiness Evidence

D2/B1C7 local no-deploy readiness is already closed in the closeout report.

Summary:

- D2/B1C7 targeted matrix: `4/4 PASS`
- D2 gated mark + SPL mint e2e: `PASS`
- D3 negative failure modes: `PASS`
- B2 valid quorum live-gated success: `PASS`
- B3 hostile live-gated matrix: `PASS`
- build_sbf_code: `0`
- candidate artifact sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- candidate artifact size: `161664`

Safety flags from closeout:

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`

## External Review Status

- Codex review: `PASS`
- Theo closeout review: clean local closeout; no overclaim; planning package should have explicit gates, hash verification, rollback, and safety flags.

This package does not involve Jack and does not create an ask for Jack.

## Public Testnet Constants To Verify Before Any Live Step

Program:

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- ProgramData: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- Upgrade authority public key: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Gateway / Mint:

- Gateway mint authority PDA: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- Gateway mint authority bump: `252`
- Target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- SPL Token program: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- System program: `11111111111111111111111111111111`

Route / Asset:

- route_id: `d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c`
- canonical_asset_id: `479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458`
- source_chain_id: `1`
- source_chain_weight_bps: `10000`

Guardian Set:

- guardian_set_id: `4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83`
- quorum: `3`

Guardian public keys:

1. `7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf`
2. `GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp`
3. `6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih`
4. `9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY`
5. `UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg`

## Gate Model

### Gate 0 — Planning Only

Status: `current package`

Allowed:

- write planning documentation;
- review hashes and expected steps;
- inspect source code;
- run local tests;
- run read-only RPC checks.

Not allowed:

- live transaction;
- deploy;
- upgrade;
- guarded mint;
- rollback transaction.

### Gate 1 — Planning Review

Required before any live step:

- planning package reviewed;
- artifact hash verification procedure accepted;
- rollback path accepted;
- guardian set spec accepted;
- safety flags present;
- no overclaim.

Outcome options:

- `PASS`
- `PASS WITH NOTES`
- `BLOCKED`

### Gate 2 — Pre-Upgrade Read-Only Verification

No live transaction yet.

Required checks:

- git status clean;
- branch and commit recorded;
- candidate SBF artifact rebuilt locally;
- candidate artifact sha256 equals `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`;
- candidate artifact size equals `161664`;
- current deployed program is dumped/read and its sha256 recorded;
- Program ID matches expected;
- ProgramData matches expected;
- upgrade authority public key matches expected;
- no private key, seed, keypair, or binary artifact staged in git;
- RPC endpoint and Solana CLI config recorded;
- rollback artifact path and sha256 recorded.

Gate 2 output must be a separate evidence directory and report.

### Gate 3 — Explicit Upgrade GO

Upgrade may only proceed after a separate explicit human decision.

Required approval phrase:

```text
I approve the D2/B1C7 testnet upgrade transaction using artifact sha256 e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766.
```

Without that explicit approval, upgrade must not be executed.

### Gate 4 — Upgrade-Only Execution

Allowed only after Gate 3.

Permitted action:

- one testnet program upgrade transaction using the verified candidate artifact.

Required evidence:

- transaction signature;
- deployed program dump after upgrade;
- deployed dump sha256;
- byte/hash comparison against the candidate artifact where applicable;
- Program ID unchanged;
- ProgramData unchanged unless Solana upgrade semantics require a recorded change;
- upgrade authority still expected or explicitly documented.

Not allowed in Gate 4:

- live guarded mint;
- replay test transaction;
- activation transaction beyond the upgrade itself;
- unrelated transaction.

### Gate 5 — Post-Upgrade Read-Only Verification

Allowed after Gate 4.

Required:

- verify deployed program bytes/hash;
- verify program account metadata;
- verify ProgramData / authority;
- verify target mint metadata still unchanged unless expected;
- verify no mint supply change from upgrade-only step.

No guarded mint transaction yet.

### Gate 6 — Separate Live Guarded Mint GO

Live guarded mint validation requires a separate planning/evidence package and separate explicit approval.

Required before live guarded mint:

- event payload fully specified;
- recipient ATA verified;
- amount specified;
- canonical_event_key specified;
- guardian evidence/signatures specified;
- replay expectations specified;
- pre-state balances and processed_event state recorded;
- dry-run/simulation plan documented where possible;
- post-state checks documented;
- rollback/non-rollback decision documented.

Required approval phrase:

```text
I approve the D2/B1C7 live guarded mint validation transaction.
```

Without that explicit approval, no live guarded mint transaction may be broadcast.

## Rollback Plan

Rollback is not authorized by this package.

Rollback requires its own explicit GO.

Before any upgrade, the following must exist:

- current deployed program dump;
- current deployed program sha256;
- known-good rollback artifact path;
- known-good rollback artifact sha256;
- rollback command prepared but not executed;
- rollback verification procedure prepared.

Rollback trigger examples:

- candidate artifact hash mismatch before upgrade;
- upgrade transaction succeeds but deployed dump does not match expected candidate;
- program metadata or authority mismatch after upgrade;
- unexpected post-upgrade state mutation;
- post-upgrade read-only verification fails;
- any unexplained discrepancy in Program ID, ProgramData, or executable data.

Required rollback approval phrase:

```text
I approve rollback to the verified previous testnet artifact.
```

## Safety Flags For This Planning Package

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- rollback_executed: `false`

## Explicit Non-Claims

This planning package does not claim:

- testnet upgrade approval;
- testnet upgrade completion;
- live guarded mint approval;
- live guarded mint completion;
- replay tested against deployed program;
- production deployment readiness;
- Jack approval or Jack review.

## Planning Decision

Planning package status: **DRAFT / REVIEW REQUIRED**

Next allowed step:

- review this planning package.

Next disallowed step without separate approval:

- any live transaction;
- deploy;
- upgrade;
- live guarded mint;
- rollback.

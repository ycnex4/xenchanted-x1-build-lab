# Phase 41K.5 D2 / B1C7 Gate 6 Live Guarded Mint Planning Package

Date UTC: 2026-08-16T18:48:03Z

## Scope

This is a planning package for the first D2/B1C7 live guarded mint validation on X1 testnet after the Gate 4 upgrade and Gate 5 post-upgrade read-only verification.

This document is **not** an approval to execute a live guarded mint transaction.

No live guarded mint, replay, rollback, deploy, upgrade, activation, or production deployment is authorized by this package.

## Current Status

- Gate 1 closeout: `PASS`
- Gate 2 pre-upgrade read-only baseline: `PASS`
- Gate 3 explicit upgrade GO: `given`
- Gate 4 upgrade-only execution: `PASS`
- Gate 5 post-upgrade read-only verification: `PASS`
- Gate 6 live guarded mint: `not approved / not executed`

## Safety Flags

- transactions_executed: `false`
- deploy_executed: `false`
- upgrade_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before Gate 6 planning: `3c95fd24e91bb91afb7927b297a8958b2a5be08a`
- Git status before Gate 6 planning: `0` changed files

## Gate 4 / Gate 5 References

- Gate 4 commit: `38febb2dc7965094e4c01e5453deed53df48f2cf`
- Gate 4 upgrade transaction: `2qrt8uQFGnHritNqxeRGResgJuYBpjRn3tLGLanZFSJ297HtVSjCD4ZNb1LzSeHZfxb6C4W4ZsLD7GuP21nLbfxY`
- Gate 5 commit: `3c95fd24e91bb91afb7927b297a8958b2a5be08a`
- Candidate artifact sha256: `e20c2de8d982c8f6b8b01f996951ad5ce4bd40174158272942fc10c56121c766`
- Deployed program sha256 after Gate 5 read: `6e7e1c7b82cf9394129a20f3fee81d653cf501a2b50dcd5ba0dd7dff4fd6d509`

## Program / Mint Baseline

Program:

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- ProgramData: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- Upgrade authority: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Mint:

- Target SPL mint: `g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`
- Token program: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- Gateway mint authority PDA: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- Current supply after Gate 5: `0`
- Decimals: `9`
- Freeze authority: `None`

## Route / Asset / Guardian Baseline

Route and asset:

- route_id: `d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c`
- canonical_asset_id: `479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458`
- source_chain_id: `1`
- source_chain_weight_bps: `10000`

Guardian set:

- guardian_set_id: `4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83`
- quorum: `3`

Guardian public keys:

1. `7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf`
2. `GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp`
3. `6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih`
4. `9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY`
5. `UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg`

## Gate 6 Sub-Gates

### Gate 6.0 — Planning Only

Status: current package.

Allowed:

- write planning documentation;
- review expected fields;
- inspect scripts and code;
- run local-only tests;
- run read-only RPC checks.

Not allowed:

- live guarded mint;
- replay transaction;
- rollback;
- deploy;
- upgrade;
- any live transaction.

### Gate 6.1 — Event Payload Finalization

Before any live guarded mint approval, the following must be fully specified and recorded in a separate evidence report:

- recipient wallet;
- recipient ATA;
- amount in base units;
- human-readable amount with decimals;
- canonical_event_key;
- source_tx_hash or source event identifier;
- source log / event index if applicable;
- route_id;
- canonical_asset_id;
- target SPL mint;
- recipient_balance PDA;
- processed_event PDA;
- guardian_set_id;
- guardian signatures used;
- exact instruction data bytes or deterministic construction command;
- exact account list and account order;
- expected pre-state;
- expected post-state;
- replay expectation.

Status now: `not finalized`.

### Gate 6.2 — Pre-Mint Read-Only Verification

Before the first live guarded mint transaction:

- verify program bytes still match candidate prefix and zero tail;
- verify ProgramData and authority still match expected;
- verify target SPL mint supply is still `0`;
- verify target SPL mint authority is still gateway PDA;
- verify recipient ATA exists or document whether ATA creation is included;
- verify processed_event account does not already exist or is not already marked;
- verify recipient_balance account pre-state;
- verify rent payer balance;
- verify guardian set account;
- verify all PDAs from the finalized payload;
- record RPC URL and CLI config;
- record git status clean.

No live transaction is allowed in this sub-gate.

### Gate 6.3 — Explicit Live Mint GO

Live guarded mint may only proceed after a separate explicit human decision.

Required approval phrase template:

```text
I approve the D2/B1C7 live guarded mint validation transaction for canonical_event_key <CANONICAL_EVENT_KEY> using amount <AMOUNT_BASE_UNITS> and target SPL mint g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM.
```

Without that explicit approval, no live guarded mint transaction may be broadcast.

### Gate 6.4 — One Live Guarded Mint Transaction

Allowed only after Gate 6.3.

Permitted action:

- one live guarded mint validation transaction for the finalized payload.

Not allowed in the same approval:

- replay transaction;
- second mint transaction;
- rollback;
- deploy;
- upgrade;
- unrelated transaction.

Required evidence:

- transaction signature;
- full command used, with no private keys printed;
- key log lines;
- pre-state snapshot;
- post-state snapshot;
- processed_event state after transaction;
- recipient token account balance after transaction;
- target SPL mint supply after transaction;
- recipient_balance state after transaction;
- exact result: PASS / BLOCKED.

### Gate 6.5 — Post-Mint Read-Only Verification

After a successful live guarded mint:

- verify target mint supply increased by exactly the approved amount;
- verify recipient ATA balance increased by exactly the approved amount;
- verify processed_event is marked;
- verify recipient_balance reflects the mint;
- verify program metadata remains unchanged;
- verify no unrelated state was modified where checkable.

No replay transaction is allowed in this sub-gate.

### Gate 6.6 — Separate Replay GO

Replay validation must be separate from the first live mint approval.

Required approval phrase template:

```text
I approve the D2/B1C7 replay rejection validation transaction for canonical_event_key <CANONICAL_EVENT_KEY>.
```

Without that explicit approval, no replay transaction may be broadcast.

### Gate 6.7 — Replay Rejection Transaction

Allowed only after Gate 6.6.

Expected result:

- transaction rejects due already processed event / replay protection;
- target SPL mint supply remains unchanged;
- recipient ATA balance remains unchanged;
- processed_event remains marked;
- no additional mint occurs.

Required evidence:

- transaction signature if broadcast;
- expected error;
- observed error;
- unchanged post-state checks.

## Missing Finalization Items

The following are intentionally not finalized in this planning package:

- recipient wallet;
- recipient ATA;
- amount;
- canonical_event_key;
- concrete source event identifier;
- final instruction bytes;
- final guardian signatures;
- final command for live mint;
- final command for replay rejection.

These must be filled in a later Gate 6.1 finalization report before any approval phrase can be valid.

## Rollback / Stop Conditions

Rollback is not authorized by this package.

Stop immediately and do not mint if any of the following occurs:

- program bytes no longer match expected candidate prefix / zero-tail pattern;
- ProgramData mismatch;
- upgrade authority mismatch;
- target SPL mint authority mismatch;
- target SPL mint supply is not expected pre-state;
- recipient ATA is wrong or ambiguous;
- processed_event is already marked before first mint;
- guardian set account mismatch;
- guardian signatures do not match required quorum;
- instruction bytes cannot be reproduced deterministically;
- account order differs from the contract account contract;
- RPC or CLI config is ambiguous;
- any private key, seed, keypair, or binary artifact is staged in git.

Rollback, if ever needed, requires a separate rollback planning package and explicit rollback GO.

## Review Request

Please review this Gate 6 planning package for:

- no overclaim;
- safety flags;
- explicit sub-gates;
- separation between first live mint and replay validation;
- required finalization fields;
- stop conditions;
- absence of any implied approval.

## Gate 6 Planning Decision

Gate 6 planning package status: **DRAFT / REVIEW REQUIRED**

Next allowed step:

- review this planning package.

Next disallowed step without separate finalization and explicit approval:

- live guarded mint;
- replay transaction;
- rollback;
- deploy;
- upgrade;
- any live transaction.

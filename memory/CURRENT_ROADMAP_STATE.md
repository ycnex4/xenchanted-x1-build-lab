# Current Roadmap State — Gate 6.1-New v4

Status date: 2026-08-17

## Completed

### U3 — Testnet upgrade only

Status: DONE.

Result:

`PASS_U3_TESTNET_UPGRADE_ONLY_EXECUTED`

Deployed artifact hash:

`ca97970eb6c4c2977918fd4ff63a97f11069ba84cd85c33693f440383b2cfc06`

Scope was upgrade only.
No provisioning, mint, replay, rollback, production deployment, or second transaction was authorized by U3.

### P4 — Testnet provisioning only

Status: DONE.

Result:

`PASS_P4_TESTNET_PROVISIONING_ONLY_EXECUTED`

Initialized:

1. active guardian_set PDA
2. active gateway_config PDA

No guarded mint, replay, rollback, second mint, production deployment, mint_state reinitialization, or target SPL mint replacement was authorized by P4.

### Gate 6.1 v3

Status: BLOCKED and superseded.

Blocked hash:

`0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d`

Reason:
v3 used recipient wallet in the B1C authorization payload hash.
On-chain B1C context uses recipient_token_account / ATA.

### Gate 6.1 v4

Status: TECHNICALLY REVIEWED.

Current v4 hash:

`bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b`

Daemon review:

`PASS`

## Not started

Signature request distribution: NOT STARTED.

Signature collection: NOT STARTED.

Quorum: NOT MET.

Live mint: NOT AUTHORIZED.

Transaction sending: NOT AUTHORIZED.

Replay: NOT AUTHORIZED.

Rollback: NOT AUTHORIZED.

Production deployment: NOT AUTHORIZED.

## Next safe work

1. Repository hygiene/status repair.
2. Local full audit:
   - git status
   - npm typecheck/test/build
   - Rust/Solana checks
   - secret/raw signature/signed tx scan
   - stale v3/current v4 scan
3. Commit hygiene/status repair.
4. Decide separately whether to start signature request distribution.

## Decision rule

No conditional GO.

Correct process:

1. Run/check step X.
2. Review result.
3. Proceed to step Y only after a separate explicit GO.

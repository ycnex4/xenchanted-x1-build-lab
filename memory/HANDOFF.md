# HANDOFF — xEnchanted X1 Build Lab / XXXL SVM

Status date: 2026-08-17

## Current branch

Current working branch:

`audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`

Current branch is the operational source of truth for D2/B1C7 / Gate 6.1 work.
`main` is not yet updated with this branch.

## Current roadmap state

U3 testnet upgrade: DONE.

P4 testnet provisioning: DONE.

v3 Gate 6.1 payload: BLOCKED and superseded.

v4 Gate 6.1 payload: corrected.

v4 daemon pre-signature review: PASS.

Signature request distribution: NOT STARTED.

Signature collection: NOT STARTED.

Live mint: NOT AUTHORIZED.

Transaction sending: NOT AUTHORIZED.

Replay: NOT AUTHORIZED.

Rollback: NOT AUTHORIZED.

Production deployment: NOT AUTHORIZED.

## Current v4 message

Current v4 authorization payload hash / guardian message:

`bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b`

This must be signed as raw 32 bytes, not ASCII hex.

Blocked v3 hash:

`0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d`

v3 must not be signed.

## Critical account distinction

Recipient wallet / args.recipient:

`DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Recipient token account / ATA / B1C payload recipient:

`9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k`

The v4 B1C authorization hash uses the ATA as payload recipient.
The instruction args recipient remains the wallet.

## Active testnet state references

Program:

`D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData:

`9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Upgrade / provisioning authority public key:

`DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Gateway mint authority PDA:

`BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Active guardian_set_id:

`5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421`

Active guardian_set PDA:

`9fRJqk7DTkNhXwQEjtSg8ZhgVwt1D6a7VoZhHSMNuP25`

Route ID:

`aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f`

Gateway config PDA:

`3UFjhhHubGnE2xgdjNayaMQrkYnSRtE6ynxLteByVig5`

Canonical event key:

`d468547c473242a9dfad84173e03ad15e6df13080e1cc028445d847044079d78`

Processed event PDA:

`9HKzkevZXHdsG3ZqFGXnLJ6jxdw5RvBVr5aC2h61V1JC`

Target SPL mint:

`g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`

## Safety boundary

Do not commit or push:

- private keys
- seed material
- raw signatures
- full signature bundles
- signed transaction bytes
- local keypair paths
- environment dumps
- `.env`
- `.local/signatures/`
- generated build artifacts

Any live mint requires:

1. signature quorum report,
2. read-only pre-mint verification,
3. separate explicit GO,
4. no conditional authorization.

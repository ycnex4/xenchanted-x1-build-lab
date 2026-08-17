# Daemon Review Package — Gate 6.1-New v4-FINAL Pre-Signature Distribution

Date UTC: 2026-08-17T18:46:23Z

## Scope

Pre-signature distribution review only.

No transaction.
No live mint.
No replay.
No rollback.
No state mutation.
No private keys.
No raw signatures.
No signed transaction bytes.

A PASS only permits distributing the v4 signing request and collecting guardian signatures.
A PASS does not authorize live mint execution.

## Repository

Repository:
https://github.com/ycnex4/xenchanted-x1-build-lab

Branch:
`audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`

Content commit with v4 files:
`a684011599866c7ea4b2edae6d56eda5559649bf`

## Primary review files

Gate 6.1-New v4-FINAL planning:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/memory/gate-6-1-new-v4-final-planning-20260817.md

v4 signing request:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/memory/gate-6-1-new-v4-signing-request-20260817.md

Signature tracking report:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/memory/gate-6-1-new-signature-report-20260817.md

HANDOFF:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/memory/HANDOFF.md

## Source files to verify against

Processor / B1C7 handler:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/processor.rs

B1C authorization payload hash:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/verifier/b1c_payload_hash_binding.rs

Instruction layout:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/instruction.rs

Processed event PDA / loader:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/verifier/processed_registry_account_loading_boundary.rs

Processed event marking boundary:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/processed_event_marking_boundary.rs

PDA derivations:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/pda.rs

Validation / ATA check:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/a684011599866c7ea4b2edae6d56eda5559649bf/programs/xxxl-svm/src/validation.rs

## v3 blocker being corrected

v3 is BLOCKED and superseded.

v3 used recipient wallet in B1C payload hash:
`DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

v3 blocked hash:
`0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d`

Reason:
On-chain B1C payload context field `recipient` is populated from `recipient_token_account.key`, not recipient wallet.

## v4 corrected critical values

Program:
`D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Active guardian_set_id:
`5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421`

Active guardian_set PDA:
`9fRJqk7DTkNhXwQEjtSg8ZhgVwt1D6a7VoZhHSMNuP25`

Route ID:
`aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f`

Gateway config PDA:
`3UFjhhHubGnE2xgdjNayaMQrkYnSRtE6ynxLteByVig5`

Canonical asset ID:
`479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458`

Canonical event key:
`d468547c473242a9dfad84173e03ad15e6df13080e1cc028445d847044079d78`

Processed event PDA:
`9HKzkevZXHdsG3ZqFGXnLJ6jxdw5RvBVr5aC2h61V1JC`

Target SPL mint:
`g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM`

Recipient wallet / args.recipient:
`DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Recipient token account / ATA / B1C payload recipient:
`9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k`

Amount:
`1` base unit

ConsumeGatewayMint discriminator:
`f2f4a868bb89fe52`

v4 authorization payload hash / guardian message:
`bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b`

Guardian message must be signed as raw 32 bytes, not as ASCII hex.

## Source-canonical B1C hash formula

The daemon should verify that v4 uses:

```text
hashv([
  b"consume_gateway_mint_authorization_v2",
  processed_event.as_ref(),
  route_id,
  mint.as_ref(),
  recipient_token_account.as_ref(),
  amount_u64_le,
  guardian_set_id,
])
```

Important separation:

```text
args.recipient / recipient_wallet:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

B1C payload context recipient / recipient_token_account / ATA:
9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k
```

`canonical_asset_id` and `nonce` are not included in the B1C authorization hash.

## Account map expected for B1C7

Expected B1C7 account count: 12

```text
0  mint_state
1  gateway_config
2  guardian_set
3  processed_event
4  recipient_balance
5  SPL token mint
6  recipient token account / ATA
7  mint authority PDA
8  token program
9  rent payer
10 system program
11 instructions sysvar
```

## Storage and report safety

Recommended raw signatures path:

```text
$HOME/.local/signatures/
```

Do not use:

```text
/.local/signatures/
```

Raw signatures remain outside repo and are never committed or pushed.

Git-safe report may include only:

```text
guardian public keys
signed message hash
signature count
unique signer count
quorum status
per-signer verified true/false
redacted signature identifiers
local-only bundle sha256
```

Git-safe report must not include:

```text
full raw signatures
full signature bundle
private keys
seed material
signed transaction bytes
```

## Required daemon checks

1. v3 hash `0e6f20fb...` is marked BLOCKED and superseded.
2. v4 hash `bf9a130c...` is CURRENT.
3. v4 B1C hash uses recipient token account / ATA, not wallet.
4. recipient wallet and recipient token account are both listed and not conflated.
5. signing request tells guardians to sign raw 32 bytes of `bf9a130c...`.
6. signing request does not mention the blocked v3 hash as a current signing message.
7. processed_event PDA is derived from:
   `["xxxl", "processed-event", canonical_event_key]`
8. signing request requires processed_event PDA absence check before signing.
9. no signatures were collected.
10. no tx was sent.
11. no live mint, no replay, no rollback, no state mutation.
12. raw signatures and raw bundles are excluded from git.

## Expected daemon output

```text
PASS / BLOCKED

Blockers:
- ...

Required corrections before distribution:
- ...

May distribute signing request to guardians:
YES / NO
```

Decision boundary:

A PASS only permits distributing the v4 signing request and collecting guardian signatures.
A PASS does not authorize live mint execution, replay, rollback, production deployment, program upgrade, or any state mutation.

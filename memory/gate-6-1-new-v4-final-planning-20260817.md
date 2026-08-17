# Gate 6.1-New v4-FINAL — Corrected B1C Authorization Hash

Status: PLANNING ONLY.
Signature collection: NOT AUTHORIZED YET.
Live mint: NOT AUTHORIZED.
Transaction sending: NOT AUTHORIZED.

## v3 Blocker

v3 is BLOCKED and superseded.

v3 hash:
0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d

Reason:
v3 computed the B1C authorization payload hash using the recipient wallet.
The on-chain handler uses recipient_token_account / ATA in B1CAuthorizationPayloadContext.recipient.

## Correct v4 Values

Program:
D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

Active guardian_set_id:
5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421

Active guardian_set PDA:
9fRJqk7DTkNhXwQEjtSg8ZhgVwt1D6a7VoZhHSMNuP25

Route ID:
aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f

Gateway config PDA:
3UFjhhHubGnE2xgdjNayaMQrkYnSRtE6ynxLteByVig5

Canonical asset ID:
479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458

Canonical event key:
d468547c473242a9dfad84173e03ad15e6df13080e1cc028445d847044079d78

Processed event PDA:
9HKzkevZXHdsG3ZqFGXnLJ6jxdw5RvBVr5aC2h61V1JC

Target SPL mint:
g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM

Recipient wallet / args.recipient:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Recipient token account / ATA / B1C payload recipient:
9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k

Amount:
1 base unit

ConsumeGatewayMint discriminator:
f2f4a868bb89fe52

## Correct v4 Guardian Message

authorization_payload_hash / guardian message:
bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b

Guardians must sign the raw 32 bytes of this hash, not the ASCII hex string.

Raw bytes:
bf 9a 13 0c a2 a9 09 a1 c9 f2 82 e2 67 47 80 32
45 60 94 3d b8 27 11 b9 ba d2 f5 b2 08 f2 f4 0b

## Source-Canonical Hash Formula

hashv([
  b"consume_gateway_mint_authorization_v2",
  processed_event.as_ref(),
  route_id,
  mint.as_ref(),
  recipient_token_account.as_ref(),
  amount_u64_le,
  guardian_set_id,
])

Important:
- args.recipient remains the wallet DTfv...
- B1C payload recipient uses ATA 9ncb...
- canonical_asset_id is not directly included in the B1C authorization hash
- nonce is not included in the B1C authorization hash

## Safety Confirmation

- v3 hash is BLOCKED and superseded
- v4 hash is CURRENT
- no signatures collected
- no tx sent
- no live mint
- no replay
- no rollback
- no state mutation
- raw signatures must remain local-only
- raw signatures and signed tx bytes must not be committed or pushed
- signature storage should use $HOME/.local/signatures/ or another home-local path, not /.local/signatures/

## Next Step

Prepare daemon review package for v4.
Only after daemon PASS and Sergey explicit GO may the v4 signing request be distributed.

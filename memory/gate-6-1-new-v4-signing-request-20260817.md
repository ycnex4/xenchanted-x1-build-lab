# Gate 6.1-New v4 Signing Request

Status: PREPARED ONLY.
Distribution to guardians: NOT AUTHORIZED YET.
Signature collection: NOT AUTHORIZED YET.
Live mint: NOT AUTHORIZED.
Transaction sending: NOT AUTHORIZED.

This document may be distributed only after daemon PASS and Sergey explicit GO for v4 signature collection.

## v3 Blocked

Do not sign v3 hash:
0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d

v3 is blocked because it used recipient wallet instead of recipient token account / ATA in the B1C authorization payload hash.

## v4 Message to Sign

Sign raw 32 bytes of:

bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b

Do not sign the ASCII hex string.

Raw bytes:
bf 9a 13 0c a2 a9 09 a1 c9 f2 82 e2 67 47 80 32
45 60 94 3d b8 27 11 b9 ba d2 f5 b2 08 f2 f4 0b

## Recipient Clarification

Recipient wallet / args.recipient:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

Recipient token account / ATA / B1C payload recipient:
9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k

Only the B1C authorization payload uses the ATA.

## Guardian Set

Threshold:
3 of 5

Guardians:
1. 7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf
2. GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp
3. 6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih
4. 9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY
5. UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg

Active guardian_set_id:
5b1424b856b2199a40ebf18c9766ee36d0f6d44be58f085ec042a8fc7626e421

Active guardian_set PDA:
9fRJqk7DTkNhXwQEjtSg8ZhgVwt1D6a7VoZhHSMNuP25

## Parameters

Program:
D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

Gateway config:
3UFjhhHubGnE2xgdjNayaMQrkYnSRtE6ynxLteByVig5

Processed event PDA:
9HKzkevZXHdsG3ZqFGXnLJ6jxdw5RvBVr5aC2h61V1JC

SPL mint:
g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM

Amount:
1 base unit

Route ID:
aac8572dddf1a3b9211cc16af14ab316eb6f3b927441037782f55b5e2e5d216f

Canonical event key:
d468547c473242a9dfad84173e03ad15e6df13080e1cc028445d847044079d78

Instruction discriminator:
f2f4a868bb89fe52

## Required Checks Before Signing

Each guardian must verify:

1. They are signing v4 hash bf9a130c..., not blocked v3 hash 0e6f20fb...
2. Message is raw 32 bytes, not ASCII hex.
3. processed_event PDA does not exist:
   solana account 9HKzkevZXHdsG3ZqFGXnLJ6jxdw5RvBVr5aC2h61V1JC --url https://rpc.testnet.x1.xyz
4. guardian_set PDA is active and contains their guardian pubkey.
5. recipient token account / ATA is the B1C payload recipient:
   9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k
6. recipient wallet remains:
   DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
7. No live mint is authorized by this signing request.

## Safety

Do not share private keys.
Do not share seed material.
Send only signature + guardian public key to the coordinator.
Raw signatures must remain local-only and must not be committed to git.
A separate Sergey GO is required for live mint execution after quorum.

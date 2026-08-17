# Gate 6.1-New v4 Signature Collection Report

Status: SIGNATURE COLLECTION BLOCKED — awaiting v4 daemon review and Sergey explicit GO.
Scope: Signature collection only after approval. No live mint. No tx. No replay. No rollback.

## Version History

| Version | Signed Message Hash | Status | Signatures Collected |
|---|---|---|---|
| v3-FINAL | 0e6f20fb... | BLOCKED — wrong recipient, wallet instead of ATA | 0 |
| v4-FINAL | bf9a130c... | PREPARED — awaiting daemon review and Sergey GO | 0 |

## v4 Message

bf9a130ca2a909a1c9f282e2674780324560943db82711b9bad2f5b208f2f40b

Must be signed as raw 32 bytes, not ASCII hex.

## Recipient Values

recipient_wallet:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

recipient_token_account / ATA:
9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k

## Signature Collection Log

| # | Guardian Pubkey | Status | Verified |
|---|---|---|---|
| 1 | 7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf | Awaiting | — |
| 2 | GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp | Awaiting | — |
| 3 | 6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih | Awaiting | — |
| 4 | 9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY | Awaiting | — |
| 5 | UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg | Awaiting | — |

## Aggregate Status

Signatures collected: 0
Unique signers: 0
Quorum met: NO
Minimum quorum: 3 of 5

## Local-Only Signature Bundle

Recommended path:
$HOME/.local/signatures/gate-6-1-new-v4-bundle.json

This path is local-only and must not be committed.
Raw signatures, signature bundles, private keys, seed material, and signed transaction bytes must not be committed or pushed.

## Safety Flags

- v3 hash blocked and superseded
- v4 hash current
- no signatures collected
- no tx sent
- no live mint
- no replay
- no rollback
- no state mutation
- raw signatures not committed
- live mint requires separate Sergey GO after quorum and pre-mint verification

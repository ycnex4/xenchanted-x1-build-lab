# Daemon Review Package — Gate 6.1-New v3-FINAL Pre-Signature Distribution

Date UTC: 2026-08-17T17:58:56Z

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

A PASS only permits distributing the signing request and collecting guardian signatures.
A PASS does not authorize live mint execution.

## Repository

Repository:
https://github.com/ycnex4/xenchanted-x1-build-lab

Branch:
`audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`

Head:
`2f8c2a80ce647b9acf5a952e3b85619349aa98a7`

## Primary review files

Gate 6.1-New v3-FINAL planning:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/memory/gate-6-1-new-v3-final-planning-20260817.md

Signing request:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/memory/gate-6-1-new-signing-request-20260817.md

Signature tracking report:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/memory/gate-6-1-new-signature-report-20260817.md

HANDOFF:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/memory/HANDOFF.md

## Recent execution/state reports

P4 provisioning report:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/reports/phase-41k5-d2-b1c7-gate-p4-testnet-provisioning-only-retry-simfix.md

U3 upgrade report:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/reports/phase-41k5-d2-b1c7-gate-u3-testnet-upgrade-only.md

## Source files to verify against

Instruction layout:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/instruction.rs

B1C authorization payload hash:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/verifier/b1c_payload_hash_binding.rs

Processed event PDA / loader:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/verifier/processed_registry_account_loading_boundary.rs

Processed event marking boundary:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/processed_event_marking_boundary.rs

PDA derivations:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/pda.rs

Processor / account map / B1C7 path:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/processor.rs

State layouts:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/state.rs

Guardian set loader:
https://github.com/ycnex4/xenchanted-x1-build-lab/blob/audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z/programs/xxxl-svm/src/verifier/guardian_set_account_loading_boundary.rs

## Critical values

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

Recipient:
`DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

Amount:
`1` base unit

ConsumeGatewayMint discriminator:
`f2f4a868bb89fe52`

Authorization payload hash / guardian message:
`0e6f20fb737f9d9fc624ce89cce75091a5216d8dee5ae96fc377f8c22c633a3d`

Guardian message must be signed as raw 32 bytes, not as ASCII hex.

## Source-canonical checks daemon should verify

1. `canonical_event_key` is exactly 32 bytes, not the old human-readable string.

2. `processed_event PDA` is derived from:
`["xxxl", "processed-event", canonical_event_key]`

3. `authorization_payload_hash` uses Solana `hashv`:

```text
hashv([
  b"consume_gateway_mint_authorization_v2",
  processed_event.as_ref(),
  route_id,
  mint.as_ref(),
  recipient.as_ref(),
  amount_u64_le,
  guardian_set_id,
])
```

4. `canonical_asset_id` and `nonce` are not included in the B1C authorization hash.

5. Consume instruction discriminator is:
`f2f4a868bb89fe52`

6. B1C7 account map is exactly 12 accounts:

```text
0  mint_state
1  gateway_config
2  guardian_set
3  processed_event
4  recipient_balance
5  SPL token mint
6  recipient token account
7  mint authority PDA
8  token program
9  rent payer
10 system program
11 instructions sysvar
```

7. Signing request instructs guardians to sign raw 32 bytes only, not ASCII hex.

8. Signing request requires guardians to verify processed_event PDA absence before signing.

9. Raw signatures and signature bundles are not committed or pushed.

10. Signature storage must not use ambiguous root-level `/.local/signatures/`.
Use repo-local `.local/signatures/` with strict gitignore or `$HOME/.local/signatures/`.

11. Git-safe report may include only:
- guardian public keys
- signed message hash
- signature count
- unique signer count
- quorum status
- per-signer verified true/false
- redacted signature identifiers
- local-only bundle sha256

12. Git-safe report must not include:
- full raw signatures
- full signature bundle
- private keys
- seed material
- signed transaction bytes

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

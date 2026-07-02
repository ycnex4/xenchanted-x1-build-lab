# XXXL X1 Testnet Local Runtime Skeleton — Phase 41G.2 Payload Hash Binding Plan Checkpoint

Date: 2026-07-03

## Phase

Phase 41G.2 — Payload Hash Binding Plan.

## Parent Gate

`c89fc59 Merge XXXL phase 41G payload evidence shape acceptance`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Plan the narrow hash binding relation:

`signed_message_bytes == compute_guardian_payload_hash(raw_payload_bytes)`

## Critical Requirements

- reuse `raw_payload.rs`;
- reuse `canonical_payload.rs`;
- use 32-byte domain separator hash;
- do not prepend literal UTF-8 label bytes directly;
- do not trust caller-provided hash;
- raw payload bytes remain untrusted until hash binding succeeds;
- structural decode proves only well-formedness.

## Boundary

41G.2 may plan payload hash binding.

41G.2 must not validate guardian membership, count quorum, authorize minting, write replay state, mutate state, CPI, mint, handle live instruction flow, or unlock live route.

## Next Gate

After external acceptance, Phase 41G.2 implementation planning may begin separately.

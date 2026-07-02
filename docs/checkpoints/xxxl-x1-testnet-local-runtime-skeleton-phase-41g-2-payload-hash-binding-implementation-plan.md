# XXXL X1 Testnet Local Runtime Skeleton — Phase 41G.2 Payload Hash Binding Implementation Plan Checkpoint

Date: 2026-07-03

## Phase

Phase 41G.2 — Payload Hash Binding Implementation Plan.

## Parent Gate

`0825dad Merge XXXL phase 41G payload hash binding plan acceptance`

## Scope

Docs-only implementation planning checkpoint.

No runtime code.

No `.rs` changes.

No verification logic change.

No guardian/quorum/auth/replay/mutation/CPI/mint/live behavior enabled.

## Purpose

Plan the future implementation of the narrow payload hash binding boundary.

Preferred future validation path:

`validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`

## Critical Implementation Flow

1. require Phase 41F verification established;
2. require `signed_message_bytes.len() == 32`;
3. checked-convert to `&[u8; 32]`;
4. call `validate_guardian_payload_hash(raw_payload_bytes, signed_hash_32)`;
5. success means only payload hash binding established;
6. failure is fail-closed.

## Boundary

Implementation planning must not include guardian validation, quorum, authorization, replay writes, mutation, CPI, mint, handler, or live route.

## Next Gate

After external acceptance, Phase 41G.2 implementation may begin under a separate reviewed boundary.

# XXXL X1 Testnet Local Runtime Skeleton — Phase 41E.2 Checkpoint

Date: 2026-07-02

## Phase

Phase 41E.2 — offset-table alias hardening.

## Parent Checkpoint

`ac7fcb3 Merge XXXL phase 41E Ed25519 byte parsing boundary acceptance record`

## Scope

Parser hardening only.

## Implemented

- Added parser status `ParsedRangeAliasesOffsetTable`.
- Added guardrail flag `rejects_offset_table_aliasing`.
- Added strict check that signature/public-key/message ranges must start at or after `ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`.
- Added tests for signature/public-key/message offset-table alias rejection.

## Still Forbidden

- Ed25519 cryptographic verification;
- signature validity acceptance;
- guardian validity acceptance;
- proof acceptance;
- evidence acceptance;
- quorum;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- handler;
- live route.

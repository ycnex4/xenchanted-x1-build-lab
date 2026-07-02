# XXXL Phase 41E.2 — Offset-Table Alias Hardening

Date: 2026-07-02

## Status

Runtime parser hardening.

## Parent Gate

Parent accepted checkpoint:

`ac7fcb3 Merge XXXL phase 41E Ed25519 byte parsing boundary acceptance record`

## Purpose

Phase 41E.2 addresses the non-blocking Audit Demon note from Phase 41E.1.

The parser now rejects parsed signature/public-key/message ranges that start inside the Ed25519 instruction header/offset-table range.

## Boundary

This is still byte parsing only.

No cryptographic verification is introduced.

No proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route is introduced.

## New Strict Rule

The parser now requires all parsed ranges to start after the Ed25519 single-signature offset table:

- `signature_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `public_key_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `message_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`.

For the currently supported single-signature layout:

- `ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN == 16`.

Any parsed range that aliases `[0, 16)` fails closed with:

- `ParsedRangeAliasesOffsetTable`.

## Why This Matters

Phase 41E.1 already checked:

- range fits inside instruction data;
- ranges do not overlap each other.

Phase 41E.2 additionally prevents ranges from aliasing the header/offset table itself.

This produces a stricter parser before future verification-oriented work.

## Still Non-Authorizing

`ParsedRangeAliasesOffsetTable` is only a structural parsing rejection.

It does not mean:

- signature invalid;
- proof rejected;
- evidence accepted/rejected;
- guardian accepted/rejected;
- quorum evaluated;
- authorization evaluated.

## Validation Targets

Expected targeted test:

- `cargo test ed25519_instruction_byte_parsing_boundary --lib`

Expected broad tests:

- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

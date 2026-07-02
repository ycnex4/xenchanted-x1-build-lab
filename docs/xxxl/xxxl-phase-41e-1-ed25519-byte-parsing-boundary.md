# XXXL Phase 41E.1 — Ed25519 Instruction Byte Parsing Boundary

Date: 2026-07-02

## Status

Runtime code boundary implemented.

## Parent Gate

Parent accepted checkpoint:

`e550a51 Merge XXXL phase 41E Ed25519 byte parsing plan acceptance record`

## Scope

Phase 41E.1 implements non-authorizing Ed25519 instruction byte parsing.

Allowed:

- consume Phase 41D3.2.2 loaded prior instructions;
- consume Phase 41D3.2.3 prefilter/descriptor result;
- gate only on located status plus matched instruction index;
- locate the already loaded matched Ed25519 instruction entry;
- parse Ed25519 instruction header and offset metadata;
- parse signature/public-key/message byte ranges as bounded indices;
- reject malformed/out-of-bounds/cross-instruction/overlap cases deterministically;
- expose parsed metadata only.

Not allowed:

- gate on `locates_prior_ed25519_instruction`;
- trust Phase 41D3.2.3 descriptor booleans as evidence;
- load referenced instructions;
- copy attacker-sized message bytes;
- verify Ed25519 signatures;
- accept signature validity;
- accept guardian validity;
- accept proof/evidence;
- count quorum;
- authorize execution;
- write replay;
- mutate accounts;
- CPI;
- mint;
- add handler;
- unlock live route.

## Entry Gate

Future downstream code must treat Phase 41E.1 output as parsed metadata only.

The parser itself proceeds only when both are true:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

It does not use `locates_prior_ed25519_instruction` as a gate.

## Cross-Instruction Reference Policy

Phase 41E.1 rejects any non-self Ed25519 signature offset instruction-index reference.

It does not load referenced instructions.

Any future support for referenced-instruction loading requires a separate reviewed gate.

## Message Range Policy

Phase 41E.1 stores the message range as bounded indices:

- `message_offset`;
- `message_len`.

It does not copy attacker-sized message data into a new `Vec`.

## Overlap Policy

Phase 41E.1 uses a deterministic strict policy:

- reject overlapping parsed byte ranges.

## Trust Boundary

`parses_ed25519_instruction_bytes: true`

means only that byte parsing occurred.

It does not mean:

- signature verified;
- proof accepted;
- evidence accepted;
- guardian accepted;
- quorum reached;
- execution authorized;
- replay writable;
- state mutable;
- CPI/mint/live route enabled.

## Validation Targets

Expected targeted test:

- `cargo test ed25519_instruction_byte_parsing_boundary --lib`

Expected broad tests:

- `cargo test verifier --lib`
- `cargo test --lib --locked`
- `npm run typecheck`
- `npm run build`

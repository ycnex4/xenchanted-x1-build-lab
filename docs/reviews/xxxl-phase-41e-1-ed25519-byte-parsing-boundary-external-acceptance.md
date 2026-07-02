# XXXL Phase 41E.1 Ed25519 Instruction Byte Parsing Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`00861cf Merge XXXL phase 41E Ed25519 byte parsing boundary`

## Scope Accepted

Phase 41E.1 is accepted as a narrow runtime code boundary for non-authorizing Ed25519 instruction byte parsing.

Accepted runtime scope:

- consume Phase 41D3.2.2 loaded prior instructions;
- consume Phase 41D3.2.3 prefilter result;
- gate only on located status plus matched instruction index;
- parse already-loaded matched Ed25519 instruction bytes;
- parse header/signature-count/offset metadata;
- parse signature/public-key/message ranges as bounded metadata;
- reject malformed/out-of-bounds/cross-instruction/overlap cases deterministically;
- keep parsed output non-authorizing.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- entry gate correctness;
- no trust drift from Phase 41D3.2.3 descriptor booleans;
- cross-instruction references fail-closed;
- no new loading surface;
- message range allocation policy;
- deterministic overlap rejection;
- byte parsing boundary as acceptable before future cryptographic verification.

Theo confirmed the parser proceeds only when both are true:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

Theo confirmed the parser does not use:

- `locates_prior_ed25519_instruction` as a gate.

Theo confirmed the output is parsed metadata only and does not enable:

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
- `invoke_signed`;
- SPL Token `mint_to`;
- handler;
- live route.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Forbidden operations detected: no.

Entry gate acceptable: yes.

Cross-instruction reference policy acceptable: yes.

Message range allocation policy acceptable: yes.

Overlap policy acceptable: yes.

Trust-sensitive boundary drift: no.

Next phase allowed: yes.

Demon accepted:

- scope is limited to byte parsing;
- no crypto verification is introduced;
- no proof/evidence acceptance is introduced;
- no guardian validity acceptance is introduced;
- no quorum/auth/replay/mutation/CPI/mint/handler/live route is introduced;
- `mod.rs` delta is one `pub mod`;
- current-design checkpoint update is append-only;
- parser consumes already-loaded Phase 41D3.2.2 entries;
- parser does not create a new loading surface;
- parser rejects cross-instruction references;
- parser does not load referenced instructions;
- message range is represented as bounded indices;
- parser does not copy attacker-sized message bytes;
- overlap policy deterministically rejects all parsed-range overlaps;
- checked offset math is used;
- no unchecked indexing/slicing/panic/unsafe was detected;
- parsed output remains non-authorizing.

## Validation Before Merge

Local validation before merge was reported as passing:

- scope check: OK;
- `mod.rs` delta check: OK;
- forbidden trust/execution grep: OK;
- raw sysvar / new loading grep: OK;
- panic token grep: OK;
- unchecked index/slice grep: OK;
- required guardrail grep: OK;
- `git diff --check`: OK;
- `cargo fmt`: OK;
- `cargo fmt --check`: OK;
- `cargo test ed25519_instruction_byte_parsing_boundary --lib`: OK;
- `cargo test verifier --lib`: OK;
- `cargo test --lib --locked`: OK;
- `npm run typecheck`: OK;
- `npm run build`: OK.

## Entry Gate Accepted

The accepted Phase 41E.1 entry gate is:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

The following must not be used as a success/progression gate:

- `locates_prior_ed25519_instruction`.

## Descriptor Boolean Guardrail Accepted

Phase 41E.1 does not trust Phase 41D3.2.3 descriptor booleans as evidence:

- `structurally_well_formed_candidate`;
- `guardian_evidence_unique`;
- `matches_expected_current_identity_binding`.

The parser independently re-checks real bytes and runtime entry properties.

## Cross-Instruction Reference Policy Accepted

Phase 41E.1 accepts only self-contained Ed25519 offset references:

- signature instruction index must be `u16::MAX`;
- public key instruction index must be `u16::MAX`;
- message instruction index must be `u16::MAX`.

Any non-self index reference fails closed.

Phase 41E.1 does not load referenced instructions.

Any future referenced-instruction loading requires a separate reviewed gate.

## Message Range Policy Accepted

Phase 41E.1 stores the variable-length message as bounded metadata:

- `message_offset`;
- `message_len`;
- `Phase41E_1ByteRange`.

It does not copy attacker-sized message bytes into a new `Vec`.

## Overlap Policy Accepted

Phase 41E.1 uses a deterministic strict parser policy:

- reject overlapping signature/public-key/message parsed ranges.

## Demon Non-Blocking Note — Offset Table Alias Hardening

Demon noted that Phase 41E.1 currently checks that parsed ranges fit inside instruction data and do not overlap each other.

The parser does not yet require parsed ranges to begin after the Ed25519 offset table.

Future hardening should require:

- `signature_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `public_key_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `message_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`.

Purpose:

- prevent parsed ranges from aliasing the header/offset-table range `[0, 16)`;
- better model a strict legitimate Ed25519 layout before future verification.

This is non-blocking for Phase 41E.1 because the boundary is non-authorizing.

This should be addressed before or during the next verification-oriented boundary.

## Still Forbidden

The following remain forbidden after Phase 41E.1 acceptance:

- Ed25519 cryptographic verification;
- signature validity acceptance;
- guardian validity acceptance;
- cryptographic signature proof acceptance;
- verification evidence acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41E.1.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41E.1 byte parsing boundary is externally accepted.

Next work may begin only as a separate reviewed boundary.

Expected next direction:

- offset-table alias hardening;
- then future Ed25519 cryptographic verification planning/code under its own gate.

Any future comparison of message hash, extraction of pubkey/signature bytes, cryptographic verification, proof acceptance, evidence acceptance, quorum, authorization, replay, mutation, CPI, mint, handler, or live route must remain under separate explicit review.

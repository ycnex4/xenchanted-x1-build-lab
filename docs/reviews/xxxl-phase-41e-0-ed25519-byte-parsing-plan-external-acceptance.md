# XXXL Phase 41E.0 Ed25519 Instruction Byte Parsing Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`332df6c Merge XXXL phase 41E Ed25519 byte parsing plan`

## Scope Accepted

Phase 41E.0 is accepted as a docs-only plan before introducing Ed25519 instruction byte parsing code.

No runtime code was introduced.

Accepted future scope:

- parse real bytes from a structurally located prior Ed25519 instruction;
- classify malformed layouts deterministically;
- classify out-of-bounds offsets deterministically;
- extract non-authorizing parsed metadata;
- keep all parsed fields non-authorizing;
- keep cryptographic verification/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route closed.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- byte parsing is the correct next micro-phase after Phase 41D3 closure;
- entry gate is correct;
- `locates_prior_ed25519_instruction` is not an evidence gate;
- Phase 41D3.2.3 descriptor booleans are not validated evidence;
- parsing is allowed without verification;
- parsed fields are non-authorizing;
- malformed cases fail closed;
- panic-safety requirements are sufficient;
- heap/allocation requirements are sufficient;
- parsing-specific flag is acceptable if it means parsing only;
- crypto/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live remain closed;
- narrow Phase 41E byte parsing code may begin after acceptance.

Theo accepted entry gate:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

Theo explicitly rejected using the following as a progression gate:

- `locates_prior_ed25519_instruction`.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Entry gate acceptable: yes.

Descriptor boolean guardrail acceptable: yes.

Byte parsing boundary acceptable: yes.

Trust-sensitive boundary drift: no.

Next code sub-step allowed: yes.

Accepted findings:

- docs-only scope is clean;
- no runtime code was introduced;
- current-design update is append-only;
- entry gate is acceptable;
- descriptor boolean guardrail is acceptable;
- real Ed25519 instruction byte parsing without verification is acceptable;
- parsed output remains non-authorizing;
- malformed/out-of-bounds/ambiguous cases fail closed;
- panic-safety requirements are sufficient;
- heap/allocation requirements are sufficient;
- parsing-specific flag is acceptable if it means parsing only;
- crypto/proof/evidence/quorum/auth/replay/processed/mutation/CPI/invoke_signed/mint/handler/live remain closed;
- all active blockers remain.

## Entry Gate Guardrail

Future Phase 41E code must not gate on:

- `locates_prior_ed25519_instruction`.

Future Phase 41E code must gate only on both:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

## Descriptor Boolean Guardrail

Future Phase 41E code must not trust Phase 41D3.2.3 descriptor booleans as validated evidence:

- `structurally_well_formed_candidate: true`;
- `guardian_evidence_unique: true`;
- `matches_expected_current_identity_binding: true`.

Those booleans are program-id-match placeholders from Phase 41D3.2.3.

Phase 41E must independently parse real Ed25519 instruction bytes.

## Demon Note 1 — Cross-Instruction Index References

Ed25519 signature offset metadata may include references such as:

- signature instruction index;
- public key instruction index;
- message instruction index.

Accepted Phase 41E code boundary:

- parse only bytes of the located Ed25519 instruction;
- reject any non-self instruction-index reference;
- do not load referenced instructions;
- do not introduce a new instruction-loading surface.

Any future support for referenced instruction loading must be a separate reviewed gate.

## Demon Note 2 — Variable-Length Message Range

Signature and public key byte lengths are fixed:

- signature: 64 bytes;
- public key: 32 bytes.

Message length is input-controlled.

Accepted Phase 41E code boundary:

- represent message data as checked bounded indices, for example `(message_offset, message_len)`;
- do not copy attacker-sized message bytes into a new `Vec`;
- avoid attacker-controlled allocation;
- use checked offset arithmetic before exposing the bounded range.

## Demon Note 3 — Overlap Policy

Future Phase 41E code must choose a deterministic overlap policy.

Recommended strict parser policy:

- reject overlapping parsed byte ranges.

If overlap is ever allowed, it must be explicitly justified in a separate reviewed boundary.

The Phase 41E byte parser must not leave overlap handling ambiguous.

## Parsing-Specific Flag Guardrail

A future parsing-specific flag may be introduced only if it means byte parsing occurred, for example:

- `parses_ed25519_instruction_bytes: true`.

It must not mean:

- signature verified;
- proof accepted;
- evidence accepted;
- guardian accepted;
- quorum reached;
- execution authorized;
- replay writable;
- state mutable;
- CPI/mint/live route enabled.

## Still Forbidden

The following remain forbidden after Phase 41E.0 acceptance:

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

No blocker is removed, weakened, or reinterpreted by Phase 41E.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

A narrow Phase 41E byte parsing code boundary may begin after this acceptance record is merged.

That code boundary remains gated under separate external review before merge.

Required code focus:

- entry gate uses located status plus matched instruction index;
- no gate on `locates_prior_ed25519_instruction`;
- no trust in Phase 41D3.2.3 descriptor booleans;
- cross-instruction references fail closed;
- no referenced-instruction loading;
- message range stored as bounded indices, not attacker-sized copy;
- deterministic overlap policy, preferably reject overlap;
- checked offset arithmetic for every parsed range;
- no parsed field is treated as proof/evidence/auth.

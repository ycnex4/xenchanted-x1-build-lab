# XXXL Phase 41E.0 — Ed25519 Instruction Byte Parsing Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41E.0.

## Parent Gate

Phase 41D3 is closed.

Closing checkpoint:

`99ba836 Merge XXXL phase 41D3 structural prior lookup closure`

Accepted structural pipeline:

- Phase 41D1 — AccountInfo presence/readability;
- Phase 41D2 — current instruction identity;
- Phase 41D3.1 — current instruction index acquisition;
- Phase 41D3.2.1 — bounded prior index range;
- Phase 41D3.2.2 — checked prior instruction loading;
- Phase 41D3.2.3 — prefilter + Phase 41C3 candidate descriptors.

## Purpose

Phase 41E.0 plans the first boundary after structural prior-instruction lookup:

Ed25519 instruction byte parsing.

This phase must define how a future code step may inspect the real bytes of a structurally located prior Ed25519 instruction without accepting proof, verifying signature, counting quorum, authorizing execution, mutating state, or enabling CPI/mint/live route.

## Core Boundary

Phase 41E is allowed to become a byte parsing boundary only.

It may parse candidate instruction data into non-authorizing parsed fields.

It must not verify or trust those fields.

## Required Entry Gate

Future Phase 41E code must not gate on:

- `locates_prior_ed25519_instruction`.

Future Phase 41E code may continue only when both are true:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

This guardrail comes from Phase 41D3 external review.

## Descriptor Boolean Guardrail

Future Phase 41E code must not trust Phase 41D3.2.3 descriptor booleans as validated evidence:

- `structurally_well_formed_candidate: true`;
- `guardian_evidence_unique: true`;
- `matches_expected_current_identity_binding: true`.

Those booleans are program-id-match placeholders from Phase 41D3.2.3.

Phase 41E must independently parse real Ed25519 instruction bytes.

## Allowed Future Parsing Scope

A future Phase 41E code step may parse only structural Ed25519 instruction fields, such as:

- instruction data length;
- signature instruction header shape;
- signature count;
- signature offset metadata;
- public key offset metadata;
- message offset metadata;
- instruction-index references encoded inside Ed25519 metadata;
- bounded byte slices for signature/public key/message locations;
- malformed layout status;
- out-of-bounds offset status;
- unsupported signature count status;
- duplicate/ambiguous structural layout status.

The parsed output must be non-authorizing.

## Explicitly Not Accepted As Evidence

Parsed fields must not mean:

- signature is valid;
- signer is a valid guardian;
- message hash is accepted;
- source event is final;
- proof is accepted;
- quorum is reached;
- mint is authorized;
- replay registry can be written;
- account state can mutate.

## Future Parsed Result Meaning

A parsed Ed25519 instruction result may mean only:

- bytes were read from a previously located Ed25519 program-id instruction;
- the byte layout was parsed deterministically;
- structural fields were extracted or rejected;
- malformed data was classified deterministically;
- no trust was conferred.

## Fail-Closed Requirements

Future Phase 41E code must fail closed on:

- no matched instruction index;
- matched instruction missing from loaded prior set;
- wrong program id;
- empty data;
- unsupported signature count;
- malformed header;
- out-of-bounds offsets;
- overlapping offsets if considered unsafe;
- offset references to unexpected instruction indexes;
- message byte range missing;
- public key byte range missing;
- signature byte range missing;
- ambiguous parsed candidate;
- duplicate parsed candidate;
- any integer overflow risk;
- any unchecked slice/index need.

Failure must be deterministic and non-authorizing.

## Panic-Safety Requirements

Future Phase 41E code must avoid:

- `unwrap`;
- `expect`;
- `panic!`;
- `unsafe`;
- unchecked indexing;
- unchecked slicing;
- unbounded allocation;
- trusting offset arithmetic without checked math.

Required style:

- checked offset arithmetic;
- `.get(...)` or equivalent checked access;
- bounded lengths;
- explicit status mapping;
- deterministic rejection statuses.

## Heap / Allocation Requirements

Future Phase 41E code should:

- avoid cloning full `Instruction`;
- avoid cloning full data unless bounded and justified;
- store only parsed metadata and bounded byte ranges;
- avoid holding multiple full copies of instruction data;
- prefer references or bounded copied arrays when exact byte lengths are known.

## Trust-Sensitive Flags

Phase 41E may introduce a parsing-specific flag only after review, for example:

- `parses_ed25519_instruction_bytes: true`.

This flag must mean only byte parsing occurred.

It must not mean proof/evidence/verifier/auth.

Must remain false:

- `ed25519_signature_verification_performed`;
- `cryptographic_signature_proof_accepted`;
- `verification_evidence_accepted`;
- `quorum_counting_enabled`;
- `authorization_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

## Proposed Status Model

A future Phase 41E code step should use explicit non-authorizing statuses, such as:

- `PriorEd25519InstructionNotStructurallyLocated`;
- `MatchedInstructionUnavailable`;
- `WrongEd25519ProgramId`;
- `EmptyInstructionData`;
- `MalformedEd25519InstructionHeader`;
- `UnsupportedSignatureCount`;
- `OutOfBoundsSignatureOffset`;
- `OutOfBoundsPublicKeyOffset`;
- `OutOfBoundsMessageOffset`;
- `UnexpectedInstructionIndexReference`;
- `AmbiguousParsedInstructionData`;
- `DuplicateParsedInstructionData`;
- `Ed25519InstructionBytesParsed`.

Final names may change during implementation, but the status model must stay deterministic and non-authorizing.

## Review Questions Before Code

External review should confirm:

1. Is byte parsing an acceptable next micro-phase after Phase 41D3 closure?
2. Is the entry gate correct: located status plus matched instruction index?
3. Is it clear that `locates_prior_ed25519_instruction` is not an evidence gate?
4. Is it clear that Phase 41D3 descriptor booleans are not validated evidence?
5. Is parsing real Ed25519 instruction bytes allowed without verification?
6. Are all parsed fields non-authorizing?
7. Are malformed/out-of-bounds/ambiguous cases fail-closed?
8. Are panic-safety requirements sufficient?
9. Are heap/allocation requirements sufficient?
10. Is a parsing-specific flag acceptable if it does not imply proof/evidence/auth?
11. Are cryptographic verification/proof/evidence/quorum/auth/replay/mutation/CPI/mint/live-route still closed?
12. Can Phase 41E byte parsing code begin after this plan is accepted?

## Still Forbidden

Phase 41E.0 does not permit:

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

If this plan is externally accepted, the next step may be a narrow Phase 41E byte parsing code boundary.

That code boundary must remain non-authorizing and must require its own review before merge.

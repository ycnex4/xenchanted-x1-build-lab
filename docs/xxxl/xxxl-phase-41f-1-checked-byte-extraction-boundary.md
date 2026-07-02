# XXXL Phase 41F.1 — Checked Ed25519 Byte Extraction Boundary

Date: 2026-07-02

## Status

Implemented as a narrow runtime-model boundary.

## Parent Gate

Phase 41F.0 external acceptance:

`e45869c Merge XXXL phase 41F Ed25519 verification plan acceptance record`

## Purpose

Phase 41F.1 extracts already parsed Ed25519 byte ranges as checked borrowed references.

It does not perform cryptographic verification.

It does not establish native Ed25519 verification.

It does not accept proof, evidence, guardian validity, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.

## Consumed Inputs

Phase 41F.1 consumes:

- `Phase41D3_2_2CheckedPriorInstructionLoadingResult`;
- `Phase41E_1Ed25519ByteParsingResult`.

It requires Phase 41E status:

- `Ed25519InstructionBytesParsed`.

It requires:

- matched instruction index exists;
- parsed offsets exist;
- matching loaded prior instruction exists;
- loaded entry remains runtime-data-only;
- loaded data length matches the Phase 41E parse result.

## Extracted References

Phase 41F.1 extracts:

- signature bytes as `&[u8; 64]`;
- public key bytes as `&[u8; 32]`;
- message bytes as borrowed `&[u8]`.

The message is not copied into a `Vec`.

The message remains borrowed from the already loaded instruction data.

## Checked Access Rule

All extraction must use checked range access.

Allowed:

- `data.get(range.offset..end)`;
- checked offset arithmetic;
- array reference conversion only after checked slice length validation.

Forbidden:

- unchecked indexing;
- unchecked slicing;
- `unwrap`;
- `expect`;
- `panic!`;
- `unsafe`;
- attacker-sized message copy.

## Model A Soundness Guardrail

Phase 41F.1 does not yet establish Model A native Ed25519 verification.

However, it preserves the data needed for a future Model A boundary.

Future Model A documentation must state:

- the prior native Ed25519 instruction already verified successfully because the current instruction was reached;
- otherwise the transaction would have aborted before the current instruction.

Phase 41F.1 only prepares checked byte references for that future gate.

## Self-Reference Guardrail

Phase 41F.1 relies on Phase 41E's already accepted invariant:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

Phase 41F.1 does not add support for cross-instruction references.

## Not Signature Verification

Phase 41F.1 must not flip:

- `ed25519_signature_verification_performed`.

Phase 41F.1 must not introduce:

- local crypto verification;
- native verification establishment;
- invalid signature judgment;
- proof acceptance;
- evidence acceptance.

## Not Message Correctness

Extracting message bytes does not mean:

- message hash matches expected payload hash;
- route binding is correct;
- mint binding is correct;
- recipient binding is correct;
- amount binding is correct;
- finality/expiration binding is correct.

All of these remain later gates.

## Not Guardian Validity

Extracting public key bytes does not mean:

- public key is an active guardian;
- guardian set membership is accepted;
- quorum is counted.

All of these remain later gates.

## Failure Modes

Phase 41F.1 fails closed on:

- Ed25519 bytes not parsed;
- matched instruction index missing;
- matched instruction unavailable;
- loaded entry not runtime-data-only;
- parsed offsets missing;
- instruction data length mismatch;
- checked signature slice unavailable;
- checked public key slice unavailable;
- checked message slice unavailable.

All failures are non-authorizing.

## Tests

Phase 41F.1 tests cover:

- successful checked extraction of signature/public-key/message references;
- rejection when Phase 41E bytes were not parsed;
- missing matched instruction index;
- missing parsed offsets;
- matched instruction unavailable;
- non-runtime-data loaded entry;
- instruction data length mismatch;
- unavailable signature slice;
- unavailable public key slice;
- unavailable message slice;
- report flags preserving extraction-only/non-authorizing boundary.

## Still Forbidden

The following remain forbidden:

- Ed25519 cryptographic verification;
- local cryptographic verification;
- native Ed25519 verification establishment;
- signature validity acceptance;
- proof acceptance;
- verification evidence acceptance;
- guardian validity acceptance;
- guardian set membership acceptance;
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

No blocker is removed, weakened, or reinterpreted by Phase 41F.1.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

After external acceptance of Phase 41F.1, the next likely step is:

Phase 41F.2 — Ed25519 signature verification boundary plan.

Phase 41F.2 must be reviewed separately.

Phase 41F.2 must attribute statuses to Model A or Model B and must preserve the distinction between signature validity and proof/evidence/guardian/quorum/auth.

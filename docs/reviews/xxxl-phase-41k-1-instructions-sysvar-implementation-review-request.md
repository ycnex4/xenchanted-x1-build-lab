# XXXL Phase 41K.1 — Instructions Sysvar Live-Wiring Implementation Review Request

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-1-instructions-sysvar-implementation`

Implementation commit:

`01405b7 Implement phase 41K.1 instructions sysvar live-wiring boundary`

Base main:

`6f88800 Merge XXXL phase 41K.1 instructions sysvar plan acceptance`

## Scope

Phase 41K.1 code implementation review.

This slice introduces a runtime-loading boundary for real Instructions sysvar live-wiring.

It does not introduce a process instruction handler.

It does not enable guardian PDA loading, processed-registry PDA loading, replay writes, CPI, mint, or live route.

## What Was Implemented

New module:

`programs/xxxl-svm/src/verifier/instructions_sysvar_live_wiring_boundary.rs`

Exports added in:

`programs/xxxl-svm/src/verifier/mod.rs`

The new boundary composes existing accepted lower runtime boundaries:

- checked current instruction index acquisition;
- strict prior index range construction;
- checked prior instruction loading;
- Ed25519 precompile program-id filtering;
- N prior Ed25519 precompile enumeration.

## Intended Runtime Chain

`real Instructions sysvar`
→ `load_current_index_checked`
→ strict prior index range
→ `load_instruction_at_checked` for each prior index
→ filter real Ed25519 precompile instructions
→ expose N prior Ed25519 precompile entries for later 41F.1 / 41F.2 / 41I composition

## Review Focus

Please verify that 41K.1 implementation preserves the accepted plan:

1. Current instruction index is not caller-provided.
2. Current instruction index comes through checked runtime loading.
3. Prior instructions are loaded through checked runtime loading.
4. Prior instruction indexes remain strictly less than current instruction index.
5. N prior Ed25519 precompile instructions are supported for quorum.
6. Non-Ed25519 prior instructions are discarded, not accepted as evidence.
7. Ed25519 program id is checked.
8. Caller-provided instruction bytes are not accepted.
9. Frontend/watcher Ed25519 proofs are not accepted as authority.
10. Model A applies per prior Ed25519 precompile instruction.
11. No guardian-set PDA loading is enabled.
12. No processed-registry PDA loading is enabled.
13. No replay write / processed event marking is enabled.
14. No mutation / CPI / mint / handler / live route is enabled.
15. Tests cover the runtime surface and safety flags sufficiently for this slice.

## Test Status

Full xxxl-svm test suite passed locally:

`full-tests: OK`

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.1 implementation acceptance:

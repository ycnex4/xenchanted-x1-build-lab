# XXXL Phase 41K.1 — Instructions Sysvar Live-Wiring Implementation Acceptance

Date: 2026-07-03

Status: accepted implementation

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-1-instructions-sysvar-implementation`

Base main:

`6f88800 Merge XXXL phase 41K.1 instructions sysvar plan acceptance`

Accepted commits:

- `01405b7 Implement phase 41K.1 instructions sysvar live-wiring boundary`
- `c3d9101 Document phase 41K.1 instructions sysvar implementation review`
- `4ff8c8a Address phase 41K.1 implementation review notes`

## Final Verdict

Phase 41K.1 Instructions sysvar live-wiring implementation is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Sufficient before 41K.1 implementation acceptance: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Sufficient before 41K.1 implementation acceptance: yes

## Notes Addressed

Demon notes were addressed in code before acceptance:

- clarified that 41K.1 preserves the sysvar-input side of Model A;
- clarified that full handler execution-context enforcement belongs to 41K.5;
- added explicit flags for 41K.5 handler-context requirement;
- added direct AccountInfo entry tests for missing Instructions sysvar;
- added direct AccountInfo entry tests for wrong Instructions sysvar account key.

## Accepted Implementation

New module:

`programs/xxxl-svm/src/verifier/instructions_sysvar_live_wiring_boundary.rs`

Exports:

`programs/xxxl-svm/src/verifier/mod.rs`

The accepted 41K.1 boundary composes existing checked runtime primitives:

- checked current instruction index acquisition;
- strict prior instruction index range construction;
- checked prior instruction loading;
- Ed25519 precompile program-id filtering;
- N prior Ed25519 precompile enumeration.

## Test Status

Focused tests passed.

Full xxxl-svm test suite passed after review-note patch.

## Still Disabled

41K.1 does not enable:

- guardian-set PDA loading;
- processed-registry PDA loading;
- replay write;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route.

## Next Gate

41K.2 real guardian-set account/PDA loading.

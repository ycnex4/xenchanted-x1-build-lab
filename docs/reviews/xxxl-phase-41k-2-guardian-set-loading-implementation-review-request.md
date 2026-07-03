# XXXL Phase 41K.2 — Guardian-Set Account Loading Implementation Review Request

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-implementation`

Implementation commits:

`cedbff5 Add phase 41K.2 guardian-set decoder boundary`

`9971efe Add phase 41K.2 guardian-set account loader`

Base main:

`1ac03a3 Merge XXXL phase 41K.2 guardian-set loading plan acceptance`

## Scope

Phase 41K.2 code implementation review.

This slice introduces a runtime-loading boundary for real guardian-set account loading.

It replaces the abstract guardian-set input path with a checked program-controlled on-chain guardian-set account path:

`real guardian-set AccountInfo`
→ non-signer / read-only account precondition checks
→ expected program owner check
→ expected guardian-set PDA check
→ checked guardian-set account data decode
→ structured loading result

It does not construct the final authoritative guardian-set wrapper yet.

It does not enable processed-registry PDA loading, replay writes, processed event marking, atomic check-mark-mint, CPI, SPL mint, process instruction handler, or live route.

## What Was Implemented

New module:

`programs/xxxl-svm/src/verifier/guardian_set_account_loading_boundary.rs`

Exports added in:

`programs/xxxl-svm/src/verifier/mod.rs`

Implementation split:

1. Pure guardian-set account data decoder.
2. Real AccountInfo / PDA / owner loader boundary.

## Guardian-Set Account Data Checks

The decoder validates:

- account data length;
- missing discriminator;
- zero discriminator;
- wrong discriminator;
- runtime layout version;
- active guardian-set status;
- threshold > 0;
- guardian_count > 0;
- guardian_count <= max supported guardian count;
- threshold <= guardian_count;
- stored guardian_set_id;
- stored guardian_set_id equals expected guardian_set_id;
- guardian public key extraction through checked offsets;
- duplicate guardian public keys rejected.

## Runtime Account Loading Checks

The AccountInfo loader validates:

- guardian-set account is present;
- guardian-set account is not signer;
- guardian-set account is not writable;
- guardian-set account owner equals expected XXXL program id;
- guardian-set account key equals expected PDA;
- PDA is derived from fixed seed format:

`["xxxl", "guardian-set", guardian_set_id]`

The loader checks account identity and owner before trusting account data.

Tests explicitly mutate account data to zero-discriminator while also using wrong owner / wrong PDA to ensure owner/PDA rejection happens before data trust.

## Intended Runtime Chain

`real guardian-set AccountInfo`
→ expected PDA derivation
→ account presence check
→ non-signer check
→ read-only check
→ owner check
→ PDA key check
→ checked data borrow
→ checked data decode
→ structured program-controlled on-chain loading result

## Explicitly Still Disabled

This slice keeps the following disabled:

- processed-registry runtime loading;
- replay write;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- process instruction handler;
- live route.

## Review Focus

Please verify that 41K.2 implementation preserves the accepted plan:

1. Guardian-set input is no longer a free caller-provided guardian list.
2. Guardian-set source is represented as program-controlled on-chain account loading.
3. AccountInfo is used only for guardian-set loading in this slice.
4. Account owner is checked before account data is trusted.
5. PDA identity is checked before account data is trusted.
6. PDA seed format is fixed and deterministic.
7. Stored guardian_set_id must match the expected guardian_set_id.
8. Zero/uninitialized discriminator is explicitly rejected.
9. Inactive/deprecated guardian-set status is rejected.
10. Empty guardian set is rejected.
11. Invalid threshold is rejected.
12. Threshold above guardian_count is rejected.
13. Duplicate guardian public keys are rejected.
14. No unchecked production slicing / unwrap / expect is used in the production decoder path.
15. No processed-registry PDA loading is enabled.
16. No replay write / processed event marking is enabled.
17. No mutation / CPI / mint / handler / live route is enabled.
18. Tests cover the account loading surface and safety flags sufficiently for this slice.

## Test Status

Focused guardian-set account loading tests passed locally:

`guardian_set_account_loading_boundary: OK`

Full xxxl-svm test suite passed locally after commit:

`after-commit-full-xxxl-svm-tests: OK`

Additional local checks:

`production-safety-scan: OK`

`git diff --check: OK`

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.2 implementation acceptance:

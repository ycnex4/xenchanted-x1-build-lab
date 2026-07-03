# XXXL X1 Testnet Local Runtime Skeleton — Phase 41K.2 Guardian-Set Loading Implementation Checkpoint

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-implementation`

Base main:

`1ac03a3 Merge XXXL phase 41K.2 guardian-set loading plan acceptance`

Implementation commits:

`cedbff5 Add phase 41K.2 guardian-set decoder boundary`

`9971efe Add phase 41K.2 guardian-set account loader`

## Status

41K.2 code implementation slice created.

Full xxxl-svm tests passed.

## Implemented

- Guardian-set account loading boundary module.
- Pure guardian-set account data decoder.
- Runtime AccountInfo loading boundary.
- Fixed guardian-set PDA derivation helper.
- Account presence check.
- Non-signer check.
- Read-only check.
- Owner check.
- PDA key check.
- Checked account data borrow.
- Checked schema/data decode.
- Stored guardian_set_id match against expected guardian_set_id.
- Zero discriminator rejection.
- Wrong discriminator rejection.
- Unsupported layout version rejection.
- Inactive/deprecated guardian-set rejection.
- Empty guardian-set rejection.
- Invalid threshold rejection.
- Threshold greater than guardian_count rejection.
- Guardian count above max-supported rejection.
- Duplicate guardian public key rejection.
- Safety flags showing only guardian-set runtime loading is enabled.

## PDA Seed Format

Guardian-set PDA seed format fixed for this slice:

`["xxxl", "guardian-set", guardian_set_id]`

## Runtime Boundary

Implemented runtime chain:

`real guardian-set AccountInfo`
→ expected PDA derivation
→ account presence check
→ non-signer check
→ read-only check
→ owner check
→ PDA key check
→ checked data borrow
→ checked data decode
→ structured loading result

## Still Disabled

- processed-registry PDA loading;
- replay write;
- processed event marking;
- atomic check-mark-mint;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route;
- guardian-set governance;
- guardian-set update instruction;
- production guardian-set deployment.

## Test Status

Focused guardian-set account loading tests:

`guardian_set_account_loading_boundary: OK`

Full xxxl-svm test suite after commit:

`after-commit-full-xxxl-svm-tests: OK`

Additional local checks:

`production-safety-scan: OK`

`git diff --check: OK`

## Next

Submit 41K.2 implementation to Theo / Demon review.

After review acceptance, update current checkpoint and proceed to 41K.3 processed-registry PDA loading.

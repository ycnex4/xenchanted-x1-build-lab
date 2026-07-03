# XXXL X1 Testnet Local Runtime Skeleton — Phase 41K.3 Processed-Registry Loading Plan Checkpoint

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Base main:

`d983663 Merge XXXL phase 41K.2 guardian-set loading implementation acceptance`

## Status

41K.3 processed-registry PDA loading plan prepared for review.

This is a planning checkpoint, not implementation.

## Documents

Plan:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`

Review request:

`docs/reviews/xxxl-phase-41k-3-processed-registry-loading-plan-review-request.md`

## Current Architecture Position

Accepted chain so far:

- 41F.1 checked Ed25519 byte extraction;
- 41F.2 native SVM Ed25519 verification;
- 41G payload hash binding;
- 41H guardian membership validation;
- 41H.1 decoded payload binding hardening;
- 41H.2 signed message binding hardening;
- 41I quorum counting / threshold authorization;
- 41J replay protection / processed marking intent;
- 41K.1 real Instructions sysvar loading;
- 41K.2 real guardian-set AccountInfo / PDA loading.

Next gate:

41K.3 real processed-registry PDA loading.

## Proposed 41K.3 Boundary

`real processed-event AccountInfo`
→ expected processed-event PDA derivation
→ account presence policy
→ non-signer / read-only checks
→ owner check before data trust
→ PDA key check before data trust
→ checked data borrow
→ checked processed-event data decode
→ consumed / unconsumed read state
→ structured program-controlled on-chain loading result

## Proposed PDA Seed Format

`["xxxl", "processed-event", canonical_event_key]`

## Open Review Questions

Before code, reviewers should decide:

1. Is the proposed PDA seed format correct?
2. Should missing processed-event PDA reject in 41K.3 or represent an unprocessed read state for a later creation phase?
3. Should 41K.3 stay read-only / non-writable?
4. Is `consumed == true` the correct already-processed signal?
5. Should successful 41K.3 loading later convert to `AuthoritativeProcessedRegistryViewRef` through one type-enforced adapter?

## Explicitly Still Disabled

The 41K.3 plan keeps disabled:

- write path;
- processed event marking;
- account mutation;
- atomic check-mark-mint;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route.

## Next

Submit 41K.3 plan to Theo / Demon review.

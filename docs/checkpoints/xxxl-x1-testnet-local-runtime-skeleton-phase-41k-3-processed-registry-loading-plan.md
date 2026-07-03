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


## Amendment 1 Update

Amendment document:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`

Amended review request:

`docs/reviews/xxxl-phase-41k-3-processed-registry-loading-plan-amended-review-request.md`

Amendment 1 supersedes conflicting wording in the base plan.

Key changes:

- missing / uninitialized expected processed-event PDA means unprocessed, not replay rejection;
- writable processed-event account must not be rejected solely for writability;
- initialized `consumed == false` lifecycle must be explicitly accepted or rejected before code;
- 41J list-based registry view must be reconciled with per-event PDA lookup;
- successful 41K.3 loading must later feed an authoritative processed-registry view through a type-enforced adapter or explicit 41J interface refinement.

Updated review target:

Reviewers should review the base plan together with Amendment 1.


## Amendment 2 Update

Amendment document:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`

Review request:

`docs/reviews/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2-review-request.md`

Amendment 2 resolves the two blocking issues from Theo and Demon review of Amendment 1:

1. `consumed == false` lifecycle;
2. 41J list-based registry view vs per-event PDA point-lookup reconciliation.

Decisions:

- missing `AccountInfo` is rejected;
- supplied expected PDA in accepted uninitialized runtime representation means unprocessed / eligible;
- initialized `consumed == true` means already processed;
- initialized `consumed == false` is rejected as invalid lifecycle state;
- writable account is allowed but not mutated in 41K.3;
- 41J reconciliation uses Option A: internal type-enforced adapter to existing 41J list-based interface;
- existing `mark_processed_event_consumed(...)` is not accepted as live semantics without later 41K.4 review;
- 41K.4 must bind marked amount / mint amount to the quorum-authorized payload;
- rent / close / recreate lifecycle risks are documented and carried forward;
- active deployment blockers remain unchanged.

Active deployment blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`;
- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`;
- `PRODUCTION_GUARDIAN_SET_UNSET`;
- `PRODUCTION_PROOF_LOG_UNSET`;
- `SPL_CPI_EXECUTION_DISABLED`;
- `LIVE_ROUTE_DISABLED`;
- `EXTERNAL_REVIEW_INCOMPLETE`.


## Amendment 3 Update

Amendment document:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-3.md`

Review request:

`docs/reviews/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-3-review-request.md`

Amendment 3 addresses Claude hostile audit findings:

- canonical bump-only PDA derivation;
- caller-supplied bump never trusted;
- exact uninitialized expected PDA representation;
- lamports ignored for uninitialized classification to avoid lamport-dusting DoS;
- XXXL-owned zero/wrong discriminator invalid, not unprocessed;
- canonical_event_key sufficiency stated as canonical source-event identity from accepted payload binding;
- 41K.4 atomic create/init/consume invariant promoted to required forward invariant;
- Option A adapter assumptions made explicit;
- adapter construction must be internal and type-enforced.

41K.3 remains read/loading/classification only.

No write / mark / mutation / CPI / mint / handler / live route surfaces are enabled.


## Amendment 4 Cleanup

Cleanup document:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-4-cleanup.md`

Purpose:

Record non-blocking Claude hostile audit notes after Amendment 3.

Status:

`ACCEPT WITH NOTES`

Required fixes:

None.

Cleanup items:

- withdraw old writable-rejection test from the base plan;
- replace it with writable-allowed-but-not-mutated test;
- preserve panic-safety test for no unchecked slicing / `unwrap` / `expect`;
- require total fail-closed account classification;
- explicitly reject system-owned nonzero-data expected PDA;
- apply signer / executable rejection to all states;
- strengthen type-enforcement language from handler discipline / broad `pub(crate)` to a private-field witness/newtype construction pattern;
- carry lamport-dusted atomic initialization proof forward to 41K.4.

This cleanup does not reopen 41K.3 architecture.

It prepares the plan for final acceptance recording.

# XXXL Phase 41K.3 — Processed-Registry PDA Loading Plan

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Base main:

`d983663 Merge XXXL phase 41K.2 guardian-set loading implementation acceptance`

## Status

Planning slice for 41K.3.

This phase is not implementation yet.

## Purpose

Phase 41K.3 introduces a real runtime loading boundary for processed-event / processed-registry PDA accounts.

The purpose is to replace the 41J abstract processed-registry view with a real program-controlled on-chain account loading path, while preserving the 41J invariant:

`raw payload -> internal 41I quorum authorization -> internal payload decode -> canonicalEventKey -> authoritative processed-registry view -> replay eligibility / marking intent`

41K.3 must remain a read/loading boundary.

It must not write replay state and must not mark events as processed.

## Current Accepted Context

41J currently requires an `AuthoritativeProcessedRegistryViewRef` and rejects caller-supplied or unauthenticated processed-registry views.

41J currently returns replay eligibility and processed-marking intent only.

41J keeps disabled:

- runtime account loading;
- replay write;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route.

41K.2 introduced real guardian-set AccountInfo / PDA loading, and was accepted with notes.

The carried 41K.2 note remains active: before 41K.5 live-handler wiring, successful 41K.2 guardian-set loading must feed `AuthoritativeGuardianSetRef` through a single type-enforced adapter.

## Existing Processed Event Runtime Layout

The current runtime state defines:

- `PROCESSED_EVENT_ACCOUNT_LEN = 144`;
- `PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR`;
- `ProcessedEventAccountView`;
- `consumed()` at byte offset 10;
- `canonical_event_key()` at byte offset 16;
- `route_id()` at byte offset 48;
- `recipient()` at byte offset 80;
- `consumed_amount()` at byte offset 112.

The current runtime state also includes `mark_processed_event_consumed(...)`, but that mutation is out of scope for 41K.3.

## Proposed PDA Seed Format

41K.3 should fix the processed-event PDA seed format before code acceptance.

Proposed seed format:

`["xxxl", "processed-event", canonical_event_key]`

Rationale:

- `canonical_event_key` is already the 41J replay key;
- a processed-event PDA should be uniquely bound to one Ethereum burn event;
- it avoids free replay keys, free decoded payload, or message nonce replay keys;
- it keeps replay protection bound to the canonical payload-derived event key.

This seed format must be reviewed before 41K.3 code implementation.

## Account Loading Boundary

The future 41K.3 loader should accept:

- optional real processed-event `AccountInfo`;
- expected XXXL program id;
- expected canonical_event_key derived internally from raw payload;
- expected route_id derived internally from raw payload;
- expected recipient derived internally from raw payload.

The loader must derive the expected processed-event PDA from the fixed seed format and expected canonical_event_key.

The loader must reject:

- missing account when the selected model requires a pre-existing processed-event PDA;
- signer processed-event account;
- writable processed-event account in the read-only loading slice;
- wrong owner;
- wrong PDA;
- failed data borrow;
- invalid data length;
- missing discriminator;
- zero discriminator;
- wrong discriminator;
- unsupported schema version;
- stored canonical_event_key mismatch;
- stored route_id mismatch;
- stored recipient mismatch.

The loader must expose:

- consumed status;
- canonical_event_key;
- route_id;
- recipient;
- consumed_amount;
- account key;
- expected account key;
- account owner;
- expected program id;
- PDA bump;
- check-progress flags;
- source marker: program-controlled on-chain.

## Replay Eligibility Semantics

41K.3 should only establish an authoritative runtime view of whether the processed-event account is already consumed.

It should not mutate account data.

A successful 41K.3 load should allow later composition with 41J replay eligibility:

- if `consumed == false`, replay check may pass and processed marking intent may be produced;
- if `consumed == true`, replay check must reject as already processed;
- no write occurs in 41K.3.

The conversion from successful 41K.3 processed-event loading to `AuthoritativeProcessedRegistryViewRef` should be type-enforced through a single adapter before live handler wiring.

The adapter must not be based on handler discipline or externally constructible result structs.

## Explicitly Out of Scope

41K.3 must not implement:

- processed-event account creation;
- replay write;
- processed event marking;
- `mark_processed_event_consumed(...)` invocation;
- account mutation;
- atomic check-mark-mint;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route.

## Safety Flags

41K.3 may enable only:

`processed_registry_runtime_loading_enabled: true`

41K.3 must keep disabled:

- `replay_write_enabled: false`;
- `processed_event_marking_enabled: false`;
- `account_mutation_enabled: false`;
- `cpi_enabled: false`;
- `invoke_signed_enabled: false`;
- `spl_token_mint_to_enabled: false`;
- `process_instruction_handler_added: false`;
- `live_route_enabled: false`.

## Required Tests For 41K.3 Code

Future implementation tests should cover:

1. valid processed-event AccountInfo / PDA / owner / schema load;
2. missing account rejection;
3. signer rejection;
4. writable rejection;
5. wrong owner rejection before data trust;
6. wrong PDA rejection before data trust;
7. borrow failure rejection;
8. invalid account length rejection;
9. zero discriminator rejection;
10. wrong discriminator rejection;
11. unsupported schema version rejection;
12. canonical_event_key mismatch rejection;
13. route_id mismatch rejection;
14. recipient mismatch rejection;
15. consumed=false loaded as eligible read state;
16. consumed=true loaded as already-processed read state;
17. no replay write enabled;
18. no processed marking enabled;
19. no mutation / CPI / mint / handler / live route enabled;
20. production decoder path has no unchecked slicing / unwrap / expect.

## Review Questions

Theo / Demon should verify:

1. Is `["xxxl", "processed-event", canonical_event_key]` the correct seed format for 41K.3?
2. Should 41K.3 require a pre-existing processed-event PDA, or should absence be represented as a valid unprocessed state for future 41K.4 account creation?
3. Should 41K.3 remain read-only, non-writable AccountInfo loading, with all writes deferred to 41K.4/41K.5?
4. Is `consumed == true` the correct runtime signal for already-processed replay rejection?
5. Should successful 41K.3 loading be converted to `AuthoritativeProcessedRegistryViewRef` through a single type-enforced adapter before 41K.5?
6. Are any additional fields required for processed-event PDA identity beyond canonical_event_key / route_id / recipient?

## Acceptance Criteria For This Plan

The plan is acceptable if reviewers agree that:

- 41K.3 is a read/loading boundary only;
- processed-event PDA identity is fixed and deterministic;
- account owner/PDA checks happen before account data trust;
- replay writes remain disabled;
- processed event marking remains disabled;
- atomic check-mark-mint remains deferred;
- the future authoritative processed-registry adapter is explicitly required before live handler wiring.

## Next After Plan Acceptance

After this plan is accepted:

1. implement `processed_registry_account_loading_boundary.rs`;
2. export it from `programs/xxxl-svm/src/verifier/mod.rs`;
3. add focused tests;
4. run full xxxl-svm tests;
5. submit 41K.3 code for review.

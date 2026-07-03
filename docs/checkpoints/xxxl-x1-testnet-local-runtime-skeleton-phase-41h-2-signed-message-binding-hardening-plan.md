# XXXL X1 Testnet Local Runtime Skeleton — Phase 41H.2 Signed Message Binding Hardening Plan Checkpoint

Date: 2026-07-03

Status: planning only

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41h-2-signed-message-binding-hardening`

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Primary plan:

`docs/xxxl/xxxl-phase-41h-2-signed-message-binding-hardening-plan.md`

## Purpose

This checkpoint records the Phase 41H.2 signed-message binding hardening plan.

41H.2 closes the gap found during the Phase 41I high-risk audit:

`41F-verified message bytes` must be the same bytes used by 41G as the signed payload hash.

## Required Closure

41H must stop accepting free `signed_message_bytes`.

41H must derive the signed message from:

`extraction_result.extracted_slices.message_bytes`

Then 41H must call 41G with:

`establish_payload_hash_binding(raw_payload_bytes, extracted_slices.message_bytes, phase_41f_result)`

## Downstream Impact

Phase 41I remains blocked until 41H.2 is implemented, reviewed, and accepted.

## Still Forbidden

41H.2 planning does not introduce:

- quorum counting;
- authorization marker;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- instruction handler;
- live route.

## Next Step

Request Theo and Audit Demon review for the 41H.2 planning document before writing any 41H.2 code.

## High-Risk Audit Update

Audit Demon required one blocking correction to the plan:

41H.2 must bind `verified_ranges.message_range` to `extracted_slices.message_range`.

The plan now also requires binding the full Ed25519 operand range set:

- public key range;
- message range;
- signature range.

Phase 41I remains blocked until this corrected 41H.2 plan is reviewed and the later 41H.2 code is implemented and accepted.

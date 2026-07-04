# Phase 41K.6 B1C.6 — Quorum Counting Spec

Status: planning checkpoint
Branch: stage-41k6-b1c6-quorum-counting-spec
Base: main after B1C.5 merge

## Purpose

B1C.6 deduplicates validated guardian signers and checks whether the number of unique guardians meets the threshold from the authoritative B1B guardian set.

## Input

B1C.6 consumes:

- B1C.5 validated signers
- B1B guardian set threshold
- B1B guardian set id

## Core rule

Validated signers may contain duplicates.

B1C.6 counts each guardian once.

If unique_guardian_count >= threshold, quorum is met.

If unique_guardian_count < threshold, quorum is not met.

## Success meaning

B1C.6 success means:

- evidence was payload-bound in B1C.4
- signers were guardian members in B1C.5
- enough unique guardians signed

B1C.6 still does not authorize handler execution.

Authorization becomes usable only when B1C.7 integrates the full chain before mark+mint.

## Failure policy

Reject if:

- no validated signers
- guardian set id is missing or mismatched
- threshold is zero
- threshold is greater than guardian count
- unique guardian count is below threshold

## Non-goals

B1C.6 does not load guardian set accounts.

B1C.6 does not parse Ed25519 instructions.

B1C.6 does not bind payload hash.

B1C.6 does not validate membership.

B1C.6 does not change process_instruction.

B1C.6 does not mark processed events.

B1C.6 does not mint.

B1C.6 does not open production gate.

## Flags

On quorum met:

- counts_unique_guardians = true
- quorum_met = true
- authorization_enabled = false
- processed_event_marking_enabled = false
- cpi_enabled = false
- live_route_enabled = false

On rejection, all execution flags stay false.

## Tests

Minimum tests:

1. Single guardian signing once -> unique count 1.
2. Same guardian signing twice -> unique count 1.
3. Two different guardians -> unique count 2.
4. Threshold exactly met -> quorum met.
5. Threshold exceeded -> quorum met.
6. Threshold not met -> reject.
7. Empty validated signers -> reject.
8. Threshold zero -> reject.
9. Threshold greater than guardian count -> reject.
10. Guardian set id mismatch -> reject.
11. All paths keep authorization, mark, CPI, and live route disabled.

## Completion criteria

B1C.6 spec is complete when Theo accepts:

- dedupe happens here, not in B1C.5
- quorum threshold comes from B1B guardian set
- quorum met still does not enable handler authorization
- B1C.7 is the first integration slice

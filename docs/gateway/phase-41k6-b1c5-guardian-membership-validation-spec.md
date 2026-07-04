# Phase 41K.6 B1C.5 — Guardian Membership Validation Spec

Status: planning checkpoint
Branch: stage-41k6-b1c5-guardian-membership-spec
Base: main after B1C.4 merge

## Purpose

B1C.5 verifies that every payload-bound Ed25519 signer belongs to the authoritative guardian set loaded from the B1B guardian set account.

## Input

B1C.5 consumes:

- payload-bound evidence from B1C.4
- authoritative guardian set loaded from B1B

## Core rule

For every payload-bound evidence item:

signer_public_key must be present in the loaded guardian set.

If any signer is not a guardian, reject.

## Failure policy

Reject on first unauthorized signer.

Reason:

- simpler
- safer
- no partial authorization ambiguity
- no need to collect attacker-controlled signer data

## Success meaning

B1C.5 success means:

- payload hash was already bound by B1C.4
- every signer is a member of the authoritative guardian set

B1C.5 does not mean quorum is reached.

## Non-goals

B1C.5 does not load guardian set accounts.

B1C.5 does not parse Ed25519 instructions.

B1C.5 does not bind payload hash.

B1C.5 does not deduplicate guardians.

B1C.5 does not count quorum.

B1C.5 does not authorize handler execution.

B1C.5 does not mark processed events.

B1C.5 does not mint.

B1C.5 does not open production gate.

## Flags

On success:

- validates_guardian_membership = true
- counts_unique_guardians = false
- authorization_enabled = false
- processed_event_marking_enabled = false
- cpi_enabled = false
- live_route_enabled = false

On rejection, all execution flags stay false.

## Tests

Minimum tests:

1. All signers are guardians -> pass.
2. First signer unauthorized -> reject.
3. Later signer unauthorized -> reject.
4. Empty evidence -> reject.
5. Duplicate guardian signatures are accepted here but not counted here.
6. Guardian set mismatch rejects.
7. Rejection paths keep execution flags false.
8. Success does not enable authorization, mark, CPI, or live route.

## Completion criteria

B1C.5 spec is complete when Theo accepts:

- B1B guardian set is the only authority source
- reject-on-first-unauthorized policy
- no dedupe/quorum in this slice
- no handler authorization in this slice

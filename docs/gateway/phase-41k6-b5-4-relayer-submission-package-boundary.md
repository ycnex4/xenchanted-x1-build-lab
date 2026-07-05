# Phase 41K.6 B5.4 — Relayer submission package boundary

## Purpose

B5.4 combines the B5.2 payload v2 hash conversion boundary and the B5.3 quorum package boundary into a single no-send relayer submission package.

B5.4 does not remove gates.

B5.4 does not introduce live RPC.

B5.4 does not sign, submit, simulate, spend SOL, or access private keys.

## Files added

- src/gateway/phase41k6RelayerSubmissionPackage.ts
- tests/phase41k6_b5_relayer_submission_package.test.ts

## Package flow

B5.4 assembles:

candidate
-> payload_v2_hash
-> quorum package
-> relayer submission package

The package records the handler instruction boundary:

- processed_event,
- route_id,
- mint,
- recipient token account,
- amount,
- guardian_set_id,
- prior evidence instruction count.

## No-send boundary

The package explicitly preserves:

- no live RPC,
- no signing,
- no submit,
- no SOL spend,
- no private-key access.

## Confirmed behavior

B5.4 tests confirm:

- a valid candidate and valid quorum package assemble into a no-send relayer package,
- empty eventId is rejected,
- empty journalId is rejected,
- guardian_set_id drift between candidate and quorum package is rejected,
- stale signatures are rejected when a handler-bound candidate field changes after signing,
- changing relayer-only operational ids does not change the payload hash.

## Boundary relation to B2/B3/B4

B2 proved the positive live-gated handler path.

B3 proved hostile evidence and account drift reject before mutation.

B4 kept the handler path gated and not production-activated.

B5.4 prepares the no-send off-chain submission package that can later feed the gated handler path, while preserving the B4 boundary.

The handler remains the final authority.

## B5.4 conclusion

B5.4 establishes the first complete no-send relayer submission package assembly boundary for Phase 41K.6.

The next step is B5.5:

B5.5 — submission package negative matrix and closure checklist.

# Phase 41K.6 B5.3 — Quorum package boundary

## Purpose

B5.3 adds a pure TypeScript quorum package boundary for the Phase 41K.6 watcher/relayer integration path.

B5.3 does not remove gates.

B5.3 does not introduce live RPC.

B5.3 does not sign, submit, simulate, spend SOL, or access private keys.

## Files added

- src/gateway/phase41k6QuorumPackage.ts
- tests/phase41k6_b5_quorum_package.test.ts

## Package model

The quorum package contains:

- payload_v2_hash,
- guardian_set_id,
- threshold,
- guardian set public keys,
- evidence format,
- prior Ed25519 evidence entries.

Each evidence entry contains:

- source instruction index,
- guardian public key,
- signature bytes as hex,
- signed message,
- payload hash binding status.

## Boundary checks

B5.3 rejects before relayer submission when:

- evidence format is invalid,
- guardian set is empty or too large,
- guardian set contains duplicate guardians,
- threshold is invalid,
- guardian evidence uses an unknown guardian,
- guardian evidence duplicates a guardian,
- signed message does not equal payload_v2_hash,
- source instruction index is invalid,
- signature hex is malformed,
- unique guardian evidence is below threshold.

## Confirmed behavior

B5.3 tests confirm:

- valid unique guardian quorum package is accepted,
- duplicate guardian evidence is rejected,
- unknown guardian evidence is rejected,
- insufficient quorum is rejected,
- signed message drift is rejected,
- malformed threshold, source instruction index, and signature are rejected.

## Boundary relation to B3

B3 proves that hostile evidence rejects before mutation at the live-gated handler boundary.

B5.3 catches the same class of package mistakes earlier, before relayer submission.

The handler remains the final authority.

## B5.3 conclusion

B5.3 establishes the quorum package boundary for off-chain watcher/relayer preparation.

The next step is B5.4:

B5.4 — relayer submission package assembly boundary.

B5.4 should combine payload v2 hash conversion and quorum package validation into a single no-send submission package.

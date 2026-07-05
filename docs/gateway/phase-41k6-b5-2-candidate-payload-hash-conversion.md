# Phase 41K.6 B5.2 — Candidate payload v2 hash conversion

## Purpose

B5.2 implements a pure TypeScript candidate-to-payload-hash conversion boundary for the Phase 41K.6 watcher/relayer path.

B5.2 does not remove gates.

B5.2 does not introduce live RPC.

B5.2 does not sign, submit, simulate, spend SOL, or access private keys.

## Runtime alignment

The Rust SVM handler computes the B1C payload hash from:

- domain: consume_gateway_mint_authorization_v2
- processed_event
- route_id
- mint
- recipient
- amount as u64 little-endian
- guardian_set_id

The Rust report labels the hash algorithm as sha256.

B5.2 mirrors that field order and amount encoding in TypeScript for deterministic off-chain package preparation.

## Files added

- src/gateway/phase41k6PayloadV2.ts
- tests/phase41k6_b5_candidate_payload_hash.test.ts

## Boundary

The B5.2 builder takes a reconciled watcher candidate and produces:

- payload domain,
- hash algorithm,
- normalized handler-bound fields,
- amount little-endian encoding,
- payload_v2_hash.

The builder only binds handler-relevant fields:

- processed_event
- route_id
- mint
- recipient token account
- amount
- guardian_set_id

Watcher metadata remains outside the payload hash unless a later handler revision explicitly binds it.

## Confirmed behavior

B5.2 tests confirm:

- payload hash construction is deterministic,
- amount is encoded as u64 little-endian,
- changing processed_event changes the payload hash,
- changing route_id changes the payload hash,
- changing mint changes the payload hash,
- changing recipient token account changes the payload hash,
- changing amount changes the payload hash,
- changing guardian_set_id changes the payload hash,
- changing watcher-only operational metadata does not change the payload hash,
- malformed bytes32 fields are rejected,
- invalid u64 amounts are rejected.

## B5.2 conclusion

B5.2 establishes the first pure off-chain conversion boundary that aligns the watcher/relayer candidate with the new Phase 41K.6 handler payload binding contract.

The next step is B5.3:

B5.3 — quorum package schema and prior Ed25519 evidence package boundary.

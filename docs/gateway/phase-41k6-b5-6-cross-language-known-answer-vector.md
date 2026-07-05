# Phase 41K.6 B5.6 — Cross-language known-answer parity vector

## Purpose

B5.6 responds to Claude's B5 hostile audit note.

Claude accepted B5 with notes, but correctly identified that the core B5 parity claim should be locked by a cross-language known-answer vector before B6.1.

B5.6 adds that lock.

## Scope

B5.6 does not open B6.

B5.6 does not deploy.

B5.6 does not submit transactions.

B5.6 does not sign transactions.

B5.6 does not access private keys.

B5.6 does not spend SOL.

B5.6 only hardens the B5 payload hash parity proof.

## Canonical known-answer vector

The canonical fixture is:

- domain: consume_gateway_mint_authorization_v2
- processed_event: [0xB2; 32]
- route_id: [0x41; 32]
- mint: [0x51; 32]
- recipient token account: [0x61; 32]
- amount: 1_234_567_890 as u64 little-endian
- guardian_set_id: [0xC7; 32]

Expected payload hash:

0x56a318440e188d864052b8518f41deb7e4f998a975e3b6e19ca63815535ec77d

The u64 max boundary fixture keeps the same fields and sets:

- amount: u64::MAX

Expected payload hash:

0xa6b9e3901a04a6da11d100912cb1f5ebf294464d5b11376f2b7eb71a0cb9f893

## Files updated

- tests/phase41k6_b5_candidate_payload_hash.test.ts
- programs/xxxl-svm/src/verifier/b1c_payload_hash_binding.rs

## Closure meaning

After B5.6, both TypeScript and Rust assert the same hardcoded digest for the same canonical input.

This prevents silent future drift in:

- domain string,
- field order,
- field size,
- recipient token account binding,
- amount u64 little-endian encoding,
- guardian_set_id placement,
- sha256/hashv concatenation semantics.

## B5.6 conclusion

B5.6 closes Claude's mandatory B5 note before B6.1.

After B5.6 is merged, B6.1 may be opened as an explicit no-send/no-sign/no-key/no-submit X1 testnet E2E opening boundary.

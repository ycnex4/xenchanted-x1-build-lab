# Proof Log Verification

Package: production-proof-log-source-config-and-resolution-package
Schema: gateway-mint-proof-v1
Environment: x1_testnet
Route: gateway_mint

## Canonical JSON rule

Canonical JSON is produced by:

- UTF-8 encoding
- lexicographically sorted object keys
- compact separators: comma and colon
- no insignificant whitespace
- SHA-256 over the canonical JSON bytes

In Python terms:

json.dumps(object, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

## Required checks

1. Load schema at docs/gateway/proof-log/schema/gateway-mint-proof-v1.schema.json.
2. Load config at docs/gateway/proof-log/config/x1-testnet-proof-log-config-v1.json.
3. Load fixture at docs/gateway/proof-log/fixtures/gateway-mint-proof-v1-dry-run-record.json.
4. Verify schema_version is gateway-mint-proof-v1.
5. Verify record_type is gateway_mint_proof.
6. Verify route is gateway_mint.
7. Verify environment is x1_testnet.
8. Recompute source_burn.canonical_event_hash from source burn fields.
9. Recompute destination_mint.mint_message_hash from canonical mint message payload.
10. Recompute destination_mint.mint_instruction_hash from destination mint instruction payload.
11. Verify guardian_set_descriptor_hash_sha256 equals 4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83.
12. Verify guardian_set_version=1, guardian_count=5, threshold=3, quorum_model=3-of-5.
13. Verify guardian approvals are ordered by descriptor order and unique.
14. Recompute guardian_quorum_transcript_hash from guardian approvals.
15. Recompute consumed_event_marker_hash from consumed event fields.
16. Recompute record_hash with record_hash temporarily set to 64 zeroes.
17. Verify previous_record_hash linkage for non-genesis records.
18. Verify no private key, keypair, seed, or mnemonic material appears in the record.
19. Verify fixture is dry-run only and contains no live mint execution.
20. Verify proof-log configuration does not authorize activation, deploy, route enablement, SPL CPI enablement, or external production publication.

## Dry-run fixture note

The fixture is a non-production dry-run record. It does not include live signatures, does not construct a signing package, does not execute minting, and does not publish to an external production endpoint.

## ProductionProofLogUnset resolution basis

ProductionProofLogUnset can be removed in this package only because the schema, config, fixture, verification checklist, hash model, guardian hash linkage, consumed-event linkage, material safety guard, and tests are all present and verifiable while LiveRouteDisabled and SplCpiExecutionDisabled remain active.

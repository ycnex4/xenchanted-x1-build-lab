# Phase 41K.6 B1C.4 — Payload Hash Binding Spec

Status: planning checkpoint
Branch: stage-41k6-b1c4-payload-hash-binding-spec
Base: main after B1C.3-connect merge

## Purpose

B1C.4 verifies that each parsed Ed25519 evidence item signed the expected authorization payload for the current ConsumeGatewayMint operation.

B1C.4 consumes parsed evidence from B1C.3-connect and checks:

signed_message == expected_authorization_payload_hash

## Core rule

The expected payload hash must be computed locally from live instruction context.

It must not be accepted from:

- caller
- frontend
- watcher
- relayer
- instruction data as a trusted precomputed hash

## Hash algorithm

Use SHA-256.

Reason:

- Solana-compatible
- available in program environment
- sufficient for compact authorization commitment
- separate from any existing canonical event hash machinery

## Domain separation

Use a fixed domain string:

consume_gateway_mint_authorization_v1

The final hash input must include the domain first.

## Payload fields

B1C.4 expected payload should bind at minimum:

- domain
- processed_event pubkey
- mint pubkey
- recipient token account pubkey
- amount u64
- guardian_set_id u32

All numeric fields use little-endian encoding.

## Slot policy

current_slot is intentionally omitted from the authorization payload.

Reason:

- guardians sign asynchronously
- exact execution slot is not predictable
- processed_event uniqueness plus processed registry provides replay protection
- including exact slot would create operational fragility without meaningful extra security

## Field sources

processed_event comes from the live processed_event account key.

mint comes from the live SPL mint account key.

recipient comes from the live recipient token account key.

amount comes from decoded ConsumeGatewayMint args.

guardian_set_id comes from decoded ConsumeGatewayMint args.

## Verification rule

For each parsed Ed25519 evidence:

- if signed_message length is not 32 bytes, reject
- if signed_message bytes do not equal expected hash, reject
- if it equals expected hash, the evidence is payload-bound

## Failure mode

Any payload mismatch is a rejection.

B1C.4 does not authorize execution by itself.

All rejection paths keep:

- validates_guardian_membership = false
- counts_unique_guardians = false
- authorization_enabled = false
- processed_event_marking_enabled = false
- cpi_enabled = false
- live_route_enabled = false

## Non-goals

B1C.4 does not read instructions_sysvar.

B1C.4 does not parse Ed25519 instruction data.

B1C.4 does not load guardian set.

B1C.4 does not validate guardian membership.

B1C.4 does not deduplicate guardians.

B1C.4 does not count quorum.

B1C.4 does not change process_instruction.

B1C.4 does not mark processed events.

B1C.4 does not mint.

B1C.4 does not open production gate.

## Tests

Minimum tests:

1. Computes stable expected hash from live-like parameters.
2. Same parameters produce same hash.
3. Different processed_event changes hash.
4. Different mint changes hash.
5. Different recipient changes hash.
6. Different amount changes hash.
7. Different guardian_set_id changes hash.
8. Matching parsed evidence passes payload binding.
9. Mismatched parsed evidence rejects.
10. Wrong signed_message length rejects.
11. All rejection paths keep execution flags false.

## Completion criteria

B1C.4 spec is complete when Theo accepts:

- local hash computation
- SHA-256
- domain separation
- live field sources, with current_slot intentionally omitted
- no caller-provided precomputed hash
- no execution authorization in this slice

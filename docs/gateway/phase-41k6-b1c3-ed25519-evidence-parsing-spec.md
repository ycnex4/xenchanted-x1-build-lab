# Phase 41K.6 B1C.3 — Prior Ed25519 Evidence Parsing Spec

Status: planning checkpoint
Branch: stage-41k6-b1c3-ed25519-evidence-parsing
Base: main after B1C.2 merge

## Purpose

B1C.3 parses prior Ed25519 precompile instruction data that B1C.2 identified through the real instructions_sysvar path.

B1C.3 extracts verified evidence material from each prior Ed25519 precompile instruction:

- guardian public key bytes
- signature bytes
- signed message bytes
- source instruction index
- instruction data length
- parsing status

B1C.3 does not decide whether the signer is a guardian.

B1C.3 does not bind the signed message to the expected authorization payload.

B1C.3 does not count quorum.

B1C.3 does not authorize mark or mint.

Those are later slices.

## Critical correction

B1C.3 must not infer guardian approval from normal transaction signer account metas.

Guardian approval for this gateway must come from Ed25519 precompile evidence.

The signer public key is encoded inside the Ed25519 precompile instruction data, together with signature and message offsets.

Solana runtime validates the Ed25519 precompile instruction before the program inspects it through instructions_sysvar.

Therefore B1C.3 parses verified precompile data. It does not perform independent Ed25519 signature verification inside the program.

## Dependency chain

B1A established account 11 as instructions_sysvar.

B1B established authoritative guardian set loading from account 2.

B1C.1 established authorization result types.

B1C.2 mapped the existing 41K.1 live instructions_sysvar wiring into B1C evidence surface.

B1C.3 consumes prior Ed25519 instruction data and extracts evidence descriptors.

## Source of instruction data

B1C.2 currently exposes descriptors, not raw instruction bytes.

For B1C.3 implementation there are two acceptable approaches:

1. Add raw prior Ed25519 instruction data to the B1C.2 loaded descriptor under the existing B1C test gate.
2. Keep B1C.2 unchanged and let B1C.3 consume the lower checked prior instruction loading boundary directly, then parse the raw Instruction data from there.

Preferred direction:

Use a pure parser first.

The pure parser takes instruction_index and ed25519_instruction_data bytes and returns parsed evidence.

This keeps B1C.3 independent from handler flow and avoids mixing sysvar loading with parsing.

A later integration slice can connect B1C.2 raw data or the lower 41D3 checked prior instruction loading output into this parser.

## Ed25519 precompile data layout

B1C.3 supports the standard single-signature Ed25519 instruction layout first.

Expected minimum structure:

- num_signatures: u8
- padding: u8
- one signature offset record:
  - signature_offset: u16
  - signature_instruction_index: u16
  - public_key_offset: u16
  - public_key_instruction_index: u16
  - message_data_offset: u16
  - message_data_size: u16
  - message_instruction_index: u16

Constants:

- signature length: 64 bytes
- public key length: 32 bytes
- header length for one signature: 16 bytes
- current instruction sentinel: u16::MAX

B1C.3 initial policy:

- num_signatures must be exactly 1
- padding must be zero
- signature_instruction_index must be current instruction sentinel
- public_key_instruction_index must be current instruction sentinel
- message_instruction_index must be current instruction sentinel
- signature range must be in bounds
- public key range must be in bounds
- message range must be in bounds
- message_data_size must be non-zero
- overlapping ranges are not automatically rejected unless they violate bounds
- multi-signature Ed25519 precompile instructions are rejected for now
- cross-instruction offset references are rejected for now

Reason:

Each guardian can provide a separate prior Ed25519 precompile instruction. That keeps B1C.3 simple and auditable. Multi-signature and cross-instruction layouts can be added later only if needed.

## Parsed evidence result

B1C.3 should introduce a parsed evidence result type with:

- status
- instruction_index
- signer_public_key
- signature
- signed_message
- instruction_data_len
- signature_offset
- public_key_offset
- message_data_offset
- message_data_size
- runtime_verified_by_ed25519_precompile
- parsed_from_prior_ed25519_instruction
- accepts_caller_provided_signature_claims
- accepts_frontend_or_watcher_proof
- binds_payload_hash
- validates_guardian_membership
- counts_unique_guardians
- authorization_enabled
- processed_event_marking_enabled
- cpi_enabled
- live_route_enabled

Safety flags must remain:

- accepts_caller_provided_signature_claims = false
- accepts_frontend_or_watcher_proof = false
- binds_payload_hash = false
- validates_guardian_membership = false
- counts_unique_guardians = false
- authorization_enabled = false
- processed_event_marking_enabled = false
- cpi_enabled = false
- live_route_enabled = false

## Rejection cases

B1C.3 should reject:

- instruction data too short
- num_signatures zero
- num_signatures greater than one
- non-zero padding if policy requires zero
- malformed offset record
- signature offset out of bounds
- public key offset out of bounds
- message offset out of bounds
- message length zero
- signature instruction index not current instruction sentinel
- public key instruction index not current instruction sentinel
- message instruction index not current instruction sentinel

All rejection results must keep all execution flags false.

## Tests

Minimum tests:

1. Valid single-signature Ed25519 instruction data parses signer public key, signature, and signed message.
2. Too-short data rejects.
3. Zero signatures rejects.
4. More than one signature rejects.
5. Signature offset out of bounds rejects.
6. Public key offset out of bounds rejects.
7. Message offset out of bounds rejects.
8. Zero message length rejects.
9. Cross-instruction signature index rejects.
10. Cross-instruction public key index rejects.
11. Cross-instruction message index rejects.
12. Rejection keeps all execution flags false.
13. Parsed evidence keeps binding, membership, quorum, authorization, mark, cpi, and live route flags false.

## Non-goals

B1C.3 does not read instructions_sysvar.

B1C.3 does not change process_instruction.

B1C.3 does not load guardian set.

B1C.3 does not check guardian membership.

B1C.3 does not bind payload hash.

B1C.3 does not deduplicate guardians.

B1C.3 does not count quorum.

B1C.3 does not enable mark, mint, CPI, or live route.

B1C.3 does not open the production gate.

## Completion criteria

B1C.3 is complete when:

- A pure Ed25519 precompile instruction data parser exists.
- It extracts signer public key, signature, and signed message from valid single-signature precompile data.
- It rejects malformed layouts safely.
- It rejects unsupported multi-signature and cross-instruction offset layouts.
- All result safety flags remain false.
- Default tests pass.
- B1C feature-gated tests pass.
- Closed-gate Mollusk consume_gateway_mint tests pass.

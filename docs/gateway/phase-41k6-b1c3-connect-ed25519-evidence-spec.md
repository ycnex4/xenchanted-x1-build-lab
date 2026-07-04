# Phase 41K.6 B1C.3-connect — Ed25519 Evidence Connection Spec

Status: planning checkpoint
Branch: stage-41k6-b1c3-connect-ed25519-evidence
Base: main after B1C.3 merge

## Purpose

B1C.3-connect wires loaded prior Ed25519 instruction data into the B1C.3 pure parser.

This slice connects:

- checked prior instruction loading result
- 41K.1 prior Ed25519 validation/filtering boundary
- B1C.3 pure Ed25519 precompile parser

It still does not bind payload hash, check guardian membership, count quorum, authorize execution, mark processed events, or mint.

## Problem

B1C.2 loaded descriptors intentionally expose metadata:

- instruction_index
- instruction_data_len
- prior Ed25519 count
- source/runtime safety flags

But they do not currently carry raw instruction bytes.

The B1C.3 parser requires raw Ed25519 precompile instruction data.

The raw instruction bytes are available in the lower checked prior loading result:

- Phase41D3_2_2LoadedPriorInstruction.instruction.data

## Design choice

Preferred approach for this slice:

Do not mutate B1C.2 descriptors yet.

Instead, introduce a connection adapter that consumes:

- Phase41D3_2_2CheckedPriorInstructionLoadingResult

Then it:

1. Reuses derive_phase_41k_1_from_checked_prior_loading to confirm that the lower result is a valid prior Ed25519 evidence source.
2. Rejects if 41K.1 does not report PriorEd25519PrecompileInstructionsLoaded.
3. Iterates the raw loaded prior instructions from Phase41D3_2_2CheckedPriorInstructionLoadingResult.
4. Filters Ed25519 program instructions.
5. Parses each Ed25519 instruction.data with parse_b1c_single_ed25519_precompile_evidence.
6. Returns parsed evidence descriptors and parse rejection descriptors.
7. Keeps all execution flags false.

Reason:

This avoids duplicating instructions_sysvar loading, avoids adding raw bytes to B1C.2 prematurely, and keeps the connection layer explicit.

## Inputs

The connection adapter input is:

- Phase41D3_2_2CheckedPriorInstructionLoadingResult

It is not:

- caller-provided proof bytes
- frontend proof bytes
- watcher proof bytes
- arbitrary raw transaction bytes
- handler arguments

## Output

The connection adapter should produce:

- status
- current_instruction_index
- loaded_prior_instruction_count
- inspected_prior_instruction_count
- prior_ed25519_precompile_count
- parsed_evidence_count
- rejected_evidence_count
- parsed_evidence: Vec<B1CParsedEd25519Evidence>
- parse_rejections: Vec<connection rejection descriptor>
- safety flags

Parsed evidence comes directly from B1C.3 parser.

Parse rejection descriptor should include:

- source_instruction_index
- instruction_data_len
- B1CEd25519EvidenceParsingRejectionKind

## Status model

Suggested statuses:

- ParsedPriorEd25519Evidence
- NoParsedPriorEd25519Evidence
- SourceRejected

SourceRejected means 41K.1 did not accept the lower checked prior loading result as prior Ed25519 evidence.

NoParsedPriorEd25519Evidence means 41K.1 found prior Ed25519 instructions, but B1C.3 parser rejected all of them.

ParsedPriorEd25519Evidence means at least one prior Ed25519 precompile instruction parsed successfully.

## Safety flags

All outputs must keep:

- accepts_caller_provided_instruction_bytes = false
- accepts_frontend_or_watcher_ed25519_proof = false
- binds_payload_hash = false
- validates_guardian_membership = false
- counts_unique_guardians = false
- authorization_enabled = false
- processed_event_marking_enabled = false
- cpi_enabled = false
- live_route_enabled = false

## Non-goals

B1C.3-connect does not read instructions_sysvar directly.

B1C.3-connect does not change process_instruction.

B1C.3-connect does not compute expected payload hash.

B1C.3-connect does not compare signed_message to payload hash.

B1C.3-connect does not load or validate guardian membership.

B1C.3-connect does not deduplicate guardians.

B1C.3-connect does not count quorum.

B1C.3-connect does not authorize mark or mint.

B1C.3-connect does not open production gate.

## Tests

Minimum tests:

1. Valid checked prior loading result with two Ed25519 instructions parses both.
2. Mixed prior instructions parse only Ed25519 instructions and discard non-Ed25519.
3. SourceRejected when 41K.1 reports MissingInstructionsSysvar.
4. SourceRejected when 41K.1 reports NoPriorInstructions.
5. SourceRejected when 41K.1 reports NoPriorEd25519PrecompileInstructions.
6. NoParsedPriorEd25519Evidence when Ed25519 instructions exist but all parser attempts reject.
7. Partial parse: one valid Ed25519 parses and one malformed Ed25519 is recorded as rejection.
8. All loaded/parsed/rejected results keep execution flags false.
9. Connection report documents no payload binding, no guardian membership, no quorum, no authorization.

## Completion criteria

B1C.3-connect is complete when:

- the adapter connects checked prior loading raw instruction data to B1C.3 parser
- 41K.1 validation remains the source of prior Ed25519 acceptance
- raw instruction bytes are not accepted from caller/frontend/watcher
- parsed evidence is available for B1C.4 payload binding
- all execution flags remain false
- default tests pass
- B1C feature-gated tests pass
- closed-gate Mollusk consume_gateway_mint tests pass

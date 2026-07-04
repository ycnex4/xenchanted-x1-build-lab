# Phase 41K.6 B1C — Ed25519 Evidence Wiring Spec

Status: planning checkpoint
Branch: stage-41k6-b1c-ed25519-evidence-wiring
Base: main after B1B merge

## Purpose

B1C wires guardian signature evidence into the ConsumeGatewayMint authorization path.

B1A added the V3 account shape with instructions_sysvar at account 11.

B1B added authoritative guardian set loading from account 2.

B1C must use those two foundations to prove that the transaction contains enough valid prior Ed25519 precompile evidence from unique guardians before any processed_event mark or mint path can proceed.

## Critical safety boundary

B1C must not open the production mint gate by default.

Any path where authorization_enabled, processed_event_marking_enabled, cpi_enabled, or live_route_enabled becomes true must be feature-gated and explicitly non-production.

Default build remains closed-gate.

## Correct source of signature evidence

Guardian signatures must be represented as prior Ed25519 precompile instructions in the same transaction.

The program must not accept caller-provided signature claims, frontend-provided proof bytes, or arbitrary dummy instructions as authorization.

Required evidence source:

1. ConsumeGatewayMint receives instructions_sysvar as account 11.
2. Program reads real instructions sysvar.
3. Program loads strictly prior instructions only.
4. Program filters prior instructions to Ed25519 precompile instructions.
5. Program parses Ed25519 instruction data.
6. Program extracts verified signer public key and verified message bytes.
7. Program binds verified message bytes to the expected guardian payload hash.
8. Program checks signer public key membership in the authoritative guardian set loaded by B1B.
9. Program counts unique valid guardian signers.
10. Program authorizes only if unique valid guardians >= threshold.

## Required B1C order

The B1C authorization check must happen before any state mutation.

Required order:

1. Decode ConsumeGatewayMint.
2. Validate V3 account contract, including instructions_sysvar account 11.
3. Prepare existing route, mint, recipient, and processed_event boundaries.
4. Load authoritative guardian set through B1B.
5. Load prior Ed25519 evidence through instructions_sysvar.
6. Bind evidence to expected payload hash.
7. Count unique guardian approvals.
8. Establish quorum authorization.
9. Only then allow the gated mark plus mint path.

Rejected order:

1. Mark processed_event.
2. Verify guardian quorum.
3. Mint.

Reason: failed authorization must leave processed_event untouched without relying on rollback.

## Payload binding rule

B1C must not count a signature merely because a guardian signed something.

The signed message must be bound to this exact authorization intent.

At minimum, the signed payload must bind:

- route_id
- guardian_set_id
- target mint id
- canonical_event_key
- recipient
- amount
- source_chain_id
- source_chain_weight_bps

Open design issue:

The historical guardian payload model includes richer Ethereum source proof fields. The current ConsumeGatewayMint runtime instruction contains a compact 208-byte payload. B1C must choose one exact payload binding and test that wrong payloads fail.

Conservative direction:

- Do not invent loose caller-provided evidence.
- Prefer using existing canonical payload hash machinery where compatible.
- If a compact B1C runtime authorization payload is needed, document it explicitly and keep it domain-separated.

## Unique guardian counting

B1C must count unique guardians, not signatures.

Rules:

- A valid guardian signature counts only once.
- Duplicate signatures from the same guardian do not increase quorum count.
- Non-guardian signatures are ignored or rejected but must not count.
- Failed/malformed evidence must not kill a valid M-of-N quorum unless the failure indicates malformed global transaction state that must be rejected.
- Threshold is loaded from the authoritative guardian set account, not from instruction data.

## Failure safety

All B1C authorization failures must occur before processed_event marking.

On failure, these must remain unchanged:

- processed_event
- recipient_balance
- SPL mint account
- recipient token account
- rent payer lamports

## B1C implementation slices

### B1C.1 — Feature gates and result type

Add B1C feature pair.

Add a B1C authorization result type that can report:

- authorization_enabled
- unique_guardian_count
- threshold
- counted guardian indexes
- counted guardian public keys
- rejected evidence count
- processed_event_marking_enabled
- cpi_enabled
- live_route_enabled

Default production flags must remain false.

### B1C.2 — Instructions sysvar evidence loading bridge

Use account 11 from B1A.

Load real instructions_sysvar.

Reject:

- missing instructions_sysvar
- wrong instructions_sysvar key
- signer instructions_sysvar
- writable instructions_sysvar
- unreadable instructions_sysvar
- no prior instructions
- no prior Ed25519 precompile instructions

### B1C.3 — Prior Ed25519 evidence parsing

For each prior Ed25519 precompile instruction:

- parse instruction data
- extract signature, public key, and message byte ranges
- require native Ed25519 verification evidence
- reject malformed instruction data safely

### B1C.4 — Payload hash binding

Bind extracted message bytes to expected payload hash.

Tests must include:

- correct payload hash succeeds
- wrong route_id fails
- wrong guardian_set_id fails
- wrong recipient fails
- wrong amount fails
- same guardian signatures over different payload fail

### B1C.5 — Quorum counting

Use B1B loaded guardian set.

Tests:

- threshold met succeeds
- below threshold rejects
- duplicate guardian counted once
- non-guardian signatures do not count
- valid quorum succeeds even with extra non-guardian evidence

### B1C.6 — Pre-mark integration

Wire B1C authorization before mark_processed_event_atomic in test-gated path.

Tests:

- invalid quorum rejects before mark
- no evidence rejects before mark
- wrong payload rejects before mark
- valid quorum reaches the already-proven gated mark plus mint path only under dangerous test gate

## Non-goals

B1C does not deploy.

B1C does not open default production mint gate.

B1C does not remove D2 or D3 gates.

B1C does not trust caller-provided proof data.

B1C does not introduce a standalone mark instruction.

B1C does not split mark and mint into separate production instructions.

## Completion criteria

B1C is complete only when:

- Real instructions_sysvar evidence is used.
- Prior Ed25519 precompile evidence is parsed and bound to expected payload.
- Guardian set comes from B1B authoritative loading.
- Unique guardian quorum is enforced.
- Authorization happens before processed_event mark.
- Invalid authorization leaves all mutable accounts unchanged.
- Default build remains closed-gate.
- B1A, B1B, D2, and D3 validation still pass.

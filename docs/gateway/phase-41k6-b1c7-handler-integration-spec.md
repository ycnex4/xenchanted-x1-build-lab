# Phase 41K.6 B1C.7 — Handler Integration Spec

Status: planning checkpoint
Branch: stage-41k6-b1c7-handler-integration-spec
Base: main after B1C.6 merge

## Purpose

B1C.7 wires the full guardian authorization pipeline into the ConsumeGatewayMint handler path before atomic mark + mint.

This is the slice that resolves the B1 blocker at runtime boundary level.

## Pipeline

The handler must complete this sequence before any mutable state change:

1. Decode ConsumeGatewayMint args.
2. Validate V3 account contract with instructions_sysvar.
3. Load authoritative guardian set from B1B.
4. Load prior Ed25519 evidence from instructions sysvar.
5. Parse prior Ed25519 instruction data.
6. Compute expected payload hash locally.
7. Bind parsed signed_message to expected payload hash.
8. Validate guardian membership.
9. Deduplicate guardians and count quorum.
10. Only if quorum is met, allow atomic mark + mint.

## Critical safety rule

No processed_event mark and no SPL mint CPI may occur before the full authorization pipeline succeeds.

Failure anywhere before quorum must leave all mutable accounts unchanged.

## Authorization meaning

B1C.7 authorization means:

- evidence came from prior Ed25519 precompile instructions
- payload hash matches the current ConsumeGatewayMint operation
- signers are members of the authoritative guardian set
- enough unique guardians signed

Only this full result may allow mark + mint.

## Feature gate policy

B1C.7 integration is feature-gated.

Default production build must remain closed.

B1C.7 must not silently open live route or production behavior outside the explicit test/integration gate.

## Non-goals

B1C.7 does not redesign canonical message encoding.

B1C.7 does not change guardian set account layout.

B1C.7 does not change processed registry semantics.

B1C.7 does not split mark and mint.

B1C.7 does not add standalone mark instruction.

B1C.7 does not remove existing safety gates.

B1C.7 does not deploy.

## Tests

Minimum tests:

1. Valid full pipeline reaches mark + mint.
2. Payload hash mismatch rejects before mark + mint.
3. Unauthorized guardian rejects before mark + mint.
4. Duplicate signer cannot fake quorum.
5. Quorum not met rejects before mark + mint.
6. Wrong guardian set rejects before mark + mint.
7. Missing instructions_sysvar rejects before mark + mint.
8. No prior Ed25519 evidence rejects before mark + mint.
9. Replay still rejected.
10. Default closed gate remains closed.

## Completion criteria

B1C.7 spec is complete when Theo accepts:

- full pipeline order
- authorization before mark + mint
- rollback/no-mutation failure rule
- feature-gated integration only
- default production closed gate preserved

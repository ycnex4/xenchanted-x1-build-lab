# XXXL Phase 41G.0 — Proof / Evidence / Payload Binding Plan

Date: 2026-07-02

## Status

Docs-only planning checkpoint.

No runtime code is introduced.

No verification logic is changed.

No guardian validity, quorum, authorization, replay write, mutation, CPI, mint, handler, or live route is enabled.

## Parent Gate

Phase 41F focused crypto-boundary audit acceptance:

`72951e8 Merge XXXL phase 41F focused crypto boundary audit acceptance`

## Purpose

Phase 41G opens the next layer after Phase 41F.

Phase 41F established only:

- native Ed25519 verification was already performed by the SVM.

Phase 41G must establish the next separate property:

- the verified Ed25519 message is bound to the expected gateway payload.

Short form:

- Phase 41F: the signature was verified by SVM.
- Phase 41G: the signed message corresponds to the expected gateway payload.

## Critical Separation

Phase 41G payload binding must remain separate from:

- guardian set membership;
- guardian validity;
- quorum;
- authorization;
- replay writes;
- account mutation;
- CPI;
- SPL Token mint;
- process instruction handler;
- live route.

Phase 41G may establish payload/evidence binding.

It must not establish that the signer is an authorized guardian.

It must not establish quorum.

It must not authorize execution.

## Inputs From Phase 41F

Phase 41G consumes the accepted Phase 41F boundary result:

- `NativeEd25519VerificationEstablished`;
- verified signature range;
- verified public key range;
- verified message range;
- extracted public key bytes;
- borrowed signed message bytes.

Important:

Phase 41G must not reinterpret Phase 41F as local cryptographic verification.

The SVM remains the verifier.

XXXL only consumes the established fact that SVM verified the native Ed25519 instruction.

## Preferred Payload Binding Model

Preferred model for Phase 41G:

- guardians sign the gateway payload hash;
- the native Ed25519 instruction message bytes are exactly the expected payload hash bytes;
- the program computes or receives the expected payload hash according to the accepted canonical gateway payload model;
- Phase 41G compares signed message bytes to the expected payload hash.

Accepted target relation:

`signed_message_bytes == expected_gateway_payload_hash_bytes`

This keeps the signed message small and prevents the native Ed25519 instruction from needing to carry the full canonical gateway payload.

## Hash Algorithm

The expected payload hash must follow the accepted Stage 1 gateway canonicalization model.

Planned hash:

- `keccak256(canonical_gateway_payload_bytes)`.

This is a planning statement only.

Implementation must separately confirm:

- exact canonical byte encoding;
- exact field order;
- exact decimal/integer encoding;
- exact byte/string/domain separators;
- hash preimage compatibility with Stage 1 vectors.

## Canonical Gateway Payload Fields

Phase 41G payload binding must bind the signed payload hash to the accepted gateway message fields.

Canonical field order carried forward from Stage 1:

1. `messageType`
2. `schemaVersion`
3. `routeId`
4. `sourceChainId`
5. `sourceToken`
6. `sourceSender`
7. `sourceBurnTxHash`
8. `sourceBurnEventIndex`
9. `sourceBlockNumber`
10. `sourceBlockHash`
11. `sourceNonce`
12. `canonicalEventKey`
13. `x1RecipientHash`
14. `burnedAmount`
15. `sourceChainWeightBps`
16. `xxxlMintAmount`
17. `mintToken`
18. `deadlineOrFinalityBlock`
19. `messageNonce`

Phase 41G must not silently reorder, omit, rename, or reinterpret fields.

## Binding Requirements

Phase 41G must bind at least:

- route;
- source chain;
- source token;
- source sender;
- burn transaction hash;
- burn event index;
- source block number;
- source block hash;
- source nonce;
- canonical event key;
- X1 recipient hash;
- burned amount;
- chain weight;
- XXXL mint amount;
- target mint token;
- deadline or finality block;
- message nonce.

## Route Binding

The payload must bind:

- `routeId`;
- `sourceChainId`;
- `sourceToken`;
- `mintToken`.

This prevents a signature for one route or asset from being replayed as another route or asset.

## Burn Event Binding

The payload must bind:

- `sourceBurnTxHash`;
- `sourceBurnEventIndex`;
- `sourceBlockNumber`;
- `sourceBlockHash`;
- `sourceNonce`;
- `canonicalEventKey`.

This prevents a signature over one burn event from being reused for a different burn event.

## Recipient Binding

The payload must bind:

- `x1RecipientHash`.

Phase 41G should compare only the expected recipient hash at this layer.

It must not yet create or mutate recipient token accounts.

Recipient account validation, associated token account handling, and mint recipient execution remain later runtime/CPI work.

## Amount Binding

The payload must bind:

- `burnedAmount`;
- `sourceChainWeightBps`;
- `xxxlMintAmount`.

Phase 41G must not authorize minting.

It only proves that the verified signed payload committed to the expected amounts.

## Finality / Expiration Binding

The payload must bind:

- `deadlineOrFinalityBlock`.

This phase may plan the binding, but it must not introduce live watcher finality acceptance unless separately reviewed.

Finality source, block safety rules, and watcher proof logs remain separate infrastructure/runtime phases.

## Message Nonce Binding

The payload must bind:

- `messageNonce`.

Message nonce binding is separate from replay write.

Phase 41G may prove the nonce is part of the signed payload.

It must not mark replay state.

## Expected Phase 41G Result Model

Future implementation may introduce a result such as:

- `GatewayPayloadBindingEstablished`.

This status should mean only:

- the Phase 41F verified message bytes match the expected gateway payload hash;
- the expected gateway payload hash was derived from canonical gateway payload fields.

It must not mean:

- signer is a valid guardian;
- guardian set membership accepted;
- quorum reached;
- authorization granted;
- replay-safe;
- mint allowed.

## Evidence Model Boundary

Phase 41G may define structured payload evidence.

But evidence acceptance must be narrow:

- accept that supplied canonical payload fields hash to the signed message;
- not accept that the source burn happened;
- not accept that a watcher is honest;
- not accept that a signer is a valid guardian;
- not accept quorum;
- not authorize minting.

Source-chain event proof and watcher proof-log policy may require a later phase.

## Public Key Handling

Phase 41G may carry forward the Ed25519 public key bytes from Phase 41F.

But Phase 41G must not decide whether that public key belongs to the guardian set.

Guardian public key membership remains a later guardian-validation phase.

## Suggested Implementation Split

Recommended future split:

### Phase 41G.1 — Payload Evidence Shape

Define the structured expected payload evidence model.

No hashing yet.

No guardian validation.

No authorization.

### Phase 41G.2 — Canonical Encoding / Hash Boundary

Implement canonical encoding and expected payload hash computation.

Compare signed message bytes to expected payload hash.

No guardian validation.

No quorum.

No authorization.

### Phase 41G.3 — Negative Matrix / Payload Binding Audit

Test and audit wrong route, wrong chain, wrong burn event, wrong recipient, wrong amount, wrong mint token, wrong finality, wrong nonce, malformed encoding, and hash mismatch.

Only after 41G is accepted should guardian validation begin.

## Required Negative Cases

Future tests should include:

- signed message length mismatch;
- signed message hash mismatch;
- wrong `routeId`;
- wrong `sourceChainId`;
- wrong `sourceToken`;
- wrong `sourceBurnTxHash`;
- wrong `sourceBurnEventIndex`;
- wrong `canonicalEventKey`;
- wrong `x1RecipientHash`;
- wrong `burnedAmount`;
- wrong `sourceChainWeightBps`;
- wrong `xxxlMintAmount`;
- wrong `mintToken`;
- wrong `deadlineOrFinalityBlock`;
- wrong `messageNonce`;
- field order mismatch;
- missing field;
- extra field;
- invalid integer/decimal encoding;
- malformed canonical bytes.

Each failure must be fail-closed and must not enable guardian/quorum/auth/mint.

## SAFETY_FLAGS Expectations

Phase 41G planning must not flip any runtime flag.

Future implementation may introduce a narrow payload-binding flag only if separately reviewed.

Even after payload binding succeeds, the following must remain false:

- guardian validity accepted;
- quorum counting enabled;
- authorization enabled;
- replay write enabled;
- processed event marking enabled;
- account mutation enabled;
- CPI enabled;
- `invoke_signed` enabled;
- SPL Token `mint_to` enabled;
- process instruction handler added;
- live route enabled.

## Still Forbidden In Phase 41G.0

The following remain forbidden:

- runtime code;
- local cryptographic verification;
- guardian validity acceptance;
- guardian set membership acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41G.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Review Questions

External review should answer:

1. Is Phase 41G.0 the correct next step after accepted Phase 41F?
2. Is payload binding correctly separated from guardian/quorum/auth?
3. Is the preferred signed-message-equals-payload-hash model acceptable?
4. Is `keccak256(canonical_gateway_payload_bytes)` the correct planned hash model?
5. Is the canonical field list complete and consistent with Stage 1?
6. Are route, burn event, recipient, amount, mint, finality, and nonce binding requirements complete?
7. Is public key handling correctly deferred to guardian validation?
8. Are negative cases sufficient?
9. Are all forbidden operations still forbidden?
10. Can Phase 41G.1 payload evidence shape begin after acceptance?

## Next Gate

Phase 41G.0 is a docs-only planning gate.

After external acceptance, Phase 41G.1 may begin under a separate reviewed boundary.

# Phase 41K.6 B5.1 — Watcher/relayer schema reconciliation inventory

## Purpose

B5.1 maps the previous Stage 2 watcher/relayer prototype fields onto the new Phase 41K.6 B1-B4 SVM handler boundary.

The goal is to avoid duplicating or blindly reusing stale watcher/relayer shapes.

B5.1 does not remove gates.

B5.1 does not introduce live RPC.

B5.1 does not sign, submit, simulate, spend SOL, or access private keys.

## Starting point

Previous evidence already exists for:

- Stage 2.18 watcher-event normalized task adapter.
- Stage 2.19 watcher-event full submit pipeline.
- Stage 2.20 watcher-event submit idempotency and retry.
- Stage 2.21 ambiguous recovery.
- Stage 2.22 watcher-event operational submit wrapper.
- Stage 2.23 watcher-event batch and queue processing.
- Stage 2.24 durable relayer journal model.
- Stage 2.25 watcher-to-relayer contract boundary.
- Stage 4 no-send and no-SOL readiness chain.
- Stage 5 external wallet live-send path.

The new Phase 41K.6 handler boundary now requires reconciliation against:

- B1C payload v2 hash binding.
- Guardian membership and unique quorum.
- processed_event PDA identity.
- route_id binding.
- target SPL mint binding.
- recipient token account binding.
- amount binding.
- guardian_set_id bytes32 binding.
- strictly prior Ed25519 evidence instruction model.
- B3 hostile rejection matrix.
- B4 gated production-readiness boundary.

## Field reconciliation table

| Old Stage 2 field | Previous meaning | B5.1 classification | New Phase 41K.6 mapping |
|---|---|---|---|
| eventId | Watcher-to-relayer event identifier | still valid | Operational id only; must not replace canonicalEventKey |
| journalId | Durable relayer journal identifier | still valid | Operational journal id only |
| dedupeKey | Relayer dedupe key | valid but needs binding rules | Should derive from or include canonicalEventKey |
| sourceChainId | Source chain id | still valid | Must match handler source_chain_id / route config |
| sourceTxHash | Ethereum burn tx hash | still valid | Source identity input for canonicalEventKey |
| sourceLogIndex | Ethereum log index | still valid | Source identity input for canonicalEventKey |
| sourceBlockNumber | Ethereum block number | still valid | Finality and source proof metadata |
| sourceFinalityState | finalized, safe, or confirmed | still valid | Watcher/finality metadata; not enough by itself for handler authorization |
| watcherEvent | Embedded watcher event payload | valid but must be reshaped | Must become Phase 41K.6 gateway mint candidate |
| mode | Operational mode | still valid | Relayer policy only; must not alter signed payload |
| expectedMintedAmountOverride | Test/prototype override | stale or restricted | Must not exist in production candidate unless explicitly test-only |
| canonicalEventKeyHex | bytes32 event key | still valid | Must map to processed_event PDA derivation and replay protection |
| recipientBase58 | Previous recipient field | needs clarification | Must distinguish recipient owner from recipient token account |
| mintedAmount | Mint amount | valid but needs strict type | Must map to handler amount and SPL u64 MintTo boundary |
| guardianSetVersion | Old guardian set identifier | stale shape | Must become guardian_set_id bytes32 |
| deadlineOrFinalityBlock | Deadline/finality guard | still valid | Candidate metadata; handler binding depends on current instruction model |
| messageNonceHex | Message nonce | valid if still required | Must be reconciled with canonical event and payload v2 model |
| guardianSigners | Guardian signer identities | still valid | Must be checked against guardian set account membership |
| minQuorum | Required quorum | still valid | Must match active guardian set threshold policy |
| currentFinalityBlock | Adapter finality context | still valid | Watcher/finality validation only |

## New required candidate shape

A Phase 41K.6 watcher candidate should contain two groups of fields.

### Source observation fields

- source_chain_id
- source_token
- source_sender
- source_burn_tx_hash
- source_burn_event_index
- source_block_number
- source_block_hash
- source_finality_state
- burned_amount
- canonical_event_key

### Handler binding fields

- route_id
- processed_event_pda
- target_spl_mint
- recipient_token_account
- amount
- guardian_set_id
- payload_v2_hash

The payload_v2_hash must be derived from the same binding fields the handler recomputes from current instruction and accounts.

## New required quorum package shape

A Phase 41K.6 quorum package should contain:

- payload_v2_hash
- guardian_set_id
- threshold
- unique guardian evidence entries
- evidence format identifier
- evidence instruction order expectation

Each evidence entry should contain:

- guardian public key
- signature or prior Ed25519 instruction data reference
- signed message bytes or message hash according to the selected evidence model
- evidence source index if represented as prior instruction position

## Boundary rules

The relayer must not change any payload-bound field after guardian signing.

Payload-bound fields include:

- processed_event_pda
- route_id
- target_spl_mint
- recipient_token_account
- amount
- guardian_set_id

If any of these drift, B3 proves the handler rejects before mutation.

Operational fields may change only if they are not payload-bound and do not affect handler semantics.

Operational fields include:

- eventId
- journalId
- retry counters
- operator report ids
- local batch ids
- queue position

## Stale-field decisions

The following old fields need special handling:

### guardianSetVersion

Decision:

Do not carry guardianSetVersion forward as the primary handler identifier.

New shape:

guardian_set_id bytes32.

Reason:

Phase 41K.6 handler authorization binds guardian_set_id, not a numeric guardian set version.

### recipientBase58

Decision:

Do not use recipientBase58 without clarification.

New shape:

Separate recipient_owner and recipient_token_account where both are needed.

Reason:

B3 recipient binding proves the payload is bound to the recipient token account used by SPL MintTo.

### expectedMintedAmountOverride

Decision:

Keep test-only unless a later production policy explicitly reintroduces it.

Reason:

Production amount must be deterministic from the burn and route policy. A relayer-side override must not silently alter signed payload semantics.

### sourceFinalityState

Decision:

Keep as watcher/finality metadata.

Reason:

Finality is required before candidate creation, but finality metadata alone is not handler authorization.

## B5.1 conclusion

B5.1 establishes that the old watcher/relayer prototype should be reused as a conceptual base, but its schema must be reconciled with the new Phase 41K.6 handler contract.

The next step is B5.2:

B5.2 — deterministic candidate builder and payload v2 hash conversion test.

B5.2 should implement a pure conversion from a reconciled watcher candidate into the handler-bound payload fields and verify that changing recipient, mint, amount, route_id, processed_event, or guardian_set_id changes the payload hash.

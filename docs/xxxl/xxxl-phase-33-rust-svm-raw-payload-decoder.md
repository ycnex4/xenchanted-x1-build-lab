# XXXL Phase 33 Rust/SVM Raw Payload Decoder

Status: Phase 33 narrow Rust/SVM raw payload decoder implementation.

## Purpose

Phase 33 is the first implementation step after the Phase 32 read-only
Rust/SVM verifier scaffold.

Phase 33 implements only the raw payload decoder boundary component.

The decoder follows the Phase 23 TypeScript canonical binary layout.

Phase 33 does not implement a full runtime verifier.

Phase 33 does not unlock runtime execution.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Runtime verifier work must not trust TypeScript authorization output.

TypeScript parity suite results are not runtime authority.

## Rust/SVM Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/raw_payload.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change was required because Phase 32 already
exports `pub mod verifier;`.

The decoder marker is:

~~~text
RAW_PAYLOAD_DECODER_PHASE_33
~~~

The decoder version is:

~~~text
1
~~~

## Decoder Behavior

The decoder consumes Phase 23 canonical guardian payload bytes in this order:

- `message_type`
- `schema_version`
- `instruction_layout_version`
- `route_id`
- `source_chain_id`
- `source_token`
- `source_sender`
- `source_burn_tx_hash`
- `source_burn_event_index`
- `source_block_number`
- `source_block_hash`
- `source_finality_block`
- `canonical_event_key`
- `x1_recipient`
- `burned_amount`
- `source_chain_weight_bps`
- `xxxl_mint_amount`
- `target_mint`
- `guardian_set_id`
- `message_nonce`
- `expiration_slot_or_unix_ts`

The decoder uses the Phase 23 binary widths:

- u16 little-endian length prefixes for variable bytes
- u16 little-endian schema and layout values
- u64 little-endian integer fields
- u128 little-endian amount fields
- 32-byte fixed-width byte fields

The decoded struct borrows payload slices where possible.

## Rejections Implemented

Phase 33 rejects:

- truncated payloads
- trailing bytes
- empty variable-length fields
- malformed length-prefixed encoding that overruns the payload
- unsupported message type
- unsupported schema version
- unsupported instruction layout version

The Phase 28 `wrong-byte-encoding` fixture is rejected at decoder level when the
first message-type length byte is corrupted.

## Wrong Field Order Honesty

Phase 33 includes a structurally detectable wrong-field-order test where
`source_token` bytes are placed before `source_chain_id`.

That malformed byte order is rejected because the decoder later observes an
empty variable-length field.

This does not prove that every possible field-order swap is rejected by the raw
decoder.

Swaps between fields with identical structural shape may remain valid raw bytes
and must be handled by later canonical validation, payload hash verification, or
semantic verifier phases.

Therefore Phase 33 does not claim that all `wrong-field-order` obligations are
satisfied.

## Future Runtime Cases

Phase 33 focuses only on decoder-level behavior for:

- `wrong-byte-encoding`
- structurally detectable `wrong-field-order`

The following Phase 30 future-runtime cases remain unsatisfied:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

Some `wrong-field-order` variants also remain future verifier obligations when
they are not structurally detectable at raw decode time.

## Explicit Non-Goals

Phase 33 does not implement Ed25519 verification.

Phase 33 does not implement guardian quorum.

Phase 33 does not implement source proof verification.

Phase 33 does not implement route config verification.

Phase 33 does not implement target mint account legitimacy verification.

Phase 33 does not implement amount cap enforcement.

Phase 33 does not implement replay storage, replay checks, or replay writes.

Phase 33 does not parse runtime accounts.

Phase 33 does not add instruction processing.

Phase 33 does not enable live route execution.

Phase 33 does not enable SPL CPI.

Phase 33 does not enable `invoke_signed`.

Phase 33 does not enable SPL Token `mint_to`.

Phase 33 does not add mint execution.

Phase 33 does not mutate runtime/account state.

Phase 33 does not mark processed events.

Phase 33 does not select a production Program ID.

Phase 33 does not regenerate production PDA fixtures.

Phase 33 does not remove deployment blockers.

Phase 33 does not claim production readiness.

Phase 33 does not claim final immutability while upgrade authority exists.

Phase 33 does not change Cargo manifests.

Phase 33 does not change package manifests.

Phase 33 does not build SBF artifacts.

Phase 33 does not touch `target/deploy`.

Phase 33 does not read or modify keypair files.

Phase 33 does not read or modify `.env`.

Phase 33 does not inspect `.local-keys`.

Phase 33 does not run deploy commands.

Phase 33 does not run network commands.

Phase 33 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 33.

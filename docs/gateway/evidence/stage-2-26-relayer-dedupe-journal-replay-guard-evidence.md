# Stage 2.26 Relayer Dedupe Journal Replay Guard Evidence

This document records Stage 2.26 relayer dedupe / journal replay guard evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-26-relayer-dedupe-journal-replay-guard

Runtime commit:

    1bd9743 Add Stage 2.26 relayer dedupe journal replay guard

Base runtime commit:

    5a8a2fb Add Stage 2.25 watcher to relayer contract boundary

## Scope

Stage 2.26 adds a prototype dedupe / replay guard for watcher-to-relayer contracts.

It builds on the Stage 2.25 watcher-to-relayer contract boundary and the Stage 2.24 durable journal model.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_relayer_dedupe_journal_replay_guard.test.ts

## New dedupe record model

New dedupe record:

    Stage2WatcherToRelayerDedupeRecord

Each record contains:

- sequence
- eventId
- journalId
- dedupeKey
- canonicalEventKeyHex
- watcherPayloadFingerprint

The dedupe records are stored on the prototype journal as:

    contractRecords

## Fingerprint policy

The watcher payload fingerprint records public watcher contract fields.

The fingerprint includes:

- eventId
- journalId
- dedupeKey
- source metadata
- source finality state
- programId
- canonicalEventKeyHex
- recipientBase58
- mintedAmount
- guardianSetVersion
- deadlineOrFinalityBlock
- messageNonceHex
- guardian public keys
- minQuorum
- currentFinalityBlock
- mode
- expectedMintedAmountOverride

The fingerprint does not serialize guardian secret keys.

## New helper

New helper:

    acceptStage2WatcherToRelayerContractWithDedupeGuardPrototype

The helper first validates the watcher-to-relayer contract through the Stage 2.25 boundary.

Then it checks the durable journal contractRecords.

## Guard outcomes

Possible outcomes:

    accepted
    duplicate_existing
    manual_review_required
    invalid_contract

Manual review reasons:

    dedupe_key_payload_mismatch
    canonical_event_key_dedupe_mismatch

## Confirmed behavior

The Stage 2.26 test confirms:

- a new watcher contract is accepted
- accepted contract creates a dedupe record
- same contract after journal reload is classified as duplicate_existing
- same dedupeKey with different payload is manual_review_required
- same canonicalEventKey with different dedupeKey is manual_review_required
- malformed watcher-to-relayer metadata returns invalid_contract
- malformed embedded watcher event returns invalid_contract
- invalid watcher event reason is propagated
- accepted dedupe-guard output can be processed through the journaled relayer path
- duplicate import after reload does not mint again

## Accepted contract behavior

For a new watcher contract:

Expected status:

    accepted

Expected record:

- sequence = 0
- eventId = contract.eventId
- journalId = contract.journalId
- dedupeKey = contract.dedupeKey
- canonicalEventKeyHex = contract.watcherEvent.canonicalEventKeyHex

Expected journal effect:

    contractRecords.length = 1

## Duplicate behavior after reload

The journal is serialized and deserialized.

The same watcher contract is submitted to the dedupe guard again.

Expected status:

    duplicate_existing

Expected behavior:

- existing record is returned
- no new contract record is appended
- contractRecords.length remains 1

## Payload conflict behavior

If the same dedupeKey is reused with a different watcher payload:

Expected status:

    manual_review_required

Expected reason:

    dedupe_key_payload_mismatch

Expected behavior:

- existing record is returned
- no new contract record is appended

## Canonical event conflict behavior

If the same canonicalEventKey is reused with a different dedupeKey:

Expected status:

    manual_review_required

Expected reason:

    canonical_event_key_dedupe_mismatch

Expected behavior:

- existing record is returned
- no new contract record is appended

## Invalid contract behavior

Malformed metadata example:

    dedupeKey = ""

Expected status:

    invalid_contract

Expected reason:

    invalid_dedupe_key

Malformed watcher event example:

    canonicalEventKeyHex = 0x1234

Expected status:

    invalid_contract

Expected reason:

    invalid_watcher_event

Expected watcherEventReason:

    invalid_canonical_event_key_hex

## Journaled relayer integration

An accepted dedupe-guard result can be processed through:

    processStage2WatcherEventOperationalBatchWithJournalPrototype

Confirmed submit status:

    submitted

Confirmed balance delta:

    78901

After serialization and deserialization of the dedupe journal, the same contract is classified as:

    duplicate_existing

Confirmed duplicate import balance behavior:

- balance after duplicate import equals balance after first submit
- no replay mint occurs

## Stage 2.26 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_relayer_dedupe_journal_replay_guard.test.ts

Result:

    Stage 2.26 relayer dedupe journal replay guard
      ✔ accepts a new watcher contract and records its dedupe entry
      ✔ classifies the same watcher contract as duplicate_existing after journal reload
      ✔ requires manual review for the same dedupeKey with a different payload
      ✔ requires manual review for the same canonicalEventKey with a different dedupeKey
      ✔ returns invalid_contract for malformed watcher-to-relayer contracts
      ✔ processes accepted dedupe-guard output through the journaled relayer path and blocks replay import

    6 passing

## Regression checks

Stage 2.25 watcher-to-relayer contract boundary remained green:

    Stage 2.25 watcher-to-relayer contract boundary
      ✔ accepts a valid watcher-to-relayer contract and converts it into a batch item
      ✔ rejects malformed watcher-to-relayer contract metadata before relayer submit
      ✔ rejects malformed watcher event payload at the watcher-to-relayer boundary
      ✔ processes an accepted watcher-to-relayer contract through the journaled relayer path

    4 passing

Stage 2.24 durable relayer journal model remained green:

    Stage 2.24 durable relayer journal model
      ✔ records, serializes, reloads, and continues watcher-event batch processing safely
      ✔ rejects malformed serialized journals

    2 passing

Stage 2.23 watcher event batch / queue processing remained green:

    Stage 2.23 watcher event batch / queue processing
      ✔ processes a watcher-event queue and returns mixed operational outcomes

    1 passing

Stage 2.22 watcher event operational submit wrapper remained green:

    Stage 2.22 watcher event operational submit wrapper
      ✔ returns submitted for normal watcher-event operational submit
      ✔ returns already_processed for repeated watcher-event operational submit
      ✔ returns safe_retry_candidate for ambiguous recovery before submit
      ✔ returns completed_no_retry for ambiguous recovery after submitted watcher event
      ✔ returns stop_manual_review for inconsistent ambiguous watcher-event recovery
      ✔ returns watcher_event_rejected for malformed watcher event

    6 passing

Stage 2.21 watcher event ambiguous recovery remained green:

    Stage 2.21 watcher event ambiguous recovery
      ✔ recovers ambiguous watcher-event submit results without blind retry

    1 passing

Stage 2.20 watcher event submit idempotency / retry remained green:

    Stage 2.20 watcher event submit idempotency / retry
      ✔ stops safely when the same watcher event is submitted twice

    1 passing

Stage 2.19 watcher event full submit pipeline remained green:

    Stage 2.19 watcher event full submit pipeline
      ✔ submits a watcher event through adapter, normalization, and protected submit path
      ✔ rejects malformed watcher event before submit
      ✔ propagates parsed watcher event preflight rejection before submit

    3 passing

Stage 2.18 watcher event adapter remained green:

    Stage 2.18 watcher event to normalized task adapter
      ✔ adapts a watcher event into a deterministic normalized relayer task
      ✔ rejects malformed canonical event key hex
      ✔ rejects malformed recipient public key
      ✔ rejects malformed decimal fields before normalization
      ✔ propagates preflight rejection after parsing watcher event fields

    5 passing

Stage 2.17 normalized task submit wrapper remained green:

    Stage 2.17 normalized task submit wrapper
      ✔ submits a normalized relayer mint task through the integrated submit path
      ✔ keeps invalid watcher input outside the normalized submit wrapper

    2 passing

Stage 2.16 task normalization remained green:

    Stage 2.16 relayer task normalization
      ✔ normalizes a valid watcher task into deterministic relayer submit fields
      ✔ rejects invalid watcher task input before normalization
      ✔ copies byte arrays so normalized task is stable after source mutation

    3 passing

Stage 2.15 preflight-integrated submit path remained green:

    Stage 2.15 relayer preflight-integrated submit path
      ✔ rejects invalid input before building/submitting a transaction and preserves state
      ✔ still submits valid input through the integrated path

    2 passing

Stage 2.14 preflight validation remained green:

    Stage 2.14 relayer event input preflight guard
      ✔ accepts a valid relayer mint input
      ✔ rejects invalid canonical event keys
      ✔ rejects invalid message nonces
      ✔ rejects invalid recipients
      ✔ rejects zero minted amount
      ✔ rejects expired deadline or finality block
      ✔ rejects invalid quorum
      ✔ rejects insufficient guardian signers
      ✔ rejects duplicate guardian signers

    9 passing

Stage 2.13 operational state machine remained green:

    Stage 2.13 relayer operational state machine
      ✔ maps recovery states to completed, retry-candidate, and manual-review decisions

    1 passing

Stage 2.12 inconsistent recovery remained green:

    Stage 2.12 relayer inconsistent recovery state handling
      ✔ classifies processed burn with unexpected balance delta as inconsistent and does not retry blindly

    1 passing

Stage 2.11 ambiguous recovery remained green:

    Stage 2.11 relayer ambiguous confirmation recovery
      ✔ recovers a completed mint after an ambiguous send result by checking processed burn and balance

    1 passing

Stage 2.10 idempotency / retry remained green:

    Stage 2.10 relayer idempotency / retry prototype
      ✔ stops safely when processed burn already exists and does not mint twice

    1 passing

Stage 2.9 relayer prototype remained green:

    Stage 2.9 TypeScript relayer prototype
      ✔ builds and submits the relayer transaction shape for a direct mint

    1 passing

Stage 2.6 rollback matrix remained green:

    Stage 2 direct mint gateway skeleton
      ✔ verifies guardian signatures, initializes processed burn, mints tokens, rejects replay, and rolls back failed CPI
      ✔ rejects missing guardian signature instruction
      ✔ rejects wrong xxxl mint and leaves no processed burn
      ✔ rejects recipient token account with wrong mint and leaves no processed burn
      ✔ rejects recipient token account with wrong owner and leaves no processed burn
      ✔ rejects unknown guardian even with valid Ed25519 signatures

    6 passing

Rust / Anchor checks remained green:

    cargo test -p hello-x1 binding_
    cargo test -p hello-x1 parser_
    anchor build

## Current conclusion

Stage 2.26 creates a relayer-side dedupe / replay guard for watcher-to-relayer contracts.

The relayer can now record accepted watcher contracts in a durable journal-side contractRecords list.

A repeated identical watcher contract is classified as duplicate_existing.

A repeated dedupeKey with different payload is routed to manual review.

A repeated canonicalEventKey with a different dedupeKey is routed to manual review.

Malformed contracts are rejected before dedupe records are created.

An accepted dedupe-guard result can still flow into the Stage 2.24 journaled relayer path.

Duplicate import after journal reload does not mint again.

The on-chain runtime remains unchanged.

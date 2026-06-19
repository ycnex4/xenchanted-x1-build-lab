# Stage 2.25 Watcher-to-Relayer Contract Boundary Evidence

This document records Stage 2.25 watcher-to-relayer contract boundary evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-25-watcher-to-relayer-contract-boundary

Runtime commit:

    5a8a2fb Add Stage 2.25 watcher to relayer contract boundary

Base runtime commit:

    4c8d930 Add Stage 2.24 durable relayer journal model

## Scope

Stage 2.25 adds a prototype watcher-to-relayer contract boundary.

It formalizes the object that a watcher hands to the relayer before the relayer converts it into a batch item.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_watcher_to_relayer_contract_boundary.test.ts

## New contract input

The watcher-to-relayer contract contains:

- eventId
- journalId
- dedupeKey
- sourceChainId
- sourceTxHash
- sourceLogIndex
- sourceBlockNumber
- sourceFinalityState
- watcherEvent
- optional mode
- optional expectedMintedAmountOverride

Supported source finality states:

    finalized
    safe
    confirmed

## New acceptance helper

New helper:

    acceptStage2WatcherToRelayerContractPrototype

The helper validates watcher-side metadata before relayer submit.

It returns either:

    ok: true
    contract
    batchItem

or:

    ok: false
    reason
    optional watcherEventReason

## Metadata validation

The helper rejects:

- empty eventId
- empty journalId
- empty dedupeKey
- invalid sourceChainId
- invalid sourceTxHash
- invalid sourceLogIndex
- invalid sourceBlockNumber
- invalid sourceFinalityState

Expected metadata failure reasons:

    invalid_event_id
    invalid_journal_id
    invalid_dedupe_key
    invalid_source_chain_id
    invalid_source_tx_hash
    invalid_source_log_index
    invalid_source_block_number
    invalid_source_finality_state

## Watcher event validation

The helper also validates the embedded watcher event payload by adapting it through the Stage 2.18 watcher-event adapter.

Malformed watcher event payloads are rejected before submit.

Expected watcher event boundary failure:

    invalid_watcher_event

Example propagated watcher event reason:

    invalid_canonical_event_key_hex

## Batch item conversion

A valid watcher-to-relayer contract is converted into a Stage 2.23 operational batch item.

The generated batch item uses:

- id = contract.eventId
- event = contract.watcherEvent
- mode = contract.mode
- expectedMintedAmountOverride = contract.expectedMintedAmountOverride

## Journaled relayer integration

The accepted contract can be processed through the Stage 2.24 journaled relayer path.

Confirmed behavior:

- accepted contract becomes one batch item
- batch result keeps contract.eventId
- result index is 0
- submit status is submitted
- journal records are created
- journal itemId equals contract.eventId
- token balance increases by the expected minted amount

Expected journal record kinds:

    event_received
    operational_result
    final_outcome

Confirmed balance delta:

    24680

## Stage 2.25 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_to_relayer_contract_boundary.test.ts

Result:

    Stage 2.25 watcher-to-relayer contract boundary
      ✔ accepts a valid watcher-to-relayer contract and converts it into a batch item
      ✔ rejects malformed watcher-to-relayer contract metadata before relayer submit
      ✔ rejects malformed watcher event payload at the watcher-to-relayer boundary
      ✔ processes an accepted watcher-to-relayer contract through the journaled relayer path

    4 passing

## Regression checks

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

Stage 2.25 creates a formal watcher-to-relayer contract boundary.

The relayer now has a prototype acceptance layer for watcher-provided objects before submit.

Metadata errors are rejected before relayer execution.

Malformed watcher event payloads are rejected at the boundary and propagate watcher-event adapter reasons.

Accepted contracts can be converted into operational batch items and processed through the durable journaled relayer path.

The on-chain runtime remains unchanged.

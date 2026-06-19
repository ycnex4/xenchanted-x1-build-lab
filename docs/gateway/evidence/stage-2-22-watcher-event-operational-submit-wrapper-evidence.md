# Stage 2.22 Watcher Event Operational Submit Wrapper Evidence

This document records Stage 2.22 watcher event operational submit wrapper evidence for the X1 direct mint gateway relayer prototype.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-2-22-watcher-event-operational-submit-wrapper

Runtime commit:

    1a75653 Add Stage 2.22 watcher event operational submit wrapper

Base runtime commit:

    2dc3ad1 Add Stage 2.21 watcher event ambiguous recovery

## Scope

Stage 2.22 adds a high-level operational submit wrapper for a single watcher event.

It combines the watcher event adapter, normalized task submit path, idempotency handling, and ambiguous recovery decision logic behind one prototype helper.

It does not change the on-chain runtime.

## Runtime changes

Runtime helper updated:

    tests/helpers/stage2RelayerPrototype.ts

Runtime test added:

    tests/stage2_watcher_event_operational_submit_wrapper.test.ts

## New helper

The new helper is:

    submitStage2WatcherMintEventOperationalPrototype

It accepts:

- provider
- program
- payer
- xxxlMint
- recipientTokenAccount
- event
- optional mode
- optional balanceBeforeOverride
- optional expectedMintedAmountOverride

## Operational modes

Supported modes:

    submit
    recover_ambiguous
    submit_then_recover_ambiguous

Default mode:

    submit

## Result model

The operational wrapper can return:

    watcher_event_rejected
    submitted
    already_processed
    completed_no_retry
    safe_retry_candidate
    stop_manual_review

## Tested path

The tested path is:

    watcher event
      -> submitStage2WatcherMintEventOperationalPrototype
      -> adaptStage2WatcherMintEventToNormalizedTask
      -> submitStage2RelayerNormalizedMintTask
      -> submitStage2RelayerMintPrototype
      -> optional ambiguous recovery inspection
      -> operational decision

Recovery helpers used:

    inspectStage2RelayerAmbiguousResult
    decideStage2RelayerOperationalAction

## Confirmed behavior

Stage 2.22 confirms six operational outcomes.

Normal watcher-event operational submit:

- returns submitted
- returns a signature
- mints the expected amount

Repeated watcher-event operational submit:

- first call returns submitted
- second call returns already_processed
- second call returns signature = null
- second call does not mint again

Ambiguous recovery before submit:

- returns safe_retry_candidate
- reason is not_processed_after_ambiguous_result
- processedBurnExists is false
- balanceDelta is zero

Submit then ambiguous recovery:

- returns completed_no_retry
- reason is confirmed_after_ambiguous_result
- processedBurnExists is true
- balanceDelta equals expected minted amount

Inconsistent ambiguous recovery:

- returns stop_manual_review
- reason is inconsistent_after_ambiguous_result
- processedBurnExists is true
- balanceDelta equals actual minted amount

Malformed watcher event:

- returns watcher_event_rejected
- reason is invalid_canonical_event_key_hex
- signature is null

## Stage 2.22 test

Command:

    ANCHOR_PROVIDER_URL=https://rpc.testnet.x1.xyz \
    ANCHOR_WALLET=~/.config/solana/id.json \
    npx ts-mocha -p ./tsconfig.json -t 120000 tests/stage2_watcher_event_operational_submit_wrapper.test.ts

Result:

    Stage 2.22 watcher event operational submit wrapper
      ✔ returns submitted for normal watcher-event operational submit
      ✔ returns already_processed for repeated watcher-event operational submit
      ✔ returns safe_retry_candidate for ambiguous recovery before submit
      ✔ returns completed_no_retry for ambiguous recovery after submitted watcher event
      ✔ returns stop_manual_review for inconsistent ambiguous watcher-event recovery
      ✔ returns watcher_event_rejected for malformed watcher event

    6 passing

## Regression checks

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

Stage 2.22 creates a production-shaped prototype boundary for processing one watcher event.

The relayer can now call one high-level helper and receive an operational outcome instead of manually stitching together watcher parsing, submit, idempotency, and ambiguous recovery.

The on-chain runtime remains unchanged.

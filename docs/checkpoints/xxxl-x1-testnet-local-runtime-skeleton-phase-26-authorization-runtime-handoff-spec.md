# XXXL X1 Testnet Local Runtime Skeleton — Phase 26 Authorization Runtime Handoff Spec

Status: Draft
Type: specification-only phase
Base commit: `0f2d712`
Base checkpoint: Off-Chain Gateway Authorization Stack Complete

## Summary

Phase 26 creates the authorization-runtime handoff specification.

This phase intentionally does not implement runtime logic.

The purpose is to define how the completed off-chain authorization stack can hand structured proof data to a future runtime verifier without introducing a boolean-trust model.

## Core security decision

The selected model is:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Runtime must not trust an off-chain boolean.

Future runtime must independently verify critical authorization data from a structured proof bundle.

## Main spec document

Primary document:

~~~text
docs/gateway/phase-26-authorization-runtime-handoff-spec.md
~~~

## Scope

Phase 26 documents:

- canonical AuthProofBundle direction
- runtime MUST-verify checks
- guardian set lifecycle requirements
- replay storage and atomicity requirements
- nonce semantics draft decision
- expiration semantics draft decision
- amount/rate limit requirements
- target mint legitimacy requirements
- TS-to-SVM parity matrix
- failure behavior taxonomy
- Phase 27 review checklist

## Draft decisions

Phase 26 currently proposes the following reviewable defaults:

- `message_nonce` is a uniqueness-only signed message identifier
- `canonical_event_key` remains the primary replay identity
- runtime expiration should be slot-based
- expiration boundary remains inclusive:
  - `current_slot == expiration_slot` is valid
  - `current_slot > expiration_slot` is expired
- runtime execution requires on-chain `ProcessedEvent` storage
- off-chain DB may exist only as index/preflight support
- minimum amount cap before runtime execution: per-route max amount per message
- emergency halt must be explicit runtime config state

These are draft decisions, not production decisions.

## Preserved blockers

Phase 26 preserves all current blockers:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Explicit non-goals

Phase 26 does not:

- implement runtime code
- modify `programs/xxxl-svm`
- modify Cargo files
- modify package files
- build SBF artifacts
- touch `target/deploy`
- touch keypair files
- inspect `.local-keys`
- read `.env`
- enable live route
- enable SPL CPI
- enable `invoke_signed`
- enable SPL Token `mint_to`
- mutate runtime/account state
- mark processed events
- claim production readiness
- claim final immutability while upgrade authority exists

## Review gate before Phase 27

Phase 27 must not start until this spec is reviewed.

The next target checkpoint is:

~~~text
Authorization-Runtime Handoff Spec Reviewed
~~~

Possible Phase 27 candidates after review:

- runtime handoff account mapping
- guardian set lifecycle model
- replay storage design model
- TS/SVM parity vector suite

No live execution should be enabled by Phase 27 without a separate reviewed boundary.

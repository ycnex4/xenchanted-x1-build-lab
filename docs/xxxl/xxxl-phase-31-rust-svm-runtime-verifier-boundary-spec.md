# XXXL Phase 31 Rust/SVM Runtime Verifier Boundary Spec

Status: Phase 31 docs-only runtime verifier boundary specification.

## Purpose

Phase 31 defines the reviewed boundary for a future Rust/SVM runtime verifier.

Phase 31 follows the completed TypeScript parity chain:

- Phase 27: TS/SVM parity vector contract
- Phase 28: concrete invalid parity fixtures
- Phase 29: verifier-oriented parity classification
- Phase 30: execution-backed TypeScript parity validation

Phase 30 proved that all 12 TypeScript-model validation cases can be rejected by
existing TypeScript verifier/model functions.

Phase 31 does not implement Rust/SVM code.

Phase 31 does not unlock runtime execution.

Phase 31 specifies what a future Rust/SVM verifier must independently check
before any later implementation phase can be considered.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The runtime verifier must not trust TypeScript authorization output.

The runtime verifier must independently verify authority-bearing data.

A TypeScript result such as `authorized=true`, `TS_EXECUTION_BACKED_REJECTION`,
or any parity suite result is not runtime authority.

## Reviewed Inputs

The future runtime verifier boundary is based on:

- Phase 22 payload and source burn identity model
- Phase 23 canonical guardian payload vector and encoding/hash model
- Phase 24 guardian approval and quorum verifier model
- Phase 25 gateway authorization boundary model
- Phase 26 authorization-runtime handoff review
- Phase 27 TS/SVM parity vector suite
- Phase 28 concrete invalid parity fixtures
- Phase 29 verifier-oriented validation matrix
- Phase 30 execution-backed TS parity validation

## Runtime Verifier Boundary Components

A future Rust/SVM verifier may be split into reviewed components.

Each component must be implemented and reviewed independently before any live
execution path exists.

### 1. Raw Payload Decoder

The runtime verifier must decode raw input bytes into the reviewed Phase 22/23
guardian payload structure.

It must reject:

- wrong field order
- wrong byte encoding
- truncated fields
- extra bytes
- malformed fixed-width fields
- malformed length-prefixed fields
- unsupported schema version
- unsupported instruction layout version

This component closes the Phase 30 future-runtime cases:

- `wrong-field-order`
- `wrong-byte-encoding`

### 2. Canonical Payload Validation

The runtime verifier must enforce the Phase 23 canonical payload rules.

It must independently check:

- message type
- schema version
- instruction layout version
- field order
- fixed-width byte lengths
- variable byte lengths
- u64/u16/u128 ranges
- canonical binary encoding
- payload hash domain
- payload hash preimage
- payload hash result

The runtime must not accept caller-provided payload hashes without recomputation
or equivalent independent verification.

### 3. Source Proof Identity Verifier

The runtime verifier must validate that the payload source burn identity matches
the reviewed source proof identity.

This boundary must cover:

- source chain id
- source token
- source sender
- source burn transaction hash
- source burn event index
- source block number
- source block hash
- source finality block
- canonical event key preimage
- canonical event key result

This component closes the Phase 30 future-runtime cases:

- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`

Phase 31 does not decide whether the source proof is provided as a watcher proof,
guardian-signed preimage bundle, finalized Ethereum proof metadata, or another
reviewed format.

That choice requires a separate phase.

### 4. Guardian Approval And Quorum Verifier

The runtime verifier must independently verify guardian approval signatures.

It must check:

- Ed25519 public key length
- Ed25519 signature length
- signature over the Phase 23 payload hash
- guardian membership in the active guardian set
- duplicate guardian approvals
- quorum threshold
- guardian set id binding
- no unknown guardian contributes to quorum

This component corresponds to the Phase 24 TypeScript verifier behavior.

### 5. Route Binding Verifier

The runtime verifier must verify that the payload matches route configuration.

It must check:

- route id
- source chain id
- target mint
- guardian set id
- source chain weight bps
- route enabled/disabled status
- route-specific amount cap
- route-specific target mint legitimacy

This component closes the Phase 30 future-runtime case:

- `amount-over-route-cap`

### 6. Target Mint Legitimacy Verifier

The runtime verifier must verify that the target mint account is the expected
runtime mint for the route.

The check must be explicit and must not rely only on payload bytes.

The future implementation must define the exact account model before coding:

- configured target mint account
- mint authority expectations
- token program expectations
- ownership expectations
- PDA relationship if any
- immutable route binding if any

This component closes the Phase 30 future-runtime case:

- `invalid-target-mint`

### 7. Replay Verifier

The runtime verifier must reject already processed canonical event keys.

Phase 31 only defines the verification boundary.

It does not implement processed-event marking.

It does not create replay storage.

It does not write replay accounts.

Replay writes require a separate reviewed implementation phase.

### 8. Expiration Verifier

The runtime verifier must reject payloads when the runtime slot is greater than
the signed expiration slot.

The preserved Phase 26 default is:

~~~text
current_slot > expiration_slot => expired
current_slot == expiration_slot => still valid
~~~

### 9. Amount Control Verifier

The runtime verifier must verify amount controls before any mint path exists.

The minimum required amount control is:

- route max amount per message

Future phases may add:

- per-route daily amount cap
- per-guardian-set amount cap
- emergency pause amount policy
- chain-weighted amount policy

Those additions require separate review.

## Required Phase 31 Error Surface

The future Rust/SVM verifier should expose deterministic error categories for
all Phase 27/28/29/30 parity cases.

Minimum error categories:

- raw payload decode error
- canonical payload validation error
- payload hash domain error
- guardian signature error
- guardian quorum error
- route binding error
- source proof identity error
- replay error
- expiration error
- amount cap error
- target mint legitimacy error

Exact Rust/SVM error names are not chosen in Phase 31.

Choosing names and account-level return behavior requires a separate
implementation phase.

## What Phase 31 Does Not Do

Phase 31 does not implement Rust/SVM verifier code.

Phase 31 does not add `programs/xxxl-svm` source.

Phase 31 does not add SVM account structs.

Phase 31 does not add instruction handlers.

Phase 31 does not add account mutation.

Phase 31 does not add replay writes.

Phase 31 does not add SPL CPI.

Phase 31 does not add `invoke_signed`.

Phase 31 does not add SPL Token `mint_to`.

Phase 31 does not unlock a live route.

Phase 31 does not select a production Program ID.

Phase 31 does not regenerate PDA fixtures.

Phase 31 does not deploy.

Phase 31 does not spend SOL.

Phase 31 does not claim production readiness.

Phase 31 does not claim final immutability while upgrade authority exists.

## Future Implementation Rule

A future Rust/SVM verifier implementation must be split into reviewed phases.

The first implementation phase should be read-only verifier code only.

It must not include:

- mint execution
- SPL CPI
- replay writes
- processed-event marking
- live route unlock
- production deployment
- Program ID finalization

Runtime verification must be proven before runtime execution is considered.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 31.

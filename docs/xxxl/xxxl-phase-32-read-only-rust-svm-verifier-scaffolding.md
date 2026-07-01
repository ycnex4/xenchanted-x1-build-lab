# XXXL Phase 32 Read-Only Rust/SVM Verifier Scaffolding

Status: Phase 32 read-only Rust/SVM verifier scaffold.

## Purpose

Phase 32 is the first Rust/SVM code step after the Phase 31 runtime verifier
boundary specification.

Phase 31 defined the reviewed boundary for a future independent Rust/SVM
verifier.

Phase 32 adds a read-only scaffold that records the verifier boundary
components, future runtime obligations, deterministic error categories, and
safety flags.

Phase 32 does not implement runtime verifier logic.

Phase 32 does not unlock runtime execution.

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

New Rust modules:

- `programs/xxxl-svm/src/verifier/mod.rs`
- `programs/xxxl-svm/src/verifier/boundary.rs`
- `programs/xxxl-svm/src/verifier/errors.rs`
- `programs/xxxl-svm/src/verifier/types.rs`

Updated Rust export:

- `programs/xxxl-svm/src/lib.rs`

The scaffold marker is:

~~~text
READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32
~~~

The scaffold version is:

~~~text
1
~~~

## Boundary Components

Phase 32 carries forward the 10 Phase 31 verifier boundary components:

- raw payload decoder
- canonical payload validation
- source proof identity verifier
- guardian approval and quorum verifier
- route binding verifier
- target mint legitimacy verifier
- replay verifier
- expiration verifier
- amount control verifier
- deterministic error surface

These are listed as scaffold components only.

Phase 32 does not implement these verifiers.

## Future Runtime Cases

Phase 32 carries forward the 7 Phase 30 future-runtime cases:

- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

Every case is marked as a future verifier obligation.

No future-runtime case is marked implemented or satisfied.

## Error Categories

Phase 32 adds scaffold-only error categories:

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

These names are category-level placeholders.

Phase 32 does not choose final production Rust/SVM error behavior.

## Explicit Safety Flags

The read-only scaffold records these flags as `false`:

- `live_route_enabled`
- `spl_cpi_enabled`
- `invoke_signed_enabled`
- `mint_execution_enabled`
- `runtime_state_mutation_enabled`
- `replay_write_enabled`
- `processed_event_marking_enabled`
- `production_program_id_selected`
- `deployment_blockers_removed`

Unit tests verify these flags remain false.

## No Runtime Logic Implemented

Phase 32 does not implement:

- raw bytes decoder
- Ed25519 verification
- route account reading
- mint account legitimacy reading
- amount cap enforcement
- source proof verification
- replay storage
- account parsing
- instruction processing

Those require separate reviewed phases.

## Explicit Non-Goals

Phase 32 does not enable live route execution.

Phase 32 does not enable SPL CPI.

Phase 32 does not enable `invoke_signed`.

Phase 32 does not enable SPL Token `mint_to`.

Phase 32 does not add mint execution.

Phase 32 does not mutate runtime/account state.

Phase 32 does not add replay writes.

Phase 32 does not mark processed events.

Phase 32 does not select a production Program ID.

Phase 32 does not regenerate production PDA fixtures.

Phase 32 does not remove deployment blockers.

Phase 32 does not claim production readiness.

Phase 32 does not claim final immutability while upgrade authority exists.

Phase 32 does not change Cargo manifests.

Phase 32 does not build SBF artifacts.

Phase 32 does not touch `target/deploy`.

Phase 32 does not read or modify keypair files.

Phase 32 does not read or modify `.env`.

Phase 32 does not inspect `.local-keys`.

Phase 32 does not run deploy commands.

Phase 32 does not run network commands.

Phase 32 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 32.

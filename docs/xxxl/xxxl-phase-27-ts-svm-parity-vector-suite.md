# XXXL Phase 27 TS/SVM Parity Vector Suite

Status: Phase 27 TypeScript-only parity vector suite.

## Purpose

Phase 27 creates a bounded TS/SVM parity vector suite so future Rust/SVM
runtime verification cannot drift from the reviewed Phase 22, Phase 23, and
Phase 26 payload and handoff model.

The suite is not a runtime implementation.

The suite is not an execution path.

The suite is a fixed parity checklist and vector surface for future independent
runtime verifier work.

## Reviewed Inputs

Phase 27 uses these reviewed inputs:

- Phase 22 guardian payload semantic field order
- Phase 23 canonical guardian payload byte encoding and payload hash vector
- Phase 24 guardian signature/quorum verifier model
- Phase 25 verifier-to-runtime authorization boundary model
- Phase 26 authorization-runtime handoff spec review closure

The preserved Phase 26 security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

The future runtime must independently verify authority-bearing data.

The TypeScript layer must not become an execution authority.

## Source And Test Boundary

New source:

- `src/xxxl/ts-svm-parity-vector-suite.ts`

New tests:

- `tests/xxxl/ts-svm-parity-vector-suite.test.ts`

New export:

- `src/index.ts`

The valid canonical parity vector reuses:

- `XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR`

The valid canonical parity vector does not redefine payload encoding rules.

The Phase 23 module remains the canonical source for payload bytes, hash
preimage, and payload hash.

## Required Case Coverage

Phase 27 requires exactly these case ids:

- `valid-canonical-payload`
- `wrong-field-order`
- `wrong-byte-encoding`
- `wrong-hash-domain`
- `malformed-bytes32-field`
- `malformed-var-bytes-field`
- `invalid-source-chain-id`
- `wrong-target-mint`
- `wrong-guardian-set-id`
- `wrong-source-chain-weight`
- `invalid-signature`
- `duplicate-guardian-approval`
- `insufficient-quorum`
- `expired-payload`
- `duplicate-canonical-event-key`
- `wrong-canonical-event-key-preimage`
- `wrong-source-burn-tx-hash`
- `wrong-source-burn-event-index`
- `amount-over-route-cap`
- `invalid-target-mint`

Only `valid-canonical-payload` is accepted by the parity matrix.

Every invalid case has expected decision:

~~~text
REJECT_BEFORE_EXECUTION
~~~

Every case includes explicit future runtime check text.

## Preserved Security Decision

Phase 27 preserves:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

Phase 27 does not convert TypeScript authorization output into runtime
authority.

Phase 27 does not add an `authorized=true` handoff shortcut.

Phase 27 does not trust caller-provided payload hashes.

Phase 27 does not weaken the requirement that future runtime verification must
recompute or independently verify payload bytes, payload hash, guardian
signature/quorum, replay status, route bindings, target mint legitimacy,
expiration, and amount controls.

## Explicit Non-Goals

Phase 27 does not enable live route execution.

Phase 27 does not enable SPL CPI.

Phase 27 does not enable `invoke_signed`.

Phase 27 does not enable SPL Token `mint_to`.

Phase 27 does not mutate runtime/account state.

Phase 27 does not mark processed events.

Phase 27 does not select a production Program ID.

Phase 27 does not regenerate production PDA fixtures.

Phase 27 does not remove deployment blockers.

Phase 27 does not claim production readiness.

Phase 27 does not claim final immutability while upgrade authority exists.

Phase 27 does not modify `programs/xxxl-svm`.

Phase 27 does not modify Cargo files.

Phase 27 does not build SBF artifacts.

Phase 27 does not touch `target/deploy`.

Phase 27 does not read or modify keypair files.

Phase 27 does not read or modify `.env`.

Phase 27 does not inspect `.local-keys`.

Phase 27 does not run deploy, network, or SOL-spend commands.

## No Live Execution Enabled

The parity suite is a pre-runtime vector suite.

It does not produce an instruction.

It does not submit a transaction.

It does not change runtime/account state.

It does not create a mint path.

It does not mark replay storage.

It only defines parity cases that future Rust/SVM verification must match before
any separately reviewed runtime implementation can be considered.

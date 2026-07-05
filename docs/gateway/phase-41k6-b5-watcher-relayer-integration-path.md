# Phase 41K.6 B5 — Watcher/relayer integration path

## Purpose

Phase 41K.6 B5 defines the off-chain integration path that prepares valid X1 gateway mint submissions for the already proven B1/B2/B3 handler boundary.

B5 starts after B4 closed the activation gate decision:

- the B2/B3 handler path remains gated,
- dangerous SBF-build allow features remain non-production,
- the Mollusk Ed25519 no-op harness stub remains test-only,
- no ungated production handler activation happens in B4,
- the next checkpoint is watcher/relayer integration.

B5 does not remove gates.

B5 does not deploy to X1 testnet.

B5 defines and tests the off-chain path that should eventually feed the gated handler path.

## Current checkpoint

Closed on main:

- C: merge checkpoint.
- D: negative/failure mode tests.
- B1: guardian quorum authorization.
- B2: valid quorum live-gated success test.
- B3: hostile live-gated matrix.
- B4: activation gate decision / production-readiness boundary.

Current main checkpoint:

d4cf1ef Merge phase 41K.6 B4 activation gate boundary

## B5 target flow

B5 defines the following integration path:

Ethereum burn event
-> Ethereum watcher observation
-> finality classification
-> canonical event key derivation
-> gateway mint payload v2 construction
-> guardian signing
-> quorum package assembly
-> relayer submission
-> X1 handler execution
-> receipt/outcome observation

The handler side was already proven by B2/B3.

B5 focuses on the off-chain preparation and submission boundary.

## Component boundaries

### 1. Ethereum watcher

The watcher observes Ethereum burn events and extracts the source event fields needed by the gateway model.

Responsibilities:

- connect to a configured Ethereum RPC endpoint,
- scan finalized or sufficiently confirmed blocks,
- find the expected XNTD burn event,
- extract source chain id,
- source token,
- source sender,
- burn transaction hash,
- burn event index,
- source block number,
- source block hash,
- source nonce if applicable,
- burned amount.

Non-responsibilities:

- the watcher does not authorize minting,
- the watcher does not mint on X1,
- the watcher does not decide guardian quorum.

### 2. Finality classifier

The finality classifier decides whether a burn event is eligible to become a gateway mint candidate.

Responsibilities:

- enforce the selected Ethereum finality rule,
- reject non-final or unstable observations,
- preserve source block number and block hash,
- prevent accidental candidate creation from reorg-prone observations.

Open policy question:

- exact production rule may be finalized, safe, or N-confirmations depending on available RPC support.

### 3. Canonical event builder

The canonical event builder converts a finalized Ethereum burn observation into deterministic gateway event data.

Responsibilities:

- build canonicalEventKey deterministically,
- normalize numeric fields,
- preserve source event identity,
- reject ambiguous event data,
- reject unsupported route/source token/source chain combinations.

Important invariant:

The same finalized burn event must always produce the same canonicalEventKey.

Different burn events must not collide under canonicalEventKey.

### 4. Payload v2 builder

The payload v2 builder constructs the exact authorization payload that guardians sign.

Responsibilities:

- include route_id,
- include processed_event identity,
- include target X1 mint,
- include recipient token account,
- include mint amount,
- include guardian_set_id,
- compute the B1C payload v2 hash.

Important invariant:

Guardians sign the exact payload hash that the X1 handler recomputes from current instruction/account inputs.

If recipient, mint, amount, processed_event, route_id, or guardian_set_id drift, B3 proves the handler rejects before mutation.

### 5. Guardian signing boundary

Guardians sign eligible payload hashes.

Responsibilities:

- verify candidate eligibility,
- verify payload hash construction,
- sign only supported route and guardian_set_id,
- return signature evidence in the expected Ed25519 instruction format or agreed production equivalent,
- avoid signing duplicate or malformed candidates.

Non-responsibilities:

- guardians do not submit the transaction unless separately configured,
- guardians do not mutate X1 state directly,
- guardians do not override the handler authorization boundary.

### 6. Quorum package assembler

The assembler collects guardian evidence and forms the submission package.

Responsibilities:

- require unique guardian keys,
- reject unknown guardians,
- enforce threshold,
- order or package evidence deterministically where needed,
- preserve the payload hash,
- prepare prior Ed25519 evidence instructions or the production equivalent evidence format.

B3 proves duplicate, unknown, and insufficient guardian evidence rejects before mutation.

### 7. Relayer

The relayer submits the prepared package to X1.

Responsibilities:

- build the X1 transaction,
- include prior evidence instructions,
- include the ConsumeGatewayMint instruction,
- include required accounts in the expected order,
- submit to X1 RPC,
- record transaction result,
- retry safely if needed.

Non-responsibilities:

- the relayer does not authorize minting,
- the relayer does not change payload contents after guardian signing,
- the relayer cannot bypass handler checks.

### 8. Outcome observer

The observer records what happened after submission.

Responsibilities:

- detect success,
- detect replay rejection,
- detect payload mismatch,
- detect quorum failure,
- detect account contract failure,
- detect stuck or dropped transactions,
- preserve logs for operational debugging.

## B5 invariants

B5 must preserve the following invariants:

1. Watcher observations are deterministic.
2. Finality is enforced before candidate creation.
3. canonicalEventKey is deterministic and replay-safe.
4. Payload v2 hash is deterministic.
5. Guardian signatures are bound to payload v2 hash.
6. Relayer cannot change signed payload semantics.
7. Handler remains the final authority.
8. Replay attempts are idempotent and safe.
9. Failure cases are observable.
10. No B5 component removes B4 activation gates.

## B5 non-goals

B5 does not:

- remove handler feature gates,
- produce a production deployment,
- introduce production guardian keys,
- store or print private keys,
- bypass Ed25519 evidence validation,
- bypass payload binding,
- bypass processed_event replay protection,
- replace B2/B3 handler proofs,
- perform X1 testnet deployment.

## B5 expected artifacts

B5 may introduce:

- watcher/relayer architecture notes,
- candidate data schema,
- guardian signature package schema,
- relayer submission package schema,
- deterministic conversion tests,
- negative integration tests around malformed watcher/relayer outputs,
- script boundaries that avoid secrets.

B5 should prefer deterministic pure conversion tests before live RPC tests.

## B5 closure requirements

B5 is closed when:

- watcher boundary is documented,
- relayer boundary is documented,
- candidate schema is documented or implemented,
- guardian quorum package schema is documented or implemented,
- deterministic watcher-to-payload conversion is tested,
- malformed package cases are rejected before handler submission or by the already proven handler boundary,
- full xxxl-svm lib tests remain green,
- B6 entry criteria are clear.

## B6 entry criteria

B6 may start only after B5 defines enough off-chain integration structure to run a testnet rehearsal.

B6 target:

Ethereum burn -> watcher observation -> guardian quorum -> relayer submission -> X1 testnet mint

B6 must include success and failure/replay cases.

## Updated checkpoint list

✅ C: merge checkpoint

✅ D: negative/failure mode tests

✅ B1: guardian quorum authorization

✅ B2: valid quorum live-gated success test

✅ B3: hostile live-gated matrix

✅ B4: activation gate decision / production-readiness boundary

👉 B5: watcher/relayer integration path

⏭ B6: X1 testnet deploy + end-to-end Ethereum burn -> X1 mint

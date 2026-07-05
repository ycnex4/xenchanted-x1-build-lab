# Phase 41K.6 B4 — Activation gate decision / production-readiness boundary

## Purpose

Phase 41K.6 B4 defines the boundary between the proven live-gated handler path and a future production activation path.

B2 proved the positive live-gated path:

valid prior Ed25519 evidence -> payload v2 binding -> guardian membership -> unique quorum -> B1C7 authorization -> processed_event mark -> SPL Token MintTo -> success

B3 proved the hostile live-gated perimeter around that path:

hostile evidence/account drift -> reject before mutation -> no processed_event mark -> no SPL Token MintTo -> no recipient token balance increase

B4 does not add new mint mechanics.

B4 records the activation decision: the current path must remain gated until the remaining production-readiness questions are explicitly resolved.

## Current checkpoint

The following checkpoints are closed on main:

- C: merge checkpoint.
- D: negative/failure mode tests.
- B1: guardian quorum authorization.
- B2: valid quorum live-gated success test.
- B3: hostile live-gated matrix.

B3 post-merge validation passed:

- Full xxxl-svm lib: 610 passed; 0 failed; 1 ignored.
- B1C7 gated lib: 697 passed; 0 failed; 1 ignored.
- B2 live-gated valid quorum success: 1 passed; 0 failed.
- B3 hostile live-gated matrix: 8 passed; 0 failed.

Current main checkpoint:

031dc7c Merge phase 41K.6 B3 hostile live-gated matrix

## What is proven

The model and live-gated harness currently prove:

1. The handler can read strictly prior Ed25519 evidence from the instructions sysvar.
2. The handler can bind evidence to the expected B1C payload v2 hash.
3. The handler can validate guardian membership against an active guardian set account.
4. The handler can require unique guardian quorum.
5. The handler can fail before mutation on hostile evidence or account drift.
6. The handler can mark processed_event and execute SPL Token MintTo only after authorization.
7. Replay protection blocks already consumed processed_event.
8. Recipient, mint, and guardian_set_id are included in authorization binding.

## What remains test-only

The current B2/B3 live-gated path is not yet a production activation path.

It depends on explicit non-production feature gates:

- phase-41k5-d2-production-path-test-gate
- dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build
- phase-41k6-b1c7-handler-integration-test-gate
- dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build

The B2/B3 Mollusk harness also loads a no-op SBF ELF under the Ed25519 precompile program id.

That no-op stub is test-only. It exists because Mollusk transaction-instruction execution requires every prior instruction program id to exist in the program cache. Production authorization does not trust the stub result; the handler reads prior instruction bytes from the instructions sysvar and routes them through the B1C evidence, payload-binding, membership, and quorum pipeline before mutation.

## Activation decision

B4 decision:

Do not remove the current dangerous gates yet.

Do not expose the B2/B3 path as an ungated production handler yet.

The current path is considered:

proved in live-gated harness, not yet production-activated

The correct next step is to document and implement the production-readiness boundary before any ungated activation.

## Production-readiness blockers

Before removing gates, the project must resolve:

### 1. Runtime Ed25519 evidence model

Decide how production X1 runtime will treat prior Ed25519 verification evidence.

Open questions:

- Does the target X1 runtime support the same Ed25519 precompile instruction semantics?
- Does the instructions sysvar expose prior instruction data in the same format?
- Are signature verification results enforced by runtime, or must the program independently parse and verify signature data?
- If X1 differs from Solana semantics, what adapter or verifier boundary is required?

### 2. Deployment account contract

Confirm the exact production account contract:

- required accounts,
- mutability,
- signer requirements,
- PDA seeds,
- guardian set account layout,
- processed_event PDA model,
- mint authority PDA model,
- SPL Token program identity on X1,
- rent/exemption semantics.

### 3. Guardian set lifecycle

Define production rules for:

- guardian set creation,
- active/inactive status,
- threshold changes,
- guardian rotation,
- replay protection across guardian_set_id changes,
- emergency invalidation policy.

### 4. Watcher/relayer boundary

Define the off-chain path that produces valid gateway mint attempts:

- Ethereum burn observation,
- Ethereum finality rule,
- canonical event key derivation,
- payload v2 construction,
- guardian signing,
- relayer submission,
- retry policy,
- failure observability.

### 5. Operational safety

Define production monitoring for:

- replay attempts,
- insufficient quorum attempts,
- unknown guardian attempts,
- payload mismatch attempts,
- successful mints,
- mark-without-mint impossibility,
- mint-without-mark impossibility,
- stuck relays.

### 6. Final end-to-end deployment rehearsal

Before production activation, run an end-to-end testnet rehearsal:

Ethereum burn -> watcher observation -> guardian quorum -> relayer submission -> X1 mint

The rehearsal must include both success and hostile/failure cases.

## Activation options considered

### Option A — Remove gates now

Rejected.

Reason:

B2/B3 are strong live-gated proofs, but production runtime assumptions are not fully recorded yet.

### Option B — Keep gates and move to watcher/relayer integration

Accepted.

Reason:

The handler boundary is now strong enough to design the off-chain integration path without exposing an ungated runtime path.

### Option C — Add more handler hostile tests before watcher/relayer

Deferred.

Reason:

B3 already covers the critical hostile authorization perimeter. Additional tests may be useful later, but they are not the next blocker.

## B4 conclusion

B4 keeps the current handler path gated.

The next phase should move toward watcher/relayer integration while preserving the production-readiness boundary.

Recommended next checkpoint:

B5: watcher/relayer integration path

B5 should not remove gates. It should define and test the off-chain flow that prepares valid gateway mint submissions for the already proven handler boundary.

## Updated checkpoint list

✅ C: merge checkpoint

✅ D: negative/failure mode tests

✅ B1: guardian quorum authorization

✅ B2: valid quorum live-gated success test

✅ B3: hostile live-gated matrix

👉 B4: activation gate decision / production-readiness boundary

⏭ B5: watcher/relayer integration path

⏭ B6: X1 testnet deploy + end-to-end Ethereum burn -> X1 mint


## B4 closure

B4 closure validation passed on the branch stage-41k6-b4-activation-gate-boundary.

Validated gate:

- Full xxxl-svm lib test suite: 610 passed; 0 failed; 1 ignored.

B4 closure decision:

- The B2/B3 handler path remains gated.
- The dangerous SBF-build allow features remain non-production.
- The Mollusk Ed25519 no-op harness stub remains test-only.
- No ungated production handler activation happens in B4.
- The next checkpoint is B5: watcher/relayer integration path.

B4 is closed as an activation gate decision and production-readiness boundary checkpoint.

## B4 closure requirements

B4 is closed when:

- the activation decision is recorded,
- the current gated/test-only pieces are explicitly listed,
- production-readiness blockers are documented,
- B5 entry criteria are clear,
- full xxxl-svm lib tests remain green.

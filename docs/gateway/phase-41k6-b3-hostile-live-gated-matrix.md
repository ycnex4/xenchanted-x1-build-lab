# Phase 41K.6 B3 — Hostile live-gated matrix

## Purpose

B2 closed the positive valid-quorum live-gated path:

`valid prior Ed25519 evidence -> payload v2 binding -> guardian membership -> unique quorum -> B1C7 authorization -> processed_event mark -> SPL Token MintTo -> success`

B3 closes the hostile live-gated matrix around that same path.

The goal is not to re-prove the happy path. The goal is to prove that hostile changes to the evidence, payload binding, guardian membership, quorum, replay status, recipient, mint, or account contract fail before mutation.

## Core invariant

Every B3 hostile case must preserve the B1/B2 safety boundary:

`authorization failure -> no processed_event mark -> no SPL MintTo -> no recipient token balance increase`

No hostile case may reach the first mutation.

## B3 test shape

B3 should reuse the B2 live-gated harness as the baseline fixture, including:

- Mollusk transaction-instruction execution.
- Two strictly prior Ed25519 evidence instructions.
- B2 no-op SBF ELF harness stub loaded under the Ed25519 precompile program id.
- B1C7 handler integration feature gate.
- D2 production-path CPI feature gate.
- SPL Token `MintTo` only reachable after B1C7 authorization.

The B2 no-op Ed25519 harness stub is test-only. It exists because Mollusk transaction-instruction execution requires every prior instruction program id to exist in the cache. Production authorization does not trust the no-op result; the current handler reads prior instruction bytes from the instructions sysvar and routes them through the B1C evidence, payload-binding, membership, and quorum pipeline before mutation.

## Hostile matrix

### B3.1 — Wrong payload hash evidence

Mutation:

- Keep guardian signers valid.
- Keep guardian set valid.
- Change the prior Ed25519 message bytes so they no longer equal the expected B1C payload v2 hash.

Expected result:

- B1C payload binding rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.2 — Unknown guardian evidence

Mutation:

- Keep the payload hash correct.
- Use an Ed25519 public key that is not present in the active guardian set.

Expected result:

- Guardian membership rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.3 — Duplicate guardian evidence

Mutation:

- Keep the payload hash correct.
- Use the same valid guardian twice.

Expected result:

- Unique quorum rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.4 — Insufficient quorum

Mutation:

- Keep the payload hash correct.
- Provide fewer unique valid guardians than the guardian set threshold.

Expected result:

- Quorum rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.5 — Processed event replay

Mutation:

- Keep authorization evidence valid.
- Start with processed_event already marked/consumed.

Expected result:

- Replay/status boundary rejects before a second mark/mint.
- No additional SPL MintTo.

### B3.6 — Recipient binding mismatch

Mutation:

- Keep prior evidence generated for one recipient token account.
- Submit current instruction/accounts with a different recipient token account.

Expected result:

- Payload binding rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.7 — Mint binding mismatch

Mutation:

- Keep prior evidence generated for one SPL mint.
- Submit current instruction/accounts with a different SPL mint.

Expected result:

- Payload binding or CPI preparation rejects.
- No processed_event mark.
- No SPL MintTo.

### B3.8 — Guardian set id mismatch

Mutation:

- Keep prior evidence bound to one guardian_set_id.
- Submit current instruction or guardian set account with a different guardian_set_id.

Expected result:

- Payload binding, guardian set loading, or account-contract boundary rejects.
- No processed_event mark.
- No SPL MintTo.

## Closure requirements

B3 is closed only when the hostile live-gated test matrix proves:

- Each hostile case fails deterministically.
- Failure occurs before processed_event mutation.
- Failure occurs before SPL Token `MintTo`.
- Recipient token balance remains unchanged.
- The positive B2 live-gated success test remains green.
- Full `xxxl-svm` lib tests remain green.
- B1C7 gated lib tests remain green.

## Non-goals

B3 does not introduce production ungated execution.

B3 does not remove the existing dangerous feature gates.

B3 does not replace B2. B2 remains the positive proof. B3 is the negative hostile matrix around the B2 path.

## B3.1/B3.2 implementation note

The first hostile live-gated test file is derived from the B2 positive harness and keeps the same transaction-instruction execution model:

- two strictly prior Ed25519 evidence instructions,
- current `ConsumeGatewayMint` instruction,
- SPL Token program loaded for CPI,
- no-op SBF harness stub loaded under the Ed25519 precompile id for Mollusk cache execution.

The initial hostile cases cover:

- B3.1 wrong payload hash evidence,
- B3.2 unknown guardian evidence.

Both cases must return `InvalidInstruction` and preserve all mutation targets unchanged: processed_event, SPL mint, recipient token account, recipient balance, and rent payer.

## B3.3/B3.4 implementation note

The second hostile live-gated test block extends the B3 matrix from payload and membership failures into quorum failures:

- B3.3 duplicate guardian evidence: the same valid guardian appears twice and must not count as a unique quorum.
- B3.4 insufficient quorum: only one unique valid guardian is supplied while the active guardian set threshold is two.

Both cases must return `InvalidInstruction` and preserve all mutation targets unchanged.

## B3.5 implementation note

B3.5 adds the replay boundary case.

The evidence is otherwise valid: payload hash is correct, both prior Ed25519 evidence instructions are from known guardians, and the unique guardian threshold is met. The hostile mutation is that `processed_event` starts in an already consumed state.

Expected result:

- the handler rejects the replay,
- no second processed_event mark occurs,
- no SPL Token `MintTo` occurs,
- recipient token account, SPL mint, recipient balance, and rent payer remain unchanged.

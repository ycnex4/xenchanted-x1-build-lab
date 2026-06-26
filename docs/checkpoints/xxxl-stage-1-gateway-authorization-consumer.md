# XXXL Stage 1 Gateway Authorization Consumer Checkpoint

Stage XXXL Program v1 now has a Stage 1 gateway authorization consumer layer.

New files:

- `src/xxxl/stage-1-gateway-consumer.ts`
- `tests/xxxl/stage-1-gateway-consumer.test.ts`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`

Main decision:

- XXXL does not accept an unrelated local mint object as its canonical gateway entry.
- XXXL consumes a successful Stage 1 mint authorization result.
- Stage 1 remains responsible for gateway verification, guardian quorum, and source replay protection.
- XXXL adds its own local consumed-event mark and supply update.
- The Genesis Phase invariant is strengthened to Stage 1 authorized gateway mints consumed exactly once.

Invariant:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

Runtime requirement:

- future X1 runtime must make Stage 1 authorization, XXXL mint, and replay mark atomic

Status:

- deterministic consumer model added
- tests added for valid authorization, supply amount, invalid quorum, Stage 1 replay, and local XXXL replay
- no production runtime code
- no RPC usage
- no secrets required

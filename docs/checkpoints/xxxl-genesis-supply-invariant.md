# XXXL Genesis Supply Invariant Checkpoint

Stage XXXL Program v1 now has a dedicated Genesis supply invariant hardening layer.

New files:

- `src/xxxl/genesis-supply-invariant.ts`
- `tests/xxxl/genesis-supply-invariant.test.ts`
- `docs/xxxl/xxxl-genesis-supply-invariant.md`

Main invariant:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

Main decisions:

- accepted gateway mint must increase supply by exactly the accepted amount
- accepted gateway mint must add exactly one consumed canonical event key
- rejected transition must not mutate supply
- rejected transition must not mutate replay state
- unauthorized direct supply increase is invalid
- manual mint remains forbidden
- Build state is not a supply source

Status:

- deterministic invariant helpers added
- tests added for accepted gateway delta, sum of accepted mints, unauthorized supply increase, rejected transition preservation, rejected supply mutation, and manual mint rejection
- no production runtime code
- no RPC usage
- no secrets required

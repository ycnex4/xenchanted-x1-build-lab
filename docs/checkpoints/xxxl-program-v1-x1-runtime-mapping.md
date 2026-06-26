# XXXL Program v1 X1 Runtime Mapping Checkpoint

Stage XXXL Program v1 now has an X1 runtime mapping document.

Authoritative design document:

- `docs/xxxl/xxxl-program-v1-x1-runtime-mapping.md`

Main mapping:

- XXXL mint state
- gateway configuration state
- guardian set state
- processed event state
- recipient balance state
- canonical consume_gateway_mint instruction

Runtime invariants:

- gateway-only Genesis Phase
- no manual mint
- no premine
- no founder allocation
- no hidden emission
- no Build dependency for gateway mint
- Stage 1 authorization required before XXXL mint
- canonical event key consumed exactly once
- supply increases only by authorized amount
- rejected transitions do not mutate supply or replay state

Atomicity requirement:

    success = balance update + supply update + consumed event mark
    failure = no balance update + no supply update + no consumed event mark

Upgradeability boundary:

- temporary upgradeability is allowed only for staged protocol finalization
- upgrade authority must not create supply rights
- upgrade authority must not bypass gateway authorization
- upgrade authority must not rewrite balances
- upgrade authority must not clear processed events
- final protocol goal remains freeze / removal of upgrade authority after X1-native emission mechanics are complete

Status:

- docs-only runtime mapping
- no production runtime code
- no RPC usage
- no secrets required

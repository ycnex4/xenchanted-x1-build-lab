# XXXL Multichain Low-Weight Route Policy Checkpoint

Stage XXXL Program v1 now has a multichain low-weight route policy.

New files:

- `src/xxxl/multichain-low-weight-route-policy.ts`
- `tests/xxxl/multichain-low-weight-route-policy.test.ts`
- `docs/xxxl/xxxl-multichain-low-weight-route-policy.md`

Core principle:

    Ethereum route is the primary full-weight route.
    Non-Ethereum routes are low-weight historical access routes.
    Their purpose is inclusion, not equal supply power.

Route-weighted mint formula:

    xxxlMintAmount = burnedSourceAmount * sourceChainWeightBps / 10000

Ethereum route:

- 10000 bps
- full weight
- primary economic anchor

Avalanche route:

- hard max: 25 bps
- conservative initial candidate: 5-10 bps
- 100 bps not allowed under current market conditions
- 500 bps not allowed under current market conditions

Other non-Ethereum routes:

- must be <= configured Avalanche route weight
- require explicit route policy approval
- require route caps
- should default to candidate/inactive until separately approved

Required non-Ethereum caps:

- per-event mint cap
- daily route mint cap
- epoch route mint cap
- global non-Ethereum supply share cap

Updated Genesis invariant:

    XXXL total supply = sum(consumed gateway mint amounts across all approved routes)

Status:

- policy only
- does not activate Avalanche route
- does not deploy XC on Avalanche
- does not define final source token addresses
- does not change current Ethereum primary route
- prevents runtime skeleton from becoming Ethereum-only

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 78 files / 563 tests passing
- Build: passing

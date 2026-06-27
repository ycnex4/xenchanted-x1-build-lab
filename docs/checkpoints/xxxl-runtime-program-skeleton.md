# XXXL Runtime Program Skeleton Checkpoint

Stage XXXL Program v1 now has a route-aware runtime program skeleton.

New files:

- `src/xxxl/runtime-program-skeleton.ts`
- `tests/xxxl/runtime-program-skeleton.test.ts`
- `docs/xxxl/xxxl-runtime-program-skeleton.md`

Core modeled instruction:

    CONSUME_GATEWAY_MINT

Runtime skeleton path:

1. load accounts
2. validate instruction serialization boundary
3. validate optional route policy
4. consume Stage 1 authorization result
5. simulate SPL Token `mint_to` CPI boundary
6. mark processed event
7. update mint state mirror
8. update recipient balance mirror
9. audit Genesis supply invariant

Key boundary decisions:

- runtime consumes Stage 1 authorization result
- runtime does not verify guardian signatures
- SPL Token `mint_to` CPI is atomic with parent transaction
- mint authority PDA is CPI signer
- routeId remains runtime input
- runtime skeleton does not hardcode Ethereum-only assumptions
- non-Ethereum routes require explicit low-weight route policy
- failed preflight/transition preserves original accounts

Route-aware status:

- Ethereum remains the primary full-weight route
- Avalanche can be represented as a low-weight policy route
- this stage does not activate Avalanche route
- initial deployment may still be Ethereum-only

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 79 files / 576 tests passing
- Build: passing

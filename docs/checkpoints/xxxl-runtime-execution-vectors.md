# XXXL Runtime Execution Vectors Checkpoint

Stage XXXL Program v1 now has deterministic runtime execution vectors.

New files:

- `src/xxxl/runtime-execution-vectors.ts`
- `tests/xxxl/runtime-execution-vectors.test.ts`
- `docs/xxxl/xxxl-runtime-execution-vectors.md`

Covered vectors:

- valid Ethereum primary full-weight gateway mint execution
- valid Avalanche low-weight route-aware execution
- invalid route policy rejection
- missing route rejection
- Stage 1 authorization rejection
- replay rejection
- event key mismatch rejection
- instruction serialization boundary rejection

Key properties:

- runtime remains route-aware
- Ethereum is not hardcoded as the only possible route
- non-Ethereum routes require explicit low-weight route policy
- runtime consumes Stage 1 authorization result only
- runtime does not verify guardian signatures
- successful vectors model SPL Token `mint_to` CPI
- failed vectors skip CPI
- rejected vectors preserve account state
- vector canonical JSON serializes bigint values as decimal strings

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 80 files / 588 tests passing
- Build: passing

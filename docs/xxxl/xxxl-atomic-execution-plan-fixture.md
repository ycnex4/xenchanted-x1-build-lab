# XXXL Atomic Execution Plan Fixture

Status: ATOMIC_EXECUTION_PLAN_FIXTURE_ONLY_NOT_LIVE_ROUTE.

This stage adds an atomic execution-plan fixture for the XXXL X1/SVM runtime path.

## Purpose

Previous stages added:

- instruction/account decode
- real PDA derivation
- SPL Token mint_to CPI boundary
- handler integration preparation
- processed-event and recipient-balance mutation helpers

This stage fixes the order in which those pieces must be combined.

## Fixed order

1. validate and prepare CPI boundary
2. mark processed event consumed
3. credit recipient balance
4. keep live route disabled

## Atomic prechecks

Before applying state mutations, the fixture checks:

- replay / already consumed processed event
- canonical event key match
- route match
- recipient match
- recipient balance owner match
- recipient balance mint match
- recipient balance overflow
- prepared CPI amount mismatch

The balance overflow precheck happens before the processed event is marked consumed, so the fixture avoids partial local state mutation in the unit path. On-chain transaction semantics still provide transaction-level atomic rollback, but the model avoids relying only on that.

## Non-goals

This stage does not add:

- live route activation
- mint_to invocation from process_instruction
- process_instruction processed-event mutation
- process_instruction recipient-balance mutation
- deployment
- authority freeze execution

## Next likely stage

The next likely stage is guarded live-handler wiring model: all pieces are present, but route activation remains explicitly disabled until deployment, authority, and freeze constraints are satisfied.

# XXXL X1 Testnet Blocker Model Transition Checkpoint

Status: Completed
Branch: `stage-xxxl-x1-testnet-blocker-model-transition`
Base: `cf77671 Assess X1 testnet blocker transition after deployment`

## Summary

The blocker model was transitioned from the old blanket `PLACEHOLDER_PROGRAM_ID` model to a network-aware model.

## Retired X1 Testnet Blanket Blocker

Retired for the X1 testnet path:

- `PLACEHOLDER_PROGRAM_ID`

## New X1 Testnet Status

Current X1 testnet status:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

X1 testnet Program ID:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

ProgramData:

- `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`

Authority:

- `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`

## New Production Blocker

Current production Program ID blocker:

- `PRODUCTION_PROGRAM_ID_UNSET`

## Active Blockers After Transition

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Safety

No runtime code changed.

No transaction was submitted.

No SOL was spent.

No upgrade was executed.

No live gateway route was enabled.

No SPL CPI, `invoke_signed`, or SPL Token `mint_to` path was enabled.

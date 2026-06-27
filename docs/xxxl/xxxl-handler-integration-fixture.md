# XXXL Handler Integration Fixture

Status: HANDLER_INTEGRATION_FIXTURE_PREPARED_NOT_LIVE_ROUTE.

This stage connects the Rust/SVM handler inputs to validation and CPI preparation without activating live route execution.

## Purpose

Previous stages added:

- production byte layout
- real PDA derivation
- account/instruction decode
- SPL Token mint_to CPI fixture

This stage connects those pieces into a preparation path:

- decode consume_gateway_mint instruction
- load canonical account indexes
- parse runtime account views
- run owner and rent checks
- validate SPL Mint and recipient token account
- verify gateway mint authority PDA and bump
- prepare the mint_to CPI boundary

## Important boundary

`process_instruction` remains scaffold-only. It does not call live `mint_to_cpi_boundary`.

This means the preparation path can be tested without activating the route or mutating supply.

## Non-goals

This stage does not add:

- live mint_to invocation from handler
- route activation
- processed-event mutation
- recipient-balance mutation
- deployment
- authority freeze execution

## Next likely stage

The next likely stage is a processed-event and recipient-balance mutation fixture, still without uncontrolled route activation.

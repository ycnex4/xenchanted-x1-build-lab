# XXXL Guarded Live Handler Wiring Fixture

Status: COMPLETED.

This stage adds a guarded live-handler wiring fixture for the XXXL SVM runtime scaffold.

It does not activate the live gateway route.

## Goal

Prepare a safe internal path from validated consume_gateway_mint accounts into an atomic execution plan, while preserving the current scaffold-only process_instruction behavior.

The stage proves that the runtime can:

- parse consume_gateway_mint
- validate the account boundary
- prepare the SPL mint_to CPI boundary
- build the atomic consume_gateway_mint execution plan
- keep live route activation disabled
- keep process_instruction non-live by default

## Non-goals

This stage does not:

- activate live gateway minting
- invoke SPL mint_to from process_instruction
- enable route activation
- introduce manual mint
- introduce hidden emission
- introduce founder allocation
- move guardian signature verification into runtime
- change the finalization/freeze policy

## Added guard

A compile-time policy flag was added:

    LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED = false

The process_instruction path remains scaffold-only.

If this flag were ever changed to true without a proper live implementation, the handler returns CpiBoundaryNotReady instead of silently minting.

## Added fixture function

A guarded fixture function was added:

    build_guarded_consume_gateway_mint_live_handler_fixture

This function:

- receives program_id, accounts, args, rent, and consumed_slot
- calls prepare_consume_gateway_mint_cpi_boundary
- calls build_atomic_consume_gateway_mint_execution_plan
- rejects any plan that claims live route activation is enabled
- rejects any plan that claims mint_to invocation from process_instruction is enabled
- returns a disabled execution plan for testing and staged wiring only

## Execution plan policy

The execution plan remains disabled:

- live_route_activation_enabled = false
- mint_to_invocation_from_process_instruction_enabled = false

This preserves the previously documented account-constraint checklist and no-live-route policy.

## Tests added

Two tests were added.

First test:

- builds a guarded live-handler fixture plan after validation
- confirms canonical_event_key, route_id, recipient, mint, amount, consumed_slot, and source_chain_weight_bps
- confirms live_route_activation_enabled is false
- confirms mint_to_invocation_from_process_instruction_enabled is false

Second test:

- corrupts the processed_event boundary
- confirms the guarded fixture rejects invalid boundary state before producing a plan

The existing process_instruction scaffold-only test remains.

## Verification

Hard checks passed:

- cargo fmt --check
- cargo test
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

Observed test result:

- 65 passed
- 0 failed

## Security result

This stage adds internal wiring coverage without enabling mint execution.

The live route remains disabled by default.

No runtime mint path is activated from process_instruction.

No user-facing emission behavior changes in this stage.

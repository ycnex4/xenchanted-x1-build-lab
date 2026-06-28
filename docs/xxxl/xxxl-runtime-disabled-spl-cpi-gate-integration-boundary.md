# XXXL Runtime Disabled SPL CPI Gate Integration Boundary

Status: COMPLETED.

This stage integrates the guarded SPL CPI execution gate into a runtime boundary while keeping it disabled.

## Goal

Compose the runtime consume-gateway-mint path up to the disabled SPL CPI execution gate:

    runtime validation
    -> execution plan
    -> CPI planning
    -> guarded SPL CPI execution gate
    -> stop

This stage proves that the runtime can reach the guarded CPI gate and fail closed without mutating local state or SPL token state.

## What changed

A new runtime boundary was added:

    build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary

It accepts:

- program id
- account list
- decoded `ConsumeGatewayMintArgs`
- rent
- consumed slot

It performs:

1. guarded account validation
2. CPI boundary preparation
3. runtime planning composition
4. guarded SPL CPI execution gate call

Because the SPL CPI execution gate is disabled, the boundary returns:

    CpiBoundaryNotReady

before real SPL CPI.

## Safety boundary

This stage does not activate live route execution.

This stage does not call `invoke_signed`.

This stage does not invoke SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient SPL token balance.

This stage does not mutate processed-event state when rejected at the disabled CPI gate.

This stage does not mutate recipient-balance state when rejected at the disabled CPI gate.

This stage does not connect CPI execution to `process_instruction`.

## Tests added

The new tests prove:

- valid runtime path reaches disabled CPI gate and returns `CpiBoundaryNotReady`
- disabled CPI gate rejection leaves processed-event unchanged
- disabled CPI gate rejection leaves recipient-balance unchanged
- disabled CPI gate rejection leaves SPL mint unchanged
- disabled CPI gate rejection leaves recipient token account unchanged
- consumed event is rejected before gate without mutation
- wrong recipient token account is rejected without mutation
- zero amount is rejected without mutation

## Verification

Hard checks passed:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Decision

The runtime disabled SPL CPI gate integration boundary is accepted.

The real SPL CPI path remains disabled.

The live `process_instruction` path remains non-live.
